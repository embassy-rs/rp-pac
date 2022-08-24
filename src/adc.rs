#[doc = "Control and data interface to SAR ADC"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc(pub *mut u8);
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[doc = "ADC Control and Status"]
    #[inline(always)]
    pub fn cs(self) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Result of most recent ADC conversion"]
    #[inline(always)]
    pub fn result(self) -> crate::common::Reg<regs::Result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "FIFO control and status"]
    #[inline(always)]
    pub fn fcs(self) -> crate::common::Reg<regs::Fcs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Conversion result FIFO"]
    #[inline(always)]
    pub fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
    #[inline(always)]
    pub fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
}
pub mod regs;
