#[doc = "Control interface to HSTX. For FIFO write access and status, see the HSTX_FIFO register block."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HstxCtrl {
    ptr: *mut u8,
}
unsafe impl Send for HstxCtrl {}
unsafe impl Sync for HstxCtrl {}
impl HstxCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Data control register for output bit 0"]
    #[inline(always)]
    pub const fn bit_(self, n: usize) -> crate::common::Reg<regs::Bit, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Configure the optional shifter inside the command expander"]
    #[inline(always)]
    pub const fn expand_shift(self) -> crate::common::Reg<regs::ExpandShift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Configure the optional TMDS encoder inside the command expander"]
    #[inline(always)]
    pub const fn expand_tmds(self) -> crate::common::Reg<regs::ExpandTmds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs;
