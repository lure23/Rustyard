//! a.rs
//!
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{debug, info, error};
use defmt_rtt as _;

use esp_backtrace as _;
#[cfg(feature = "dingdong")]
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    peripherals::Peripherals,
    system::SystemControl,
};

use esp_hal::prelude::*;

use static_cell::StaticCell;

use rustyard::{
    Context, RustPlatform, tunnel
};

#[entry]
fn main() -> ! {
    info!("Hello");

    #[cfg(feature = "dingdong")]
    {
        let peripherals = Peripherals::take();
        let system = SystemControl::new(peripherals.SYSTEM);
        let clocks = ClockControl::boot_defaults(system.clock_control).freeze();
        let d_provider = Delay::new(&clocks);
        let delay_ms: _ = {     // synchronous (busy wait) delays
            |ms| { d_provider.delay_millis(ms); }
        };

        let mut ding: bool = true;
        loop {
            delay_ms(1000);
            debug!("{}", if ding {"Ding!"} else {"Dong!"} );
            ding = !ding;
        }
    }

    let mp: *mut Context = {
        static X: StaticCell<RustPlatform> = StaticCell::new();
        let pt = X.init( RustPlatform::new() );

        &mut Context::new(pt) as _
    };

    unsafe {
        tunnel(mp)     // should cause 'pt::print' to be called (Rust -> C -> Rust)
    };
    debug!("Out of 'tunnel()'");

    loop {}
}

