#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysinfo(pub *mut u8);
unsafe impl Send for Sysinfo {}
unsafe impl Sync for Sysinfo {}
impl Sysinfo {
    #[doc = "JEDEC JEP-106 compliant chip identifier."]
    #[inline(always)]
    pub fn chip_id(self) -> crate::common::Reg<regs::ChipId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Platform register. Allows software to know what environment it is running in."]
    #[inline(always)]
    pub fn platform(self) -> crate::common::Reg<regs::Platform, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Git hash of the chip source. Used to identify chip version."]
    #[inline(always)]
    pub fn gitref_rp2040(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
}
pub mod regs;
