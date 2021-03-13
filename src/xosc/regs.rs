use crate::generic::*;
#[doc = "Controls the startup delay"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Startup(pub u32);
impl Startup {
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
impl Default for Startup {
    fn default() -> Startup {
        Startup(0)
    }
}
#[doc = "A down counter running at the xosc frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Count(pub u32);
impl Count {
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    pub fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Count {
    fn default() -> Count {
        Count(0)
    }
}
#[doc = "Crystal Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Status(pub u32);
impl Status {
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
    pub const fn freq_range(&self) -> super::vals::StatusFreqRange {
        let val = (self.0 >> 0u32) & 0x03;
        super::vals::StatusFreqRange(val as u8)
    }
    #[doc = "The current frequency range setting, always reads 0"]
    pub fn set_freq_range(&mut self, val: super::vals::StatusFreqRange) {
        self.0 = (self.0 & !(0x03 << 0u32)) | (((val.0 as u32) & 0x03) << 0u32);
    }
}
impl Default for Status {
    fn default() -> Status {
        Status(0)
    }
}
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    pub const fn enable(&self) -> super::vals::CtrlEnable {
        let val = (self.0 >> 12u32) & 0x0fff;
        super::vals::CtrlEnable(val as u16)
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    pub fn set_enable(&mut self, val: super::vals::CtrlEnable) {
        self.0 = (self.0 & !(0x0fff << 12u32)) | (((val.0 as u32) & 0x0fff) << 12u32);
    }
    #[doc = "Frequency range. This resets to 0xAA0 and cannot be changed."]
    pub const fn freq_range(&self) -> super::vals::CtrlFreqRange {
        let val = (self.0 >> 0u32) & 0x0fff;
        super::vals::CtrlFreqRange(val as u16)
    }
    #[doc = "Frequency range. This resets to 0xAA0 and cannot be changed."]
    pub fn set_freq_range(&mut self, val: super::vals::CtrlFreqRange) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val.0 as u32) & 0x0fff) << 0u32);
    }
}
impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
