### Device code

This embedded rust program runs on the ch32 line of chips via the [ch32-hal](https://github.com/ch32-rs/ch32-hal/tree/main). It requires Nightly Rust at the time of this publication. The exact chip model is defined in the `ch32-hal` features in `Cargo.toml`.

To flash and monitor SDI output, install [wlink](https://github.com/ch32-rs/wlink). Consult the docs for your chip model's SDI pins.

`$ cargo run --release`

### Functionality

The program reads pressure, temperature and altitude from the BMP085 or BMP180 Bosch sensors and sends the serialized data via CAN bus.

### Dependency showcase

Using [bmp085-180-rs](https://crates.io/crates/bmp085-180-rs) driver Crate to read BMP data and [ch32-can-rs](https://github.com/marti157/ch32-can-rs) HAL to interface with the CAN bus.
