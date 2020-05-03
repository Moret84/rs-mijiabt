# rs-mijiabt
This is a simple rust listener app for the mijia bt thermometer/hygrometer.
It is built upon bluez dbus api using [dbus-rs](https://github.com/diwic/dbus-rs) crate.

## Usage
The app relies on a simple abstraction of the bluez dbus api that reads the advertisement data broadcasted by ble devices.  
On top of that, a mijia bt sensor-specific wrapper abstracts access to the device. It provides a callback to be notified of the temperature/humidity changes.  
Each of the two values are stored on a 16 bits unsigned integer. You will need to divide the result by 10 to get the actual decimal value.  
The api also provides a `start_listening` method that take an `Option<u64>` as timeout. If `None` is passed, the program runs forever. It still can be interrupted through Ctrl-C.
