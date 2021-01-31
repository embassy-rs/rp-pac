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
    pub fn voltage_select(self) -> Reg<fields::VoltageSelect, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::VoltageSelect::from_bits(0)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio0(self) -> Reg<fields::Gpio0, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Gpio0::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio1(self) -> Reg<fields::Gpio1, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Gpio1::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio2(self) -> Reg<fields::Gpio2, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Gpio2::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio3(self) -> Reg<fields::Gpio3, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Gpio3::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio4(self) -> Reg<fields::Gpio4, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Gpio4::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio5(self) -> Reg<fields::Gpio5, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Gpio5::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio6(self) -> Reg<fields::Gpio6, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Gpio6::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio7(self) -> Reg<fields::Gpio7, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Gpio7::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio8(self) -> Reg<fields::Gpio8, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Gpio8::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio9(self) -> Reg<fields::Gpio9, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::Gpio9::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio10(self) -> Reg<fields::Gpio10, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Gpio10::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio11(self) -> Reg<fields::Gpio11, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Gpio11::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio12(self) -> Reg<fields::Gpio12, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Gpio12::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio13(self) -> Reg<fields::Gpio13, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Gpio13::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio14(self) -> Reg<fields::Gpio14, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Gpio14::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio15(self) -> Reg<fields::Gpio15, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Gpio15::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio16(self) -> Reg<fields::Gpio16, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::Gpio16::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio17(self) -> Reg<fields::Gpio17, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::Gpio17::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio18(self) -> Reg<fields::Gpio18, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::Gpio18::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio19(self) -> Reg<fields::Gpio19, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::Gpio19::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio20(self) -> Reg<fields::Gpio20, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::Gpio20::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio21(self) -> Reg<fields::Gpio21, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::Gpio21::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio22(self) -> Reg<fields::Gpio22, RW> {
        unsafe { Reg::new(self.0.add(92usize), fields::Gpio22::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio23(self) -> Reg<fields::Gpio23, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::Gpio23::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio24(self) -> Reg<fields::Gpio24, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::Gpio24::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio25(self) -> Reg<fields::Gpio25, RW> {
        unsafe { Reg::new(self.0.add(104usize), fields::Gpio25::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio26(self) -> Reg<fields::Gpio26, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::Gpio26::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio27(self) -> Reg<fields::Gpio27, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::Gpio27::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio28(self) -> Reg<fields::Gpio28, RW> {
        unsafe { Reg::new(self.0.add(116usize), fields::Gpio28::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio29(self) -> Reg<fields::Gpio29, RW> {
        unsafe { Reg::new(self.0.add(120usize), fields::Gpio29::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn swclk(self) -> Reg<fields::Swclk, RW> {
        unsafe { Reg::new(self.0.add(124usize), fields::Swclk::from_bits(218)) }
    }
    #[doc = "Pad control register"]
    pub fn swd(self) -> Reg<fields::Swd, RW> {
        unsafe { Reg::new(self.0.add(128usize), fields::Swd::from_bits(90)) }
    }
}
pub mod fields;
