#[doc = "Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powman {
    ptr: *mut u8,
}
unsafe impl Send for Powman {}
unsafe impl Sync for Powman {}
impl Powman {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Indicates a bad password has been used"]
    #[inline(always)]
    pub const fn badpasswd(self) -> crate::common::Reg<regs::Badpasswd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Voltage Regulator Control"]
    #[inline(always)]
    pub const fn vreg_ctrl(self) -> crate::common::Reg<regs::VregCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Voltage Regulator Status"]
    #[inline(always)]
    pub const fn vreg_sts(self) -> crate::common::Reg<regs::VregSts, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Voltage Regulator Settings"]
    #[inline(always)]
    pub const fn vreg(self) -> crate::common::Reg<regs::Vreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Voltage Regulator Low Power Entry Settings"]
    #[inline(always)]
    pub const fn vreg_lp_entry(self) -> crate::common::Reg<regs::VregLpEntry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Voltage Regulator Low Power Exit Settings"]
    #[inline(always)]
    pub const fn vreg_lp_exit(self) -> crate::common::Reg<regs::VregLpExit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Brown-out Detection Control"]
    #[inline(always)]
    pub const fn bod_ctrl(self) -> crate::common::Reg<regs::BodCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Brown-out Detection Settings"]
    #[inline(always)]
    pub const fn bod(self) -> crate::common::Reg<regs::Bod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Brown-out Detection Low Power Entry Settings"]
    #[inline(always)]
    pub const fn bod_lp_entry(self) -> crate::common::Reg<regs::BodLpEntry, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Brown-out Detection Low Power Exit Settings"]
    #[inline(always)]
    pub const fn bod_lp_exit(self) -> crate::common::Reg<regs::BodLpExit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Low power oscillator control register."]
    #[inline(always)]
    pub const fn lposc(self) -> crate::common::Reg<regs::Lposc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Chip reset control and status"]
    #[inline(always)]
    pub const fn chip_reset(self) -> crate::common::Reg<regs::ChipReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect."]
    #[inline(always)]
    pub const fn wdsel(self) -> crate::common::Reg<regs::Wdsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1"]
    #[inline(always)]
    pub const fn seq_cfg(self) -> crate::common::Reg<regs::SeqCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT & POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes."]
    #[inline(always)]
    pub const fn state(self) -> crate::common::Reg<regs::State, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[inline(always)]
    pub const fn pow_fastdiv(self) -> crate::common::Reg<regs::PowFastdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "power state machine delays"]
    #[inline(always)]
    pub const fn pow_delay(self) -> crate::common::Reg<regs::PowDelay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Configures a gpio as a power mode aware control output"]
    #[inline(always)]
    pub const fn ext_ctrl0(self) -> crate::common::Reg<regs::ExtCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Configures a gpio as a power mode aware control output"]
    #[inline(always)]
    pub const fn ext_ctrl1(self) -> crate::common::Reg<regs::ExtCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register."]
    #[inline(always)]
    pub const fn ext_time_ref(self) -> crate::common::Reg<regs::ExtTimeRef, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC."]
    #[inline(always)]
    pub const fn lposc_freq_khz_int(
        self,
    ) -> crate::common::Reg<regs::LposcFreqKhzInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC."]
    #[inline(always)]
    pub const fn lposc_freq_khz_frac(
        self,
    ) -> crate::common::Reg<regs::LposcFreqKhzFrac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the XOSC."]
    #[inline(always)]
    pub const fn xosc_freq_khz_int(
        self,
    ) -> crate::common::Reg<regs::XoscFreqKhzInt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC."]
    #[inline(always)]
    pub const fn xosc_freq_khz_frac(
        self,
    ) -> crate::common::Reg<regs::XoscFreqKhzFrac, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn set_time_63to48(
        self,
    ) -> crate::common::Reg<regs::SetTime63to48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[inline(always)]
    pub const fn set_time_47to32(
        self,
    ) -> crate::common::Reg<regs::SetTime47to32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[inline(always)]
    pub const fn set_time_31to16(
        self,
    ) -> crate::common::Reg<regs::SetTime31to16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[inline(always)]
    pub const fn set_time_15to0(self) -> crate::common::Reg<regs::SetTime15to0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[inline(always)]
    pub const fn read_time_upper(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[inline(always)]
    pub const fn read_time_lower(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn alarm_time_63to48(
        self,
    ) -> crate::common::Reg<regs::AlarmTime63to48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn alarm_time_47to32(
        self,
    ) -> crate::common::Reg<regs::AlarmTime47to32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[inline(always)]
    pub const fn alarm_time_31to16(
        self,
    ) -> crate::common::Reg<regs::AlarmTime31to16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[inline(always)]
    pub const fn alarm_time_15to0(
        self,
    ) -> crate::common::Reg<regs::AlarmTime15to0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[inline(always)]
    pub const fn timer(self) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
    #[inline(always)]
    pub const fn pwrup(self, n: usize) -> crate::common::Reg<regs::Pwrup, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize + n * 4usize) as _) }
    }
    #[doc = "Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
    #[inline(always)]
    pub const fn current_pwrup_req(
        self,
    ) -> crate::common::Reg<regs::CurrentPwrupReq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
    #[inline(always)]
    pub const fn last_swcore_pwrup(
        self,
    ) -> crate::common::Reg<regs::LastSwcorePwrup, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[inline(always)]
    pub const fn dbg_pwrcfg(self) -> crate::common::Reg<regs::DbgPwrcfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register."]
    #[inline(always)]
    pub const fn bootdis(self) -> crate::common::Reg<regs::Bootdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[inline(always)]
    pub const fn dbgconfig(self) -> crate::common::Reg<regs::Dbgconfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn scratch(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize + n * 4usize) as _) }
    }
    #[doc = "Scratch register. Information persists in low power mode"]
    #[inline(always)]
    pub const fn boot(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd0usize + n * 4usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
}
pub mod regs;
pub mod vals;
