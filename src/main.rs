//! This example demonstrates how to use the RTC.
//! Note that the LSI can be quite inaccurate.
//! The tolerance is up to ±47% (Min 17 kHz, Typ 32 kHz, Max 47 kHz).

#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use stm32f4xx_hal::{pac, prelude::*, rtc::Rtc};

#[entry]
fn main() -> ! {
    // Initilialize RTT
    rtt_init_print!();

    // Initilialize allocator
    allocator_init();

    rprintln!("Hello world");

    let mut test: Vec<u32> = vec![];

    loop {
        rprintln!("Test: {}", test[0]);
    }
}

fn allocator_init() {
    const HEAP_SIZE: usize = 1024;
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
