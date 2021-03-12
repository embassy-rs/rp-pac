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
    pub fn voltage_select(self) -> Reg<regs::VoltageSelect, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sclk(self) -> Reg<regs::GpioQspiSclk, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd0(self) -> Reg<regs::GpioQspiSd0, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd1(self) -> Reg<regs::GpioQspiSd1, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd2(self) -> Reg<regs::GpioQspiSd2, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_sd3(self) -> Reg<regs::GpioQspiSd3, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio_qspi_ss(self) -> Reg<regs::GpioQspiSs, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
}
pub mod regs;
