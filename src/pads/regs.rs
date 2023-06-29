#[doc = "Pad control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioCtrl(pub u32);
impl GpioCtrl {
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub const fn slewfast(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Slew rate control. 1 = Fast, 0 = Slow"]
    #[inline(always)]
    pub fn set_slewfast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable schmitt trigger"]
    #[inline(always)]
    pub const fn schmitt(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable schmitt trigger"]
    #[inline(always)]
    pub fn set_schmitt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pull down enable"]
    #[inline(always)]
    pub const fn pde(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pull down enable"]
    #[inline(always)]
    pub fn set_pde(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Pull up enable"]
    #[inline(always)]
    pub const fn pue(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Pull up enable"]
    #[inline(always)]
    pub fn set_pue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Drive strength."]
    #[inline(always)]
    pub const fn drive(&self) -> super::vals::Drive {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Drive::from_bits(val as u8)
    }
    #[doc = "Drive strength."]
    #[inline(always)]
    pub fn set_drive(&mut self, val: super::vals::Drive) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Input enable"]
    #[inline(always)]
    pub const fn ie(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Input enable"]
    #[inline(always)]
    pub fn set_ie(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub const fn od(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Output disable. Has priority over output enable from peripherals"]
    #[inline(always)]
    pub fn set_od(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for GpioCtrl {
    #[inline(always)]
    fn default() -> GpioCtrl {
        GpioCtrl(0)
    }
}
#[doc = "Voltage select. Per bank control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VoltageSelect(pub u32);
impl VoltageSelect {
    #[inline(always)]
    pub const fn voltage_select(&self) -> super::vals::VoltageSelect {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::VoltageSelect::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_voltage_select(&mut self, val: super::vals::VoltageSelect) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
}
impl Default for VoltageSelect {
    #[inline(always)]
    fn default() -> VoltageSelect {
        VoltageSelect(0)
    }
}
