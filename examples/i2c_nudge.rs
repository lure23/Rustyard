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

use embedded_hal::i2c::{
    //I2c,
    Error as I2cError,
    Operation as I2cOperation
};

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::Io,
    i2c::{I2C, Error as I2CError},
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
    Blocking
};
use esp_hal::peripherals::I2C0;
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

// Pings the ST.com VL53L5CX time-of-flight sensor.
//
const I2C_ADDR: u8 = 0x52 >> 1;

fn vl_is_alive(i2c: &mut I2C<I2C0,Blocking>) -> Result<[u8;2],I2cError>
{
    let mut buf: [u8; 2] = [0; 2];

    let rd = |addr: u16, buf: &mut [u8]| -> Result<(),_> {
        let addr: [u8;2] = addr.to_be_bytes();
        i2c.write_read(I2C_ADDR, &addr, buf)
    };

    // '.transaction' taken from how 'embedded_hal' implements '.write_read()'.
    //
    // tbd. If there's a way to '[addr, *vs].concat()' (concat slice-of-slices to a slice),
    //      we don't need to do the '.transaction' (and don't need to specifically import 'embedded_hal').
    let wr = |addr: u16, vs: &[u8]| -> Result<(),_> {
        let addr: [u8;2] = addr.to_be_bytes();

        i2c.transaction(I2C_ADDR, &mut [I2cOperation::Write(&addr), I2cOperation::Write(vs)])
    };

    wr(0x7fff, &[0])?;
    rd(0, &mut buf)?;            // rd indices 0,1 (device id, rev id); should give (0xf0, 0x02)
    wr(0x7fff, &[2])?;

    Ok(buf)
}
