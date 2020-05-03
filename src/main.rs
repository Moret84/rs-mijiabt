mod mijia_bt;
mod ble;

use mijia_bt::MijiaBt;

use std::thread;
use std::time::Duration;

fn main() {
    let mut mijia_bt = MijiaBt::new();
    mijia_bt.set_on_data_updated_callback(Some(|mijiabt_data: (u16, u16)| {
        println!("Temperature: {}, Humidity: {}", mijiabt_data.0, mijiabt_data.1);
    }));

    mijia_bt.start_listening();

    let mut i = 30;
    while i >= 0 {
        thread::sleep(Duration::from_secs(1));
        i = i -1;
    }

    mijia_bt.stop_listening();
}
