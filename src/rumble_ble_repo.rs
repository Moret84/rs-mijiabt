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

                found_devices_clone.lock().unwrap().push(address.address);

                if let Some(filter) = device_filter {
                    if (filter)(address.address, device_name) {
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
        self.adapter.start_scan();

        while timeout > 0 && !scan_done.load(Ordering::Relaxed) {
            thread::sleep(Duration::from_secs(1));
            timeout -= 1;
        }

        self.adapter.stop_scan();

        return found_devices.lock().unwrap().clone().to_vec();
    }

    /// Populate the device filter
    pub fn set_device_filter(&mut self, device_filter: fn([u8; 6], String) -> bool) {
        self.device_filter = Some(device_filter);
    }

    /// Init the underlying adapter.
    fn init_adapter(&mut self) {
        let manager = Manager::new().unwrap();

        // Get the first adapter
        let adapters = manager.adapters().unwrap();
        let mut adapter = adapters.into_iter().nth(0).unwrap();

        // Reset the adapter -- clears out any errant state
        adapter = manager.down(&adapter).unwrap();
        adapter = manager.up(&adapter).unwrap();

        // Connect to adapter
        self.adapter = Some(Arc::new(adapter.connect().unwrap()));
    }
}
