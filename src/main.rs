#![deny(unsafe_code)]
#![no_main]
#![no_std]

use crate::hal::{prelude::*, stm32};
use cortex_m;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();

    let gpio = dp.GPIOA.split();

    let mut led = gpio.pa6.into_push_pull_output();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

    loop {
        for _ in 0..3 {
            led.set_high().unwrap();
            delay.delay_ms(200u32);
            led.set_low().unwrap();
            delay.delay_ms(100u32);
        }

        for _ in 0..3 {
            led.set_high().unwrap();
            delay.delay_ms(600u32);
            led.set_low().unwrap();
            delay.delay_ms(100u32);
        }
    }
}
