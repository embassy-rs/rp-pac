use crate::generic::*;
#[derive(Copy, Clone)]
pub struct IoQspi(*mut u8);
unsafe impl Send for IoQspi {}
unsafe impl Sync for IoQspi {}
impl IoQspi {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sclk_status(self) -> Reg<fields::GpioQspiSclkStatus, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::GpioQspiSclkStatus::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sclk_ctrl(self) -> Reg<fields::GpioQspiSclkCtrl, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::GpioQspiSclkCtrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_ss_status(self) -> Reg<fields::GpioQspiSsStatus, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::GpioQspiSsStatus::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_ss_ctrl(self) -> Reg<fields::GpioQspiSsCtrl, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::GpioQspiSsCtrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd0_status(self) -> Reg<fields::GpioQspiSd0Status, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::GpioQspiSd0Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd0_ctrl(self) -> Reg<fields::GpioQspiSd0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::GpioQspiSd0Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd1_status(self) -> Reg<fields::GpioQspiSd1Status, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::GpioQspiSd1Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd1_ctrl(self) -> Reg<fields::GpioQspiSd1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::GpioQspiSd1Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd2_status(self) -> Reg<fields::GpioQspiSd2Status, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::GpioQspiSd2Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd2_ctrl(self) -> Reg<fields::GpioQspiSd2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::GpioQspiSd2Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd3_status(self) -> Reg<fields::GpioQspiSd3Status, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::GpioQspiSd3Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd3_ctrl(self) -> Reg<fields::GpioQspiSd3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::GpioQspiSd3Ctrl::from_bits(31)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte(self) -> Reg<fields::Proc0Inte, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Proc0Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf(self) -> Reg<fields::Proc0Intf, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Proc0Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints(self) -> Reg<fields::Proc0Ints, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Proc0Ints::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte(self) -> Reg<fields::Proc1Inte, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Proc1Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf(self) -> Reg<fields::Proc1Intf, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::Proc1Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints(self) -> Reg<fields::Proc1Ints, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::Proc1Ints::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte(self) -> Reg<fields::DormantWakeInte, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::DormantWakeInte::from_bits(0)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf(self) -> Reg<fields::DormantWakeIntf, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::DormantWakeIntf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints(self) -> Reg<fields::DormantWakeInts, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::DormantWakeInts::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
