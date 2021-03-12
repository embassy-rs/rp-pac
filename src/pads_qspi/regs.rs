use crate::generic::*;
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSclk(u32);
impl GpioQspiSclk {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSclk {
        GpioQspiSclk(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSclk {
    fn default() -> GpioQspiSclk {
        GpioQspiSclk(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd2(u32);
impl GpioQspiSd2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd2 {
        GpioQspiSd2(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd2 {
    fn default() -> GpioQspiSd2 {
        GpioQspiSd2(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd1(u32);
impl GpioQspiSd1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd1 {
        GpioQspiSd1(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd1 {
    fn default() -> GpioQspiSd1 {
        GpioQspiSd1(0)
    }
}
#[doc = "Voltage select. Per bank control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct VoltageSelect(u32);
impl VoltageSelect {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> VoltageSelect {
        VoltageSelect(val)
    }
    pub const fn voltage_select(&self) -> super::super::pads::vals::VoltageSelect {
        let val = (self.0 >> 0u32) & 0x01;
        super::super::pads::vals::VoltageSelect::from_bits(val as u8)
    }
    pub fn set_voltage_select(&mut self, val: super::super::pads::vals::VoltageSelect) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.to_bits() as u32) & 0x01) << 0u32);
    }
}
impl Default for VoltageSelect {
    fn default() -> VoltageSelect {
        VoltageSelect(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd0(u32);
impl GpioQspiSd0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd0 {
        GpioQspiSd0(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd0 {
    fn default() -> GpioQspiSd0 {
        GpioQspiSd0(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSs(u32);
impl GpioQspiSs {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSs {
        GpioQspiSs(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSs {
    fn default() -> GpioQspiSs {
        GpioQspiSs(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioQspiSd3(u32);
impl GpioQspiSd3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> GpioQspiSd3 {
        GpioQspiSd3(val)
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Input enable"]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Drive strength."]
    pub const fn drive(&self) -> super::super::pads::vals::Drive {
        let val = (self.0 >> 4u32) & 0x03;
        super::super::pads::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    pub fn set_drive(&mut self, val: super::super::pads::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.to_bits() as u32) & 0x03) << 4u32);
    }
    #[doc = "Pull up enable"]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Pull down enable"]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enable schmitt trigger"]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for GpioQspiSd3 {
    fn default() -> GpioQspiSd3 {
        GpioQspiSd3(0)
    }
}
