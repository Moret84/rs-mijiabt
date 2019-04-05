extern crate rumble;

mod rumble_ble_repo;
use rumble_ble_repo::RumbleBleRepo;

use std::{str, process, thread};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use rumble::bluez::manager::Manager;
use rumble::api::{UUID, ValueNotification, Central, Peripheral};

const TARGET_DEVICE_NAME: &str = "MJ_HT_V1";
const TARGET_CHARACTERISTIC_UUID: UUID =
UUID::B128([0x6d, 0x66, 0x70, 0x44, 0x73, 0x66, 0x62, 0x75, 0x66, 0x45, 0x76, 0x64, 0x55, 0xaa, 0x6c, 0x22]);

fn main() {
    let mut ble = RumbleBleRepo::new();

    ble.set_device_filter(|address, name| {
        name.contains(TARGET_DEVICE_NAME)
    });

    let devices = ble.scan(10, true);

    /*    let manager = Manager::new().unwrap();

    // get the first adapter
    let adapters = manager.adapters().unwrap();
    let mut adapter = adapters.into_iter().nth(0).unwrap();

    // reset the adapter -- clears out any errant state
    adapter = manager.down(&adapter).unwrap();
    adapter = manager.up(&adapter).unwrap();

    // connect to adapter
    let central = adapter.connect().unwrap();

    // start scanning for devices
    central.start_scan().unwrap();

    // instead of waiting, you can use central.on_event to be notified of new devices
    thread::sleep(Duration::from_secs(1));

    let device = central.peripherals().into_iter()
    .find(|p| p.properties().local_name.iter()
    .any(|name| name.contains(TARGET_DEVICE_NAME)));

    let temperature_sensor;
    match device {
    Some(device_found) => {
    temperature_sensor = device_found;
    println!("Device found");
    },
    None => {
    println!("Device not found");
    process::exit(1);
    }
    };

    // connect to device
    temperature_sensor.connect().unwrap();

    // discover characteristics
    temperature_sensor.discover_characteristics().unwrap();

    // get characs
    let characs = temperature_sensor.characteristics();

    // get temperature charac
    let temperature_char = characs.iter().find(|c| c.uuid == TARGET_CHARACTERISTIC_UUID).unwrap();

    let charac_read = Arc::new(AtomicBool::new(false));
    let clone = charac_read.clone();

    temperature_sensor.on_notification(Box::new(move |n: ValueNotification| {
    println!("{}", String::from_utf8(n.value).unwrap());
    clone.store(true, Ordering::Relaxed);
    }));

    temperature_sensor.subscribe(temperature_char);

    while charac_read.load(Ordering::Relaxed) == false {}

    temperature_sensor.disconnect();
    */
}
