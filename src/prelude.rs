pub use embedded_hal::{
    adc::OneShot as _,
    digital::v2::*,
    prelude::*,
    timer::Cancel as _,
    watchdog::{Watchdog as _, WatchdogEnable as _},
};

pub use crate::{
    adc::AdcExt as _,
    delay::DelayExt as _,
    gpio::GpioExt as _,
    pwr::PowerMode as _,
    rcc::RccExt as _,
    spi::SpiExt as _,
    time::U32Ext as _,
    timer::TimerExt as _,
    watchdog::{IndependedWatchdogExt as _, WindowWatchdogExt as _},
};

#[cfg(any(
    feature = "io-STM32L021",
    feature = "io-STM32L031",
    feature = "io-STM32L051",
    feature = "io-STM32L071",
))]
pub use crate::i2c::I2cExt as _;

#[cfg(any(feature = "io-STM32L051", feature = "io-STM32L071",))]
pub use crate::serial::Serial1LpExt as _;
#[cfg(any(
    feature = "io-STM32L021",
    feature = "io-STM32L031",
    feature = "io-STM32L051",
    feature = "io-STM32L071",
))]
pub use crate::serial::{Serial1LpExt as _, Serial2Ext as _};
#[cfg(any(feature = "io-STM32L071",))]
pub use crate::serial::{Serial4Ext as _, Serial5Ext as _};
