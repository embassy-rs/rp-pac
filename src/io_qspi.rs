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
    pub fn gpio_qspi_sclk_status(self) -> Reg<regs::GpioQspiSclkStatus, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sclk_ctrl(self) -> Reg<regs::GpioQspiSclkCtrl, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_ss_status(self) -> Reg<regs::GpioQspiSsStatus, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_ss_ctrl(self) -> Reg<regs::GpioQspiSsCtrl, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd0_status(self) -> Reg<regs::GpioQspiSd0Status, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd0_ctrl(self) -> Reg<regs::GpioQspiSd0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd1_status(self) -> Reg<regs::GpioQspiSd1Status, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd1_ctrl(self) -> Reg<regs::GpioQspiSd1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd2_status(self) -> Reg<regs::GpioQspiSd2Status, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd2_ctrl(self) -> Reg<regs::GpioQspiSd2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio_qspi_sd3_status(self) -> Reg<regs::GpioQspiSd3Status, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio_qspi_sd3_ctrl(self) -> Reg<regs::GpioQspiSd3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte(self) -> Reg<regs::Proc0Inte, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf(self) -> Reg<regs::Proc0Intf, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints(self) -> Reg<regs::Proc0Ints, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte(self) -> Reg<regs::Proc1Inte, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf(self) -> Reg<regs::Proc1Intf, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints(self) -> Reg<regs::Proc1Ints, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte(self) -> Reg<regs::DormantWakeInte, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf(self) -> Reg<regs::DormantWakeIntf, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints(self) -> Reg<regs::DormantWakeInts, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
}
pub mod regs;
pub mod vals;
