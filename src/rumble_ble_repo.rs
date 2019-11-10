extern crate rumble;

use std::thread;
use std::time::Duration;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use rumble::bluez::manager::Manager;
use rumble::bluez::adapter::ConnectedAdapter;

use rumble::api::{BDAddr, Central, CentralEvent, Peripheral};
use rumble::api::CentralEvent::DeviceUpdated;

use rumble::Error;

// Temporary
use rumble::api::{UUID, ValueNotification};
const TARGET_CHARACTERISTIC_UUID: UUID =
UUID::B128([0x6d, 0x66, 0x70, 0x44, 0x73, 0x66, 0x62, 0x75, 0x66, 0x45, 0x76, 0x64, 0x55, 0xaa, 0x6c, 0x22]);

/// An implementation of a BleRepo using rumble crate
pub struct RumbleBleRepo {
    /// The reference to the underlying device adapter
    adapter: Arc<ConnectedAdapter>,
    /// The device filter to use, or not.
    device_filter: Option<fn([u8; 6], String) -> bool>,
}

impl RumbleBleRepo {
    /// Return a new instance of a Rumble ble repo.
    pub fn new() -> RumbleBleRepo {
        let manager = Manager::new().unwrap();

        // Get the first adapter
        let adapters = manager.adapters().unwrap();
        let mut adapter = adapters.into_iter().nth(0).unwrap();

        // Reset the adapter -- clears out any errant state
        adapter = manager.down(&adapter).unwrap();
        adapter = manager.up(&adapter).unwrap();

        RumbleBleRepo {
            adapter: Arc::new(adapter.connect().unwrap()),
            device_filter: None,
        }
    }

    /// Scan for around, looking for devices.
    ///
    /// Returns a GUID list of found devices.
    ///
    /// # Arguments:
    /// * `timeout` - A timeout for the scan.
    /// * `stop_on_found` - Whether to stop when a device is found or not.
    pub fn scan(&self, mut timeout: u64, stop_on_found: bool) -> Vec<[u8; 6]> {

        let found_devices : Arc<Mutex<Vec<[u8; 6]>>> = Arc::new(Mutex::new(Vec::new()));
        let found_devices_clone = found_devices.clone();

        let scan_done : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));

        let adapter = self.adapter.clone();
        let device_filter = self.device_filter;
        let scan_done_clone = scan_done.clone();

        let on_device_updated = move |address: BDAddr| {
            let device_properties = adapter.clone().peripheral(address)
                .unwrap().properties();

            if device_properties.discovery_count == 2 {
                let device_name = match device_properties.local_name {
                    None => String::from("Unknown"),
                    Some(name) => name,
                };

                if let Some(filter) = device_filter {
                    if (filter)(address.address, device_name) {
                        found_devices_clone.lock().unwrap().push(address.address);
                        if stop_on_found {
                            scan_done_clone.store(true, Ordering::Relaxed);
                        }
                    }
                }
            }
        };

        self.adapter.on_event(Box::new(move |event: CentralEvent| {
            // If a DeviceUpdated event occured
            if let DeviceUpdated(address) = event {
                on_device_updated(address);
            }
        }));

        // Actually start the scan
        self.adapter.start_scan().unwrap();

        while timeout > 0 && !scan_done.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            timeout -= 1;
        }

        self.adapter.stop_scan().unwrap();

        thread::sleep(Duration::from_secs(1));

        return found_devices.lock().unwrap().clone().to_vec();
    }

    /// Set the device filter
    ///
    /// Set the device filter with a function that return a bool and taking an address and a name as inputs.
    pub fn set_device_filter(&mut self, device_filter: fn([u8; 6], String) -> bool) {
        self.device_filter = Some(device_filter);
    }

    /// Connect to a device.
    ///
    /// Connect to the device associated with provided address.
    ///
    /// # Arguments:
    /// * `device_address` - The address of the device to connect.
    ///
    /// Returns true whether the connection succeeded, false otherwise.
    ///
    /// As for the current state of rumble library, we can't do more refactoring from this point since we can't store a Peripheral to operate on it later.
    /// Therefore, and for now, we have to operate straight on.
    pub fn connect(self, device_address: [u8; 6]) -> String {
        let device = self.adapter.peripheral(BDAddr { address: device_address });

        let result = match &device {
            Some(_device) => true,
            None => false
        };

        let result_str = Arc::new(Mutex::new(String::new()));

        // If device found
        if result {
            match device.as_ref().unwrap().connect() {
                Err(e) => {
                    match e {
                        Error::PermissionDenied => println!("Permission denied"),
                        Error::DeviceNotFound => println!("Device not found"),
                        Error::NotConnected => println!("Not connected"),
                        Error::NotSupported(d) => println!("Not supported: {}", d),
                        Error::TimedOut(d) => println!("Timed out: {}", d.as_secs()),
                        Error::Other(d) => println!("Other: {}", d)
                    }
                    false
                },
                Ok(_s) => {
                    let connected_device = device.unwrap();

                    // Discover characteristics
                    connected_device.discover_characteristics().unwrap();

                    // Get characteristics
                    let characs = connected_device.characteristics();

                    // Get temperature characteristic
                    let temperature_char = characs.iter().find(|c| c.uuid == TARGET_CHARACTERISTIC_UUID).unwrap();

                    // Whether the hacaretristic has been read or not.
                    let charac_read = Arc::new(AtomicBool::new(false));

                    let charac_read_clone = charac_read.clone();
                    let result_str_clone = result_str.clone();
                    connected_device.on_notification(Box::new(move |n: ValueNotification| {
                        result_str_clone.lock().unwrap().
                            push_str(&String::from_utf8(n.value).unwrap());
                        charac_read_clone.store(true, Ordering::Relaxed);
                    }));

                    connected_device.subscribe(temperature_char).unwrap();

                    while charac_read.load(Ordering::Relaxed) == false {}

                    connected_device.disconnect().unwrap();

                    true
                }
            };
        }
        else
        {
            println!("No device found");
        }

        return result_str.lock().unwrap().to_string();
    }
}
