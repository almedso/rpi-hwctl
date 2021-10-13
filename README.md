# Embedded Linux controlled Hardware Manager

An embedded linux hardware with simple display (oled/epaper) and a few button
plus a web connection is controlled by a wep api.

Accessing device requires being litterally in front of the hardware operating the
display via buttons. e.g. (configure wifi, read out ip address, connect to web URI,
reset the administration connection)

This is a rust based project making use of

* embedded-hal and linux-embedded-hal
* embedded-graphics

The implementation specifically addresses a RaspberryPi (3) 
with explorer 700 JoyIt shield attachted.

Thus, SPI, I2c and GPIOs are heavily used.
The default target is set is set to *armv7-unknown-linux-gnueabihf*

## Notes

* RaspberryPI must have switch I2c and SPI devices on on kernel level
* `build` command is configured to use raspberry cpu architecture as default.
* `run` command is configured to (in .cargo) to deploy on a raspberry named ricardo as root.
  Precondition: ricardo is up and running and root user can login via ssh priv/pub key w/o password

## Project Structure

* It is a binary crate - just one *main.rs* file
* explorer 700 JoyIt shield is a separated crate within this repository

# Building

## Crosscompile environment

The cross compile environment can be easily setup like this (on an Ubuntu system)

```
sudo apt install gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf

```

See all the details for cross compiling at https://github.com/japaric/rust-cross
