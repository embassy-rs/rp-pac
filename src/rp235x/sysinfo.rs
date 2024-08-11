#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysinfo {
    ptr: *mut u8,
}
unsafe impl Send for Sysinfo {}
unsafe impl Sync for Sysinfo {}
impl Sysinfo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "JEDEC JEP-106 compliant chip identifier."]
    #[inline(always)]
    pub const fn chip_id(self) -> crate::common::Reg<regs::ChipId, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[inline(always)]
    pub const fn package_sel(self) -> crate::common::Reg<regs::PackageSel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM."]
    #[inline(always)]
    pub const fn platform(self) -> crate::common::Reg<regs::Platform, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Git hash of the chip source. Used to identify chip version."]
    #[inline(always)]
    pub const fn gitref_rp2350(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
}
pub mod regs;
