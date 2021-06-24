#[doc = "Simple PWM"]
#[derive(Copy, Clone)]
pub struct Pwm(pub *mut u8);
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[doc = "Control and status register"]
    pub fn ch0_csr(self) -> crate::common::Reg<regs::Ch0Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch0_div(self) -> crate::common::Reg<regs::Ch0Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch0_ctr(self) -> crate::common::Reg<regs::Ch0Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch0_cc(self) -> crate::common::Reg<regs::Ch0Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch0_top(self) -> crate::common::Reg<regs::Ch0Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch1_csr(self) -> crate::common::Reg<regs::Ch1Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch1_div(self) -> crate::common::Reg<regs::Ch1Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch1_ctr(self) -> crate::common::Reg<regs::Ch1Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch1_cc(self) -> crate::common::Reg<regs::Ch1Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch1_top(self) -> crate::common::Reg<regs::Ch1Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch2_csr(self) -> crate::common::Reg<regs::Ch2Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch2_div(self) -> crate::common::Reg<regs::Ch2Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch2_ctr(self) -> crate::common::Reg<regs::Ch2Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch2_cc(self) -> crate::common::Reg<regs::Ch2Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch2_top(self) -> crate::common::Reg<regs::Ch2Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch3_csr(self) -> crate::common::Reg<regs::Ch3Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch3_div(self) -> crate::common::Reg<regs::Ch3Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch3_ctr(self) -> crate::common::Reg<regs::Ch3Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch3_cc(self) -> crate::common::Reg<regs::Ch3Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch3_top(self) -> crate::common::Reg<regs::Ch3Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch4_csr(self) -> crate::common::Reg<regs::Ch4Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch4_div(self) -> crate::common::Reg<regs::Ch4Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch4_ctr(self) -> crate::common::Reg<regs::Ch4Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch4_cc(self) -> crate::common::Reg<regs::Ch4Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch4_top(self) -> crate::common::Reg<regs::Ch4Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch5_csr(self) -> crate::common::Reg<regs::Ch5Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch5_div(self) -> crate::common::Reg<regs::Ch5Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch5_ctr(self) -> crate::common::Reg<regs::Ch5Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch5_cc(self) -> crate::common::Reg<regs::Ch5Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch5_top(self) -> crate::common::Reg<regs::Ch5Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch6_csr(self) -> crate::common::Reg<regs::Ch6Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch6_div(self) -> crate::common::Reg<regs::Ch6Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch6_ctr(self) -> crate::common::Reg<regs::Ch6Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch6_cc(self) -> crate::common::Reg<regs::Ch6Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch6_top(self) -> crate::common::Reg<regs::Ch6Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "Control and status register"]
    pub fn ch7_csr(self) -> crate::common::Reg<regs::Ch7Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    pub fn ch7_div(self) -> crate::common::Reg<regs::Ch7Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    pub fn ch7_ctr(self) -> crate::common::Reg<regs::Ch7Ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "Counter compare values"]
    pub fn ch7_cc(self) -> crate::common::Reg<regs::Ch7Cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }
    #[doc = "Counter wrap value"]
    pub fn ch7_top(self) -> crate::common::Reg<regs::Ch7Top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }
    #[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
    pub fn en(self) -> crate::common::Reg<regs::En, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> crate::common::Reg<regs::Inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> crate::common::Reg<regs::Intf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> crate::common::Reg<regs::Ints, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
}
pub mod regs;
pub mod vals;
