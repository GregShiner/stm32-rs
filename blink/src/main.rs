#![no_std]
#![no_main]

// use semihosting panic handler
use panic_semihosting as _;

use cortex_m::asm;
use cortex_m_rt::entry;

use stm32h755_cm7_pac as cm7_pac;

#[entry]
fn main() -> ! {
    let peripherals = cm7_pac::Peripherals::take().unwrap();

    // Enable gpio port b clock
    peripherals.rcc.ahb4enr().write(|w| w.gpioben().bit(true));

    let gpiob = peripherals.gpiob;

    // Set gpio port b pin 14 mode to output
    gpiob.moder().write(|w| unsafe { w.mode14().bits(0b01) });
    // Set output type to push-pull
    gpiob.otyper().write(|w| w.ot14().bit(false));
    // Set output speed to medium speed
    gpiob
        .ospeedr()
        .write(|w| unsafe { w.ospeed14().bits(0b01) });
    // Set pin to use internal pull up resistor
    gpiob.pupdr().write(|w| unsafe { w.pupd14().bits(0b01) });

    let mut counter;
    loop {
        // Turn on LED
        gpiob.odr().write(|w| w.od14().bit(true));
        counter = 100000;
        while counter != 0 {
            counter -= 1;
            asm::nop();
        }

        // Turn off LED
        gpiob.odr().write(|w| w.od14().bit(false));
        counter = 100000;
        while counter != 0 {
            counter -= 1;
            asm::nop();
        }
    }
}
