#[doc = "Simple PWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm(pub *mut u8);
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    #[inline(always)]
    pub fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inte(self) -> crate::common::Reg<regs::Inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub fn intf(self) -> crate::common::Reg<regs::Intf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub fn ints(self) -> crate::common::Reg<regs::Ints, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
    #[inline(always)]
    pub fn ch(self, n: usize) -> Channel {
        assert!(n < 8usize);
        unsafe { Channel(self.0.add(0usize + n * 20usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel(pub *mut u8);
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn csr(self) -> crate::common::Reg<regs::ChCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn div(self) -> crate::common::Reg<regs::ChDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ctr(self) -> crate::common::Reg<regs::ChCtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn cc(self) -> crate::common::Reg<regs::ChCc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn top(self) -> crate::common::Reg<regs::ChTop, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
}
pub mod regs;
pub mod vals;
