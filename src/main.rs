#![deny(unsafe_code)]
#![no_main]
#![no_std]

use crate::hal::{prelude::*, stm32};
use cortex_m_rt::entry;
use hal::{delay::Delay, pwm};
use panic_halt as _;
use stm32f4xx_hal as hal;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        stm32::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();

        // configure pins
        let gpioa = dp.GPIOA.split();
        let channels = gpioa.pa6.into_alternate_af2();

        // configure PWM
        let mut ch1 = pwm::tim3(dp.TIM3, channels, clocks, 20u32.khz());

        let mut delay = Delay::new(cp.SYST, clocks);
        // initial state
        let max_duty = ch1.get_max_duty();
        ch1.set_duty(0);
        ch1.enable();
        let period = 400u32;
        loop {
            for offset in 1..=max_duty {
                ch1.set_duty(offset);
                delay.delay_us(period);
            }

            for offset in (0..max_duty).rev() {
                ch1.set_duty(offset);
                delay.delay_us(period);
            }
        }
    }

    loop {
        cortex_m::asm::nop();
    }
}
