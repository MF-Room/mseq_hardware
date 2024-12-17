#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use embedded_alloc::LlffHeap as Heap;

#[global_allocator]
static HEAP: Heap = Heap::empty();

use stm32f4xx_hal::{
    pac,
    prelude::*,
    serial::{self, config::StopBits, Config, Serial},
};

#[entry]
fn main() -> ! {
    // Initilialize RTT
    rtt_init_print!();

    // Initilialize allocator
    allocator_init();

    let dp = pac::Peripherals::take().unwrap();

    // Initialize the clock
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.freeze();

    // Configure UART
    let gpioa = dp.GPIOA.split();
    let rx_1 = gpioa.pa10.into_alternate();
    let tx_1 = gpioa.pa9.into_alternate();

    let mut uart1 = Serial::new(
        dp.USART1,
        (tx_1, rx_1),
        Config::default()
            .baudrate(31250.bps())
            .parity_none()
            .stopbits(StopBits::STOP1)
            .wordlength_8(),
        &clocks,
    )
    .unwrap()
    .with_u8_data();

    let mut test: Vec<u32> = vec![];
    test.push(1);
    rprintln!("Test: {}", test[0]);

    loop {
        let res1 = uart1.read();
        if let Ok(res) = res1 {
            rprintln!("MIDI read: {}", res);
        }
    }
}

fn allocator_init() {
    const HEAP_SIZE: usize = 1024;
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
