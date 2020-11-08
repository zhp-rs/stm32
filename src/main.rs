#![no_std]
#![no_main]

extern crate panic_semihosting;

use cortex_m::asm::delay;
use cortex_m_rt::entry;
use stm32ral::{gpio, modify_reg, rcc, read_reg};

#[entry]
fn main() -> ! {
    let rcc1 = rcc::RCC::take().unwrap();

    modify_reg!(rcc, rcc1, CR, HSEON: On);
    while read_reg!(rcc, rcc1, CR, HSERDY == NotReady) {}

    modify_reg!(rcc, rcc1, AHB4ENR, GPIOBEN: Enabled);
    let gpiob = gpio::GPIOB::take().unwrap();

    modify_reg!(
        gpio,
        gpiob,
        MODER,
        MODER0: Output,
        MODER7: Output,
        MODER14: Output
    );
    let mut val = gpio::ODR::ODR14::RW::Low;
    loop {
        modify_reg!(gpio, gpiob, ODR, ODR0: val, ODR7: val, ODR14: val);
        delay(100_000_000);
        if val == gpio::ODR::ODR14::RW::Low {
            val = gpio::ODR::ODR14::RW::High;
        } else {
            val = gpio::ODR::ODR14::RW::Low;
        }
    }
}
