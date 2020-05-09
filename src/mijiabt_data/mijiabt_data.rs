use std::sync::atomic::{AtomicU16, Ordering};

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
/// An abstraction of the mijiabt sensor data.
pub struct MijiaBtData {
    /// The temperature of the thermometer.
    temperature: AtomicU16,
    /// The humidity of the thermometer.
    humidity: AtomicU16,
}

impl MijiaBtData {
    /// Create a new instance of MijiaBtData.
    ///
    /// Returns a new instance of MijiaBtData.
    pub fn new(temperature: u16, humidity: u16) -> MijiaBtData {
        MijiaBtData {
            temperature: AtomicU16::new(temperature),
            humidity: AtomicU16::new(humidity)
        }
    }

    /// Update MijiaBtData with new values.
    ///
    /// # Arguments:
    /// * `temperature` - The new temperature.
    /// * `humidity` - The new humidity.
    pub fn update(&self, temperature: u16, humidity: u16) {
        self.temperature.store(temperature, Ordering::Relaxed);
        self.humidity.store(humidity, Ordering::Relaxed);
    }

    /// Get the current value of the MijiaBtData.
    ///
    /// Returns a tuple containing the temperature and the humidity as a (u16, u16).
    /// The values have to be divided by 10 to obtain the right values.
    pub fn get(&self) -> (u16, u16) {
        (
            self.temperature.load(Ordering::Relaxed),
            self.humidity.load(Ordering::Relaxed)
        )
    }
}
