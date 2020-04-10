mod btleplug_ble_repo;
use btleplug_ble_repo::BtleplugBleRepo;

use std::str;

const TARGET_DEVICE_ADDRESS: [u8; 6] = [0x4C, 0x65, 0xA8, 0xDB, 0xBD, 0x7D];
const TARGET_SERVICE_UUID: &str = "0000fe95-0000-1000-8000-00805f9b34fb";
const TARGET_DEVICE_NAME: &str = "MJ_HT_V1";

fn main() {
    let device = dbus_test();
    if !device.is_empty() {
        follow(device);
    }
    //btleplug_test();
}

fn btleplug_test() {
    let mut ble = BtleplugBleRepo::new();

    ble.set_device_filter(|address, name| {
        name.contains(TARGET_DEVICE_NAME) || address.eq(&TARGET_DEVICE_ADDRESS)
    });

    let devices = ble.scan(10, true);

    if devices.len() > 0 {
        println!("{}", ble.connect(devices[0]));
    }
}

mod bluez_dbus;

extern crate dbus;
use dbus::Message;
use dbus::blocking::Connection;
use dbus::arg::{RefArg, Variant};
use dbus::message::{MatchRule,MessageType};
use dbus::strings::{Interface, Path, Member};

use std::time::Duration;
use std::collections::HashMap;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use bluez_dbus::{OrgBluezAdapter1, OrgBluezDevice1};

const DBUS_DESTINATION: &str = "org.bluez";

fn dbus_test() -> String
{
    // First open up a connection to the system bus bus.
    let mut conn = Connection::new_system().expect("Error");

    let mut device_match_rule = MatchRule::new();
    device_match_rule.interface = Option::Some(Interface::new("org.freedesktop.DBus.ObjectManager").unwrap());
    device_match_rule.msg_type = Option::Some(MessageType::Signal);
    device_match_rule.member = Option::Some(Member::new("InterfacesAdded").unwrap());

    let path = Arc::new(Mutex::new(String::new()));

    let on_device_discovered =  {
        let path_clone = path.clone();
        move |_: () , _ : &Connection, message: &Message| {

            println!("{:#?}", message);

            let first_arg = message.get1::<Path>();

            match first_arg {
                None => (),
                Some(arg) => {
                    if arg.starts_with("/org/bluez/hci0/") {
                        path_clone.lock().unwrap().push_str(arg.as_str().unwrap());
                    }
                }
            }
            true
        }
    };

    conn.add_match(device_match_rule, on_device_discovered).expect("Mon vier");

    {
        let adapter_proxy = conn.with_proxy(DBUS_DESTINATION, "/org/bluez/hci0", Duration::from_millis(5000));

        adapter_proxy.start_discovery().unwrap();

            let mut i = 30;
            while i >= 0 && path.lock().unwrap().is_empty() {
                conn.process(Duration::from_secs(1));
                thread::sleep(Duration::from_secs(1));
                i = i -1;
        }
    }

    let adapter_proxy = conn.with_proxy(DBUS_DESTINATION, "/org/bluez/hci0", Duration::from_millis(5000));

    return path.lock().unwrap().clone();
}

fn follow(device: String)
{
    let conn = Connection::new_system().expect("Error");

    let adapter_proxy = conn.with_proxy(DBUS_DESTINATION, device, Duration::from_millis(5000));

    let filter = match OrgBluezDevice1::name(&adapter_proxy) {
        Err(e) => false,
        Ok(data) => data == TARGET_DEVICE_NAME,
    };

    if filter {
        match adapter_proxy.service_data() {
            Err(e) => (),
            Ok(data) => {
                let mut r : Vec<u8> = Vec::new();
                 for b in data[TARGET_SERVICE_UUID].0.as_iter().unwrap() {
                     r.push(b.as_u64().unwrap() as u8);
                 }

                 println!("{:#?}", r);
            }
        }
    }
}
