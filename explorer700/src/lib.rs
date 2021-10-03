//! Provide JOY IT Explorer 700 raspberrypi shield as a business like API
//!
//! Business like API hides the specific pin binding and exposes types that
//!     read like JoystickLeftButton, Display, TemperatureSensor
//!
//! https://joy-it.net/en/products/RB-Explorer700

#[cfg(target_arch = "arm")]
#[cfg(target_os = "linux")]
pub mod explorer700;
