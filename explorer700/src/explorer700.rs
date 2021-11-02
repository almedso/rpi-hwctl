//! Provide JOY IT Explorer 700 raspberrypi shield as a business like API
//!
//! Business like API hides the specific pin binding and exposes types that
//!     read like JoystickLeftButton, Display, TemperatureSensor
//!
//! https://joy-it.net/en/products/RB-Explorer700
#[cfg(target_arch = "arm")]
use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, I2cdev, Pin, Spidev,
};

use pcf857x::{self, Pcf8574, SlaveAddr, P0, P1, P2, P3, P4, P7};
#[allow(unused_imports)]
use pcf857x::{InputPin, OutputPin};
use ssd1306::{prelude::*, Builder};

#[allow(unused_imports)]
use linux_embedded_hal::i2cdev::linux::LinuxI2CError;

/// A graphic display that implements embedded graphics display driver trait
pub type GraphicDisplay128x64Monochrome = GraphicsMode<SpiInterface<Spidev, Pin>>;

/// Creates and returns a monochromatic display of 128x64 pixel (oled)
///
/// The returned graphics display is ready to be used as embedded graphics display driver
/// The creation includes hardware reset and initialization with an empty output.
///
/// # Panics
///
/// The builder function panics in case the hardware access fails
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
        .connect_spi(spi, dc)
        .into();

    disp.reset(&mut rst, &mut delay).expect("Display reset");
    disp.init().expect("Display init");

    disp
}

/// Binary output devices like led or buzzer that can be switched on or off
pub trait OnOff {
    /// Switch something on
    ///
    /// Panics in case of an hardware error
    fn on(&mut self);
    /// switch something off
    ///
    /// Panics in case of an hardware error
    fn off(&mut self);
}

/// The Led1
pub struct Led1(Pin);

impl OnOff for Led1 {
    fn on(&mut self) {
        self.0.set_value(1).expect("LED1 to be switched on");
    }

    fn off(&mut self) {
        self.0.set_value(0).expect("LED1 to be switched off");
    }
}

/// IT Explorer700 shield for RaspberryPi
///
/// # Supported features
///
/// * 128x64 OLED display (SSD1306)
/// * Led1, Led2
/// * Buzzer
/// * Joystick buttons
/// * Temperature; preasure sensor (BMP180)
pub struct Explorer700 {
    expander: Box<Pcf8574<I2cdev>>,
}


impl Default for Explorer700 {
    /// Creates a shield representation
    ///
    /// # Panics
    ///
    /// The builder function panics in case the hardware access fails
    fn default() -> Self {
        let i2cbus = I2cdev::new("/dev/i2c-1").expect("i2c-1 to be initialized");
        let expander: Box<Pcf8574<I2cdev>> = Box::new(Pcf8574::new(i2cbus, SlaveAddr::default()));
        Explorer700 { expander }
    }

}

impl Explorer700 {

    /// Creates access to LED1 output
    ///
    /// # Panics
    ///
    /// The builder function panics in case the hardware access fails
    pub fn init_led1(&self) -> Led1 {
        // lED1 is mapped to a GPIO pin
        let led1 = Pin::new(26);
        led1.export().expect("led1 export");
        while !led1.is_exported() {}
        led1.set_direction(Direction::Out).expect("led1 Direction");
        Led1(led1)
    }

    /// Creates access to LED2 output
    ///
    /// site effect: switch of buzzer
    pub fn init_led2(&self) -> Led2 {
        let mut parts = self.expander.split();
        // site effects:
        parts.p7.set_high().unwrap(); // make sure the buzzer is switched off

        Led2(parts.p4) // LED2 is at port 4
    }

    /// Creates access to Buzzer output
    ///
    /// site effect: switch off LED2
    pub fn init_buzzer(&self) -> Buzzer {
        let mut parts = self.expander.split();
        // site effects:
        parts.p4.set_high().unwrap(); // make sure LED2 is switched off

        Buzzer(parts.p7) // Buzzer is at port 7
    }

    /// Initialize the joystick
    ///
    /// site effect: switch of buzzer and led
    pub fn init_joystick(&self) -> Joystick {
        let mut parts = self.expander.split();
        // site effects:
        parts.p4.set_high().unwrap(); // make sure LED2 is switched off
        parts.p7.set_high().unwrap(); // make sure the buzzer is switched off

        Joystick {
            up: parts.p2,
            down: parts.p1,
            left: parts.p3,
            right: parts.p0,
        }
    }
}

/// The Led2
pub struct Led2<'a>(P4<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>);

impl OnOff for Led2<'_> {
    /// Switch led2 on
    fn on(&mut self) {
        self.0.set_low().expect("LED2 to be switched on")
    }

    /// Switch led2 off
    fn off(&mut self) {
        self.0.set_high().expect("LED2 to be switched off")
    }
}

/// The buzzer
pub struct Buzzer<'a>(P7<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>);

impl OnOff for Buzzer<'_> {
    /// Switch led2 on
    fn on(&mut self) {
        self.0.set_low().expect("Buzzer to be switched on")
    }

    /// Switch led2 off
    fn off(&mut self) {
        self.0.set_high().expect("Buzzer to be switched off")
    }
}

#[derive(PartialEq, Debug, Clone)]
pub enum JoystickState {
    Up,
    Down,
    Left,
    Right,
    NoAction,
}


unsafe impl Send for JoystickState {}
unsafe impl Sync for JoystickState {}

pub struct Joystick<'a> {
    up: P2<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>,
    down: P1<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>,
    left: P3<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>,
    right: P0<'a, pcf857x::Pcf8574<I2cdev>, LinuxI2CError>,
}

impl Joystick<'_> {
    pub fn state(&self) -> JoystickState {
        if self.up.is_low().unwrap() {
            return JoystickState::Up;
        }
        if self.down.is_low().unwrap() {
            return JoystickState::Down;
        }
        if self.left.is_low().unwrap() {
            return JoystickState::Left;
        }
        if self.right.is_low().unwrap() {
            return JoystickState::Right;
        }
        JoystickState::NoAction
    }
}
