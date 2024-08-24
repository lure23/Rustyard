# Rust yard

Minimal experiments to see, how things can be done in Rust.

## Requires

- Gnu `make`
- `bindgen` >= 0.70.0
- `probe-rs` > 0.24.0

	```
	$ cargo install probe-rs-tools --git https://github.com/probe-rs/probe-rs --locked --force
	```
	

## Running

```
$ [DEFMT_LOG=debug] cargo run --release --example a
```

### `i2c_nudge` example

```
$ cargo run --release --features=embedded-hal --example i2c_nudge
```

The code serves as a sample on how to use the I2C bus. It uses the ST.com [VL53L5CX-SATEL](https://www.st.com/en/evaluation-tools/vl53l5cx-satel.html) breakout board<!-- #whisper marking "4.322" on the PCB -->. 

To run the example, wire these:

|ESP32-C3/C6|signal|SATEL|
|---|---|---|
|GPIO4|SDA|SDA|
|GPIO5|SCL|SCL|
|---|chip enable|LPn, via 47kΩ to GND|
|---|power enable|PWREN, via 22kΩ to AVDD (5V)|

<!--
|(GPIO6)|reset; active high|I2C_RST|
-->
Without having its firmware uploaded (by a driver), the board isn't fully operational. You may get:

**with board connected**

```
INFO  Got: 0x03, 0x00
```

>Note: The right values should be `0xf0`, `0x02`.

**without the board**

```
ERROR Failed with: AckCheckFailed
```

## Troubleshooting

### ESP32-C3 dev board not showing log output

```
$ DEFMT_LOG=debug cargo run --release --example a
   Compiling rustyard v0.0.0 (/home/ubuntu/Rustyard)
    Finished `release` profile [optimized + debuginfo] target(s) in 54.20s
     Running `probe-rs run --chip=esp32c3 '--log-format={L} {s}' target/riscv32imc-unknown-none-elf/release/examples/a`
      Erasing ✔ [00:00:02] [#################] 192.00 KiB/192.00 KiB @ 85.21 KiB/s (eta 0s )
  Programming ✔ [00:00:11] [##################] 22.52 KiB/22.52 KiB @ 2.01 KiB/s (eta 0s )
  Finished in 11.232046s

```

Occasionally, they need resetting to a "download state". Do this by:

- Press both `RESET` and `BOOT` buttons
- Release `RESET`
- Release `BOOT`
- reconnect using USB/IP: e.g. `sudo usbip attach -r 192.168.1.29 -b 3-1`
- try flashing and running

The logs should show like this:

```
INFO  Hello
INFO  Platform is: RustPlatform { a: 2, b: 772 }
DEBUG Out of 'tunnel()'
```

>Note: This is a common thing using ESP32-C3 boards. Has not been observed on ESP32-C6 (ESP32-DevKit-M01, in particular).
>
>If you have both boards, use `set-target` and retry on that one.


## Advanced

You can change the target board between ESP32-C3 (default) and ESP32-C6, by:

```
$ ./set-target.sh
```
