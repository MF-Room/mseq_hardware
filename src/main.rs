//! This example demonstrates how to use the RTC.
//! Note that the LSI can be quite inaccurate.
//! The tolerance is up to ±47% (Min 17 kHz, Typ 32 kHz, Max 47 kHz).

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use stm32f4xx_hal::{pac, prelude::*, rtc::Rtc};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("Hello world");

    loop {}
}
