#![cfg_attr(not(test), no_std)]
#![allow(non_camel_case_types)]

#[cfg(not(any(feature = "gd32e230", feature = "gd32e231", feature = "gd32e232")))]
compile_error!(
    "This crate requires one of the following features enabled: gd32e230, gd32e231, gd32e232"
);

use embedded_hal as hal;

#[cfg(feature = "gd32e230")]
pub use gd32e23x_pac::gd32e230 as pac;
#[cfg(feature = "gd32e231")]
pub use gd32e23x_pac::gd32e231 as pac;
#[cfg(feature = "gd32e232")]
pub use gd32e23x_pac::gd32e232 as pac;

pub mod adc;
pub mod aes;
pub mod calibration;
pub mod delay;
pub mod dma;
pub mod exti;
pub mod flash;
pub mod gpio;
pub mod i2c;
pub mod lptim;
pub mod prelude;
pub mod pwm;
pub mod pwr;
pub mod rcc;
pub mod rng;
pub mod rtc;
#[cfg(any(
    feature = "io-STM32L021",
    feature = "io-STM32L031",
    feature = "io-STM32L051",
    feature = "io-STM32L071",
))]
pub mod serial;
pub mod spi;
pub mod syscfg;
pub mod time;
pub mod timer;
#[cfg(all(
    feature = "stm32-usbd",
    any(feature = "gd32e231", feature = "gd32e232")
))]
pub mod usb;
pub mod watchdog;
