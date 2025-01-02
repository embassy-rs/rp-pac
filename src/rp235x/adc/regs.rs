#[doc = "ADC Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Power on ADC and enable its clock. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Power on ADC and enable its clock. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub const fn ts_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Power on temperature sensor. 1 - enabled. 0 - disabled."]
    #[inline(always)]
    pub fn set_ts_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub const fn start_once(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    #[inline(always)]
    pub fn set_start_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub const fn start_many(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    #[inline(always)]
    pub fn set_start_many(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed. 0 whilst conversion in progress."]
    #[inline(always)]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed. 0 whilst conversion in progress."]
    #[inline(always)]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "The most recent ADC conversion encountered an error; result is undefined or noisy."]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "The most recent ADC conversion encountered an error; result is undefined or noisy."]
    #[inline(always)]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub const fn err_sticky(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Some past ADC conversion encountered an error. Write 1 to clear."]
    #[inline(always)]
    pub fn set_err_sticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Select analog mux input. Updated automatically in round-robin mode. This is corrected for the package option so only ADC channels which are bonded are available, and in the correct order"]
    #[inline(always)]
    pub const fn ainsel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Select analog mux input. Updated automatically in round-robin mode. This is corrected for the package option so only ADC channels which are bonded are available, and in the correct order"]
    #[inline(always)]
    pub fn set_ainsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion. The first channel to be sampled will be the one currently indicated by AINSEL. AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub const fn rrobin(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x01ff;
        val as u16
    }
    #[doc = "Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion. The first channel to be sampled will be the one currently indicated by AINSEL. AINSEL will be updated after each conversion with the newly-selected channel."]
    #[inline(always)]
    pub fn set_rrobin(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 16usize)) | (((val as u32) & 0x01ff) << 16usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
impl core::fmt::Debug for Cs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cs")
            .field("en", &self.en())
            .field("ts_en", &self.ts_en())
            .field("start_once", &self.start_once())
            .field("start_many", &self.start_many())
            .field("ready", &self.ready())
            .field("err", &self.err())
            .field("err_sticky", &self.err_sticky())
            .field("ainsel", &self.ainsel())
            .field("rrobin", &self.rrobin())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cs {
            en: bool,
            ts_en: bool,
            start_once: bool,
            start_many: bool,
            ready: bool,
            err: bool,
            err_sticky: bool,
            ainsel: u8,
            rrobin: u16,
        }
        let proxy = Cs {
            en: self.en(),
            ts_en: self.ts_en(),
            start_once: self.start_once(),
            start_many: self.start_many(),
            ready: self.ready(),
            err: self.err(),
            err_sticky: self.err_sticky(),
            ainsel: self.ainsel(),
            rrobin: self.rrobin(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub u32);
impl Div {
    #[doc = "Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional part of clock divisor. First-order delta-sigma."]
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Integer part of clock divisor."]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer part of clock divisor."]
    #[inline(always)]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
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
        f.debug_struct("Div")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Div {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Div {
            frac: u8,
            int: u16,
        }
        let proxy = Div {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO control and status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fcs(pub u32);
impl Fcs {
    #[doc = "If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1: write result to the FIFO after each conversion."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    pub const fn shift(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1: conversion error bit appears in the FIFO alongside the result"]
    #[inline(always)]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    pub const fn dreq_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1: assert DMA requests when FIFO contains data"]
    #[inline(always)]
    pub fn set_dreq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    pub const fn under(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "1 if the FIFO has been underflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn set_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    pub const fn over(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "1 if the FIFO has been overflowed. Write 1 to clear."]
    #[inline(always)]
    pub fn set_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "The number of conversion results currently waiting in the FIFO"]
    #[inline(always)]
    pub const fn level(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of conversion results currently waiting in the FIFO"]
    #[inline(always)]
    pub fn set_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    pub const fn thresh(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "DREQ/IRQ asserted when level >= threshold"]
    #[inline(always)]
    pub fn set_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fcs {
    #[inline(always)]
    fn default() -> Fcs {
        Fcs(0)
    }
}
impl core::fmt::Debug for Fcs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fcs")
            .field("en", &self.en())
            .field("shift", &self.shift())
            .field("err", &self.err())
            .field("dreq_en", &self.dreq_en())
            .field("empty", &self.empty())
            .field("full", &self.full())
            .field("under", &self.under())
            .field("over", &self.over())
            .field("level", &self.level())
            .field("thresh", &self.thresh())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fcs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fcs {
            en: bool,
            shift: bool,
            err: bool,
            dreq_en: bool,
            empty: bool,
            full: bool,
            under: bool,
            over: bool,
            level: u8,
            thresh: u8,
        }
        let proxy = Fcs {
            en: self.en(),
            shift: self.shift(),
            err: self.err(),
            dreq_en: self.dreq_en(),
            empty: self.empty(),
            full: self.full(),
            under: self.under(),
            over: self.over(),
            level: self.level(),
            thresh: self.thresh(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Conversion result FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub u32);
impl Fifo {
    #[inline(always)]
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    #[inline(always)]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    #[inline(always)]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Fifo {
    #[inline(always)]
    fn default() -> Fifo {
        Fifo(0)
    }
}
impl core::fmt::Debug for Fifo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fifo")
            .field("val", &self.val())
            .field("err", &self.err())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fifo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fifo {
            val: u16,
            err: bool,
        }
        let proxy = Fifo {
            val: self.val(),
            err: self.err(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub const fn fifo(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    #[inline(always)]
    pub fn set_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int").field("fifo", &self.fifo()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            fifo: bool,
        }
        let proxy = Int { fifo: self.fifo() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Result of most recent ADC conversion"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Result(pub u32);
impl Result {
    #[inline(always)]
    pub const fn result(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for Result {
    #[inline(always)]
    fn default() -> Result {
        Result(0)
    }
}
impl core::fmt::Debug for Result {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Result")
            .field("result", &self.result())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Result {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Result {
            result: u16,
        }
        let proxy = Result {
            result: self.result(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
