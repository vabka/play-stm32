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
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpioa = dp.GPIOA.split();
        let mut led1 = gpioa.pa6.into_push_pull_output();
        let mut led2 = gpioa.pa7.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, clocks);

        loop {
            led1.set_high().unwrap();
            led2.set_low().unwrap();

            delay.delay_ms(500u32);

            led1.set_low().unwrap();
            led2.set_high().unwrap();

            delay.delay_ms(500u32);
        }
    }

    loop {}
}
