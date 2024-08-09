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
