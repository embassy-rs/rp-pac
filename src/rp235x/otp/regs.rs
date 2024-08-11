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
#[doc = "Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
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
