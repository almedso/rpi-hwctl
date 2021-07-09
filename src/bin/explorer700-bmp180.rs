//! Measure temperature and pressure via BMP280 chip on Explorer 700 shield
//!

use linux_embedded_hal::{Delay, I2cdev};
use i2cdev_bmp180::{BMP180PressureMode, BMP180BarometerThermometer};

use i2cdev::linux::*;
// use i2cdev::sensors::{Barometer, Thermometer};


fn main() {
    let i2c_bus = I2cdev::new("/dev/i2c-1").unwrap();
    let mut measurements = BMP180BarometerThermometer::new(i2c_bus, BMP180PressureMode::BMP180Standard);

    // or, initialize the BME280 using the secondary I2C address 0x77
    // let mut bme280 = BME280::new_secondary(i2c_bus, Delay);

    // or, initialize the BME280 using a custom I2C address
    // let bme280_i2c_addr = 0x88;
    // let mut bme280 = BME280::new(i2c_bus, bme280_i2c_addr, Delay);

    // initialize the sensor
    // bme280.init().unwrap();

    // measure temperature, pressure, and humidity
    // let measurements = bme280.measure().unwrap();

    println!("Temperature = {} deg C", measurements.temperature_celsius());
    println!("Pressure = {} pascals", measurements.pressure_pa());
}

