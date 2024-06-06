### Building

Find out the GTK 4 version on your machine by running

`$ pkg-config --modversion gtk4`

At the time of this writing the newest version is 4.12. Set the required version as a `gkt` feature in `Cargo.toml`. For example, version `4.6.x` would be `4_6`.

`$ cargo build --release`

### Running/Debugging

`$ cargo run` will build and run the _debug_ profile by default.

`$ cargo run --release`
