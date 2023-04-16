# rp-pac

This is a [Peripheral Access Crate](https://rust-embedded.github.io/book/start/registers.html) for Raspberry Pi Silicon microcontrollers.


This crate has been automatically generated from the SVD file in [pico-sdk v1.5.0](https://github.com/raspberrypi/pico-sdk/blob/1.5.0/src/rp2040/hardware_regs/rp2040.svd), using [chiptool](https://github.com/embassy-rs/chiptool/). Fixes are ade to the SVD file to make the
crate more amenable to writing HALs with, such as converting sets of identical registers/fields to arrays, merging identical registers and enums, etc.

This crate is used for the [`embassy-rp`](github.com/embassy-rs/embassy/) Rust Hardware Abstraction Layer (HAL) for the RP2040 microcontroller.

## Supported chips

- **RP2040**: [Datasheet](https://datasheets.raspberrypi.org/rp2040/rp2040_datasheet.pdf)

## License

The contents of this crate are auto-generated and licensed under the same terms as the underlying SVD file, which is licensed by Raspberry Pi Trading Ltd under a BSD-3-Clause licence.