mod btleplug_ble_repo;
use btleplug_ble_repo::BtleplugBleRepo;

mod dbus_ble;
use dbus_ble::dbus_ble_repo::DbusBleRepo;

mod api;

use std::thread;
use std::time::Duration;

const TARGET_DEVICE_ADDRESS: [u8; 6] = [0x4C, 0x65, 0xA8, 0xDB, 0xBD, 0x7D];
const TARGET_SERVICE_UUID: &str = "0000fe95-0000-1000-8000-00805f9b34fb";
const TARGET_DEVICE_NAME: &str = "MJ_HT_V1";

fn main() {
    dbus_test();
//   let device = dbus_test();
/*    if !device.is_empty() {
        follow(device);
    }*/
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

fn dbus_test() {
    let mut dbus_ble_repo = DbusBleRepo::new();
    dbus_ble_repo.set_on_device_discovered_cb(|device| {
        println!("{}", device.path);
        println!("{}", device.local_name);
        println!("{:#?}", device.service_data);
    });

    dbus_ble_repo.start_scan();

    let mut i = 30;
    while i >= 0 {
        thread::sleep(Duration::from_secs(1));
        i = i -1;
    }

    dbus_ble_repo.stop_scan();
}
