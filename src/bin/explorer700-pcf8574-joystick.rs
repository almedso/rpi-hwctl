use linux_embedded_hal::I2cdev;
use pcf857x::{Pcf8574, SlaveAddr, OutputPin };
// #[cfg(feature="unproven")]
use pcf857x::InputPin;

use std::{thread, time};


fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let address = SlaveAddr::default();
    let expander = Pcf8574::new(dev, address);
    let mut parts = expander.split();

    parts.p4.set_low().unwrap();  // led - low active
    parts.p7.set_low().unwrap();   // buzzer - low active

    let hundred_millis = time::Duration::from_millis(100);
    loop {
        #[cfg(feature="unproven")]
        {
            if parts.p0.is_low().unwrap() {
                println!("right")
            }
            if parts.p1.is_low().unwrap() {
                println!("down")
            }
            if parts.p2.is_low().unwrap() {
                println!("up")
            }
            if parts.p3.is_low().unwrap() {
                break;  // left direction
            }
        }
        thread::sleep(hundred_millis);

    }
    parts.p4.set_high().unwrap();  // led - low active
    parts.p7.set_high().unwrap();   // buzzer - low active
}
