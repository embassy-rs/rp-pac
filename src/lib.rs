#![no_std]
#![allow(non_camel_case_types)]
#![doc = include_str!("../README.md")]

#[cfg(feature = "rp2040")]
pub mod rp2040;
#[cfg(feature = "rp2040")]
pub use rp2040::*;

#[cfg(feature = "rp235x")]
pub mod rp2350;
#[cfg(feature = "rp235x")]
pub use rp2350::*;
