#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll {
    ptr: *mut u8,
}
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz"]
    #[inline(always)]
    pub const fn cs(self) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Controls the PLL power modes."]
    #[inline(always)]
    pub const fn pwr(self) -> crate::common::Reg<regs::Pwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
    #[inline(always)]
    pub const fn fbdiv_int(self) -> crate::common::Reg<regs::FbdivInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
    #[inline(always)]
    pub const fn prim(self) -> crate::common::Reg<regs::Prim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
pub mod regs;
