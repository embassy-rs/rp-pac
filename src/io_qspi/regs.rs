use crate::generic::*;
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3Status(u32);
impl GpioQspiSd3Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd3Status {
        GpioQspiSd3Status(val)
    }
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
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2Ctrl(u32);
impl GpioQspiSd2Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd2Ctrl {
        GpioQspiSd2Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd2CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd2CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd2CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd2Ctrl {
    fn default() -> GpioQspiSd2Ctrl {
        GpioQspiSd2Ctrl(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0Ctrl(u32);
impl GpioQspiSd0Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd0Ctrl {
        GpioQspiSd0Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd0CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd0CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd0CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
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
pub struct Intr(u32);
impl Intr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr {
        Intr(val)
    }
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
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0Status(u32);
impl GpioQspiSd0Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd0Status {
        GpioQspiSd0Status(val)
    }
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
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte(u32);
impl Proc1Inte {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Inte {
        Proc1Inte(val)
    }
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
pub struct GpioQspiSclkStatus(u32);
impl GpioQspiSclkStatus {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSclkStatus {
        GpioQspiSclkStatus(val)
    }
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
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1Status(u32);
impl GpioQspiSd1Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd1Status {
        GpioQspiSd1Status(val)
    }
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
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2Status(u32);
impl GpioQspiSd2Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd2Status {
        GpioQspiSd2Status(val)
    }
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
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints(u32);
impl Proc1Ints {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Ints {
        Proc1Ints(val)
    }
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
pub struct GpioQspiSd3Ctrl(u32);
impl GpioQspiSd3Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd3Ctrl {
        GpioQspiSd3Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd3CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd3CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd3CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd3Ctrl {
    fn default() -> GpioQspiSd3Ctrl {
        GpioQspiSd3Ctrl(0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints(u32);
impl Proc0Ints {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Ints {
        Proc0Ints(val)
    }
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
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte(u32);
impl Proc0Inte {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Inte {
        Proc0Inte(val)
    }
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
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1Ctrl(u32);
impl GpioQspiSd1Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd1Ctrl {
        GpioQspiSd1Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSd1CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSd1CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSd1CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSd1Ctrl {
    fn default() -> GpioQspiSd1Ctrl {
        GpioQspiSd1Ctrl(0)
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts(u32);
impl DormantWakeInts {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInts {
        DormantWakeInts(val)
    }
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
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclkCtrl(u32);
impl GpioQspiSclkCtrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSclkCtrl {
        GpioQspiSclkCtrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSclkCtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSclkCtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSclkCtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSclkCtrl {
    fn default() -> GpioQspiSclkCtrl {
        GpioQspiSclkCtrl(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsCtrl(u32);
impl GpioQspiSsCtrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSsCtrl {
        GpioQspiSsCtrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::vals::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::vals::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::vals::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::vals::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::vals::GpioQspiSsCtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::vals::GpioQspiSsCtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::vals::GpioQspiSsCtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioQspiSsCtrl {
    fn default() -> GpioQspiSsCtrl {
        GpioQspiSsCtrl(0)
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf(u32);
impl DormantWakeIntf {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeIntf {
        DormantWakeIntf(val)
    }
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
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSsStatus(u32);
impl GpioQspiSsStatus {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSsStatus {
        GpioQspiSsStatus(val)
    }
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
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf(u32);
impl Proc0Intf {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Intf {
        Proc0Intf(val)
    }
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
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte(u32);
impl DormantWakeInte {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInte {
        DormantWakeInte(val)
    }
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
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf(u32);
impl Proc1Intf {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Intf {
        Proc1Intf(val)
    }
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
