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
    pub fn cs(self) -> Reg<regs::Cs, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Result of most recent ADC conversion"]
    pub fn result(self) -> Reg<regs::Result, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "FIFO control and status"]
    pub fn fcs(self) -> Reg<regs::Fcs, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Conversion result FIFO"]
    pub fn fifo(self) -> Reg<regs::Fifo, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
    pub fn div(self) -> Reg<regs::Div, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Inte, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Intf, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Ints, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
}
pub mod regs;
