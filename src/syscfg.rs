//! Interface to the SYSCFG peripheral
//!
//! See STM32L0x2 reference manual, chapter 10.

use crate::{pac, rcc::Rcc};

type PacSyscfg = pac::SYSCFG;

pub struct SYSCFG {
    pub(crate) syscfg: PacSyscfg,
}

impl SYSCFG {
    pub fn new(syscfg: PacSyscfg, rcc: &mut Rcc) -> Self {
        // Reset SYSCFG peripheral
        rcc.rb.apb2rstr.modify(|_, w| w.syscfgrst().set_bit());
        rcc.rb.apb2rstr.modify(|_, w| w.syscfgrst().clear_bit());

        // Enable SYSCFG peripheral
        rcc.rb.apb2enr.modify(|_, w| w.syscfgen().set_bit());

        SYSCFG { syscfg }
    }
}
