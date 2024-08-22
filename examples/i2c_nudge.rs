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

    let mut i2c = I2C::new(
        peripherals.I2C0,   // controller 0
        io.pins.gpio4,
        io.pins.gpio5,
        100.kHz(),
        &clocks,
        None
    );
    let mut i2c = MyI2C::new(I2C_ADDR, i2c);

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

type MyI2C = I2C<'_,I2C0,Blocking>;

// Pings the ST.com VL53L5CX time-of-flight sensor.
//
const I2C_ADDR: u8 = 0x52 >> 1;

fn vl_is_alive(i2c: &mut MyI2C) -> Result<[u8;2],I2CError>
{
    let mut buf: [u8; 2] = [0; 2];

    i2c.wr(0x7fff, &[0]);
    i2c.rd(0, &mut buf);       // indices 0,1 are (device id, rev id); should give (0xf0, 0x02)
    i2c.wr(0x7fff, &[2]);

    Ok(buf)
}

// Return codes were a nightmare (never got them right!) so instead panicking inside these two. #giving-up
//
fn rd(&mut i2c: MyI2C, index: u16, buf: &mut [u8]) /*-> Result<(),I2CError>*/ {
    i2c.write_read(I2C_ADDR, &index.to_be_bytes(), buf)
        .ok();
}

// Q1: no 'write_write' in 'esp-hal::i2c::I2C'
// Q2: any way to concatenate slices (of unknown size) together, in 'no_std'?
//
fn wr(&mut i2c: MyI2C, index: u16, vs: &[u8;1]) /*-> Result<(),I2CError>*/ {
    let index: [u8;2] = index.to_be_bytes();
    let data: [u8;3] = [index[0], index[1], vs[0]];
    i2c.write(I2C_ADDR, &data).ok();
}

/*** reserve
trait I2C_WW {
    fn write_write(&mut self, addr: u8, prelude: &[u8;2], bytes: &[u8]) -> Result<(), I2C::Error>;
}
impl I2C_WW for I2C<I2C0,Blocking> {
    // For symmetry with 'write_read' - ideas are welcome!!!
    //
    fn write_write(&mut self, addr: u8, prelude: &[u8;2], bytes: &[u8]) -> Result<(), I2C::Error> {

        // #hack Since we don't know how to concatenate '[u8]'s in 'no_std', we support only the
        //      1-byte long case.
        //
        #[cfg(not(feature = "embedded-hal"))]
        let wr = |index: u16, vs: &[u8;1]| -> Result<(),_> {
            let index: [u8;2] = index.to_be_bytes();
            let data: [u8;3] = [index[0], index[1], vs[0]];
            i2c.write(I2C_ADDR, &data)
        };

        // '.transaction' taken from how 'embedded_hal' implements '.write_read()'.
        //
        // tbd. If there's a way to '[addr, *vs].concat()' (concat slice-of-slices to a slice),
        //      we don't need to do the '.transaction' (and don't need to specifically import 'embedded_hal').
        #[cfg(feature = "embedded-hal")]
        let wr = |addr: u16, vs: &[u8]| -> Result<(),_> {
            let addr: [u8;2] = addr.to_be_bytes();
            i2c.transaction(I2C_ADDR, &mut [I2cOperation::Write(&addr), I2cOperation::Write(vs)])
        };

    }
}
***/