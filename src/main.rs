//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _;
use stm32f4xx_hal::pac::USART3;
use stm32f4xx_hal::serial::Tx;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, serial::Serial};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let cp = cortex_m::peripheral::Peripherals::take().unwrap();
    let gpiog = dp.GPIOG.split();
    let mut led = gpiog.pg6.into_push_pull_output();

    // Set up the system clock. We want to run at 48MHz for this one.
    let rcc = dp.RCC.constrain();
    let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

    // Create a delay abstraction based on SysTick
    let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

    // define RX/TX pins
    let gpiob = dp.GPIOB.split();
    //let rx_pin = gpiob.pb11.into_alternate::<7>();
    let tx_pin = gpiob.pb10.into_alternate::<7>();
    let mut tx: Tx<USART3, u8> = Serial::tx(dp.USART3, tx_pin, 9600.bps(), &clocks).unwrap();

    let mut value = 0;

    loop {
        // On for 1s, off for 1s.
        led.set_high();
        delay.delay_ms(200_u32);
        led.set_low();
        delay.delay_ms(1000_u32);
        writeln!(tx, "value: {:02}\r", value).unwrap();
        value = value + 1;
    }
}
