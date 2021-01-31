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
    pub fn ch0_csr(self) -> Reg<fields::Ch0Csr, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Ch0Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch0_div(self) -> Reg<fields::Ch0Div, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Ch0Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch0_ctr(self) -> Reg<fields::Ch0Ctr, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Ch0Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch0_cc(self) -> Reg<fields::Ch0Cc, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Ch0Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch0_top(self) -> Reg<fields::Ch0Top, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Ch0Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch1_csr(self) -> Reg<fields::Ch1Csr, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Ch1Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch1_div(self) -> Reg<fields::Ch1Div, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Ch1Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch1_ctr(self) -> Reg<fields::Ch1Ctr, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Ch1Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch1_cc(self) -> Reg<fields::Ch1Cc, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Ch1Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch1_top(self) -> Reg<fields::Ch1Top, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Ch1Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch2_csr(self) -> Reg<fields::Ch2Csr, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::Ch2Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch2_div(self) -> Reg<fields::Ch2Div, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Ch2Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch2_ctr(self) -> Reg<fields::Ch2Ctr, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Ch2Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch2_cc(self) -> Reg<fields::Ch2Cc, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Ch2Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch2_top(self) -> Reg<fields::Ch2Top, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Ch2Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch3_csr(self) -> Reg<fields::Ch3Csr, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Ch3Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch3_div(self) -> Reg<fields::Ch3Div, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Ch3Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch3_ctr(self) -> Reg<fields::Ch3Ctr, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::Ch3Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch3_cc(self) -> Reg<fields::Ch3Cc, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::Ch3Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch3_top(self) -> Reg<fields::Ch3Top, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::Ch3Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch4_csr(self) -> Reg<fields::Ch4Csr, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::Ch4Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch4_div(self) -> Reg<fields::Ch4Div, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::Ch4Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch4_ctr(self) -> Reg<fields::Ch4Ctr, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::Ch4Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch4_cc(self) -> Reg<fields::Ch4Cc, RW> {
        unsafe { Reg::new(self.0.add(92usize), fields::Ch4Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch4_top(self) -> Reg<fields::Ch4Top, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::Ch4Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch5_csr(self) -> Reg<fields::Ch5Csr, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::Ch5Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch5_div(self) -> Reg<fields::Ch5Div, RW> {
        unsafe { Reg::new(self.0.add(104usize), fields::Ch5Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch5_ctr(self) -> Reg<fields::Ch5Ctr, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::Ch5Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch5_cc(self) -> Reg<fields::Ch5Cc, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::Ch5Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch5_top(self) -> Reg<fields::Ch5Top, RW> {
        unsafe { Reg::new(self.0.add(116usize), fields::Ch5Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch6_csr(self) -> Reg<fields::Ch6Csr, RW> {
        unsafe { Reg::new(self.0.add(120usize), fields::Ch6Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch6_div(self) -> Reg<fields::Ch6Div, RW> {
        unsafe { Reg::new(self.0.add(124usize), fields::Ch6Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch6_ctr(self) -> Reg<fields::Ch6Ctr, RW> {
        unsafe { Reg::new(self.0.add(128usize), fields::Ch6Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch6_cc(self) -> Reg<fields::Ch6Cc, RW> {
        unsafe { Reg::new(self.0.add(132usize), fields::Ch6Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch6_top(self) -> Reg<fields::Ch6Top, RW> {
        unsafe { Reg::new(self.0.add(136usize), fields::Ch6Top::from_bits(65535)) }
    }
    #[doc = "Control and status register"]
    pub fn ch7_csr(self) -> Reg<fields::Ch7Csr, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::Ch7Csr::from_bits(0)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch7_div(self) -> Reg<fields::Ch7Div, RW> {
        unsafe { Reg::new(self.0.add(144usize), fields::Ch7Div::from_bits(16)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch7_ctr(self) -> Reg<fields::Ch7Ctr, RW> {
        unsafe { Reg::new(self.0.add(148usize), fields::Ch7Ctr::from_bits(0)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch7_cc(self) -> Reg<fields::Ch7Cc, RW> {
        unsafe { Reg::new(self.0.add(152usize), fields::Ch7Cc::from_bits(0)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch7_top(self) -> Reg<fields::Ch7Top, RW> {
        unsafe { Reg::new(self.0.add(156usize), fields::Ch7Top::from_bits(65535)) }
    }
    #[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    pub fn en(self) -> Reg<fields::En, RW> {
        unsafe { Reg::new(self.0.add(160usize), fields::En::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(164usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(168usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(172usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(176usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
