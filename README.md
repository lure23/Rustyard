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
