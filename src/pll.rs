#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pll(pub *mut u8);
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz"]
    #[inline(always)]
    pub fn cs(self) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Controls the PLL power modes."]
    #[inline(always)]
    pub fn pwr(self) -> crate::common::Reg<regs::Pwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
    #[inline(always)]
    pub fn fbdiv_int(self) -> crate::common::Reg<regs::FbdivInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
    #[inline(always)]
    pub fn prim(self) -> crate::common::Reg<regs::Prim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
