use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use dbus::arg::{RefArg, Variant};
use dbus::blocking::SyncConnection;
use dbus::blocking::stdintf::org_freedesktop_dbus::{PropertiesPropertiesChanged, ObjectManagerInterfacesAdded};
use dbus::channel::Token;
use dbus::message::{MatchRule, MessageType, Message};
use dbus::strings::{Interface, Member};

use crate::dbus_ble::bluez_dbus::{OrgBluezAdapter1, OrgFreedesktopDBusObjectManager};

const BLUEZ_DBUS_DESTINATION: &str = "org.bluez";
const BLUEZ_DBUS_DEVICE_INTERFACE: &str = "org.bluez.Device1";
const DBUS_CONNECTION_TIMEOUT_MS: u64 = 5000;

static STANDARD_TIMEOUT: Duration = Duration::from_millis(DBUS_CONNECTION_TIMEOUT_MS);

pub struct BleDevice {
    pub path: String,
    pub local_name: String,
    pub service_data: HashMap<String, Vec<u8>>
}

impl BleDevice {
    /// Constructs a new ble device abstraction.
    ///
    /// # Arguments:
    /// * `input_message` - The input raw message
    /// * `input_interface` - The input dictionary that match the org.bluez.Device1 interface
    ///
    /// Returns a high level representation of a ble device.
    pub fn new(device_path: String, input_interface: &HashMap<String, Variant<Box<dyn RefArg>>>) -> BleDevice {
        let mut local_name = String::from("<unknown>");
        if input_interface.contains_key("Alias") {
            match input_interface["Alias"].as_str() {
                None => (),
                Some(name) => local_name = String::from(name)
            }
        }

        let path = device_path;

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
    dbus_connection: Arc<Mutex<SyncConnection>>,
    found_devices : Arc<Mutex<Vec<BleDevice>>>,
    on_device_found: Option<fn(&BleDevice)>,
    interface_added_match_rule_token: Option<Token>,
    properties_changed_match_rule_token: Option<Token>,

}

impl DbusBleRepo {
    /// Return a new instance of a Dbus ble repo.
    pub fn new() -> DbusBleRepo {

        let connection = SyncConnection::new_system().expect("Error getting dbus connection");

        let mut dbus_ble_repo = DbusBleRepo {
            dbus_connection: Arc::new(Mutex::new(connection)),
            found_devices: Arc::new(Mutex::new(Vec::new())),
            on_device_found: None,
            interface_added_match_rule_token: None,
            properties_changed_match_rule_token: None,
        };

        dbus_ble_repo.add_interface_added_match_rule();
        dbus_ble_repo.add_properties_changed_match_rule();

        let managed_objects = dbus_ble_repo.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/", STANDARD_TIMEOUT)
            .get_managed_objects().unwrap();

        for (path, payload) in &managed_objects {
            if payload.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {
                let path = path.to_string();
                let ble_device = BleDevice::new(path, &payload[BLUEZ_DBUS_DEVICE_INTERFACE]);
                dbus_ble_repo.found_devices.lock().unwrap().push(ble_device);
            }
        }

        thread::spawn({
            let connection = dbus_ble_repo.dbus_connection.clone();
            println!("thread spawn connection: {}", connection.lock().unwrap().unique_name());
            move || {
            loop {
                connection.lock().unwrap().process(Duration::from_secs(1));
                thread::sleep(Duration::from_secs(1));
            }
        }});

        dbus_ble_repo
    }

    pub fn start_scan(&self) {
        let connection = self.dbus_connection.lock().unwrap();
        //self.dbus_connection.lock().unwrap()
        connection
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .start_discovery().expect("Error starting discovery");
    }

    pub fn stop_scan(&self) {
        self.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", STANDARD_TIMEOUT)
            .stop_discovery().expect("Error stopping discovery");
    }

    pub fn set_on_device_discovered_cb(&mut self, callback: Option<fn(&BleDevice)>) {
        self.on_device_found = callback;
        self.add_interface_added_match_rule();
        self.add_properties_changed_match_rule();
    }

    fn add_interface_added_match_rule(&mut self) {
        let mut interface_added_match_rule = MatchRule::new();
        interface_added_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.ObjectManager").unwrap());
        interface_added_match_rule.msg_type = Option::Some(MessageType::Signal);
        interface_added_match_rule.member = Option::Some(Member::new("InterfacesAdded").unwrap());

        let on_interface_added = {
            let on_device_found = self.on_device_found;
            let found_devices_clone = self.found_devices.clone();
            move | p: ObjectManagerInterfacesAdded, _: &SyncConnection, _: &Message| {
                // If this is a ble device which has been discovered
                if p.interfaces.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {
                    let mut devices = found_devices_clone.lock().unwrap();
                    let path = m.path().unwrap().to_string();

                    if let Some(device) = devices.iter_mut().find(|d| d.path == path) {
                        device.update_service_data(&p.interfaces[BLUEZ_DBUS_DEVICE_INTERFACE]);
                    } else {
                        let device = BleDevice::new(path, &p.interfaces[BLUEZ_DBUS_DEVICE_INTERFACE]);

                        match on_device_found {
                            None => (),
                            Some(on_device_found) => on_device_found(&device)
                        }

                        found_devices_clone.lock().unwrap().push(device);
                    }
                }
                true
            }
        };

        let dbus_connection = self.dbus_connection.lock().unwrap();

        match self.interface_added_match_rule_token {
            None => (),
            Some(token) => dbus_connection.remove_match(token).unwrap()
        }

        self.interface_added_match_rule_token = Some(dbus_connection
            .add_match(interface_added_match_rule, on_interface_added).unwrap()
        );
    }

    fn add_properties_changed_match_rule(&mut self) {
        let mut properties_changed_match_rule = MatchRule::new();
        properties_changed_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.Properties").unwrap());
        properties_changed_match_rule.msg_type = Option::Some(MessageType::Signal);
        properties_changed_match_rule.member = Option::Some(Member::new("PropertiesChanged").unwrap());

        let on_properties_changed = {
            let on_device_found = self.on_device_found;
            let found_devices_clone = self.found_devices.clone();
            move | p: PropertiesPropertiesChanged, _: &SyncConnection, m: &Message | {
                println!("{:#?}", p);
                if p.interface_name == BLUEZ_DBUS_DEVICE_INTERFACE {
                    let mut devices = found_devices_clone.lock().unwrap();

                    let path = m.path().unwrap().to_string();

                    if let Some(device) = devices.iter_mut().find(|d| d.path == path) {
                        device.update_service_data(&p.changed_properties);

                        match on_device_found {
                            None => (),
                            Some(on_device_found) => on_device_found(&device)
                        }
                    }
                }
                true
            }
        };

        let dbus_connection = self.dbus_connection.lock().unwrap();

        match self.properties_changed_match_rule_token {
            None => (),
            Some(token) => dbus_connection.remove_match(token).unwrap()
        }

        self.properties_changed_match_rule_token = Some(dbus_connection
            .add_match(properties_changed_match_rule, on_properties_changed).unwrap()
        );
    }
}
