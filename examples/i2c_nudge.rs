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

use core::cell::RefCell;

#[cfg(feature="embedded-hal")]
use embedded_hal::i2c::{
    Operation as I2cOperation
};

use embedded_hal::i2c::SevenBitAddress;
use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    gpio::Io,
    i2c::I2C,
    delay::Delay,
    peripherals::Peripherals,
    prelude::*,
    system::SystemControl,
};
use static_cell::StaticCell;

const I2C_ADDR: u8 = 0x52;  // default of VL sensor

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // Resource sharing the I2C bus is not needed by this simple sample, but comes handy with
    // more than one device on the bus.
    //
    let i2c = RefCell::new( I2C::new(
        peripherals.I2C0,   // controller
        io.pins.gpio4,
        io.pins.gpio5,
        100.kHz(),          // try 400
        &clocks,
        None
    ));
    let mut vl: VL<_> = VL::new(i2c, I2C_ADDR);

    let delay_ms = {
        static D: StaticCell<Delay> = StaticCell::new();
        let d = D.init( Delay::new(&clocks) );
        |ms| d.delay_millis(ms)
    };

    loop {
        match vl.is_alive() {
            Ok([dev_id,rev_id]) =>
                info!("Got: {=u8:#04x}, {=u8:#04x}", dev_id, rev_id),     // expecting '0xf0', '0x02'
            Err(e) =>
                error!("Failed with: {}", e)
        }

        delay_ms(3000);
    }
}

struct VL<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>
{
    bus: RefCell<T>,
    addr: u8
}

impl<T> VL<T>
    where T: embedded_hal::i2c::I2c<SevenBitAddress>
{
    // Cannot do this, because -> https://github.com/rust-lang/rust/issues/8995#issuecomment-1569208403
    //type Result<X> = core::result::Result<X,T::Error>;
    //type E = T::Error;

    fn new(bus: RefCell<T>, addr_8b: u8) -> Self {
        Self {
            bus,
            addr: addr_8b >> 1     // make 8-bit addr to 7-bit
        }
    }

    // Pings the ST.com VL53L5CX time-of-flight sensor.
    //
    fn is_alive(&mut self) -> Result<[u8;2],T::Error> {
        let mut buf = [u8::MAX;2];

        self.wr(0x7fff, &[0])?;
        self.rd(0, &mut buf)?;       // indices 0,1 are (device id, rev id); should give (0xf0, 0x02)
        self.wr(0x7fff, &[2])?;
        Ok(buf)
    }

    // Return codes were a nightmare (never got them right!) so instead panicking inside these two. #giving-up
    //
    fn rd(&mut self, index: u16, buf: &mut [u8]) -> Result<(),T::Error> {
        let mut bus = self.bus.borrow_mut();
        bus.write_read(self.addr, &index.to_be_bytes(), buf)
    }

    // '.transaction' is only available if feature "embedded-hal" is enabled (wonder why)
    //
    #[cfg(feature = "embedded-hal")]
    fn wr(&mut self, index: u16, vs: &[u8]) -> Result<(),T::Error> {
        // Cannot concatenate 'index' and 'vs' without 'alloc' (could, if we limit or use generics
        // on the length), but '.transaction' allows the writes to be atomic on the bus.

        let mut bus = self.bus.borrow_mut();
        let index: [u8; 2] = index.to_be_bytes();
        bus.transaction(self.addr, &mut [I2cOperation::Write(&index), I2cOperation::Write(vs)])
    }
}
