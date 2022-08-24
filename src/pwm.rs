#[doc = "Simple PWM"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm(pub *mut u8);
unsafe impl Send for Pwm {}
unsafe impl Sync for Pwm {}
impl Pwm {
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch0_csr(self) -> crate::common::Reg<regs::Ch0csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch0_div(self) -> crate::common::Reg<regs::Ch0div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch0_ctr(self) -> crate::common::Reg<regs::Ch0ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch0_cc(self) -> crate::common::Reg<regs::Ch0cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch0_top(self) -> crate::common::Reg<regs::Ch0top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch1_csr(self) -> crate::common::Reg<regs::Ch1csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch1_div(self) -> crate::common::Reg<regs::Ch1div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch1_ctr(self) -> crate::common::Reg<regs::Ch1ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch1_cc(self) -> crate::common::Reg<regs::Ch1cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch1_top(self) -> crate::common::Reg<regs::Ch1top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch2_csr(self) -> crate::common::Reg<regs::Ch2csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch2_div(self) -> crate::common::Reg<regs::Ch2div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch2_ctr(self) -> crate::common::Reg<regs::Ch2ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch2_cc(self) -> crate::common::Reg<regs::Ch2cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch2_top(self) -> crate::common::Reg<regs::Ch2top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch3_csr(self) -> crate::common::Reg<regs::Ch3csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch3_div(self) -> crate::common::Reg<regs::Ch3div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch3_ctr(self) -> crate::common::Reg<regs::Ch3ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch3_cc(self) -> crate::common::Reg<regs::Ch3cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch3_top(self) -> crate::common::Reg<regs::Ch3top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch4_csr(self) -> crate::common::Reg<regs::Ch4csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch4_div(self) -> crate::common::Reg<regs::Ch4div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch4_ctr(self) -> crate::common::Reg<regs::Ch4ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch4_cc(self) -> crate::common::Reg<regs::Ch4cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch4_top(self) -> crate::common::Reg<regs::Ch4top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch5_csr(self) -> crate::common::Reg<regs::Ch5csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch5_div(self) -> crate::common::Reg<regs::Ch5div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch5_ctr(self) -> crate::common::Reg<regs::Ch5ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch5_cc(self) -> crate::common::Reg<regs::Ch5cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch5_top(self) -> crate::common::Reg<regs::Ch5top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch6_csr(self) -> crate::common::Reg<regs::Ch6csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch6_div(self) -> crate::common::Reg<regs::Ch6div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch6_ctr(self) -> crate::common::Reg<regs::Ch6ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch6_cc(self) -> crate::common::Reg<regs::Ch6cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch6_top(self) -> crate::common::Reg<regs::Ch6top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub fn ch7_csr(self) -> crate::common::Reg<regs::Ch7csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
    #[inline(always)]
    pub fn ch7_div(self) -> crate::common::Reg<regs::Ch7div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "Direct access to the PWM counter"]
    #[inline(always)]
    pub fn ch7_ctr(self) -> crate::common::Reg<regs::Ch7ctr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "Counter compare values"]
    #[inline(always)]
    pub fn ch7_cc(self) -> crate::common::Reg<regs::Ch7cc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }
    #[doc = "Counter wrap value"]
    #[inline(always)]
    pub fn ch7_top(self) -> crate::common::Reg<regs::Ch7top, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }
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
}
pub mod regs;
pub mod vals;
