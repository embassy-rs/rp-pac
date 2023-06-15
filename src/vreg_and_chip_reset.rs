#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregAndChipReset {
    ptr: *mut u8,
}
unsafe impl Send for VregAndChipReset {}
unsafe impl Sync for VregAndChipReset {}
impl VregAndChipReset {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Voltage regulator control and status"]
    #[inline(always)]
    pub const fn vreg(self) -> crate::common::Reg<regs::Vreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "brown-out detection control"]
    #[inline(always)]
    pub const fn bod(self) -> crate::common::Reg<regs::Bod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Chip reset control and status"]
    #[inline(always)]
    pub const fn chip_reset(self) -> crate::common::Reg<regs::ChipReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
pub mod regs;
