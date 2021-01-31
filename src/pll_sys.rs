use crate::generic::*;
#[derive(Copy, Clone)]
pub struct PllSys(*mut u8);
unsafe impl Send for PllSys {}
unsafe impl Sync for PllSys {}
impl PllSys {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=400MHz, max=1600MHz"]
    pub fn cs(self) -> Reg<fields::Cs, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Cs::from_bits(1)) }
    }
    #[doc = "Controls the PLL power modes."]
    pub fn pwr(self) -> Reg<fields::Pwr, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Pwr::from_bits(45)) }
    }
    #[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
    pub fn fbdiv_int(self) -> Reg<fields::FbdivInt, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::FbdivInt::from_bits(0)) }
    }
    #[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
    pub fn prim(self) -> Reg<fields::Prim, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Prim::from_bits(487424)) }
    }
}
pub mod fields;
