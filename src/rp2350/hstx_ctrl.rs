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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Data control register for output bit 0"]
    #[inline(always)]
    pub const fn bit0(self) -> crate::common::Reg<regs::Bit0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Data control register for output bit 1"]
    #[inline(always)]
    pub const fn bit1(self) -> crate::common::Reg<regs::Bit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Data control register for output bit 2"]
    #[inline(always)]
    pub const fn bit2(self) -> crate::common::Reg<regs::Bit2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Data control register for output bit 3"]
    #[inline(always)]
    pub const fn bit3(self) -> crate::common::Reg<regs::Bit3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Data control register for output bit 4"]
    #[inline(always)]
    pub const fn bit4(self) -> crate::common::Reg<regs::Bit4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Data control register for output bit 5"]
    #[inline(always)]
    pub const fn bit5(self) -> crate::common::Reg<regs::Bit5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Data control register for output bit 6"]
    #[inline(always)]
    pub const fn bit6(self) -> crate::common::Reg<regs::Bit6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Data control register for output bit 7"]
    #[inline(always)]
    pub const fn bit7(self) -> crate::common::Reg<regs::Bit7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Configure the optional shifter inside the command expander"]
    #[inline(always)]
    pub const fn expand_shift(self) -> crate::common::Reg<regs::ExpandShift, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Configure the optional TMDS encoder inside the command expander"]
    #[inline(always)]
    pub const fn expand_tmds(self) -> crate::common::Reg<regs::ExpandTmds, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
}
pub mod regs;
