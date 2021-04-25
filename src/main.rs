#![no_std]
#![no_main]

use panic_semihosting as _;
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

    modify_reg!(gpio, gpiob, MODER, MODER0: Output, MODER7: Output, MODER14: Output);

    loop {
        modify_reg!(gpio, gpiob, ODR, ODR0: Low, ODR7: High, ODR14: Low);
        delay(10_000_000);
        modify_reg!(gpio, gpiob, ODR, ODR0: High, ODR7: Low, ODR14: High);
        delay(10_000_000);
    }
}
