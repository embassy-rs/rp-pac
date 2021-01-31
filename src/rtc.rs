use crate::generic::*;
#[doc = "Register block to control RTC"]
#[derive(Copy, Clone)]
pub struct Rtc(*mut u8);
unsafe impl Send for Rtc {}
unsafe impl Sync for Rtc {}
impl Rtc {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Divider minus 1 for the 1 second counter. Safe to change the value when RTC is not enabled."]
    pub fn clkdiv_m1(self) -> Reg<fields::ClkdivM1, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::ClkdivM1::from_bits(0)) }
    }
    #[doc = "RTC setup register 0"]
    pub fn setup_0(self) -> Reg<fields::Setup0, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Setup0::from_bits(0)) }
    }
    #[doc = "RTC setup register 1"]
    pub fn setup_1(self) -> Reg<fields::Setup1, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Setup1::from_bits(0)) }
    }
    #[doc = "RTC Control and status"]
    pub fn ctrl(self) -> Reg<fields::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Ctrl::from_bits(0)) }
    }
    #[doc = "Interrupt setup register 0"]
    pub fn irq_setup_0(self) -> Reg<fields::IrqSetup0, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::IrqSetup0::from_bits(0)) }
    }
    #[doc = "Interrupt setup register 1"]
    pub fn irq_setup_1(self) -> Reg<fields::IrqSetup1, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::IrqSetup1::from_bits(0)) }
    }
    #[doc = "RTC register 1."]
    pub fn rtc_1(self) -> Reg<fields::Rtc1, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Rtc1::from_bits(0)) }
    }
    #[doc = "RTC register 0 Read this before RTC 1!"]
    pub fn rtc_0(self) -> Reg<fields::Rtc0, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Rtc0::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
