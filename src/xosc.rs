use crate::generic::*;
#[doc = "Controls the crystal oscillator"]
#[derive(Copy, Clone)]
pub struct Xosc(*mut u8);
unsafe impl Send for Xosc {}
unsafe impl Sync for Xosc {}
impl Xosc {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Crystal Oscillator Control"]
    pub fn ctrl(self) -> Reg<fields::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Ctrl::from_bits(0)) }
    }
    #[doc = "Crystal Oscillator Status"]
    pub fn status(self) -> Reg<fields::Status, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Status::from_bits(0)) }
    }
    #[doc = "Crystal Oscillator pause control This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE WARNING: stop the PLLs before selecting dormant mode WARNING: setup the irq before selecting dormant mode"]
    pub fn dormant(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(8usize), 0) }
    }
    #[doc = "Controls the startup delay"]
    pub fn startup(self) -> Reg<fields::Startup, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Startup::from_bits(0)) }
    }
    #[doc = "A down counter running at the xosc frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
    pub fn count(self) -> Reg<fields::Count, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Count::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
