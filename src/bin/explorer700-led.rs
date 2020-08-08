
//! Power on user LED on Explorer 700 schield
//!
use linux_embedded_hal::{
    sysfs_gpio::Direction,
    Pin,
};


fn main() -> Result<(), std::io::Error> {
    // Configure SPI
    // Settings are taken from

    // Configure Led pin
    let dc = Pin::new(26); //
    dc.export().expect("dc export");
    while !dc.is_exported() {}
    dc.set_direction(Direction::Out).expect("dc Direction");
    dc.set_value(1).expect("dc Value set to 1");
    Ok (())

}
