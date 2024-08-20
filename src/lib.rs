#![no_std]

mod some;

#[cfg(feature = "defmt")]
use defmt::trace;

#[cfg(feature="defmt")]
use defmt::{debug, warn};

/**
* @brief App provides
*/
pub trait PlatformT {
    fn print_a(&mut self) -> ();
    fn print_b(&mut self) -> ();
}
