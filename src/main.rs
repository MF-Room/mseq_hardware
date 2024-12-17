#![no_main]
#![no_std]

mod midi;

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

extern crate alloc;
use alloc::vec;
use alloc::vec::Vec;
use embedded_alloc::LlffHeap as Heap;
use midi::Midi;

#[global_allocator]
static HEAP: Heap = Heap::empty();

#[entry]
fn main() -> ! {
    // Initilialize RTT
    rtt_init_print!();

    // Initilialize allocator
    allocator_init();

    // Initilialize Midi
    let mut midi = Midi::new();

    let mut test: Vec<u32> = vec![];
    test.push(1);
    rprintln!("Test: {}", test[0]);

    loop {
        midi.read_u8().map(|x| rprintln!("MIDI read: {}", x));
    }
}

fn allocator_init() {
    const HEAP_SIZE: usize = 1024;
    use core::mem::MaybeUninit;
    static mut HEAP_MEM: [MaybeUninit<u8>; HEAP_SIZE] = [MaybeUninit::uninit(); HEAP_SIZE];
    unsafe { HEAP.init(HEAP_MEM.as_ptr() as usize, HEAP_SIZE) }
}
