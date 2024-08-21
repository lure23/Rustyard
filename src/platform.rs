use core::ffi::c_void;
use crate::some::Context;

pub trait Platform {
    fn print(&self);
}

impl Context {
    /*
    * Create a context that has (or can point to) our Rust-side 'RustPlatform'
    */
    pub fn new<P : Platform>(pt: &'static mut P) -> Self {
        Self {
            vp: pt as *mut P as *mut c_void,
            count: 0
        }
    }
}
