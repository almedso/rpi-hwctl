extern crate i2cdev;
extern crate si7021;
extern crate i2csensors;

use i2cdev::linux::LinuxI2CDevice;
use si7021::{Si7021, SI7021_I2C_ADDRESS};
use i2csensors::{Hygrometer, Thermometer};

fn main() {
    let device = LinuxI2CDevice::new("/dev/i2c-0", SI7021_I2C_ADDRESS).unwrap();
    let mut si7021 = Si7021::new(device);

    println!("humidity:    {:6.2} %", si7021.relative_humidity().unwrap());
    println!("temperature: {:6.2} °C", si7021.temperature_celsius().unwrap());
}
