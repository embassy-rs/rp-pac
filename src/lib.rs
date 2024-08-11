#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg(all(feature = "rp2040", feature = "rp235x"))]
compile_error!("You must not enable both the `rp2040` and `rp235x` Cargo features.");
#[cfg(not(any(feature = "rp2040", feature = "rp235x")))]
compile_error!("You must enable either the `rp2040` or the `rp235x` Cargo features.");

#[cfg_attr(feature = "rp2040", path = "./rp2040/mod.rs")]
#[cfg_attr(feature = "rp235x", path = "./rp235x/mod.rs")]
mod inner;
pub use inner::*;
