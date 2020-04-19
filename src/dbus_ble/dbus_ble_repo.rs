use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use dbus::arg::{RefArg, Variant};
use dbus::blocking::{Connection, SyncConnection};
use dbus::blocking::stdintf::org_freedesktop_dbus::PropertiesPropertiesChanged;
use dbus::message::{MatchRule, MessageType, Message};
use dbus::strings::{Interface, Path, Member};

use crate::bluez_dbus::{OrgBluezAdapter1, OrgBluezDevice1, OrgFreedesktopDBusObjectManagerInterfacesAdded, OrgFreedesktopDBusObjectManager};

const BLUEZ_DBUS_DESTINATION: &str = "org.bluez";
const BLUEZ_DBUS_DEVICE_INTERFACE: &str = "org.bluez.Device1";
const DBUS_CONNECTION_TIMEOUT_MS: u64 = 5000;

static STANDARD_TIMEOUT: Duration = Duration::from_millis(DBUS_CONNECTION_TIMEOUT_MS);

pub struct BleDevice {
    path: String,
    local_name: String,
    service_data: HashMap<String, Vec<u8>>
}

impl BleDevice {
    /// Constructs a new ble device abstraction.
    ///
    /// # Arguments:
    /// * `input_interface` - The input dictionary that match the org.bluez.Device1 interface
    /// * `input_message` - The input raw message
    ///
    /// Returns a high level representation of a ble device.
    pub fn new(input_interface: &HashMap<String, Variant<Box<dyn RefArg>>>, input_message: &Message) -> BleDevice {
        let mut local_name = String::from("<unknown>");
        if input_interface.contains_key("Alias") {
            match input_interface["Alias"].as_str() {
                None => (),
                Some(name) => local_name = String::from(name)
            }
        }

        let path = input_message.path().unwrap().to_string();

        let service_data = Self::parse_service_data(&input_interface);

        BleDevice {
            path,
            local_name,
            service_data
        }
    }

    /// Update service data with new values.
    ///
    /// # Arguments:
    /// * `update_data` - The updated input raw data.
    ///
    fn update_service_data(&mut self, update_data: &HashMap<String, Variant<Box<dyn RefArg>>>) {
        self.service_data = Self::parse_service_data(update_data);
    }

    /// Parse service data.
    ///
    /// # Arguments:
    /// * `input` - The input raw data.
    ///
    /// Returns a rust representation of data.
    ///
    fn parse_service_data(input: &HashMap<String, Variant<Box<dyn RefArg>>>) -> HashMap<String, Vec<u8>> {
        let mut output_data : HashMap<String, Vec<u8>> = HashMap::new();
        if input.contains_key("ServiceData") {
            let service_data = &input["ServiceData"].0;
            let mut service_data_iter = service_data.as_iter().unwrap();

            while let Some(key) = service_data_iter.next() {
                key.as_str().unwrap();

                let mut raw_data : Vec<u8> = Vec::new();
                let value = service_data_iter.next().unwrap();
                let inner_value = value.as_iter().unwrap().next().unwrap();
                for b in inner_value.as_iter().unwrap() {
                    match b.as_u64() {
                        None => (),
                        Some(b) => raw_data.push(b as u8)
                    }
                }

                output_data.insert(String::from(key.as_str().unwrap()), raw_data);
            }

        }
        output_data
    }
}

pub struct DbusBleRepo {
    dbus_connection: Connection,
    found_devices : Arc<Mutex<Vec<BleDevice>>>
}

impl DbusBleRepo {
    /// Return a new instance of a Dbus ble repo.
    pub fn new() -> DbusBleRepo {

        let connection = Connection::new_system().expect("Error getting dbus connection");

        let dbus_ble_repo = DbusBleRepo {
            dbus_connection: connection,
            found_devices: Arc::new(Mutex::new(Vec::new()))
        };

        dbus_ble_repo.add_interface_added_match_rule();
        dbus_ble_repo.add_properties_changed_match_rule();

        let adapter_proxy = dbus_ble_repo.dbus_connection.with_proxy(BLUEZ_DBUS_DESTINATION, "/", STANDARD_TIMEOUT);
        let objects = adapter_proxy.get_managed_objects();

        dbus_ble_repo
    }

    pub fn start_scan(&self, timeout: u64) {
        let adapter_proxy = self.dbus_connection.with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT);

        adapter_proxy.start_discovery().expect("Error starting discovery");
    }

    pub fn stop_scan(&self) {
        let adapter_proxy = self.dbus_connection.with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT);

        adapter_proxy.stop_discovery().expect("Error stopping discovery");
    }

    fn add_interface_added_match_rule(&self) {
        let mut interface_added_match_rule = MatchRule::new();
        interface_added_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.ObjectManager").unwrap());
        interface_added_match_rule.msg_type = Option::Some(MessageType::Signal);
        interface_added_match_rule.member = Option::Some(Member::new("InterfacesAdded").unwrap());

        let on_interface_added = {
            let found_devices_clone = self.found_devices.clone();
            move | p: OrgFreedesktopDBusObjectManagerInterfacesAdded, c: &Connection, m: &Message| {
                // If this is a ble device which has been discovered
                if p.interfaces.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {
                    let ble_device = BleDevice::new(&p.interfaces[BLUEZ_DBUS_DEVICE_INTERFACE], m);
                    found_devices_clone.lock().unwrap().push(ble_device);
                }
                true
            }
        };

        self.dbus_connection.add_match(interface_added_match_rule, on_interface_added).unwrap();
    }

    fn add_properties_changed_match_rule(&self) {
        let mut properties_changed_match_rule = MatchRule::new();
        properties_changed_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.Properties").unwrap());
        properties_changed_match_rule.msg_type = Option::Some(MessageType::Signal);
        properties_changed_match_rule.member = Option::Some(Member::new("PropertiesChanged").unwrap());

        let on_properties_changed = {
            let found_devices_clone = self.found_devices.clone();
            move | p: PropertiesPropertiesChanged, _: &Connection, m: &Message | {
                if p.interface_name == BLUEZ_DBUS_DEVICE_INTERFACE {
                    let mut devices = found_devices_clone.lock().unwrap();

                    let path = m.path().unwrap().to_string();

                    if let Some(device) = devices.iter_mut().find(|d| d.path == path) {
                        device.update_service_data(&p.changed_properties);
                    }
                }
                true
            }
        };

        self.dbus_connection.add_match(properties_changed_match_rule, on_properties_changed).unwrap();
    }
}
