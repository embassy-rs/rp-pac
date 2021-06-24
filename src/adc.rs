#[doc = "Control and data interface to SAR ADC"]
#[derive(Copy, Clone)]
pub struct Adc(pub *mut u8);
unsafe impl Send for Adc {}
unsafe impl Sync for Adc {}
impl Adc {
    #[doc = "ADC Control and Status"]
    pub fn cs(self) -> crate::common::Reg<regs::Cs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Result of most recent ADC conversion"]
    pub fn result(self) -> crate::common::Reg<regs::Result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "FIFO control and status"]
    pub fn fcs(self) -> crate::common::Reg<regs::Fcs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Conversion result FIFO"]
    pub fn fifo(self) -> crate::common::Reg<regs::Fifo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
    pub fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
}
pub mod regs;
