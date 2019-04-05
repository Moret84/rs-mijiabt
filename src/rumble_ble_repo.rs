extern crate rumble;

use std::thread;
use std::time::Duration;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};

use rumble::bluez::manager::Manager;
use rumble::bluez::adapter::ConnectedAdapter;
use rumble::api::{BDAddr, Central, CentralEvent, Peripheral};
use rumble::api::CentralEvent::{DeviceDiscovered, DeviceUpdated};

pub struct RumbleBleRepo {
    central: Option<Arc<ConnectedAdapter>>,
    device_filter: Option<fn([u8; 6], String) -> bool>,
}

impl RumbleBleRepo {
    pub fn new() -> RumbleBleRepo {
        let mut repo = RumbleBleRepo {
            central: None,
            device_filter: None,
        };
        repo.init_central();
        repo
    }

    pub fn scan(self, mut timeout: u64, stop_on_found: bool) -> Vec<[u8; 6]> {
        let central = self.central.unwrap();
        let central_clone = central.clone();

        let found_devices : Arc<Mutex<Vec<[u8; 6]>>> = Arc::new(Mutex::new(Vec::new()));
        let found_devices_clone = found_devices.clone();

        let device_filter = self.device_filter;

        let scan_done : Arc<AtomicBool> = Arc::new(AtomicBool::new(false));
        let scan_done_clone = scan_done.clone();

        // Defining the on device found callback.
        central.on_event(Box::new(move |event: CentralEvent| {
            if let DeviceUpdated(address) = event {
                let device_properties = central_clone.peripheral(address)
                    .unwrap().properties();

                if device_properties.discovery_count == 2 {
                    let device_name = match device_properties.local_name {
                        None => String::from("Unknown"),
                        Some(name) => name,
                    };

                    found_devices_clone.lock().unwrap().push(address.address);

                    if let Some(filter) = device_filter {
                        if (filter)(address.address, device_name) {
                            if stop_on_found {
                                scan_done_clone.store(true, Ordering::Relaxed);
                            }
                        }
                    }
                }
            }
        }));

        // Actually start the scan
        central.start_scan();

        while timeout > 0 && !scan_done.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            timeout -= 1;
        }

        central.stop_scan();

        return found_devices.lock().unwrap().clone().to_vec();
    }

    pub fn set_device_filter(&mut self, device_filter: fn([u8; 6], String) -> bool) {
        self.device_filter = Some(device_filter);
    }

    fn init_central(&mut self) {
        let manager = Manager::new().unwrap();
        // get the first adapter
        let adapters = manager.adapters().unwrap();
        let mut adapter = adapters.into_iter().nth(0).unwrap();

        // reset the adapter -- clears out any errant state
        adapter = manager.down(&adapter).unwrap();
        adapter = manager.up(&adapter).unwrap();

        // connect to adapter
        self.central = Some(Arc::new(adapter.connect().unwrap()));
    }
}
