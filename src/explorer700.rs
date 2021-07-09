//! Provide JOY IT Explorer 700 raspberrypi shield as a business like API
//!
//! Business like API hides the specific pin binding and exposes types that
//!     read like JoystickLeftButton, Display, TemperatureSensor
//!
//! https://joy-it.net/en/products/RB-Explorer700

use linux_embedded_hal::{
    Delay,
    I2cdev,
    Pin,
    Spidev,
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
};
use ssd1306::{prelude::*, Builder};
use pcf857x::{self, Pcf8574, SlaveAddr, OutputPin};
// #[cfg(feature="unproven")]
use pcf857x::InputPin;
use i2cdev::linux::LinuxI2CError;

use port_expander;
use shared_bus; 

/// A graphic display that implements embedded graphics display driver trait
pub type GraphicDisplay128x64Monochrome = GraphicsMode<SpiInterface<Spidev,Pin>>;

/// Creates and returns a monochromatic display of 128x64 pixel (oled)
///
/// The returned graphics display is ready to be used as embedded graphics display driver
/// The creation includes hardware reset and initialization with an empty output.
///
/// # Panics
///
/// The builder function pancics in case the hardeware access fails
pub fn build_display() -> GraphicDisplay128x64Monochrome {
    // Configure SPI
    let mut spi = Spidev::open("/dev/spidev0.0").expect("spidev directory");
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(8_000_000)
        .mode(spidev::SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options).expect("spi configuration");

    // Configure Data/Command pin
    let dc = Pin::new(16);
    dc.export().expect("rst export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    // Configure Reset pin
    let mut rst = Pin::new(19); // pin 18 GPIO 24
    rst.export().expect("rst export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    let mut delay = Delay {};

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate180)
        .connect_spi(spi, dc).into();

    disp.reset(&mut rst, &mut delay).expect("Display reset");
    disp.init().expect("Display init");

    disp
}

/// Creates access to LED1 output
///
/// # Panics
///
/// The builder function pancics in case the hardeware access fails
pub fn build_led1() -> Pin {
    // Configure led pin
    let led1 = Pin::new(26); 
    led1.export().expect("led1 export");
    while !led1.is_exported() {}
    led1.set_direction(Direction::Out).expect("led1 Direction");

    // switch led off
    led1.set_value(0).expect("LED1 to be switched off");
    led1
}


// /// The LED2
// // pub struct Led2<'a>(port_expander::Pin<'a, 
// //     portexpander::mode::QuasiBidirectional,
// //     shared_bus::NullMutex<shared_bus::BusManagersimple::BUS>>
// // );
// // pub struct Led2(Box<dyn OutputPin<Error = pcf857x::Error<LinuxI2CError>>>);
// // INTERNAL
// // appraoch to have a type retured that implements the OutputPin  trait
// // is difficult .. in embedded, because we run into lifetime issues with exander
// // It will end up in sharing only allowed within one thread. such that the i2c bus is approached
// // only once. port expander is the better implementation
// // alternative -> dig into yocto and try to extend the device tree.


// impl Led2

// {
//     /// Creates access to LED2 output
//     ///
//     /// # Panics
//     ///
//     /// The builder function pancics in case the hardeware access fails
//     pub fn new() -> Self {
//         // Configure led pin - connected to port expander
//         // let dev = I2cdev::new("/dev/i2c-1").expect("i2c-1 to be initialized");
//         // A0: LOW, A1: LOW, A2: LOW
//         // let mut pcf8574 = port_expander::Pcf8574::new(dev, false, false, false);
//         // let pca_pins = pcf8574.split();
//         // let address = SlaveAddr::default();
//         // let expander = Pcf8574::new(dev, address);
//         let expander: Box<Pcf8574<I2cdev>> = Box::new(Pcf8574::new(
//             I2cdev::new("/dev/i2c-1").expect("i2c-1 to be initialized"),
//             SlaveAddr::default()
//         ));
//         let parts = expander.split();
//         let mut led2 = parts.p4;

//         // switch led off
//         led2.set_low().expect("LED2 to be switched off");  // led - low active
//         Self(Box::new(led2))
//     }

//     /// Switch led on
//     pub fn on(&mut self) {
//         self.0.set_high().expect("LED2 to be switched on")
//     }

//     /// Switch led off
//     pub fn off(&mut self) {
//         self.0.set_low().expect("LED2 to be switched off")
//     }
// }


// /// Creates access to BUZZER output
// ///
// /// # Panics
// ///
// /// The builder function pancics in case the hardeware access fails
// pub fn build_buzzer() -> OutputPin {
//     // Configure buzzer pin - connected to port expander
//     let dev = I2cdev::new("/dev/i2c-1").expect("i2c-1 to be initialized");
//     let address = SlaveAddr::default();
//     let expander = Pcf8574::new(dev, address);
//     let parts = expander.split();
//     let mut buzzer = parts.p7;

//     // switch led off
//     buzzer.set_low().expect("BUZZER to be switched off");  // buzzer - low active
//     buzzer
// }
