use crate::generic::*;
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Swd(u32);
impl Swd {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Swd {
        Swd(val)
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
impl Default for Swd {
    fn default() -> Swd {
        Swd(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio29(u32);
impl Gpio29 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio29 {
        Gpio29(val)
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
impl Default for Gpio29 {
    fn default() -> Gpio29 {
        Gpio29(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio23(u32);
impl Gpio23 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio23 {
        Gpio23(val)
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
impl Default for Gpio23 {
    fn default() -> Gpio23 {
        Gpio23(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio28(u32);
impl Gpio28 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio28 {
        Gpio28(val)
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
impl Default for Gpio28 {
    fn default() -> Gpio28 {
        Gpio28(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio12(u32);
impl Gpio12 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio12 {
        Gpio12(val)
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
impl Default for Gpio12 {
    fn default() -> Gpio12 {
        Gpio12(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Swclk(u32);
impl Swclk {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Swclk {
        Swclk(val)
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
impl Default for Swclk {
    fn default() -> Swclk {
        Swclk(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio11(u32);
impl Gpio11 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio11 {
        Gpio11(val)
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
impl Default for Gpio11 {
    fn default() -> Gpio11 {
        Gpio11(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio10(u32);
impl Gpio10 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio10 {
        Gpio10(val)
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
impl Default for Gpio10 {
    fn default() -> Gpio10 {
        Gpio10(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio9(u32);
impl Gpio9 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio9 {
        Gpio9(val)
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
impl Default for Gpio9 {
    fn default() -> Gpio9 {
        Gpio9(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio21(u32);
impl Gpio21 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio21 {
        Gpio21(val)
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
impl Default for Gpio21 {
    fn default() -> Gpio21 {
        Gpio21(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio26(u32);
impl Gpio26 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio26 {
        Gpio26(val)
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
impl Default for Gpio26 {
    fn default() -> Gpio26 {
        Gpio26(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio17(u32);
impl Gpio17 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio17 {
        Gpio17(val)
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
impl Default for Gpio17 {
    fn default() -> Gpio17 {
        Gpio17(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio6(u32);
impl Gpio6 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio6 {
        Gpio6(val)
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
impl Default for Gpio6 {
    fn default() -> Gpio6 {
        Gpio6(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio3(u32);
impl Gpio3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio3 {
        Gpio3(val)
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
impl Default for Gpio3 {
    fn default() -> Gpio3 {
        Gpio3(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio7(u32);
impl Gpio7 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio7 {
        Gpio7(val)
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
impl Default for Gpio7 {
    fn default() -> Gpio7 {
        Gpio7(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio24(u32);
impl Gpio24 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio24 {
        Gpio24(val)
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
impl Default for Gpio24 {
    fn default() -> Gpio24 {
        Gpio24(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio4(u32);
impl Gpio4 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio4 {
        Gpio4(val)
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
impl Default for Gpio4 {
    fn default() -> Gpio4 {
        Gpio4(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio1(u32);
impl Gpio1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio1 {
        Gpio1(val)
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
impl Default for Gpio1 {
    fn default() -> Gpio1 {
        Gpio1(0)
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
pub struct Gpio0(u32);
impl Gpio0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio0 {
        Gpio0(val)
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
impl Default for Gpio0 {
    fn default() -> Gpio0 {
        Gpio0(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio27(u32);
impl Gpio27 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio27 {
        Gpio27(val)
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
impl Default for Gpio27 {
    fn default() -> Gpio27 {
        Gpio27(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio22(u32);
impl Gpio22 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio22 {
        Gpio22(val)
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
impl Default for Gpio22 {
    fn default() -> Gpio22 {
        Gpio22(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio14(u32);
impl Gpio14 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio14 {
        Gpio14(val)
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
impl Default for Gpio14 {
    fn default() -> Gpio14 {
        Gpio14(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio2(u32);
impl Gpio2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio2 {
        Gpio2(val)
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
impl Default for Gpio2 {
    fn default() -> Gpio2 {
        Gpio2(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio19(u32);
impl Gpio19 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio19 {
        Gpio19(val)
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
impl Default for Gpio19 {
    fn default() -> Gpio19 {
        Gpio19(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio25(u32);
impl Gpio25 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio25 {
        Gpio25(val)
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
impl Default for Gpio25 {
    fn default() -> Gpio25 {
        Gpio25(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio20(u32);
impl Gpio20 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio20 {
        Gpio20(val)
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
impl Default for Gpio20 {
    fn default() -> Gpio20 {
        Gpio20(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio15(u32);
impl Gpio15 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio15 {
        Gpio15(val)
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
impl Default for Gpio15 {
    fn default() -> Gpio15 {
        Gpio15(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio18(u32);
impl Gpio18 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio18 {
        Gpio18(val)
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
impl Default for Gpio18 {
    fn default() -> Gpio18 {
        Gpio18(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio8(u32);
impl Gpio8 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio8 {
        Gpio8(val)
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
impl Default for Gpio8 {
    fn default() -> Gpio8 {
        Gpio8(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio13(u32);
impl Gpio13 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio13 {
        Gpio13(val)
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
impl Default for Gpio13 {
    fn default() -> Gpio13 {
        Gpio13(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio16(u32);
impl Gpio16 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio16 {
        Gpio16(val)
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
impl Default for Gpio16 {
    fn default() -> Gpio16 {
        Gpio16(0)
    }
}
#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio5(u32);
impl Gpio5 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio5 {
        Gpio5(val)
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
impl Default for Gpio5 {
    fn default() -> Gpio5 {
        Gpio5(0)
    }
}
