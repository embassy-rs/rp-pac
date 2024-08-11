#[doc = "A down counter running at the xosc frequency which counts to zero and stops. Can be used for short software pauses when setting up time sensitive hardware. To start the counter, write a non-zero value. Reads will return 1 while the count is running and 0 when it has finished. Minimum count value is 4. Count values <4 will be treated as count value =4. Note that synchronisation to the register clock domain costs 2 register clock cycles and the counter cannot compensate for that."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE"]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::CtrlFreqRange {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::CtrlFreqRange::from_bits(val as u16)
    }
    #[doc = "The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE"]
    #[inline(always)]
    pub fn set_freq_range(&mut self, val: super::vals::CtrlFreqRange) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED"]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 12usize) & 0x0fff;
        super::vals::Enable::from_bits(val as u16)
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then setting this field to DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_ENABLED"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: super::vals::Enable) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val.to_bits() as u32) & 0x0fff) << 12usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Crystal Oscillator pause control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dormant(pub u32);
impl Dormant {
    #[doc = "This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    pub const fn dormant(&self) -> super::vals::Dormant {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Dormant::from_bits(val as u32)
    }
    #[doc = "This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: stop the PLLs before selecting dormant mode Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    pub fn set_dormant(&mut self, val: super::vals::Dormant) {
        self.0 = (self.0 & !(0xffff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0xffff_ffff) << 0usize);
    }
}
impl Default for Dormant {
    #[inline(always)]
    fn default() -> Dormant {
        Dormant(0)
    }
}
#[doc = "Controls the startup delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Startup(pub u32);
impl Startup {
    #[doc = "in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    pub const fn delay(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "in multiples of 256*xtal_period. The reset value of 0xc4 corresponds to approx 50 000 cycles."]
    #[inline(always)]
    pub fn set_delay(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0."]
    #[inline(always)]
    pub const fn x4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Multiplies the startup_delay by 4, just in case. The reset value is controlled by a mask-programmable tiecell and is provided in case we are booting from XOSC and the default startup delay is insufficient. The reset value is 0x0."]
    #[inline(always)]
    pub fn set_x4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for Startup {
    #[inline(always)]
    fn default() -> Startup {
        Startup(0)
    }
}
#[doc = "Crystal Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "The current frequency range setting"]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::StatusFreqRange {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::StatusFreqRange::from_bits(val as u8)
    }
    #[doc = "The current frequency range setting"]
    #[inline(always)]
    pub fn set_freq_range(&mut self, val: super::vals::StatusFreqRange) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Oscillator is enabled but not necessarily running and stable, resets to 0"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator is enabled but not necessarily running and stable, resets to 0"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub const fn badwrite(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or DORMANT"]
    #[inline(always)]
    pub fn set_badwrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Oscillator is running and stable"]
    #[inline(always)]
    pub const fn stable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator is running and stable"]
    #[inline(always)]
    pub fn set_stable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Status {
    #[inline(always)]
    fn default() -> Status {
        Status(0)
    }
}
