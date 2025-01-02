#[doc = "A down counter running at the xosc frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Count(pub u32);
impl Count {
    #[inline(always)]
    pub const fn count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Count {
    #[inline(always)]
    fn default() -> Count {
        Count(0)
    }
}
impl core::fmt::Debug for Count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Count")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Count {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Count {
            count: u8,
        }
        let proxy = Count {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crystal Oscillator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Frequency range. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE. This resets to 0xAA0 and cannot be changed."]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::CtrlFreqRange {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::CtrlFreqRange::from_bits(val as u16)
    }
    #[doc = "Frequency range. An invalid setting will retain the previous value. The actual value being used can be read from STATUS_FREQ_RANGE. This resets to 0xAA0 and cannot be changed."]
    #[inline(always)]
    pub fn set_freq_range(&mut self, val: super::vals::CtrlFreqRange) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 12usize) & 0x0fff;
        super::vals::Enable::from_bits(val as u16)
    }
    #[doc = "On power-up this field is initialised to DISABLE and the chip runs from the ROSC. If the chip has subsequently been programmed to run from the XOSC then DISABLE may lock-up the chip. If this is a concern then run the clk_ref from the ROSC and enable the clk_sys RESUS feature. The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
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
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("freq_range", &self.freq_range())
            .field("enable", &self.enable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            freq_range: super::vals::CtrlFreqRange,
            enable: super::vals::Enable,
        }
        let proxy = Ctrl {
            freq_range: self.freq_range(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Dormant {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dormant")
            .field("dormant", &self.dormant())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dormant {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dormant {
            dormant: super::vals::Dormant,
        }
        let proxy = Dormant {
            dormant: self.dormant(),
        };
        defmt::write!(f, "{}", proxy)
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
    #[doc = "Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
    #[inline(always)]
    pub const fn x4(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Multiplies the startup_delay by 4. This is of little value to the user given that the delay can be programmed directly."]
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
impl core::fmt::Debug for Startup {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Startup")
            .field("delay", &self.delay())
            .field("x4", &self.x4())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Startup {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Startup {
            delay: u16,
            x4: bool,
        }
        let proxy = Startup {
            delay: self.delay(),
            x4: self.x4(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Crystal Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "The current frequency range setting, always reads 0"]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::StatusFreqRange {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::StatusFreqRange::from_bits(val as u8)
    }
    #[doc = "The current frequency range setting, always reads 0"]
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
impl core::fmt::Debug for Status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Status")
            .field("freq_range", &self.freq_range())
            .field("enabled", &self.enabled())
            .field("badwrite", &self.badwrite())
            .field("stable", &self.stable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Status {
            freq_range: super::vals::StatusFreqRange,
            enabled: bool,
            badwrite: bool,
            stable: bool,
        }
        let proxy = Status {
            freq_range: self.freq_range(),
            enabled: self.enabled(),
            badwrite: self.badwrite(),
            stable: self.stable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
