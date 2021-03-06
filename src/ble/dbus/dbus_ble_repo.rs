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

use crate::ble::dbus::bluez_dbus::{OrgBluezAdapter1, OrgFreedesktopDBusObjectManager};

use crate::ble::api::BleDevice;

const BLUEZ_DBUS_DESTINATION: &str = "org.bluez";
const BLUEZ_DBUS_DEVICE_INTERFACE: &str = "org.bluez.Device1";
const DBUS_CONNECTION_TIMEOUT_MS: u64 = 5000;
const DBUS_CONNECTION_PROCESS_TIMEOUT_MS: u64 = 1000;
const DBUS_CONNECTION_PROCESS_PERIOD_MS: u64 = 50;

static DBUS_CONNECTION_TIMEOUT: Duration = Duration::from_millis(DBUS_CONNECTION_TIMEOUT_MS);
static DBUS_CONNECTION_PROCESS_TIMEOUT: Duration = Duration::from_millis(DBUS_CONNECTION_PROCESS_TIMEOUT_MS);
static DBUS_CONNECTION_PROCESS_PERIOD: Duration = Duration::from_millis(DBUS_CONNECTION_PROCESS_PERIOD_MS);

/// A ble repo using Dbus.
/// It allows to access bluetooth using bluez dbus api.
pub struct DbusBleRepo {
    /// The underlying dbus connection.
    dbus_connection: Arc<Mutex<SyncConnection>>,
    /// The list of cached found_devices.
    found_devices : Arc<Mutex<Vec<BleDevice>>>,
    /// The on device found callback.
    on_advertisement_data: Arc<Mutex<Box<dyn FnMut(&BleDevice) + Send + Sync + 'static>>>,
    /// The token to the interface added match rule. Allows to delete it when needed.
    interface_added_match_rule_token: Option<Token>,
    /// The token to the properties changed match rule. Allows to delete it when needed.
    properties_changed_match_rule_token: Option<Token>
}

impl DbusBleRepo {
    /// Return a new instance of a Dbus ble repo.
    pub fn new() -> DbusBleRepo {
        let connection = SyncConnection::new_system().expect("Error getting dbus connection");

        let mut dbus_ble_repo = DbusBleRepo {
            dbus_connection: Arc::new(Mutex::new(connection)),
            found_devices: Arc::new(Mutex::new(Vec::new())),
            on_advertisement_data: Arc::new(Mutex::new(Box::new(|_device| {}))),
            interface_added_match_rule_token: None,
            properties_changed_match_rule_token: None,
        };

        dbus_ble_repo.add_interface_added_match_rule();
        dbus_ble_repo.add_properties_changed_match_rule();

        let managed_objects = dbus_ble_repo.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/", DBUS_CONNECTION_TIMEOUT)
            .get_managed_objects().unwrap();

        for (path, payload) in &managed_objects {
            if payload.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {
                let path = path.to_string();
                let ble_device = Self::get_ble_device(path, &payload[BLUEZ_DBUS_DEVICE_INTERFACE]);
                dbus_ble_repo.found_devices.lock().unwrap().push(ble_device);
            }
        }

        thread::spawn({
            let connection = dbus_ble_repo.dbus_connection.clone();
            move || {
                loop {
                    if let Ok(mut connection) = connection.try_lock() {
                        connection.process(DBUS_CONNECTION_PROCESS_TIMEOUT);
                    }

                    thread::sleep(DBUS_CONNECTION_PROCESS_PERIOD);
                }
            }
        });

        dbus_ble_repo
    }

    /// Start the ble scan.
    pub fn start_scan(&self) {
        self.dbus_connection.lock().unwrap()
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", DBUS_CONNECTION_TIMEOUT)
            .start_discovery().expect("Error starting discovery");
    }

    /// Stop the ble scan.
    pub fn stop_scan(&self) {
        let connection = self.dbus_connection.lock().unwrap();
        let is_discovering = connection
            .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", DBUS_CONNECTION_TIMEOUT)
            .discovering().unwrap();

        if is_discovering {
            connection
                .with_proxy(BLUEZ_DBUS_DESTINATION, "/org/bluez/hci0", DBUS_CONNECTION_TIMEOUT)
                .stop_discovery().expect("Error stopping discovery");
        }
    }

    /// Set the on advertisement data callback.
    ///
    /// # Arguments:
    /// * `callback` - The callback to call when a new advertisement record is found.
    ///                The callback take a reference on a High level BleDevice abstraction as parameter.
    pub fn set_on_advertisement_data_callback(&mut self, callback: impl FnMut(&BleDevice)+ Send + Sync + 'static) {
        self.on_advertisement_data = Arc::new(Mutex::new(Box::new(callback)));
        self.add_interface_added_match_rule();
        self.add_properties_changed_match_rule();
    }

    /// Add the interface added match rule to the connection.
    /// Replaces the existing match rule with a new.
    fn add_interface_added_match_rule(&mut self) {
        let mut interface_added_match_rule = MatchRule::new();
        interface_added_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.ObjectManager").unwrap());
        interface_added_match_rule.msg_type = Option::Some(MessageType::Signal);
        interface_added_match_rule.member = Option::Some(Member::new("InterfacesAdded").unwrap());

        let on_interface_added = {
            let on_advertisement_data = self.on_advertisement_data.clone();
            let found_devices_clone = self.found_devices.clone();
            move | p: ObjectManagerInterfacesAdded, _: &SyncConnection, _: &Message| {
                // If this is a ble device which has been discovered
                if p.interfaces.contains_key(BLUEZ_DBUS_DEVICE_INTERFACE) {
                    let mut devices = found_devices_clone.lock().unwrap();
                    let path = p.object.to_string();

                    if let Some(device) = devices.iter_mut().find(|d| d.path == path) {
                        device.service_data = Self::parse_service_data(&p.interfaces[BLUEZ_DBUS_DEVICE_INTERFACE]);
                    } else {
                        let device = Self::get_ble_device(path, &p.interfaces[BLUEZ_DBUS_DEVICE_INTERFACE]);

                        (&mut *on_advertisement_data.lock().unwrap())(&device);

                        devices.push(device);
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

    /// Add the properties changed match rule to the connection.
    /// Replaces the existing match rule with a new.
    fn add_properties_changed_match_rule(&mut self) {
        let mut properties_changed_match_rule = MatchRule::new();
        properties_changed_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.Properties").unwrap());
        properties_changed_match_rule.msg_type = Option::Some(MessageType::Signal);
        properties_changed_match_rule.member = Option::Some(Member::new("PropertiesChanged").unwrap());

        let on_properties_changed = {
            let on_advertisement_data = self.on_advertisement_data.clone();
            let found_devices_clone = self.found_devices.clone();
            move | p: PropertiesPropertiesChanged, _: &SyncConnection, m: &Message | {
                if p.interface_name == BLUEZ_DBUS_DEVICE_INTERFACE {
                    let mut devices = found_devices_clone.lock().unwrap();

                    let path = m.path().unwrap().to_string();

                    if let Some(device) = devices.iter_mut().find(|d| d.path == path) {
                        device.service_data = Self::parse_service_data(&p.changed_properties);

                        (&mut *on_advertisement_data.lock().unwrap())(&device);
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

    /// Constructs a new ble device abstraction from dbus data.
    ///
    /// # Arguments:
    /// * `input_message` - The input raw message from dbus.
    /// * `input_interface` - The input dictionary that match the org.bluez.Device1 interface
    ///
    /// Returns a high level representation of a ble device.
    fn get_ble_device(device_path: String, input_interface: &HashMap<String, Variant<Box<dyn RefArg>>>) -> BleDevice {
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

    /// Parse service data.
    ///
    /// # Arguments:
    /// * `input` - The input raw data.
    ///
    /// Returns a rust representation of data.
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
