use std::collections::HashMap;

/// A high-level ble device representation.
pub struct BleDevice {
    /// The ble identifier of the device.
    pub path: String,
    /// The local name advertised by the device.
    pub local_name: String,
    /// The service data advertised by the device.
    pub service_data: HashMap<String, Vec<u8>>
}
