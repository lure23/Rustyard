// Adapted from -> https://github.com/esp-rs/esp-hal/blob/main/examples/src/bin/hello_rgb.rs
//  - supporting less chips (only C3, C6)
//  - changed some coding
//
// WIRING:
//  - GPIO8 -> Smart led    | is so on both ESP32-C3 and -C6 official boards

#![no_std]
#![no_main]

const BRIGHTNESS: u8 = 5;   // 0..10 (technically, could go to 255 but do not)
const DELAY: u32 = 200;

#[allow(unused_imports)]
use defmt::{debug, info, Format, write};
use defmt::Formatter;
use defmt_rtt as _;

use esp_backtrace as _;
use esp_hal::{
    clock::ClockControl,
    delay::Delay,
    gpio::Io,
    peripherals::Peripherals,
    prelude::*,
    rmt::Rmt,
    system::SystemControl,
};
use esp_hal_smartled::{smartLedBuffer, SmartLedsAdapter};
use rand::{
    //Rng,
    RngCore,
    SeedableRng,
    rngs::SmallRng
};

use smart_leds::{
    brightness,
    gamma,
    hsv::{hsv2rgb, Hsv},
    SmartLedsWrite,
    RGB8
};
use static_cell::StaticCell;

#[entry]
fn main() -> ! {
    info!("🌈");

    let peripherals = Peripherals::take();
    let system = SystemControl::new(peripherals.SYSTEM);
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);

    // We _only_ support ESP32-C3 and -C6, both of which happen to have the led in GPIO8.
    // (this means we need no features)
    //
    let led_pin = io.pins.gpio8;    // GPIO_8

    // Configure RMT peripheral globally
    let rmt = Rmt::new(peripherals.RMT, 80.MHz(), &clocks, None).unwrap();

    // Use one of the RMT channels to instantiate a `SmartLedsAdapter` which can be used directly
    // with all `smart_led` implementations.    <-- tbd. reformulate to one line
    let rmt_buffer = smartLedBuffer!(1);
    let mut led = SmartLedsAdapter::new(rmt.channel0, led_pin, rmt_buffer, &clocks);

    let delay_ms = {
        static D: StaticCell<Delay> = StaticCell::new();
        let d = D.init( Delay::new(&clocks) );
        |ms| d.delay_millis(ms)
    };

    let mut next_u32 /* || -> u32 */ = {
        static R: StaticCell<SmallRng> = StaticCell::new();
        let srng = R.init( SmallRng::seed_from_u64(0) );     // nb. repeating doesn't matter to us
        || srng.next_u32()
    };

    let mut color = Hsv {
        hue: 0,
        sat: 0,
        val: 0,
    };

    // There's a reason the value is defined here; helps its lifespan cover the 'loop'.
    let mut data;

    loop {
        let r = next_u32();     // Random HSV, each time!
        color.hue = (r >> 24) as u8;
        color.sat = (r >> 16) as u8;
        color.val = r as u8;

        // HSV: we can easily transition from one color to the other
        // RGB: what the LED wants
        let rgb = RGB8_Wrap(hsv2rgb(color));
        debug!("Cycling: {}", rgb);
        data = [rgb];

        // Gamma correction first (as seen in 'smart_leds' docs). Limiting brightness is
        // IMPORTANT FOR YOUR EYE SIGHT! (max 10) 🧑‍🔬🥽
        //
        let it = { gamma(data.iter().map(|x| &x.0).cloned()) };

        led.write(brightness(it,BRIGHTNESS))
            .unwrap();

        delay_ms(DELAY);
    }
}

#[allow(non_camel_case_types)]
struct RGB8_Wrap(RGB8);

impl Format for RGB8_Wrap {
    fn format(&self, fmt: Formatter) {
        let (r,g,b) = (self.0.r, self.0.g, self.0.b);
        write!(fmt, "{=u8:02x}{=u8:02x}{=u8:02x}", r,g,b);
    }
}