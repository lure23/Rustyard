# Challenge!

## Example `a`

Trying to pass a Rust object, via C (which would be drivers), back to Rust.

This can be done if the type of the object is known. We'd *like* to have the actual type limited *only to the application* (not `lib`), and know the trait it implements.

This... is where the challenge lies.

### 1.

Nightly Rust would have decomposition of `dyn Trait` to `addr` and `meta`, and allow us to reassemble the "fat pointer" when resurfacing in Rust. Stable doesn't have this.

### 2.

In stable, we moved the `surface` function (that the C side calls) *up from the `lib` to the application*. This seems to be possible, and takes away the need for an intermediate `dyn Trait`, since the application knows the concrete type, and can convert to it directly.
