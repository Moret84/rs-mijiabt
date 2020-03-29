mod btleplug_ble_repo;
use btleplug_ble_repo::BtleplugBleRepo;

use std::str;

const TARGET_DEVICE_NAME: &str = "MJ_HT_V1";

fn main() {
    let mut ble = BtleplugBleRepo::new();

    ble.set_device_filter(|_address, name| {
        name.contains(TARGET_DEVICE_NAME)
    });

    let devices = ble.scan(10, true);


    if devices.len() > 0 {
        println!("{}", ble.connect(devices[0]));
    }
}
