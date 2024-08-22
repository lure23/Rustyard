//! i2c_nudge.rs
//!
//! Based on -> esp-hal > examples > i2c_bmp180_calibration_data.rs
//!
//! Example for _very_ simple I2C access of a peripheral. Checks that the transport works.
//!
//! Wiring:
//!   - GPIO_4 -> SDA   // with|without pull-ups; tbd. test results!!
//!   - GPIO_5 -> SCL   // -''-
//!
#![no_std]
#![no_main]

#[allow(unused_imports)]
use defmt::{debug, info, error};
use defmt_rtt as _;

#[cfg(feature="embedded-hal")]
use embedded_hal::i2c::Operation as I2cOperation;
use embedded_hal::i2c::SevenBitAddress;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::Io,
    i2c::{I2C, Error as I2CError},
    delay::Delay,
    peripherals::{I2C0, Peripherals},
    prelude::*,
    system::SystemControl,
    Blocking,
};
use static_cell::StaticCell;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    let mut i2c: MyI2C = I2C::new(
        peripherals.I2C0,   // controller 0
        io.pins.gpio4,
        io.pins.gpio5,
        100.kHz(),
        &clocks,
        None
    );

    let delay_ms = {
        static D: StaticCell<Delay> = StaticCell::new();
        let d = D.init( Delay::new(&clocks) );
        |ms| d.delay_millis(ms)
    };

    loop {
        match vl_is_alive(&mut i2c) {
            Ok([dev_id, rev_id]) =>
                info!("Got: {=u8}, {=u8}", dev_id, rev_id),
            Err(e) =>
                error!("Failed: {:?}", e)
        }

        delay_ms(3000);
    }
}

type MyI2C = impl embedded_hal::i2c::I2c<SevenBitAddress>;

//type MyI2C<'a> = I2C<'a,I2C0,Blocking>;

// Pings the ST.com VL53L5CX time-of-flight sensor.
//
const I2C_ADDR: u8 = 0x52 >> 1;

fn vl_is_alive(i2c: &mut MyI2C) -> Result<[u8;2],I2CError>
{
    let mut buf: [u8; 2] = [0; 2];

    wr(i2c,0x7fff, &[0]);
    rd(i2c,0, &mut buf);       // indices 0,1 are (device id, rev id); should give (0xf0, 0x02)
    wr(i2c,0x7fff, &[2]);

    Ok(buf)
}

// Return codes were a nightmare (never got them right!) so instead panicking inside these two. #giving-up
//
fn rd(i2c: &mut MyI2C, index: u16, buf: &mut [u8]) /*-> Result<(),I2CError>*/ {
    i2c.write_read(I2C_ADDR, &index.to_be_bytes(), buf)
        .ok();
}

// Q1: no 'write_write' in 'esp-hal::i2c::I2C'
// Q2: any way to concatenate slices (of unknown size) together, in 'no_std' (without 'alloc')? Likely not,
//      unless the template approach is okay (would bloat size tho).
//
/*** could work; disabled
fn wr/*<N: usize>*/(i2c: &mut MyI2C, index: u16, vs: &[u8;1/*N*/]) /*-> Result<(),I2CError>*/ {
    let index: [u8;2] = index.to_be_bytes();
    let mut data: [u8;2+1/*N*/] = [index[0], index[1], 0];
    // loop 1..N, placing 'vs' in 'data'
    data[2] = vs[0];    // currently, just for one

    i2c.write(I2C_ADDR, &data).ok();
}
***/

// '.transaction' is only available if feature "embedded-hal" is enabled
//
#[cfg(feature="embedded-hal")]
fn wr(i2c: &mut MyI2C, index: u16, vs: &[u8]) {
    let index: [u8;2] = index.to_be_bytes();
    i2c.transaction(I2C_ADDR, &mut [I2cOperation::Write(&index), I2cOperation::Write(vs)])
        .ok();
}
