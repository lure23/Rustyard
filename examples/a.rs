//! a.rs
//!
#![no_std]
#![no_main]
extern crate alloc;

use alloc::string::String;
#[allow(unused_imports)]
use defmt::{debug, info, error};
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};

// The struct that keeps state, passed through the C side.
//
struct MyPlatform {
    a: u8,
    b: String
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let d_provider = Delay::new(&clocks);
    let delay_ms: _ = {     // synchronous (busy wait) delays
        |ms| { d_provider.delay_millis(ms); }
    };

    info!("Hello");

    let mut ding: bool = true;
    loop {
        delay_ms(1000);
        debug!("{}", if ding {"Ding!"} else {"Dong!"} ); ding = !ding;
    }
}
