use crate::generic::*;
#[doc = "Simple PWM"]
#[derive(Copy, Clone)]
pub struct Pwm(*mut u8);
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Control and status register"]
    pub fn ch0_csr(self) -> Reg<regs::Ch0Csr, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch0_div(self) -> Reg<regs::Ch0Div, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch0_ctr(self) -> Reg<regs::Ch0Ctr, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch0_cc(self) -> Reg<regs::Ch0Cc, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch0_top(self) -> Reg<regs::Ch0Top, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch1_csr(self) -> Reg<regs::Ch1Csr, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch1_div(self) -> Reg<regs::Ch1Div, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch1_ctr(self) -> Reg<regs::Ch1Ctr, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch1_cc(self) -> Reg<regs::Ch1Cc, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch1_top(self) -> Reg<regs::Ch1Top, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch2_csr(self) -> Reg<regs::Ch2Csr, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch2_div(self) -> Reg<regs::Ch2Div, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch2_ctr(self) -> Reg<regs::Ch2Ctr, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch2_cc(self) -> Reg<regs::Ch2Cc, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch2_top(self) -> Reg<regs::Ch2Top, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch3_csr(self) -> Reg<regs::Ch3Csr, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch3_div(self) -> Reg<regs::Ch3Div, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch3_ctr(self) -> Reg<regs::Ch3Ctr, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch3_cc(self) -> Reg<regs::Ch3Cc, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch3_top(self) -> Reg<regs::Ch3Top, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch4_csr(self) -> Reg<regs::Ch4Csr, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch4_div(self) -> Reg<regs::Ch4Div, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch4_ctr(self) -> Reg<regs::Ch4Ctr, RW> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch4_cc(self) -> Reg<regs::Ch4Cc, RW> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch4_top(self) -> Reg<regs::Ch4Top, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch5_csr(self) -> Reg<regs::Ch5Csr, RW> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch5_div(self) -> Reg<regs::Ch5Div, RW> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch5_ctr(self) -> Reg<regs::Ch5Ctr, RW> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch5_cc(self) -> Reg<regs::Ch5Cc, RW> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch5_top(self) -> Reg<regs::Ch5Top, RW> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch6_csr(self) -> Reg<regs::Ch6Csr, RW> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch6_div(self) -> Reg<regs::Ch6Div, RW> {
        unsafe { Reg::new(self.0.add(124usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch6_ctr(self) -> Reg<regs::Ch6Ctr, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch6_cc(self) -> Reg<regs::Ch6Cc, RW> {
        unsafe { Reg::new(self.0.add(132usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch6_top(self) -> Reg<regs::Ch6Top, RW> {
        unsafe { Reg::new(self.0.add(136usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch7_csr(self) -> Reg<regs::Ch7Csr, RW> {
        unsafe { Reg::new(self.0.add(140usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch7_div(self) -> Reg<regs::Ch7Div, RW> {
        unsafe { Reg::new(self.0.add(144usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch7_ctr(self) -> Reg<regs::Ch7Ctr, RW> {
        unsafe { Reg::new(self.0.add(148usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch7_cc(self) -> Reg<regs::Ch7Cc, RW> {
        unsafe { Reg::new(self.0.add(152usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch7_top(self) -> Reg<regs::Ch7Top, RW> {
        unsafe { Reg::new(self.0.add(156usize)) }
    }
    #[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    pub fn en(self) -> Reg<regs::En, RW> {
        unsafe { Reg::new(self.0.add(160usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(164usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Inte, RW> {
        unsafe { Reg::new(self.0.add(168usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Intf, RW> {
        unsafe { Reg::new(self.0.add(172usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Ints, RW> {
        unsafe { Reg::new(self.0.add(176usize)) }
    }
}
pub mod regs;
pub mod vals;
