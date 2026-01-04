//! Minimal example: blink the LED on the Nucleo G474RE board.
//!
//! This binary is built with `no_std` and `cortex-m-rt` and demonstrates a
//! simple loop that toggles a GPIO pin to turn an LED on and off.
//! Provides information via Real Time Transfer (RTT) logs 
//! Inline comments provide guidance for learning and documentation.

// Deny warnings and unsafe code to simplify teaching and testing.
// #![deny(warnings)]
#![deny(unsafe_code)]
// `no_main`: use the entry point provided by `cortex-m-rt`.
#![no_main]
// `no_std`: embedded environment without the standard library.
#![no_std]


// Import convenience traits for configuring pins and clocks.
use hal::prelude::*;
// Access device peripheral structures from the HAL.
use hal::stm32;
// Alias the HAL crate for consistent usage in the code.
use stm32g4xx_hal as hal;


// `#[entry]` macro marks the program entry point.
use cortex_m_rt::entry;

use core::panic::PanicInfo;

use defmt;

use defmt_rtt as _;


// Minimal panic handler for `no_std` embedded programs.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    defmt::error!("Error type: {}", _info);
    loop {}
}

// Application entry point.
#[entry]
fn main() -> ! {
    // Acquire access to microcontroller peripherals.
    // `take()` returns `Some(Peripherals)` only once; it will fail if
    // peripherals have already been taken elsewhere.
    let dp = stm32::Peripherals::take().expect("cannot take peripherals");

    // Build the Reset & Clock Control (RCC) configuration.
    let mut rcc = dp.RCC.constrain();
    // Hardware initialization.

    // Split GPIOA and GPIOC for pin configuration.
    let gpioa = dp.GPIOA.split(&mut rcc);
    let gpioc = dp.GPIOC.split(&mut rcc);

    // Configure PA5 as push-pull output — LED pin on Nucleo boards.
    let mut led = gpioa.pa5.into_push_pull_output();
    // Configure PC13 as input. No need to me mutable, we're only reading it.
    let button = gpioc.pc13.into_floating_input();
    
    // Delay initial value
    let mut delay_value = 100_000_u32;

    // Initially, led is setted low
    led.set_low().unwrap();
    defmt::info!("All Set");
    // Main loop: toggle the LED with simple busy-wait delays.
    loop {
        // Call delay funcion and update delay variable once done
        delay_value = loop_delay(delay_value, &button);
        
        // Toggle LED
        defmt::info!("Toggle! Delay Value: {}", delay_value);
        led.toggle().unwrap();
    }
}

// Delay Function
fn loop_delay<P: InputPin>(mut delay_value: u32, button: &P) -> u32 {
    let mut change_delay: u8 = 0;
    // Loop for until value of del
    for _i in 1..(delay_value) {
        if button.is_high().unwrap_or(false) {
            // Para mostrar o efeito do bouncing do botão no terminal
            defmt::info!("Bounce!");
            change_delay = 1;
        }
    }
    if change_delay == 1 {
        // If button pressed decrease the delay value
        delay_value -= 25_000_u32;
        if delay_value < 25_000_u32 {
            delay_value = 100_000_u32;
        }
    }
    delay_value
}
