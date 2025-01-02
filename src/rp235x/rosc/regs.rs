#[doc = "A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
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
            count: u16,
        }
        let proxy = Count {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ring Oscillator control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::FreqRange {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::FreqRange::from_bits(val as u16)
    }
    #[doc = "Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 2 to 7 HIGH uses stages 4 to 7 TOOHIGH uses stages 6 to 7 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub fn set_freq_range(&mut self, val: super::vals::FreqRange) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
    #[doc = "On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
    #[inline(always)]
    pub const fn enable(&self) -> super::vals::Enable {
        let val = (self.0 >> 12usize) & 0x0fff;
        super::vals::Enable::from_bits(val as u16)
    }
    #[doc = "On power-up this field is initialised to ENABLE The system clock must be switched to another source before setting this field to DISABLE otherwise the chip will lock up The 12-bit code is intended to give some protection against accidental writes. An invalid setting will enable the oscillator."]
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
            freq_range: super::vals::FreqRange,
            enable: super::vals::Enable,
        }
        let proxy = Ctrl {
            freq_range: self.freq_range(),
            enable: self.enable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the output divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "set to 0xaa00 + div where div = 0 divides by 128 div = 1-127 divides by div any other value sets div=128 this register resets to div=32"]
    #[inline(always)]
    pub const fn div(&self) -> super::vals::Div {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Div::from_bits(val as u16)
    }
    #[doc = "set to 0xaa00 + div where div = 0 divides by 128 div = 1-127 divides by div any other value sets div=128 this register resets to div=32"]
    #[inline(always)]
    pub fn set_div(&mut self, val: super::vals::Div) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Div {
    #[inline(always)]
    fn default() -> Div {
        Div(0)
    }
}
impl core::fmt::Debug for Div {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Div").field("div", &self.div()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Div {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Div {
            div: super::vals::Div,
        }
        let proxy = Div { div: self.div() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ring Oscillator pause control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dormant(pub u32);
impl Dormant {
    #[doc = "This is used to save power by pausing the ROSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    pub const fn dormant(&self) -> super::vals::Dormant {
        let val = (self.0 >> 0usize) & 0xffff_ffff;
        super::vals::Dormant::from_bits(val as u32)
    }
    #[doc = "This is used to save power by pausing the ROSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: setup the irq before selecting dormant mode"]
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
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength For frequency randomisation set both DS0_RANDOM=1 & DS1_RANDOM=1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqa(pub u32);
impl Freqa {
    #[doc = "Stage 0 drive strength"]
    #[inline(always)]
    pub const fn ds0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 0 drive strength"]
    #[inline(always)]
    pub fn set_ds0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Randomises the stage 0 drive strength"]
    #[inline(always)]
    pub const fn ds0_random(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Randomises the stage 0 drive strength"]
    #[inline(always)]
    pub fn set_ds0_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Stage 1 drive strength"]
    #[inline(always)]
    pub const fn ds1(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 1 drive strength"]
    #[inline(always)]
    pub fn set_ds1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Randomises the stage 1 drive strength"]
    #[inline(always)]
    pub const fn ds1_random(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Randomises the stage 1 drive strength"]
    #[inline(always)]
    pub fn set_ds1_random(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Stage 2 drive strength"]
    #[inline(always)]
    pub const fn ds2(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 2 drive strength"]
    #[inline(always)]
    pub fn set_ds2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Stage 3 drive strength"]
    #[inline(always)]
    pub const fn ds3(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 3 drive strength"]
    #[inline(always)]
    pub fn set_ds3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub const fn passwd(&self) -> super::vals::Passwd {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Passwd::from_bits(val as u16)
    }
    #[doc = "Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn set_passwd(&mut self, val: super::vals::Passwd) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Freqa {
    #[inline(always)]
    fn default() -> Freqa {
        Freqa(0)
    }
}
impl core::fmt::Debug for Freqa {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freqa")
            .field("ds0", &self.ds0())
            .field("ds0_random", &self.ds0_random())
            .field("ds1", &self.ds1())
            .field("ds1_random", &self.ds1_random())
            .field("ds2", &self.ds2())
            .field("ds3", &self.ds3())
            .field("passwd", &self.passwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freqa {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Freqa {
            ds0: u8,
            ds0_random: bool,
            ds1: u8,
            ds1_random: bool,
            ds2: u8,
            ds3: u8,
            passwd: super::vals::Passwd,
        }
        let proxy = Freqa {
            ds0: self.ds0(),
            ds0_random: self.ds0_random(),
            ds1: self.ds1(),
            ds1_random: self.ds1_random(),
            ds2: self.ds2(),
            ds3: self.ds3(),
            passwd: self.passwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "For a detailed description see freqa register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Freqb(pub u32);
impl Freqb {
    #[doc = "Stage 4 drive strength"]
    #[inline(always)]
    pub const fn ds4(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 4 drive strength"]
    #[inline(always)]
    pub fn set_ds4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Stage 5 drive strength"]
    #[inline(always)]
    pub const fn ds5(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 5 drive strength"]
    #[inline(always)]
    pub fn set_ds5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "Stage 6 drive strength"]
    #[inline(always)]
    pub const fn ds6(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 6 drive strength"]
    #[inline(always)]
    pub fn set_ds6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "Stage 7 drive strength"]
    #[inline(always)]
    pub const fn ds7(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Stage 7 drive strength"]
    #[inline(always)]
    pub fn set_ds7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub const fn passwd(&self) -> super::vals::Passwd {
        let val = (self.0 >> 16usize) & 0xffff;
        super::vals::Passwd::from_bits(val as u16)
    }
    #[doc = "Set to 0x9696 to apply the settings Any other value in this field will set all drive strengths to 0"]
    #[inline(always)]
    pub fn set_passwd(&mut self, val: super::vals::Passwd) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val.to_bits() as u32) & 0xffff) << 16usize);
    }
}
impl Default for Freqb {
    #[inline(always)]
    fn default() -> Freqb {
        Freqb(0)
    }
}
impl core::fmt::Debug for Freqb {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Freqb")
            .field("ds4", &self.ds4())
            .field("ds5", &self.ds5())
            .field("ds6", &self.ds6())
            .field("ds7", &self.ds7())
            .field("passwd", &self.passwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Freqb {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Freqb {
            ds4: u8,
            ds5: u8,
            ds6: u8,
            ds7: u8,
            passwd: super::vals::Passwd,
        }
        let proxy = Freqb {
            ds4: self.ds4(),
            ds5: self.ds5(),
            ds6: self.ds6(),
            ds7: self.ds7(),
            passwd: self.passwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the phase shifted output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Phase(pub u32);
impl Phase {
    #[doc = "phase shift the phase-shifted output by SHIFT input clocks this can be changed on-the-fly must be set to 0 before setting div=1"]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "phase shift the phase-shifted output by SHIFT input clocks this can be changed on-the-fly must be set to 0 before setting div=1"]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "invert the phase-shifted output this is ignored when div=1"]
    #[inline(always)]
    pub const fn flip(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "invert the phase-shifted output this is ignored when div=1"]
    #[inline(always)]
    pub fn set_flip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "enable the phase-shifted output this can be changed on-the-fly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "enable the phase-shifted output this can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "set to 0xaa any other value enables the output with shift=0"]
    #[inline(always)]
    pub const fn passwd(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[doc = "set to 0xaa any other value enables the output with shift=0"]
    #[inline(always)]
    pub fn set_passwd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Phase {
    #[inline(always)]
    fn default() -> Phase {
        Phase(0)
    }
}
impl core::fmt::Debug for Phase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Phase")
            .field("shift", &self.shift())
            .field("flip", &self.flip())
            .field("enable", &self.enable())
            .field("passwd", &self.passwd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Phase {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Phase {
            shift: u8,
            flip: bool,
            enable: bool,
            passwd: u8,
        }
        let proxy = Phase {
            shift: self.shift(),
            flip: self.flip(),
            enable: self.enable(),
            passwd: self.passwd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randombit(pub u32);
impl Randombit {
    #[inline(always)]
    pub const fn randombit(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_randombit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Randombit {
    #[inline(always)]
    fn default() -> Randombit {
        Randombit(0)
    }
}
impl core::fmt::Debug for Randombit {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Randombit")
            .field("randombit", &self.randombit())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Randombit {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Randombit {
            randombit: bool,
        }
        let proxy = Randombit {
            randombit: self.randombit(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Ring Oscillator Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Status(pub u32);
impl Status {
    #[doc = "Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Oscillator is enabled but not necessarily running and stable this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "post-divider is running this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub const fn div_running(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "post-divider is running this resets to 0 but transitions to 1 during chip startup"]
    #[inline(always)]
    pub fn set_div_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT"]
    #[inline(always)]
    pub const fn badwrite(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "An invalid value has been written to CTRL_ENABLE or CTRL_FREQ_RANGE or FREQA or FREQB or DIV or PHASE or DORMANT"]
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
            .field("enabled", &self.enabled())
            .field("div_running", &self.div_running())
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
            enabled: bool,
            div_running: bool,
            badwrite: bool,
            stable: bool,
        }
        let proxy = Status {
            enabled: self.enabled(),
            div_running: self.div_running(),
            badwrite: self.badwrite(),
            stable: self.stable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
