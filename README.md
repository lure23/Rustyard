# Rust yard

Minimal experiments to see, how things can be done in Rust.

## Requires

- Gnu `make`
- `bindgen` >= 0.70.0

## Steps

```
$ DEFMT_LOG=debug cargo build --release --example a
```

```
$ probe-rs run --chip=esp32c3 '--log-format={L} {s}' target/riscv32imc-unknown-none-elf/release/examples/a
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
$ set-target esp32c6
```
