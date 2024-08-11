# rp-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for Raspberry Pi Silicon microcontrollers.


This crate has been automatically generated from the SVD files in [pico-sdk](https://github.com/raspberrypi/pico-sdk), using [chiptool](https://github.com/embassy-rs/chiptool/). Fixes are added to the SVD file to make the
crate more amenable to writing HALs with, such as converting sets of identical registers/fields to arrays, merging identical registers and enums, etc.

This crate is used for the [`embassy-rp`](github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the RPxxx microcontrollers.

## Supported chips

- **RP2040**: [Datasheet](https://datasheets.raspberrypi.org/rp2040/rp2040_datasheet.pdf)
- **RP2350A, RP2350B, RP2354A, RP2354B**: [Datasheet](https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf)

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by Raspberry Pi Trading Ltd under a BSD-3-Clause licence.
