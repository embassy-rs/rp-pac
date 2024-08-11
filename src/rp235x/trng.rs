#[doc = "ARM TrustZone RNG register block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng {
    ptr: *mut u8,
}
unsafe impl Send for Trng {}
unsafe impl Sync for Trng {}
impl Trng {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt masking."]
    #[inline(always)]
    pub const fn rng_imr(self) -> crate::common::Reg<regs::RngImr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "RNG status register. If corresponding RNG_IMR bit is unmasked, an interrupt will be generated."]
    #[inline(always)]
    pub const fn rng_isr(self) -> crate::common::Reg<regs::RngIsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Interrupt/status bit clear Register."]
    #[inline(always)]
    pub const fn rng_icr(self) -> crate::common::Reg<regs::RngIcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Selecting the inverter-chain length."]
    #[inline(always)]
    pub const fn trng_config(self) -> crate::common::Reg<regs::TrngConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "192 bit collection indication."]
    #[inline(always)]
    pub const fn trng_valid(self) -> crate::common::Reg<regs::TrngValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "RNG collected bits."]
    #[inline(always)]
    pub const fn ehr_data5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Enable signal for the random source."]
    #[inline(always)]
    pub const fn rnd_source_enable(
        self,
    ) -> crate::common::Reg<regs::RndSourceEnable, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Counts clocks between sampling of random bit."]
    #[inline(always)]
    pub const fn sample_cnt1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Statistic about Autocorrelation test activations."]
    #[inline(always)]
    pub const fn autocorr_statistic(
        self,
    ) -> crate::common::Reg<regs::AutocorrStatistic, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Debug register."]
    #[inline(always)]
    pub const fn trng_debug_control(
        self,
    ) -> crate::common::Reg<regs::TrngDebugControl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Generate internal SW reset within the RNG block."]
    #[inline(always)]
    pub const fn trng_sw_reset(self) -> crate::common::Reg<regs::TrngSwReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Enable the RNG debug mode"]
    #[inline(always)]
    pub const fn rng_debug_en_input(
        self,
    ) -> crate::common::Reg<regs::RngDebugEnInput, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "RNG Busy indication."]
    #[inline(always)]
    pub const fn trng_busy(self) -> crate::common::Reg<regs::TrngBusy, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "Reset the counter of collected bits in the RNG."]
    #[inline(always)]
    pub const fn rst_bits_counter(
        self,
    ) -> crate::common::Reg<regs::RstBitsCounter, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "Displays the version settings of the TRNG."]
    #[inline(always)]
    pub const fn rng_version(self) -> crate::common::Reg<regs::RngVersion, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_0(
        self,
    ) -> crate::common::Reg<regs::RngBistCntr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_1(
        self,
    ) -> crate::common::Reg<regs::RngBistCntr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
    #[doc = "Collected BIST results."]
    #[inline(always)]
    pub const fn rng_bist_cntr_2(
        self,
    ) -> crate::common::Reg<regs::RngBistCntr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e8usize) as _) }
    }
}
pub mod regs;
