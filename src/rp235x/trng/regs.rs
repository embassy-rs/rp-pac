#[doc = "Statistic about Autocorrelation test activations."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AutocorrStatistic(pub u32);
impl AutocorrStatistic {
    #[doc = "Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub const fn autocorr_trys(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Count each time an autocorrelation test starts. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub fn set_autocorr_trys(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub const fn autocorr_fails(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0xff;
        val as u8
    }
    #[doc = "Count each time an autocorrelation test fails. Any write to the register reset the counter. Stop collecting statistic if one of the counters reached the limit."]
    #[inline(always)]
    pub fn set_autocorr_fails(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 14usize)) | (((val as u32) & 0xff) << 14usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for AutocorrStatistic {
    #[inline(always)]
    fn default() -> AutocorrStatistic {
        AutocorrStatistic(0)
    }
}
impl core::fmt::Debug for AutocorrStatistic {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AutocorrStatistic")
            .field("autocorr_trys", &self.autocorr_trys())
            .field("autocorr_fails", &self.autocorr_fails())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AutocorrStatistic {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AutocorrStatistic {
            autocorr_trys: u16,
            autocorr_fails: u8,
            reserved: u16,
        }
        let proxy = AutocorrStatistic {
            autocorr_trys: self.autocorr_trys(),
            autocorr_fails: self.autocorr_fails(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable signal for the random source."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RndSourceEnable(pub u32);
impl RndSourceEnable {
    #[doc = "* 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
    #[inline(always)]
    pub const fn rnd_src_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - entropy source is enabled. *1'b0 - entropy source is disabled"]
    #[inline(always)]
    pub fn set_rnd_src_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RndSourceEnable {
    #[inline(always)]
    fn default() -> RndSourceEnable {
        RndSourceEnable(0)
    }
}
impl core::fmt::Debug for RndSourceEnable {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RndSourceEnable")
            .field("rnd_src_en", &self.rnd_src_en())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RndSourceEnable {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RndSourceEnable {
            rnd_src_en: bool,
            reserved: u32,
        }
        let proxy = RndSourceEnable {
            rnd_src_en: self.rnd_src_en(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Collected BIST results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngBistCntr0(pub u32);
impl RngBistCntr0 {
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub fn set_rosc_cntr_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for RngBistCntr0 {
    #[inline(always)]
    fn default() -> RngBistCntr0 {
        RngBistCntr0(0)
    }
}
impl core::fmt::Debug for RngBistCntr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngBistCntr0")
            .field("rosc_cntr_val", &self.rosc_cntr_val())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngBistCntr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngBistCntr0 {
            rosc_cntr_val: u32,
            reserved: u16,
        }
        let proxy = RngBistCntr0 {
            rosc_cntr_val: self.rosc_cntr_val(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Collected BIST results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngBistCntr1(pub u32);
impl RngBistCntr1 {
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub fn set_rosc_cntr_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for RngBistCntr1 {
    #[inline(always)]
    fn default() -> RngBistCntr1 {
        RngBistCntr1(0)
    }
}
impl core::fmt::Debug for RngBistCntr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngBistCntr1")
            .field("rosc_cntr_val", &self.rosc_cntr_val())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngBistCntr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngBistCntr1 {
            rosc_cntr_val: u32,
            reserved: u16,
        }
        let proxy = RngBistCntr1 {
            rosc_cntr_val: self.rosc_cntr_val(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Collected BIST results."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngBistCntr2(pub u32);
impl RngBistCntr2 {
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub const fn rosc_cntr_val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Reflects the results of RNG BIST counter."]
    #[inline(always)]
    pub fn set_rosc_cntr_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u16 {
        let val = (self.0 >> 22usize) & 0x03ff;
        val as u16
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 22usize)) | (((val as u32) & 0x03ff) << 22usize);
    }
}
impl Default for RngBistCntr2 {
    #[inline(always)]
    fn default() -> RngBistCntr2 {
        RngBistCntr2(0)
    }
}
impl core::fmt::Debug for RngBistCntr2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngBistCntr2")
            .field("rosc_cntr_val", &self.rosc_cntr_val())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngBistCntr2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngBistCntr2 {
            rosc_cntr_val: u32,
            reserved: u16,
        }
        let proxy = RngBistCntr2 {
            rosc_cntr_val: self.rosc_cntr_val(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable the RNG debug mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngDebugEnInput(pub u32);
impl RngDebugEnInput {
    #[doc = "* 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
    #[inline(always)]
    pub const fn rng_debug_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - debug mode is enabled. *1'b0 - debug mode is disabled"]
    #[inline(always)]
    pub fn set_rng_debug_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RngDebugEnInput {
    #[inline(always)]
    fn default() -> RngDebugEnInput {
        RngDebugEnInput(0)
    }
}
impl core::fmt::Debug for RngDebugEnInput {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngDebugEnInput")
            .field("rng_debug_en", &self.rng_debug_en())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngDebugEnInput {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngDebugEnInput {
            rng_debug_en: bool,
            reserved: u32,
        }
        let proxy = RngDebugEnInput {
            rng_debug_en: self.rng_debug_en(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt/status bit clear Register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngIcr(pub u32);
impl RngIcr {
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn set_ehr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub const fn autocorr_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Cannot be cleared by SW! Only RNG reset clears this bit."]
    #[inline(always)]
    pub fn set_autocorr_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub const fn crngt_err(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn set_crngt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub const fn vn_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1'b1 - clear corresponding bit in RNG_ISR."]
    #[inline(always)]
    pub fn set_vn_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for RngIcr {
    #[inline(always)]
    fn default() -> RngIcr {
        RngIcr(0)
    }
}
impl core::fmt::Debug for RngIcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngIcr")
            .field("ehr_valid", &self.ehr_valid())
            .field("autocorr_err", &self.autocorr_err())
            .field("crngt_err", &self.crngt_err())
            .field("vn_err", &self.vn_err())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngIcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngIcr {
            ehr_valid: bool,
            autocorr_err: bool,
            crngt_err: bool,
            vn_err: bool,
            reserved: u32,
        }
        let proxy = RngIcr {
            ehr_valid: self.ehr_valid(),
            autocorr_err: self.autocorr_err(),
            crngt_err: self.crngt_err(),
            vn_err: self.vn_err(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt masking."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngImr(pub u32);
impl RngImr {
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub const fn ehr_valid_int_mask(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn set_ehr_valid_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub const fn autocorr_err_int_mask(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn set_autocorr_err_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub const fn crngt_err_int_mask(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn set_crngt_err_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub const fn vn_err_int_mask(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1-mask interrupt, no interrupt will be generated. See RNG_ISR for an explanation on this interrupt."]
    #[inline(always)]
    pub fn set_vn_err_int_mask(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for RngImr {
    #[inline(always)]
    fn default() -> RngImr {
        RngImr(0)
    }
}
impl core::fmt::Debug for RngImr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngImr")
            .field("ehr_valid_int_mask", &self.ehr_valid_int_mask())
            .field("autocorr_err_int_mask", &self.autocorr_err_int_mask())
            .field("crngt_err_int_mask", &self.crngt_err_int_mask())
            .field("vn_err_int_mask", &self.vn_err_int_mask())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngImr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngImr {
            ehr_valid_int_mask: bool,
            autocorr_err_int_mask: bool,
            crngt_err_int_mask: bool,
            vn_err_int_mask: bool,
            reserved: u32,
        }
        let proxy = RngImr {
            ehr_valid_int_mask: self.ehr_valid_int_mask(),
            autocorr_err_int_mask: self.autocorr_err_int_mask(),
            crngt_err_int_mask: self.crngt_err_int_mask(),
            vn_err_int_mask: self.vn_err_int_mask(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngIsr(pub u32);
impl RngIsr {
    #[doc = "1'b1 indicates that 192 bits have been collected in the RNG, and are ready to be read."]
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1 indicates that 192 bits have been collected in the RNG, and are ready to be read."]
    #[inline(always)]
    pub fn set_ehr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1'b1 indicates Autocorrelation test failed four times in a row. When set, RNG cease from functioning until next reset."]
    #[inline(always)]
    pub const fn autocorr_err(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1 indicates Autocorrelation test failed four times in a row. When set, RNG cease from functioning until next reset."]
    #[inline(always)]
    pub fn set_autocorr_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    pub const fn crngt_err(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1 indicates CRNGT in the RNG test failed. Failure occurs when two consecutive blocks of 16 collected bits are equal."]
    #[inline(always)]
    pub fn set_crngt_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecutive collected bits are identical, ZERO or ONE."]
    #[inline(always)]
    pub const fn vn_err(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1 indicates Von Neuman error. Error in von Neuman occurs if 32 consecutive collected bits are identical, ZERO or ONE."]
    #[inline(always)]
    pub fn set_vn_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 4usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 4usize)) | (((val as u32) & 0x0fff_ffff) << 4usize);
    }
}
impl Default for RngIsr {
    #[inline(always)]
    fn default() -> RngIsr {
        RngIsr(0)
    }
}
impl core::fmt::Debug for RngIsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngIsr")
            .field("ehr_valid", &self.ehr_valid())
            .field("autocorr_err", &self.autocorr_err())
            .field("crngt_err", &self.crngt_err())
            .field("vn_err", &self.vn_err())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngIsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngIsr {
            ehr_valid: bool,
            autocorr_err: bool,
            crngt_err: bool,
            vn_err: bool,
            reserved: u32,
        }
        let proxy = RngIsr {
            ehr_valid: self.ehr_valid(),
            autocorr_err: self.autocorr_err(),
            crngt_err: self.crngt_err(),
            vn_err: self.vn_err(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Displays the version settings of the TRNG."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RngVersion(pub u32);
impl RngVersion {
    #[doc = "* 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR"]
    #[inline(always)]
    pub const fn ehr_width_192(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - 192-bit EHR. *1'b0 - 128-bit EHR"]
    #[inline(always)]
    pub fn set_ehr_width_192(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn crngt_exists(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_crngt_exists(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn autocorr_exists(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_autocorr_exists(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn trng_tests_bypass_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_trng_tests_bypass_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn prng_exists(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_prng_exists(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn kat_exists(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_kat_exists(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub const fn reseeding_exists(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - Exists. *1'b0 - Does not exist"]
    #[inline(always)]
    pub fn set_reseeding_exists(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "* 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES"]
    #[inline(always)]
    pub const fn rng_use_5_sboxes(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "* 1'b1 - 5 SBOX AES. *1'b0 - 20 SBOX AES"]
    #[inline(always)]
    pub fn set_rng_use_5_sboxes(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 8usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8usize)) | (((val as u32) & 0x00ff_ffff) << 8usize);
    }
}
impl Default for RngVersion {
    #[inline(always)]
    fn default() -> RngVersion {
        RngVersion(0)
    }
}
impl core::fmt::Debug for RngVersion {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RngVersion")
            .field("ehr_width_192", &self.ehr_width_192())
            .field("crngt_exists", &self.crngt_exists())
            .field("autocorr_exists", &self.autocorr_exists())
            .field("trng_tests_bypass_en", &self.trng_tests_bypass_en())
            .field("prng_exists", &self.prng_exists())
            .field("kat_exists", &self.kat_exists())
            .field("reseeding_exists", &self.reseeding_exists())
            .field("rng_use_5_sboxes", &self.rng_use_5_sboxes())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RngVersion {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RngVersion {
            ehr_width_192: bool,
            crngt_exists: bool,
            autocorr_exists: bool,
            trng_tests_bypass_en: bool,
            prng_exists: bool,
            kat_exists: bool,
            reseeding_exists: bool,
            rng_use_5_sboxes: bool,
            reserved: u32,
        }
        let proxy = RngVersion {
            ehr_width_192: self.ehr_width_192(),
            crngt_exists: self.crngt_exists(),
            autocorr_exists: self.autocorr_exists(),
            trng_tests_bypass_en: self.trng_tests_bypass_en(),
            prng_exists: self.prng_exists(),
            kat_exists: self.kat_exists(),
            reseeding_exists: self.reseeding_exists(),
            rng_use_5_sboxes: self.rng_use_5_sboxes(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reset the counter of collected bits in the RNG."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RstBitsCounter(pub u32);
impl RstBitsCounter {
    #[doc = "Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
    #[inline(always)]
    pub const fn rst_bits_counter(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing any value to this address will reset the bits counter and RNG valid registers. RND_SORCE_ENABLE register must be unset in order for the reset to take place."]
    #[inline(always)]
    pub fn set_rst_bits_counter(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for RstBitsCounter {
    #[inline(always)]
    fn default() -> RstBitsCounter {
        RstBitsCounter(0)
    }
}
impl core::fmt::Debug for RstBitsCounter {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RstBitsCounter")
            .field("rst_bits_counter", &self.rst_bits_counter())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RstBitsCounter {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RstBitsCounter {
            rst_bits_counter: bool,
            reserved: u32,
        }
        let proxy = RstBitsCounter {
            rst_bits_counter: self.rst_bits_counter(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RNG Busy indication."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngBusy(pub u32);
impl TrngBusy {
    #[doc = "Reflects rng_busy status."]
    #[inline(always)]
    pub const fn trng_busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reflects rng_busy status."]
    #[inline(always)]
    pub fn set_trng_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TrngBusy {
    #[inline(always)]
    fn default() -> TrngBusy {
        TrngBusy(0)
    }
}
impl core::fmt::Debug for TrngBusy {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrngBusy")
            .field("trng_busy", &self.trng_busy())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrngBusy {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrngBusy {
            trng_busy: bool,
            reserved: u32,
        }
        let proxy = TrngBusy {
            trng_busy: self.trng_busy(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Selecting the inverter-chain length."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngConfig(pub u32);
impl TrngConfig {
    #[doc = "Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
    #[inline(always)]
    pub const fn rnd_src_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Selects the number of inverters (out of four possible selections) in the ring oscillator (the entropy source)."]
    #[inline(always)]
    pub fn set_rnd_src_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for TrngConfig {
    #[inline(always)]
    fn default() -> TrngConfig {
        TrngConfig(0)
    }
}
impl core::fmt::Debug for TrngConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrngConfig")
            .field("rnd_src_sel", &self.rnd_src_sel())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrngConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrngConfig {
            rnd_src_sel: u8,
            reserved: u32,
        }
        let proxy = TrngConfig {
            rnd_src_sel: self.rnd_src_sel(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngDebugControl(pub u32);
impl TrngDebugControl {
    #[doc = "N/A"]
    #[inline(always)]
    pub const fn reserved(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "N/A"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
    #[inline(always)]
    pub const fn vnc_bypass(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When set, the Von-Neuman balancer is bypassed (including the 32 consecutive bits test)."]
    #[inline(always)]
    pub fn set_vnc_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When set, the CRNGT test in the RNG is bypassed."]
    #[inline(always)]
    pub const fn trng_crngt_bypass(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When set, the CRNGT test in the RNG is bypassed."]
    #[inline(always)]
    pub fn set_trng_crngt_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When set, the autocorrelation test in the TRNG module is bypassed."]
    #[inline(always)]
    pub const fn auto_correlate_bypass(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When set, the autocorrelation test in the TRNG module is bypassed."]
    #[inline(always)]
    pub fn set_auto_correlate_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TrngDebugControl {
    #[inline(always)]
    fn default() -> TrngDebugControl {
        TrngDebugControl(0)
    }
}
impl core::fmt::Debug for TrngDebugControl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrngDebugControl")
            .field("reserved", &self.reserved())
            .field("vnc_bypass", &self.vnc_bypass())
            .field("trng_crngt_bypass", &self.trng_crngt_bypass())
            .field("auto_correlate_bypass", &self.auto_correlate_bypass())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrngDebugControl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrngDebugControl {
            reserved: bool,
            vnc_bypass: bool,
            trng_crngt_bypass: bool,
            auto_correlate_bypass: bool,
        }
        let proxy = TrngDebugControl {
            reserved: self.reserved(),
            vnc_bypass: self.vnc_bypass(),
            trng_crngt_bypass: self.trng_crngt_bypass(),
            auto_correlate_bypass: self.auto_correlate_bypass(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Generate internal SW reset within the RNG block."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngSwReset(pub u32);
impl TrngSwReset {
    #[doc = "Writing 1'b1 to this register causes an internal RNG reset."]
    #[inline(always)]
    pub const fn trng_sw_reset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Writing 1'b1 to this register causes an internal RNG reset."]
    #[inline(always)]
    pub fn set_trng_sw_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TrngSwReset {
    #[inline(always)]
    fn default() -> TrngSwReset {
        TrngSwReset(0)
    }
}
impl core::fmt::Debug for TrngSwReset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrngSwReset")
            .field("trng_sw_reset", &self.trng_sw_reset())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrngSwReset {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrngSwReset {
            trng_sw_reset: bool,
            reserved: u32,
        }
        let proxy = TrngSwReset {
            trng_sw_reset: self.trng_sw_reset(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "192 bit collection indication."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrngValid(pub u32);
impl TrngValid {
    #[doc = "1'b1 indicates that collection of bits in the RNG is completed, and data can be read from EHR_DATA register."]
    #[inline(always)]
    pub const fn ehr_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1'b1 indicates that collection of bits in the RNG is completed, and data can be read from EHR_DATA register."]
    #[inline(always)]
    pub fn set_ehr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub const fn reserved(&self) -> u32 {
        let val = (self.0 >> 1usize) & 0x7fff_ffff;
        val as u32
    }
    #[doc = "RESERVED"]
    #[inline(always)]
    pub fn set_reserved(&mut self, val: u32) {
        self.0 = (self.0 & !(0x7fff_ffff << 1usize)) | (((val as u32) & 0x7fff_ffff) << 1usize);
    }
}
impl Default for TrngValid {
    #[inline(always)]
    fn default() -> TrngValid {
        TrngValid(0)
    }
}
impl core::fmt::Debug for TrngValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrngValid")
            .field("ehr_valid", &self.ehr_valid())
            .field("reserved", &self.reserved())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrngValid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrngValid {
            ehr_valid: bool,
            reserved: u32,
        }
        let proxy = TrngValid {
            ehr_valid: self.ehr_valid(),
            reserved: self.reserved(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
