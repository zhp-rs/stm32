#![no_std]
#![no_main]

extern crate cortex_m_rt as rt;
use rt::entry;

extern crate cortex_m as cm;
extern crate panic_semihosting;

extern crate stm32ral;
use stm32ral::gpio;
use stm32ral::modify_reg;
use stm32ral::rcc;
use stm32ral::read_reg;

#[entry]
fn main() -> ! {
    // We can `take()` each peripheral to provide safe synchronised access.
    let rcc1 = rcc::RCC::take().unwrap();

    // modify_reg!(rcc, rcc1, CR, HSEON: Off);
    // modify_reg!(rcc, rcc1, CR, HSEBYP: Bypassed);
    // rcc1.CR.write(rcc::RCC::reset.CR);
    modify_reg!(rcc, rcc1, CR, HSEON: On);
    while read_reg!(rcc, rcc1, CR, HSERDY == NotReady) {}

    modify_reg!(rcc, rcc1, AHB4ENR, GPIOBEN: Enabled);
    let gpiob = gpio::GPIOB::take().unwrap();

    modify_reg!(
        gpio,
        gpiob,
        MODER,
        MODER14: Output,
        MODER0: Output,
        MODER7: Output
    );
    loop {
        modify_reg!(gpio, gpiob, ODR, ODR14: Low, ODR0: Low, ODR7: Low);
        cortex_m::asm::delay(100_000_000);
        modify_reg!(gpio, gpiob, ODR, ODR14: High, ODR0: High, ODR7: High);
        cortex_m::asm::delay(100_000_000);
    }
}
