//! a.rs
//!
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{debug, info, error, Format};
use defmt_rtt as _;

use core::{
    ffi::c_void,
    ptr::NonNull
};
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

use rustyard::{Context, tunnel, Platform};

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

// The struct that keeps state, passed through the C side.
//
#[derive(Format)]
pub struct RustPlatform {
    a: u8,
    b: u16
}

impl RustPlatform {
    pub fn new() -> Self {
        Self {
            a: 2, b: 0x0304
        }
    }
}

impl Platform for RustPlatform {
    fn print(&self) {
        info!("Platform is: {}", self)
    }
}

#[no_mangle]
extern "C"
fn surface(vp: *mut c_void) {   // Called by C; 'vp' actually is '*mut RustPlatform'

    let p: &mut RustPlatform = unsafe {
        NonNull::new_unchecked(vp).cast::<RustPlatform>().as_mut()
    };

    p.print();
}
