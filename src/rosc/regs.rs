#[doc = "A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
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
#[doc = "Ring Oscillator control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 0 to 5 HIGH uses stages 0 to 3 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
    #[inline(always)]
    pub const fn freq_range(&self) -> super::vals::FreqRange {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::FreqRange::from_bits(val as u16)
    }
    #[doc = "Controls the number of delay stages in the ROSC ring LOW uses stages 0 to 7 MEDIUM uses stages 0 to 5 HIGH uses stages 0 to 3 TOOHIGH uses stages 0 to 1 and should not be used because its frequency exceeds design specifications The clock output will not glitch when changing the range up one step at a time The clock output will glitch when changing the range down Note: the values here are gray coded which is why HIGH comes before TOOHIGH"]
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
#[doc = "Controls the output divider"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "set to 0xaa0 + div where div = 0 divides by 32 div = 1-31 divides by div any other value sets div=31 this register resets to div=16"]
    #[inline(always)]
    pub const fn div(&self) -> super::vals::Div {
        let val = (self.0 >> 0usize) & 0x0fff;
        super::vals::Div::from_bits(val as u16)
    }
    #[doc = "set to 0xaa0 + div where div = 0 divides by 32 div = 1-31 divides by div any other value sets div=31 this register resets to div=16"]
    #[inline(always)]
    pub fn set_div(&mut self, val: super::vals::Div) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val.to_bits() as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Div {
    #[inline(always)]
    fn default() -> Div {
        Div(0)
    }
}
#[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength"]
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
