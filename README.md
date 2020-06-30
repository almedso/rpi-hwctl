# Embedded Linux controlled Hardware Manager


This is a rust based project making use of

* embedded-hal and linux-embedded-hal
* embedded-graphics

SPI, I2c and GPIOs are heavily used.

The default target is set is set to *armv7-unknown-linux-gnueabihf*


Many of the examples work fine with RaspberryPi 3 and Explorer 700 JoyIt shield.

## Notes

* RaspberryPI must have switch I2c and SPI devices on on kernel level
