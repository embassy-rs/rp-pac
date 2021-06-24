#[doc = "Register block to control RTC"]
#[derive(Copy, Clone)]
pub struct Rtc(pub *mut u8);
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    pub fn clkdiv_m1(self) -> crate::common::Reg<regs::ClkdivM1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "RTC setup register 0"]
    pub fn setup_0(self) -> crate::common::Reg<regs::Setup0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "RTC setup register 1"]
    pub fn setup_1(self) -> crate::common::Reg<regs::Setup1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "RTC Control and status"]
    pub fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Interrupt setup register 0"]
    pub fn irq_setup_0(self) -> crate::common::Reg<regs::IrqSetup0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Interrupt setup register 1"]
    pub fn irq_setup_1(self) -> crate::common::Reg<regs::IrqSetup1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "RTC register 1."]
    pub fn rtc_1(self) -> crate::common::Reg<regs::Rtc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "RTC register 0 Read this before RTC 1!"]
    pub fn rtc_0(self) -> crate::common::Reg<regs::Rtc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
}
pub mod regs;
