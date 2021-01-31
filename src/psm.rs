use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Psm(*mut u8);
unsafe impl Send for Psm {}
unsafe impl Sync for Psm {}
impl Psm {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Force block out of reset (i.e. power it on)"]
    pub fn frce_on(self) -> Reg<fields::FrceOn, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::FrceOn::from_bits(0)) }
    }
    #[doc = "Force into reset (i.e. power it off)"]
    pub fn frce_off(self) -> Reg<fields::FrceOff, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::FrceOff::from_bits(0)) }
    }
    #[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
    pub fn wdsel(self) -> Reg<fields::Wdsel, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Wdsel::from_bits(0)) }
    }
    #[doc = "Indicates the peripheral's registers are ready to access."]
    pub fn done(self) -> Reg<fields::Done, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Done::from_bits(0)) }
    }
}
pub mod fields;
