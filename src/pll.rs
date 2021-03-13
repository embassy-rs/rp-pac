use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Pll(pub *mut u8);
unsafe impl Send for Pll {}
unsafe impl Sync for Pll {}
impl Pll {
    #[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=400MHz, max=1600MHz"]
    pub fn cs(self) -> Reg<regs::Cs, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Controls the PLL power modes."]
    pub fn pwr(self) -> Reg<regs::Pwr, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
    pub fn fbdiv_int(self) -> Reg<regs::FbdivInt, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
    pub fn prim(self) -> Reg<regs::Prim, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
