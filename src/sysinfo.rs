use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Sysinfo(*mut u8);
unsafe impl Send for Sysinfo {}
unsafe impl Sync for Sysinfo {}
impl Sysinfo {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "JEDEC JEP-106 compliant chip identifier."]
    pub fn chip_id(self) -> Reg<fields::ChipId, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::ChipId::from_bits(0)) }
    }
    #[doc = "Platform register. Allows software to know what environment it is running in."]
    pub fn platform(self) -> Reg<fields::Platform, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Platform::from_bits(0)) }
    }
    #[doc = "Git hash of the chip source. Used to identify chip version."]
    pub fn gitref_rp2040(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(64usize), 0) }
    }
}
pub mod fields;
