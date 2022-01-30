//! Demonstrate the use of a blocking `Delay` using the SYST (sysclock) timer.

#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

// Halt on panic
use crate::hal::serial::config::Config;
use panic_halt as _;
use stm32f4xx_hal::pac::USART3;
use stm32f4xx_hal::serial::Tx;

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*, serial::Serial};

use core::fmt::Write; // for pretty formatting of the serial output

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the LED. On the Nucleo-446RE it's connected to pin PA5.
        let gpiog = dp.GPIOG.split();
        let mut led = gpiog.pg6.into_push_pull_output();

        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = hal::delay::Delay::new(cp.SYST, &clocks);

        // define RX/TX pins
        let gpiob = dp.GPIOB.split();

        let rx_pin = gpiob.pb11.into_alternate::<7>();
        let tx_pin = gpiob.pb10.into_alternate::<7>();

        //Result<Tx<USART, WORD>, config::InvalidConfig>
        let mut tx: Tx<USART3, u8> = Serial::tx(dp.USART3, tx_pin, 9600.bps(), &clocks).unwrap();

        /*
                // configure serial
                let serial = Serial::new(
                    dp.USART3,
                    (tx_pin, rx_pin),
                    Config::default().baudrate(9600.bps()).wordlength_9(),
                    &clocks,
                )
                .unwrap()
                // Make this Serial object use u16s instead of u8s
                .with_u16_data();
        */

        let mut value = 0;

        loop {
            // On for 1s, off for 1s.
            led.set_high();
            delay.delay_ms(500_u32);
            led.set_low();
            delay.delay_ms(1000_u32);
            writeln!(tx, "value: {:02}\r", value).unwrap();
            value = value + 1;
        }
    }

    loop {}
}
