use ctrlc;

use mijiabt::MijiaBt;

use std::sync::Arc;

fn main() {
    let mut mijia_bt = MijiaBt::new();
    mijia_bt.set_on_data_updated_callback(Some(|(temperature, humidity)| {
        println!("Temperature: {}, Humidity: {}", temperature, humidity);
    }));

    let mijia_bt = Arc::new(mijia_bt);

    ctrlc::set_handler({
        let mijia_bt_clone = mijia_bt.clone();
        move || {
            mijia_bt_clone.stop_listening();
            println!("SIGINT received. Exiting...");
        }
    }).expect("Error setting Ctrl-C handler");

    mijia_bt.start_listening(None);
}
