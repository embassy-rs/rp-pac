use crate::generic::*;
#[doc = "Register block to control RTC"]
#[derive(Copy, Clone)]
pub struct Rtc(pub *mut u8);
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    #[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    pub fn clkdiv_m1(self) -> Reg<regs::ClkdivM1, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "RTC setup register 0"]
    pub fn setup_0(self) -> Reg<regs::Setup0, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "RTC setup register 1"]
    pub fn setup_1(self) -> Reg<regs::Setup1, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "RTC Control and status"]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Interrupt setup register 0"]
    pub fn irq_setup_0(self) -> Reg<regs::IrqSetup0, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Interrupt setup register 1"]
    pub fn irq_setup_1(self) -> Reg<regs::IrqSetup1, RW> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "RTC register 1."]
    pub fn rtc_1(self) -> Reg<regs::Rtc1, RW> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "RTC register 0 Read this before RTC 1!"]
    pub fn rtc_0(self) -> Reg<regs::Rtc0, RW> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Inte, RW> {
        unsafe { Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Intf, RW> {
        unsafe { Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Ints, RW> {
        unsafe { Reg::from_ptr(self.0.add(44usize)) }
    }
}
pub mod regs;
