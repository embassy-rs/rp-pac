use crate::generic::*;
#[doc = "Crystal Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Status(u32);
impl Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Status {
        Status(val)
    }
    #[doc = "Oscillator is running and stable"]
    pub const fn stable(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Oscillator is running and stable"]
    pub fn set_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    pub const fn badwrite(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    pub fn set_badwrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Oscillator is enabled but not necessarily running and stable, resets to 0"]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Oscillator is enabled but not necessarily running and stable, resets to 0"]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "The current frequency range setting, always reads 0"]
    pub const fn freq_range(&self) -> super::values::StatusFreqRange {
        let val = (self.0 >> 0u32) & 0x03;
        super::values::StatusFreqRange::from_bits(val as u8)
    }
    #[doc = "The current frequency range setting, always reads 0"]
    pub fn set_freq_range(&mut self, val: super::values::StatusFreqRange) {
        self.0 = (self.0 & !(0x03 << 0u32)) | (((val.to_bits() as u32) & 0x03) << 0u32);
    }
}
#[doc = "A down counter running at the xosc frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Count(u32);
impl Count {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Count {
        Count(val)
    }
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    pub fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrl(u32);
impl Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ctrl {
        Ctrl(val)
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    pub const fn enable(&self) -> super::values::CtrlEnable {
        let val = (self.0 >> 12u32) & 0x0fff;
        super::values::CtrlEnable::from_bits(val as u16)
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    pub fn set_enable(&mut self, val: super::values::CtrlEnable) {
        self.0 = (self.0 & !(0x0fff << 12u32)) | (((val.to_bits() as u32) & 0x0fff) << 12u32);
    }
    #[doc = "Frequency range. This resets to 0xAA0 and cannot be changed."]
    pub const fn freq_range(&self) -> super::values::CtrlFreqRange {
        let val = (self.0 >> 0u32) & 0x0fff;
        super::values::CtrlFreqRange::from_bits(val as u16)
    }
    #[doc = "Frequency range. This resets to 0xAA0 and cannot be changed."]
    pub fn set_freq_range(&mut self, val: super::values::CtrlFreqRange) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val.to_bits() as u32) & 0x0fff) << 0u32);
    }
}
#[doc = "Controls the startup delay"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Startup(u32);
impl Startup {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Startup {
        Startup(val)
    }
    #[doc = "Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly"]
    pub const fn x4(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly"]
    pub fn set_x4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "in multiples of 256*xtal_period"]
    pub const fn delay(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x3fff;
        val as u16
    }
    #[doc = "in multiples of 256*xtal_period"]
    pub fn set_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0u32)) | (((val as u32) & 0x3fff) << 0u32);
    }
}
