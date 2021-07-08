# Embedded Linux controlled Hardware Manager


This is a rust based project making use of

* embedded-hal and linux-embedded-hal
* embedded-graphics

SPI, I2c and GPIOs are heavily used.

The default target is set is set to *armv7-unknown-linux-gnueabihf*


Many of the examples work fine with RaspberryPi 3 and Explorer 700 JoyIt shield.

## Notes

* RaspberryPI must have switch I2c and SPI devices on on kernel level


# Building on linux host

## Crosscompile environment

The cross compile environment can be easily setup like this (on an upbuntu system)

```
sudo apt install gcc-arm-linux-gnueabihf
rustup target add armv7-unknown-linux-gnueabihf

```

See all the details for cross compiling at https://github.com/japaric/rust-cross