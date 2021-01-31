use crate::generic::*;
#[doc = "Control and data interface to SAR ADC"]
#[derive(Copy, Clone)]
pub struct Adc(*mut u8);
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "ADC Control and Status"]
    pub fn cs(self) -> Reg<fields::Cs, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Cs::from_bits(0)) }
    }
    #[doc = "Result of most recent ADC conversion"]
    pub fn result(self) -> Reg<fields::Result, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Result::from_bits(0)) }
    }
    #[doc = "FIFO control and status"]
    pub fn fcs(self) -> Reg<fields::Fcs, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Fcs::from_bits(0)) }
    }
    #[doc = "Conversion result FIFO"]
    pub fn fifo(self) -> Reg<fields::Fifo, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Fifo::from_bits(0)) }
    }
    #[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
    pub fn div(self) -> Reg<fields::Div, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Div::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
