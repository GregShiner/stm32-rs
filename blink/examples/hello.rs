//! Prints "Hello, world!" on the host console using semihosting

#![no_main]
#![no_std]

// use panic_halt as _;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};

extern crate panic_semihosting;

#[entry]
fn main() -> ! {
    hprintln!("Hello, world!");
    panic!("AHHHHHHHHHHHHH");

    // exit QEMU
    // NOTE do not run this on hardware; it can corrupt OpenOCD state
    // debug::exit(debug::EXIT_SUCCESS);

    loop {}
}
