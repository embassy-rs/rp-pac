use crate::generic::*;
#[derive(Copy, Clone)]
pub struct PadsQspi(*mut u8);
unsafe impl Send for PadsQspi {}
unsafe impl Sync for PadsQspi {}
impl PadsQspi {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Voltage select. Per bank control"]
    pub fn voltage_select(self) -> Reg<fields::VoltageSelect, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::VoltageSelect::from_bits(0)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sclk(self) -> Reg<fields::GpioQspiSclk, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::GpioQspiSclk::from_bits(86)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd0(self) -> Reg<fields::GpioQspiSd0, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::GpioQspiSd0::from_bits(82)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd1(self) -> Reg<fields::GpioQspiSd1, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::GpioQspiSd1::from_bits(82)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd2(self) -> Reg<fields::GpioQspiSd2, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::GpioQspiSd2::from_bits(82)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd3(self) -> Reg<fields::GpioQspiSd3, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::GpioQspiSd3::from_bits(82)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_ss(self) -> Reg<fields::GpioQspiSs, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::GpioQspiSs::from_bits(90)) }
    }
}
pub mod fields;
