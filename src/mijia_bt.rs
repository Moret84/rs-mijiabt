use crate::ble::api::BleDevice;
use crate::ble::dbus::dbus_ble_repo::DbusBleRepo;

use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

const TARGET_DEVICE_NAME: &str = "MJ_HT_V1";
const TARGET_SERVICE_UUID: &str = "0000fe95-0000-1000-8000-00805f9b34fb";

pub struct MijiaBt {
    ble_repo: DbusBleRepo,
    current_data: Arc<(AtomicU16, AtomicU16)>,
    on_data_updated: Arc<Mutex<Option<Box<dyn FnMut((u16, u16)) + Send + Sync + 'static>>>>,
    listening: Arc<AtomicBool>
}

impl MijiaBt {
    /// Returns a new instance of the mijia_bt sensor abstraction.
    pub fn new() -> MijiaBt {
        let mijia_bt = MijiaBt {
            ble_repo: DbusBleRepo::new(),
            current_data: Arc::new((AtomicU16::new(0), AtomicU16::new(0))),
            on_data_updated: Arc::new(Mutex::new(None)),
            listening: Arc::new(AtomicBool::new(false))
        };

        mijia_bt
    }

    /// Start listening the mijia bt sensor.
    ///
    /// # Arguments:
    /// * `timeout` - The time in seconds to listen the bt sensor.
    ///               If None is passed, the program waits forever.
    pub fn start_listening(&self, timeout: Option<u64>) {
        println!("Start listening the mijia bt sensor...\n\
                 Ctrl-C to stop");

        self.ble_repo.start_scan();
        self.listening.store(true, Ordering::SeqCst);

        let now = Instant::now();

        while self.listening.load(Ordering::SeqCst) {
            thread::sleep(Duration::from_millis(20));

            if let Some(timeout) = timeout {
                if now.elapsed().as_secs() >= timeout {
                    self.listening.store(false, Ordering::SeqCst);
                }
            }
        }
    }

    /// Stop listening the mijia bt sensor.
    ///
    pub fn stop_listening(&self) {
        self.listening.store(false, Ordering::SeqCst);
    }

    /// Set the on data updated callback.
    ///
    /// # Arguments:
    /// * `callback` - The callback to call when a mijia bt data update occurs
    ///                The callback take a (temperature, humidity) u16 tuple as parameter.
    pub fn set_on_data_updated_callback(&mut self, callback: Option<impl FnMut((u16, u16)) + Send + Sync + 'static>) {
        match callback {
            None => self.on_data_updated = Arc::new(Mutex::new(None)),
            Some(callback) => {
                self.on_data_updated = Arc::new(Mutex::new(Some(Box::new(callback))));

                let on_advertisement_data = {
                    let mijiabt_data_clone = self.current_data.clone();
                    let on_data_updated_clone = self.on_data_updated.clone();
                    move |device: &BleDevice| {
                        if device.local_name == TARGET_DEVICE_NAME {
                            if device.service_data.contains_key(TARGET_SERVICE_UUID) {
                                let new_data = Self::parse_mijia_bt_data(&device.service_data[TARGET_SERVICE_UUID]);

                                let mut data_changed = false;

                                if mijiabt_data_clone.0.load(Ordering::Relaxed) != new_data.0 && new_data.0 != 0 {
                                    mijiabt_data_clone.0.store(new_data.0, Ordering::Relaxed);
                                    data_changed = true;
                                }

                                if mijiabt_data_clone.1.load(Ordering::Relaxed) != new_data.1  && new_data.1 != 0 {
                                    mijiabt_data_clone.1.store(new_data.1, Ordering::Relaxed);
                                    data_changed = true;
                                }

                                if data_changed {
                                    let temperature = mijiabt_data_clone.0.load(Ordering::Relaxed);
                                    let humidity = mijiabt_data_clone.1.load(Ordering::Relaxed);

                                    if let Some(on_data_updated) = &mut *on_data_updated_clone.lock().unwrap() {
                                        on_data_updated((temperature, humidity));
                                    }
                                }
                            }
                        }
                    }
                };

                self.ble_repo.set_on_advertisement_data_callback(on_advertisement_data);
            }
        }
    }

    /// Parse mijia btd advertisement data into temperature and humidity.
    ///
    /// # Arguments:
    /// * `input` - The input raw data.
    ///
    /// Returns a (temperature, humidity) u16 tuple.
    fn parse_mijia_bt_data(input: &Vec<u8>) -> (u16, u16) {
        let mut temperature: u16 = 0;
        let mut humidity : u16 = 0;
        if input.len() == 18 {
            temperature = ((input[15] as u16) << 8) | input[14] as u16;
            humidity = ((input[17] as u16) << 8) | input[16] as u16;
        } else if input.len() == 16 {
            if input[11] == 6 {
                humidity = ((input[15] as u16) << 8) | input[14] as u16;
            } else if input[11] == 4 {
                temperature = ((input[15] as u16) << 8) | input[14] as u16;
            }
        }

        (temperature, humidity)
    }
}

impl Drop for MijiaBt {
    fn drop(&mut self) {
        self.ble_repo.stop_scan();
    }
}
