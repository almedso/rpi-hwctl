//! Draw a square, circle and triangle on the screen using the `embedded_graphics` crate.
//!

use embedded_graphics::{
    fonts::{Font6x8, Text},
    geometry::Point,
    pixelcolor::BinaryColor,
    prelude::*,
    primitives::{Circle, Rectangle, Triangle},
    style::{PrimitiveStyleBuilder, TextStyle},
};

use explorer700::explorer700::{
    build_display, Explorer700, GraphicDisplay128x64Monochrome, JoystickState, OnOff,
};

use clap::{App, Arg, SubCommand};

fn main() -> Result<(), std::io::Error> {
    let matches = App::new("My Super Program")
        .about("Controls the Explorer700 Shield for RPI")
        .subcommand(SubCommand::with_name("ipaddress").about("Show the IP sddress"))
        .subcommand(
            SubCommand::with_name("graphic").about("Show a square, a triangle and a circle"),
        )
        .subcommand(
            SubCommand::with_name("led1")
                .about("Switch LED 1 on or off")
                .arg(
                    Arg::with_name("STATE")
                        .index(1)
                        .possible_values(&["on", "off"])
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("led2")
                .about("Switch LED 2 on or off")
                .arg(
                    Arg::with_name("STATE")
                        .index(1)
                        .possible_values(&["on", "off"])
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("buzzer")
                .about("Turn the BUZZER on or off")
                .arg(
                    Arg::with_name("STATE")
                        .index(1)
                        .possible_values(&["on", "off"])
                        .required(true),
                ),
        )
        .subcommand(SubCommand::with_name("joystick").about("Read the state of the joystick"))
        .get_matches();
    if let Some(_matches) = matches.subcommand_matches("graphic") {
        return show_graphic();
    }
    if let Some(_matches) = matches.subcommand_matches("ipaddress") {
        return show_ip_address();
    }
    if let Some(submatches) = matches.subcommand_matches("led1") {
        let board = Explorer700::default();
        let mut led = board.init_led1();
        match submatches.value_of("STATE").unwrap() {
            "on" => led.on(),
            "off" => led.off(),
            _ => unreachable!(),
        }
    }
    if let Some(submatches) = matches.subcommand_matches("led2") {
        let board = Explorer700::default();
        let mut led = board.init_led2();
        match submatches.value_of("STATE").unwrap() {
            "on" => led.on(),
            "off" => led.off(),
            _ => unreachable!(),
        }
    }
    if let Some(submatches) = matches.subcommand_matches("buzzer") {
        let board = Explorer700::default();
        let mut buzzer = board.init_buzzer();
        match submatches.value_of("STATE").unwrap() {
            "on" => buzzer.on(),
            "off" => buzzer.off(),
            _ => unreachable!(),
        }
    }
    if let Some(_matches) = matches.subcommand_matches("joystick") {
        let board = Explorer700::default();
        let joystick = board.init_joystick();
        match joystick.state() {
            JoystickState::Up => println!("Up"),
            JoystickState::Down => println!("Down"),
            JoystickState::Left => println!("Left"),
            JoystickState::Right => println!("Right"),
            JoystickState::NoAction => println!("No_Action"),
        }
    }

    Ok(())
}

fn show_graphic() -> Result<(), std::io::Error> {
    let mut disp: GraphicDisplay128x64Monochrome = build_display();

    let yoffset = 20;

    let style = PrimitiveStyleBuilder::new()
        .stroke_width(1)
        .stroke_color(BinaryColor::On)
        .build();

    Rectangle::new(Point::new(0, 0), Point::new(127, 63))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // triangle
    Triangle::new(
        Point::new(16, 16 + yoffset),
        Point::new(16 + 16, 16 + yoffset),
        Point::new(16 + 8, yoffset),
    )
    .into_styled(style)
    .draw(&mut disp)
    .unwrap();

    // square
    Rectangle::new(Point::new(52, yoffset), Point::new(52 + 16, 16 + yoffset))
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    // circle
    Circle::new(Point::new(96, yoffset + 8), 8)
        .into_styled(style)
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    Ok(())
}

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
                    }
                }
            }
            None => {
                println!(
                    "interface {} with unsupported address family",
                    ifaddr.interface_name
                );
            }
        }
    }
    None
}

fn show_ip_address() -> Result<(), std::io::Error> {
    let mut disp: GraphicDisplay128x64Monochrome = build_display();

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
        .draw(&mut disp)
        .unwrap();

    disp.flush().unwrap();

    Ok(())
}
