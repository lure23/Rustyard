use core::ffi::c_void;
use core::ptr::NonNull;
use crate::some::Context;
use defmt::{info, Format};

impl Context {
    /*
    * Create a context that has (or can point to) our Rust-side 'RustPlatform'
    */
    pub fn new(pt: &'static mut RustPlatform) -> Self {
        Self {
            vp: pt as *mut RustPlatform as *mut c_void,
            count: 0
        }
    }
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

    pub fn print(&self) {
        info!("Platform is: {}", self)
    }
}

#[no_mangle]
extern "C"
fn surface(vp: *mut c_void) {   // Called by C; 'vp' actually is '*mut RustPlatform'

    // turn
    let p: &mut RustPlatform = unsafe {
        NonNull::new_unchecked(vp).cast::<RustPlatform>().as_mut()
    };

    p.print();
}
