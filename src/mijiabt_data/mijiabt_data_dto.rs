use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MijiaBtDataDto {
    pub temperature: u16,
    pub humidity: u16,
}

impl MijiaBtDataDto {
    pub fn new(temperature: u16, humidity: u16) -> MijiaBtDataDto {
        MijiaBtDataDto {
            temperature,
            humidity
        }
    }
}
