use std::sync::atomic::{AtomicU16, Ordering};

pub struct MijiaBtData {
    temperature: AtomicU16,
    humidity: AtomicU16,
}

impl MijiaBtData {
    pub fn new() -> MijiaBtData {
        MijiaBtData {
            temperature: AtomicU16::new(0),
            humidity: AtomicU16::new(0)
        }
    }

    pub fn update(&self, temperature: u16, humidity: u16) {
        self.temperature.store(temperature, Ordering::Relaxed);
        self.humidity.store(humidity, Ordering::Relaxed);
    }

    pub fn get(&self) -> (u16, u16) {
        (
            self.temperature.load(Ordering::Relaxed),
            self.humidity.load(Ordering::Relaxed)
        )
    }
}
