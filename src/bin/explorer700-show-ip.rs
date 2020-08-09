use nix;

use embedded_graphics::{
    fonts::{Font6x8, Text},
    pixelcolor::BinaryColor,
    prelude::*,
    // primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyle, TextStyle},
};

use ssd1306::{prelude::*, Builder};

use linux_embedded_hal::{
    spidev::{self, SpidevOptions},
    sysfs_gpio::Direction,
    Delay, Pin, Spidev,
};


fn internal() -> Option<String> {
    let addrs = nix::ifaddrs::getifaddrs().unwrap();
    for ifaddr in addrs {
      match ifaddr.address {
        Some(address) => {
        if let nix::sys::socket::SockAddr::Inet(addr) = address {
            if let nix::sys::socket::InetAddr::V4(_v4_addr) = addr {
                println!("interface {} address {}", ifaddr.interface_name, addr);
                if ifaddr.interface_name != "lo" {
                    return Some(address.to_string());
                }
            }}
        },
        None => {
          println!("interface {} with unsupported address family",
                   ifaddr.interface_name);
        }
      }
    }
    None
}


fn main() -> Result<(), std::io::Error> {
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
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");

    // Configure Reset pin
    let mut rst = Pin::new(19); // pin 18 GPIO 24
    rst.export().expect("dc export");
    while !rst.is_exported() {}
    rst.set_direction(Direction::Out).expect("rst Direction");
    rst.set_value(1).expect("rst Value set to 1");

    let mut delay = Delay {};

    let mut disp: GraphicsMode<_> = Builder::new()
        .with_rotation(DisplayRotation::Rotate180)
        .connect_spi(spi, dc).into();


    disp.reset(&mut rst, &mut delay).unwrap();
    disp.init().unwrap();

    // let style = PrimitiveStyleBuilder::new()
    //     .stroke_width(1)
    //     .stroke_color(BinaryColor::On)
    //     .build();
    let ip = match internal() {
        Some(addr) => addr,
        None => "".to_owned(),
    };
    let text_style = TextStyle::new(Font6x8, BinaryColor::On);
    // Draw centered text.
    let width = ip.len() as i32 * 6;
    Text::new(&ip, Point::new(64 - width / 2, 40))
        .into_styled(text_style)
        .draw(&mut disp).unwrap();

    disp.flush().unwrap();

    Ok (())
}