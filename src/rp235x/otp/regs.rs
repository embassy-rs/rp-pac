#[doc = "Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Archsel(pub u32);
impl Archsel {
    #[doc = "Select architecture for core 0."]
    #[inline(always)]
    pub const fn core0(&self) -> super::vals::ArchselCore0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ArchselCore0::from_bits(val as u8)
    }
    #[doc = "Select architecture for core 0."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: super::vals::ArchselCore0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Select architecture for core 1."]
    #[inline(always)]
    pub const fn core1(&self) -> super::vals::ArchselCore1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ArchselCore1::from_bits(val as u8)
    }
    #[doc = "Select architecture for core 1."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: super::vals::ArchselCore1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for Archsel {
    #[inline(always)]
    fn default() -> Archsel {
        Archsel(0)
    }
}
impl core::fmt::Debug for Archsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Archsel")
            .field("core0", &self.core0())
            .field("core1", &self.core1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Archsel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Archsel {
            core0: super::vals::ArchselCore0,
            core1: super::vals::ArchselCore1,
        }
        let proxy = Archsel {
            core0: self.core0(),
            core1: self.core1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ArchselStatus(pub u32);
impl ArchselStatus {
    #[doc = "Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub const fn core0(&self) -> super::vals::ArchselStatusCore0 {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ArchselStatusCore0::from_bits(val as u8)
    }
    #[doc = "Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: super::vals::ArchselStatusCore0) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub const fn core1(&self) -> super::vals::ArchselStatusCore1 {
        let val = (self.0 >> 1usize) & 0x01;
        super::vals::ArchselStatusCore1::from_bits(val as u8)
    }
    #[doc = "Current architecture for core 0. Updated on processor warm reset."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: super::vals::ArchselStatusCore1) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val.to_bits() as u32) & 0x01) << 1usize);
    }
}
impl Default for ArchselStatus {
    #[inline(always)]
    fn default() -> ArchselStatus {
        ArchselStatus(0)
    }
}
impl core::fmt::Debug for ArchselStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ArchselStatus")
            .field("core0", &self.core0())
            .field("core1", &self.core1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ArchselStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ArchselStatus {
            core0: super::vals::ArchselStatusCore0,
            core1: super::vals::ArchselStatusCore1,
        }
        let proxy = ArchselStatus {
            core0: self.core0(),
            core1: self.core1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "During BIST, count address locations that have at least one leaky bit"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bist(pub u32);
impl Bist {
    #[doc = "Number of locations that have at least one leaky bit. Note: This count is true only if the BIST was initiated without the fix option."]
    #[inline(always)]
    pub const fn cnt(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x1fff;
        val as u16
    }
    #[doc = "Number of locations that have at least one leaky bit. Note: This count is true only if the BIST was initiated without the fix option."]
    #[inline(always)]
    pub fn set_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x1fff << 0usize)) | (((val as u32) & 0x1fff) << 0usize);
    }
    #[doc = "The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
    #[inline(always)]
    pub const fn cnt_max(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x0fff;
        val as u16
    }
    #[doc = "The cnt_fail flag will be set if the number of leaky locations exceeds this number"]
    #[inline(always)]
    pub fn set_cnt_max(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 16usize)) | (((val as u32) & 0x0fff) << 16usize);
    }
    #[doc = "Enable the counter before the BIST function is initiated"]
    #[inline(always)]
    pub const fn cnt_ena(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the counter before the BIST function is initiated"]
    #[inline(always)]
    pub fn set_cnt_ena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Clear counter before use"]
    #[inline(always)]
    pub const fn cnt_clr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Clear counter before use"]
    #[inline(always)]
    pub fn set_cnt_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Flag if the count of address locations with at least one leaky bit exceeds cnt_max"]
    #[inline(always)]
    pub const fn cnt_fail(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Flag if the count of address locations with at least one leaky bit exceeds cnt_max"]
    #[inline(always)]
    pub fn set_cnt_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Bist {
    #[inline(always)]
    fn default() -> Bist {
        Bist(0)
    }
}
impl core::fmt::Debug for Bist {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bist")
            .field("cnt", &self.cnt())
            .field("cnt_max", &self.cnt_max())
            .field("cnt_ena", &self.cnt_ena())
            .field("cnt_clr", &self.cnt_clr())
            .field("cnt_fail", &self.cnt_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bist {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bist {
            cnt: u16,
            cnt_max: u16,
            cnt_ena: bool,
            cnt_clr: bool,
            cnt_fail: bool,
        }
        let proxy = Bist {
            cnt: self.cnt(),
            cnt_max: self.cnt_max(),
            cnt_ena: self.cnt_ena(),
            cnt_clr: self.cnt_clr(),
            cnt_fail: self.cnt_fail(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootdis(pub u32);
impl Bootdis {
    #[doc = "When the core is powered down, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the boot scratch registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    pub const fn now(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When the core is powered down, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the boot scratch registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    pub fn set_now(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the boot scratch registers will be ignored following the next core power down. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it via watchdog reset."]
    #[inline(always)]
    pub const fn next(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the boot scratch registers will be ignored following the next core power down. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it via watchdog reset."]
    #[inline(always)]
    pub fn set_next(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Bootdis {
    #[inline(always)]
    fn default() -> Bootdis {
        Bootdis(0)
    }
}
impl core::fmt::Debug for Bootdis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Bootdis")
            .field("now", &self.now())
            .field("next", &self.next())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Bootdis {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Bootdis {
            now: bool,
            next: bool,
        }
        let proxy = Bootdis {
            now: self.now(),
            next: self.next(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Quickly check values of critical flags read during boot up"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Critical(pub u32);
impl Critical {
    #[inline(always)]
    pub const fn secure_boot_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_secure_boot_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn secure_debug_disable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_secure_debug_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn debug_disable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_debug_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn default_archsel(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_default_archsel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn glitch_detector_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_glitch_detector_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn glitch_detector_sens(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_glitch_detector_sens(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[inline(always)]
    pub const fn arm_disable(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_arm_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn riscv_disable(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_riscv_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Critical {
    #[inline(always)]
    fn default() -> Critical {
        Critical(0)
    }
}
impl core::fmt::Debug for Critical {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Critical")
            .field("secure_boot_enable", &self.secure_boot_enable())
            .field("secure_debug_disable", &self.secure_debug_disable())
            .field("debug_disable", &self.debug_disable())
            .field("default_archsel", &self.default_archsel())
            .field("glitch_detector_enable", &self.glitch_detector_enable())
            .field("glitch_detector_sens", &self.glitch_detector_sens())
            .field("arm_disable", &self.arm_disable())
            .field("riscv_disable", &self.riscv_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Critical {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Critical {
            secure_boot_enable: bool,
            secure_debug_disable: bool,
            debug_disable: bool,
            default_archsel: bool,
            glitch_detector_enable: bool,
            glitch_detector_sens: u8,
            arm_disable: bool,
            riscv_disable: bool,
        }
        let proxy = Critical {
            secure_boot_enable: self.secure_boot_enable(),
            secure_debug_disable: self.secure_debug_disable(),
            debug_disable: self.debug_disable(),
            default_archsel: self.default_archsel(),
            glitch_detector_enable: self.glitch_detector_enable(),
            glitch_detector_sens: self.glitch_detector_sens(),
            arm_disable: self.arm_disable(),
            riscv_disable: self.riscv_disable(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug for OTP power-on state machine"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbg(pub u32);
impl Dbg {
    #[doc = "PSM done status flag"]
    #[inline(always)]
    pub const fn psm_done(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PSM done status flag"]
    #[inline(always)]
    pub fn set_psm_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PSM boot done status flag"]
    #[inline(always)]
    pub const fn boot_done(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "PSM boot done status flag"]
    #[inline(always)]
    pub fn set_boot_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Ring oscillator was seen up and running"]
    #[inline(always)]
    pub const fn rosc_up_seen(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Ring oscillator was seen up and running"]
    #[inline(always)]
    pub fn set_rosc_up_seen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Ring oscillator is up and running"]
    #[inline(always)]
    pub const fn rosc_up(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Ring oscillator is up and running"]
    #[inline(always)]
    pub fn set_rosc_up(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Monitor the PSM FSM's state"]
    #[inline(always)]
    pub const fn psm_state(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Monitor the PSM FSM's state"]
    #[inline(always)]
    pub fn set_psm_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "The chip is in RMA mode"]
    #[inline(always)]
    pub const fn customer_rma_flag(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "The chip is in RMA mode"]
    #[inline(always)]
    pub fn set_customer_rma_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Dbg {
    #[inline(always)]
    fn default() -> Dbg {
        Dbg(0)
    }
}
impl core::fmt::Debug for Dbg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbg")
            .field("psm_done", &self.psm_done())
            .field("boot_done", &self.boot_done())
            .field("rosc_up_seen", &self.rosc_up_seen())
            .field("rosc_up", &self.rosc_up())
            .field("psm_state", &self.psm_state())
            .field("customer_rma_flag", &self.customer_rma_flag())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbg {
            psm_done: bool,
            boot_done: bool,
            rosc_up_seen: bool,
            rosc_up: bool,
            psm_state: u8,
            customer_rma_flag: bool,
        }
        let proxy = Dbg {
            psm_done: self.psm_done(),
            boot_done: self.boot_done(),
            rosc_up_seen: self.rosc_up_seen(),
            rosc_up: self.rosc_up(),
            psm_state: self.psm_state(),
            customer_rma_flag: self.customer_rma_flag(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Debugen(pub u32);
impl Debugen {
    #[doc = "Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable core 0's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub const fn proc0_secure(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Permit core 0's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 0 (SPIDEN and SPNIDEN). Secure debug of core 0 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD. Note also that core Mem-APs are unconditionally disabled when a core is switched to RISC-V mode (by setting the ARCHSEL bit and performing a warm reset of the core)."]
    #[inline(always)]
    pub fn set_proc0_secure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable core 1's Mem-AP if it is currently disabled. The Mem-AP is disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
    #[inline(always)]
    pub const fn proc1_secure(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Permit core 1's Mem-AP to generate Secure accesses, assuming it is enabled at all. Also enable secure debug of core 1 (SPIDEN and SPNIDEN). Secure debug of core 1 is disabled by default if the secure debug disable critical flag is set, or if at least one debug key has been enrolled and the most secure of these enrolled key values not yet provided over SWD."]
    #[inline(always)]
    pub fn set_proc1_secure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub const fn misc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable other debug components. Specifically, the CTI, and the APB-AP used to access the RISC-V Debug Module. These components are disabled by default if either of the debug disable critical flags is set, or if at least one debug key has been enrolled and the least secure of these enrolled key values has not been provided over SWD."]
    #[inline(always)]
    pub fn set_misc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Debugen {
    #[inline(always)]
    fn default() -> Debugen {
        Debugen(0)
    }
}
impl core::fmt::Debug for Debugen {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Debugen")
            .field("proc0", &self.proc0())
            .field("proc0_secure", &self.proc0_secure())
            .field("proc1", &self.proc1())
            .field("proc1_secure", &self.proc1_secure())
            .field("misc", &self.misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Debugen {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Debugen {
            proc0: bool,
            proc0_secure: bool,
            proc1: bool,
            proc1_secure: bool,
            misc: bool,
        }
        let proxy = Debugen {
            proc0: self.proc0(),
            proc0_secure: self.proc0_secure(),
            proc1: self.proc1(),
            proc1_secure: self.proc1_secure(),
            misc: self.misc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DebugenLock(pub u32);
impl DebugenLock {
    #[doc = "Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to lock the PROC0 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub const fn proc0_secure(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to lock the PROC0_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn set_proc0_secure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to lock the PROC1 bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub const fn proc1_secure(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to lock the PROC1_SECURE bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn set_proc1_secure(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub const fn misc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to lock the MISC bit of DEBUGEN. Can't be cleared once set."]
    #[inline(always)]
    pub fn set_misc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for DebugenLock {
    #[inline(always)]
    fn default() -> DebugenLock {
        DebugenLock(0)
    }
}
impl core::fmt::Debug for DebugenLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DebugenLock")
            .field("proc0", &self.proc0())
            .field("proc0_secure", &self.proc0_secure())
            .field("proc1", &self.proc1())
            .field("proc1_secure", &self.proc1_secure())
            .field("misc", &self.misc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DebugenLock {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DebugenLock {
            proc0: bool,
            proc0_secure: bool,
            proc1: bool,
            proc1_secure: bool,
            misc: bool,
        }
        let proxy = DebugenLock {
            proc0: self.proc0(),
            proc0_secure: self.proc0_secure(),
            proc1: self.proc1(),
            proc1_secure: self.proc1_secure(),
            misc: self.misc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[inline(always)]
    pub const fn sbpi_flag_n(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sbpi_flag_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn sbpi_wr_fail(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sbpi_wr_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn apb_dctrl_fail(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_apb_dctrl_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn apb_rd_sec_fail(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_apb_rd_sec_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn apb_rd_nsec_fail(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_apb_rd_nsec_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
        f.debug_struct("Int")
            .field("sbpi_flag_n", &self.sbpi_flag_n())
            .field("sbpi_wr_fail", &self.sbpi_wr_fail())
            .field("apb_dctrl_fail", &self.apb_dctrl_fail())
            .field("apb_rd_sec_fail", &self.apb_rd_sec_fail())
            .field("apb_rd_nsec_fail", &self.apb_rd_nsec_fail())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            sbpi_flag_n: bool,
            sbpi_wr_fail: bool,
            apb_dctrl_fail: bool,
            apb_rd_sec_fail: bool,
            apb_rd_nsec_fail: bool,
        }
        let proxy = Int {
            sbpi_flag_n: self.sbpi_flag_n(),
            sbpi_wr_fail: self.sbpi_wr_fail(),
            apb_dctrl_fail: self.apb_dctrl_fail(),
            apb_rd_sec_fail: self.apb_rd_sec_fail(),
            apb_rd_nsec_fail: self.apb_rd_nsec_fail(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Which keys were valid (enrolled) at boot time"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyValid(pub u32);
impl KeyValid {
    #[inline(always)]
    pub const fn key_valid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_key_valid(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for KeyValid {
    #[inline(always)]
    fn default() -> KeyValid {
        KeyValid(0)
    }
}
impl core::fmt::Debug for KeyValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyValid")
            .field("key_valid", &self.key_valid())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyValid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KeyValid {
            key_valid: u8,
        }
        let proxy = KeyValid {
            key_valid: self.key_valid(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Dispatch instructions to the SBPI interface, used for programming the OTP fuses."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbpiInstr(pub u32);
impl SbpiInstr {
    #[doc = "wdata to be used only when payload_size_m1=0"]
    #[inline(always)]
    pub const fn short_wdata(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "wdata to be used only when payload_size_m1=0"]
    #[inline(always)]
    pub fn set_short_wdata(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn cmd(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
    #[inline(always)]
    pub const fn target(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Instruction target, it can be PMC (0x3a) or DAP (0x02)"]
    #[inline(always)]
    pub fn set_target(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
    #[doc = "Instruction payload size in bytes minus 1"]
    #[inline(always)]
    pub const fn payload_size_m1(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Instruction payload size in bytes minus 1"]
    #[inline(always)]
    pub fn set_payload_size_m1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Instruction has payload (data to be written or to be read)"]
    #[inline(always)]
    pub const fn has_payload(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction has payload (data to be written or to be read)"]
    #[inline(always)]
    pub fn set_has_payload(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Payload type is write"]
    #[inline(always)]
    pub const fn is_wr(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Payload type is write"]
    #[inline(always)]
    pub fn set_is_wr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Execute instruction"]
    #[inline(always)]
    pub const fn exec(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Execute instruction"]
    #[inline(always)]
    pub fn set_exec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for SbpiInstr {
    #[inline(always)]
    fn default() -> SbpiInstr {
        SbpiInstr(0)
    }
}
impl core::fmt::Debug for SbpiInstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbpiInstr")
            .field("short_wdata", &self.short_wdata())
            .field("cmd", &self.cmd())
            .field("target", &self.target())
            .field("payload_size_m1", &self.payload_size_m1())
            .field("has_payload", &self.has_payload())
            .field("is_wr", &self.is_wr())
            .field("exec", &self.exec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbpiInstr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SbpiInstr {
            short_wdata: u8,
            cmd: u8,
            target: u8,
            payload_size_m1: u8,
            has_payload: bool,
            is_wr: bool,
            exec: bool,
        }
        let proxy = SbpiInstr {
            short_wdata: self.short_wdata(),
            cmd: self.cmd(),
            target: self.target(),
            payload_size_m1: self.payload_size_m1(),
            has_payload: self.has_payload(),
            is_wr: self.is_wr(),
            exec: self.exec(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SbpiStatus(pub u32);
impl SbpiStatus {
    #[doc = "Read command has returned data"]
    #[inline(always)]
    pub const fn rdata_vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Read command has returned data"]
    #[inline(always)]
    pub fn set_rdata_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Last instruction done"]
    #[inline(always)]
    pub const fn instr_done(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Last instruction done"]
    #[inline(always)]
    pub fn set_instr_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Last instruction missed (dropped), as the previous has not finished running"]
    #[inline(always)]
    pub const fn instr_miss(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Last instruction missed (dropped), as the previous has not finished running"]
    #[inline(always)]
    pub fn set_instr_miss(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "SBPI flag"]
    #[inline(always)]
    pub const fn flag(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "SBPI flag"]
    #[inline(always)]
    pub fn set_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "SBPI MISO (master in - slave out): response from SBPI"]
    #[inline(always)]
    pub const fn miso(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "SBPI MISO (master in - slave out): response from SBPI"]
    #[inline(always)]
    pub fn set_miso(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for SbpiStatus {
    #[inline(always)]
    fn default() -> SbpiStatus {
        SbpiStatus(0)
    }
}
impl core::fmt::Debug for SbpiStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SbpiStatus")
            .field("rdata_vld", &self.rdata_vld())
            .field("instr_done", &self.instr_done())
            .field("instr_miss", &self.instr_miss())
            .field("flag", &self.flag())
            .field("miso", &self.miso())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SbpiStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SbpiStatus {
            rdata_vld: bool,
            instr_done: bool,
            instr_miss: bool,
            flag: bool,
            miso: u8,
        }
        let proxy = SbpiStatus {
            rdata_vld: self.rdata_vld(),
            instr_done: self.instr_done(),
            instr_miss: self.instr_miss(),
            flag: self.flag(),
            miso: self.miso(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock(pub u32);
impl SwLock {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLockSec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLockSec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLockSec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLockNsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLockNsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLockNsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock {
    #[inline(always)]
    fn default() -> SwLock {
        SwLock(0)
    }
}
impl core::fmt::Debug for SwLock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SwLock")
            .field("sec", &self.sec())
            .field("nsec", &self.nsec())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SwLock {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SwLock {
            sec: super::vals::SwLockSec,
            nsec: super::vals::SwLockNsec,
        }
        let proxy = SwLock {
            sec: self.sec(),
            nsec: self.nsec(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls for APB data read interface (USER interface)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usr(pub u32);
impl Usr {
    #[doc = "1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
    #[inline(always)]
    pub const fn dctrl(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "1 enables USER interface; 0 disables USER interface (enables SBPI). This bit must be cleared before performing any SBPI access, such as when programming the OTP. The APB data read interface (USER interface) will be inaccessible during this time, and will return a bus error if any read is attempted."]
    #[inline(always)]
    pub fn set_dctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Power-down; 1 disables current reference. Must be 0 to read data from the OTP."]
    #[inline(always)]
    pub fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Usr {
    #[inline(always)]
    fn default() -> Usr {
        Usr(0)
    }
}
impl core::fmt::Debug for Usr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Usr")
            .field("dctrl", &self.dctrl())
            .field("pd", &self.pd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Usr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Usr {
            dctrl: bool,
            pd: bool,
        }
        let proxy = Usr {
            dctrl: self.dctrl(),
            pd: self.pd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
