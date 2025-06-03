#![no_std]
#![no_main]

use core::ops::BitOr;

// pick a panicking behavior
use panic_semihosting as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;

use volatile_register::{RO, RW, WO};

type RccAhb4enr = RW<u32>;

#[repr(C)]
struct GpioPort {
    mode: RW<u32>,
    otype: RW<u32>,
    ospeed: RW<u32>,
    pupd: RW<u32>,
    idr: RO<u32>,
    odr: RW<u32>,
    bsr: WO<u32>,
    lckr: RW<u32>,
    afrl: RW<u32>,
    afrh: RW<u32>,
}

#[inline(always)]
fn set_bits<T: BitOr<Output = T>>(bits: T) -> impl FnOnce(T) -> T {
    |val| val | bits
}

#[inline(always)]
fn clear_bits<T: core::ops::Not + core::ops::BitAnd<<T as core::ops::Not>::Output, Output = T>>(
    bits: T,
) -> impl FnOnce(T) -> T {
    |val| val & !bits
}

#[inline(always)]
fn set_clear_bits<
    T: BitOr<Output = T>
        + core::ops::Not
        + core::ops::BitAnd<<T as core::ops::Not>::Output, Output = T>,
>(
    set_bits: T,
    clear_bits: T,
) -> impl FnOnce(T) -> T {
    |val| (val | set_bits) & (!clear_bits)
}

#[entry]
fn main() -> ! {
    let rcc_ahb4enr = unsafe { &mut *(0x580244E0 as *mut RccAhb4enr) };
    let gpio_b = unsafe { &mut *(0x58020400 as *mut GpioPort) };
    unsafe {
        // rcc_ahb4enr.write(rcc_ahb4enr.read() | 0b10);
        rcc_ahb4enr.modify(set_bits(0b10));
        gpio_b.mode.modify(set_clear_bits(0b1 << 28, 0b1 << 29));
        gpio_b.otype.modify(clear_bits(0b1 << 14));
        gpio_b.ospeed.modify(set_bits(0b1 << 28));
        gpio_b.pupd.modify(set_bits(0b1 << 28));
    }
    let mut counter;
    loop {
        unsafe {
            gpio_b.odr.modify(set_bits(0b1 << 14));
        }
        counter = 100000;
        while counter != 0 {
            counter -= 1;
            asm::nop();
        }

        unsafe {
            gpio_b.odr.modify(clear_bits(0b1 << 14));
        }
        counter = 100000;
        while counter != 0 {
            counter -= 1;
            asm::nop();
        }
    }
}
