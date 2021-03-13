use crate::generic::*;
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints(pub u32);
impl Proc0Ints {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc0Ints {
    fn default() -> Proc0Ints {
        Proc0Ints(0)
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts(pub u32);
impl DormantWakeInts {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for DormantWakeInts {
    fn default() -> DormantWakeInts {
        DormantWakeInts(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0Status(pub u32);
impl GpioQspiSd0Status {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSd0Status {
    fn default() -> GpioQspiSd0Status {
        GpioQspiSd0Status(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2Ctrl(pub u32);
impl GpioQspiSd2Ctrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSd2CtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSd2CtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSd2CtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSd2CtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSd2CtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSd2CtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSd2CtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSd2CtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSd2CtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSd2CtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSd2CtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSd2CtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd2CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd2CtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd2CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd2Ctrl {
    fn default() -> GpioQspiSd2Ctrl {
        GpioQspiSd2Ctrl(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2Status(pub u32);
impl GpioQspiSd2Status {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSd2Status {
    fn default() -> GpioQspiSd2Status {
        GpioQspiSd2Status(0)
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf(pub u32);
impl DormantWakeIntf {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for DormantWakeIntf {
    fn default() -> DormantWakeIntf {
        DormantWakeIntf(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrl(pub u32);
impl GpioQspiSsCtrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSsCtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSsCtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSsCtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSsCtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSsCtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSsCtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSsCtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSsCtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSsCtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSsCtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSsCtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSsCtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSsCtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSsCtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSsCtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSsCtrl {
    fn default() -> GpioQspiSsCtrl {
        GpioQspiSsCtrl(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0Ctrl(pub u32);
impl GpioQspiSd0Ctrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSd0CtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSd0CtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSd0CtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSd0CtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSd0CtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSd0CtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSd0CtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSd0CtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSd0CtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSd0CtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSd0CtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSd0CtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd0CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd0CtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd0CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd0Ctrl {
    fn default() -> GpioQspiSd0Ctrl {
        GpioQspiSd0Ctrl(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(pub u32);
impl Intr {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrl(pub u32);
impl GpioQspiSclkCtrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSclkCtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSclkCtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSclkCtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSclkCtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSclkCtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSclkCtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSclkCtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSclkCtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSclkCtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSclkCtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSclkCtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSclkCtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSclkCtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSclkCtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSclkCtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSclkCtrl {
    fn default() -> GpioQspiSclkCtrl {
        GpioQspiSclkCtrl(0)
    }
}
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf(pub u32);
impl Proc1Intf {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc1Intf {
    fn default() -> Proc1Intf {
        Proc1Intf(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsStatus(pub u32);
impl GpioQspiSsStatus {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSsStatus {
    fn default() -> GpioQspiSsStatus {
        GpioQspiSsStatus(0)
    }
}
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte(pub u32);
impl Proc0Inte {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc0Inte {
    fn default() -> Proc0Inte {
        Proc0Inte(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkStatus(pub u32);
impl GpioQspiSclkStatus {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSclkStatus {
    fn default() -> GpioQspiSclkStatus {
        GpioQspiSclkStatus(0)
    }
}
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte(pub u32);
impl Proc1Inte {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc1Inte {
    fn default() -> Proc1Inte {
        Proc1Inte(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3Status(pub u32);
impl GpioQspiSd3Status {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSd3Status {
    fn default() -> GpioQspiSd3Status {
        GpioQspiSd3Status(0)
    }
}
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf(pub u32);
impl Proc0Intf {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc0Intf {
    fn default() -> Proc0Intf {
        Proc0Intf(0)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1Status(pub u32);
impl GpioQspiSd1Status {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioQspiSd1Status {
    fn default() -> GpioQspiSd1Status {
        GpioQspiSd1Status(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3Ctrl(pub u32);
impl GpioQspiSd3Ctrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSd3CtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSd3CtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSd3CtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSd3CtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSd3CtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSd3CtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSd3CtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSd3CtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSd3CtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSd3CtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSd3CtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSd3CtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd3CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd3CtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd3CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd3Ctrl {
    fn default() -> GpioQspiSd3Ctrl {
        GpioQspiSd3Ctrl(0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints(pub u32);
impl Proc1Ints {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Proc1Ints {
    fn default() -> Proc1Ints {
        Proc1Ints(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1Ctrl(pub u32);
impl GpioQspiSd1Ctrl {
    pub const fn irqover(&self) -> super::vals::GpioQspiSd1CtrlIrqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::GpioQspiSd1CtrlIrqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::GpioQspiSd1CtrlIrqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::GpioQspiSd1CtrlInover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::GpioQspiSd1CtrlInover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::GpioQspiSd1CtrlInover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::GpioQspiSd1CtrlOeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::GpioQspiSd1CtrlOeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::GpioQspiSd1CtrlOeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::GpioQspiSd1CtrlOutover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::GpioQspiSd1CtrlOutover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::GpioQspiSd1CtrlOutover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd1CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd1CtrlFuncsel(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd1CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.0 as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd1Ctrl {
    fn default() -> GpioQspiSd1Ctrl {
        GpioQspiSd1Ctrl(0)
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte(pub u32);
impl DormantWakeInte {
    pub const fn gpio_qspi_sd3_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio_qspi_sd3_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio_qspi_sd3_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio_qspi_sd3_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio_qspi_sd2_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio_qspi_sd2_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio_qspi_sd2_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio_qspi_sd2_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio_qspi_sd1_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio_qspi_sd1_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio_qspi_sd1_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio_qspi_sd1_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio_qspi_sd0_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio_qspi_sd0_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio_qspi_sd0_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio_qspi_sd0_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sd0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio_qspi_ss_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio_qspi_ss_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio_qspi_ss_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio_qspi_ss_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_ss_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio_qspi_sclk_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio_qspi_sclk_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio_qspi_sclk_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio_qspi_sclk_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio_qspi_sclk_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for DormantWakeInte {
    fn default() -> DormantWakeInte {
        DormantWakeInte(0)
    }
}
