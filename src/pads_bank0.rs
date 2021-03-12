use crate::generic::*;
#[derive(Copy, Clone)]
pub struct PadsBank0(*mut u8);
unsafe impl Send for PadsBank0 {}
unsafe impl Sync for PadsBank0 {}
impl PadsBank0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Voltage select. Per bank control"]
    pub fn voltage_select(self) -> Reg<regs::VoltageSelect, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio0(self) -> Reg<regs::Gpio0, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio1(self) -> Reg<regs::Gpio1, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio2(self) -> Reg<regs::Gpio2, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio3(self) -> Reg<regs::Gpio3, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio4(self) -> Reg<regs::Gpio4, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio5(self) -> Reg<regs::Gpio5, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio6(self) -> Reg<regs::Gpio6, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio7(self) -> Reg<regs::Gpio7, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio8(self) -> Reg<regs::Gpio8, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio9(self) -> Reg<regs::Gpio9, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio10(self) -> Reg<regs::Gpio10, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio11(self) -> Reg<regs::Gpio11, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio12(self) -> Reg<regs::Gpio12, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio13(self) -> Reg<regs::Gpio13, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio14(self) -> Reg<regs::Gpio14, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio15(self) -> Reg<regs::Gpio15, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio16(self) -> Reg<regs::Gpio16, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio17(self) -> Reg<regs::Gpio17, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio18(self) -> Reg<regs::Gpio18, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio19(self) -> Reg<regs::Gpio19, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio20(self) -> Reg<regs::Gpio20, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio21(self) -> Reg<regs::Gpio21, RW> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio22(self) -> Reg<regs::Gpio22, RW> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio23(self) -> Reg<regs::Gpio23, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio24(self) -> Reg<regs::Gpio24, RW> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio25(self) -> Reg<regs::Gpio25, RW> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio26(self) -> Reg<regs::Gpio26, RW> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio27(self) -> Reg<regs::Gpio27, RW> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio28(self) -> Reg<regs::Gpio28, RW> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio29(self) -> Reg<regs::Gpio29, RW> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    #[doc = "Pad control register"]
    pub fn swclk(self) -> Reg<regs::Swclk, RW> {
        unsafe { Reg::new(self.0.add(124usize)) }
    }
    #[doc = "Pad control register"]
    pub fn swd(self) -> Reg<regs::Swd, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
}
pub mod regs;
