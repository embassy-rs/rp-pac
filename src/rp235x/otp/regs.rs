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
pub struct Inte(pub u32);
impl Inte {
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
impl Default for Inte {
    #[inline(always)]
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf(pub u32);
impl Intf {
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
impl Default for Intf {
    #[inline(always)]
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
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
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints(pub u32);
impl Ints {
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
impl Default for Ints {
    #[inline(always)]
    fn default() -> Ints {
        Ints(0)
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
#[doc = "Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock0(pub u32);
impl SwLock0 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock0sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock0sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock0sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock0nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock0nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock0nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock0 {
    #[inline(always)]
    fn default() -> SwLock0 {
        SwLock0(0)
    }
}
#[doc = "Software lock register for page 1. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock1(pub u32);
impl SwLock1 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock1sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock1sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock1sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock1nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock1nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock1nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock1 {
    #[inline(always)]
    fn default() -> SwLock1 {
        SwLock1(0)
    }
}
#[doc = "Software lock register for page 10. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock10(pub u32);
impl SwLock10 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock10sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock10sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock10sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock10nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock10nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock10nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock10 {
    #[inline(always)]
    fn default() -> SwLock10 {
        SwLock10(0)
    }
}
#[doc = "Software lock register for page 11. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock11(pub u32);
impl SwLock11 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock11sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock11sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock11sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock11nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock11nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock11nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock11 {
    #[inline(always)]
    fn default() -> SwLock11 {
        SwLock11(0)
    }
}
#[doc = "Software lock register for page 12. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock12(pub u32);
impl SwLock12 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock12sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock12sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock12sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock12nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock12nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock12nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock12 {
    #[inline(always)]
    fn default() -> SwLock12 {
        SwLock12(0)
    }
}
#[doc = "Software lock register for page 13. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock13(pub u32);
impl SwLock13 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock13sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock13sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock13sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock13nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock13nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock13nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock13 {
    #[inline(always)]
    fn default() -> SwLock13 {
        SwLock13(0)
    }
}
#[doc = "Software lock register for page 14. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock14(pub u32);
impl SwLock14 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock14sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock14sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock14sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock14nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock14nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock14nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock14 {
    #[inline(always)]
    fn default() -> SwLock14 {
        SwLock14(0)
    }
}
#[doc = "Software lock register for page 15. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock15(pub u32);
impl SwLock15 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock15sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock15sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock15sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock15nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock15nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock15nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock15 {
    #[inline(always)]
    fn default() -> SwLock15 {
        SwLock15(0)
    }
}
#[doc = "Software lock register for page 16. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock16(pub u32);
impl SwLock16 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock16sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock16sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock16sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock16nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock16nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock16nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock16 {
    #[inline(always)]
    fn default() -> SwLock16 {
        SwLock16(0)
    }
}
#[doc = "Software lock register for page 17. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock17(pub u32);
impl SwLock17 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock17sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock17sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock17sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock17nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock17nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock17nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock17 {
    #[inline(always)]
    fn default() -> SwLock17 {
        SwLock17(0)
    }
}
#[doc = "Software lock register for page 18. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock18(pub u32);
impl SwLock18 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock18sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock18sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock18sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock18nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock18nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock18nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock18 {
    #[inline(always)]
    fn default() -> SwLock18 {
        SwLock18(0)
    }
}
#[doc = "Software lock register for page 19. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock19(pub u32);
impl SwLock19 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock19sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock19sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock19sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock19nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock19nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock19nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock19 {
    #[inline(always)]
    fn default() -> SwLock19 {
        SwLock19(0)
    }
}
#[doc = "Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock2(pub u32);
impl SwLock2 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock2sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock2sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock2sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock2nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock2nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock2nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock2 {
    #[inline(always)]
    fn default() -> SwLock2 {
        SwLock2(0)
    }
}
#[doc = "Software lock register for page 20. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock20(pub u32);
impl SwLock20 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock20sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock20sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock20sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock20nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock20nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock20nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock20 {
    #[inline(always)]
    fn default() -> SwLock20 {
        SwLock20(0)
    }
}
#[doc = "Software lock register for page 21. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock21(pub u32);
impl SwLock21 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock21sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock21sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock21sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock21nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock21nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock21nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock21 {
    #[inline(always)]
    fn default() -> SwLock21 {
        SwLock21(0)
    }
}
#[doc = "Software lock register for page 22. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock22(pub u32);
impl SwLock22 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock22sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock22sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock22sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock22nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock22nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock22nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock22 {
    #[inline(always)]
    fn default() -> SwLock22 {
        SwLock22(0)
    }
}
#[doc = "Software lock register for page 23. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock23(pub u32);
impl SwLock23 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock23sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock23sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock23sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock23nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock23nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock23nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock23 {
    #[inline(always)]
    fn default() -> SwLock23 {
        SwLock23(0)
    }
}
#[doc = "Software lock register for page 24. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock24(pub u32);
impl SwLock24 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock24sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock24sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock24sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock24nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock24nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock24nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock24 {
    #[inline(always)]
    fn default() -> SwLock24 {
        SwLock24(0)
    }
}
#[doc = "Software lock register for page 25. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock25(pub u32);
impl SwLock25 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock25sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock25sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock25sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock25nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock25nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock25nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock25 {
    #[inline(always)]
    fn default() -> SwLock25 {
        SwLock25(0)
    }
}
#[doc = "Software lock register for page 26. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock26(pub u32);
impl SwLock26 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock26sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock26sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock26sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock26nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock26nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock26nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock26 {
    #[inline(always)]
    fn default() -> SwLock26 {
        SwLock26(0)
    }
}
#[doc = "Software lock register for page 27. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock27(pub u32);
impl SwLock27 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock27sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock27sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock27sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock27nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock27nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock27nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock27 {
    #[inline(always)]
    fn default() -> SwLock27 {
        SwLock27(0)
    }
}
#[doc = "Software lock register for page 28. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock28(pub u32);
impl SwLock28 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock28sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock28sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock28sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock28nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock28nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock28nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock28 {
    #[inline(always)]
    fn default() -> SwLock28 {
        SwLock28(0)
    }
}
#[doc = "Software lock register for page 29. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock29(pub u32);
impl SwLock29 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock29sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock29sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock29sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock29nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock29nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock29nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock29 {
    #[inline(always)]
    fn default() -> SwLock29 {
        SwLock29(0)
    }
}
#[doc = "Software lock register for page 3. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock3(pub u32);
impl SwLock3 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock3sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock3sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock3sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock3nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock3nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock3nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock3 {
    #[inline(always)]
    fn default() -> SwLock3 {
        SwLock3(0)
    }
}
#[doc = "Software lock register for page 30. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock30(pub u32);
impl SwLock30 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock30sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock30sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock30sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock30nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock30nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock30nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock30 {
    #[inline(always)]
    fn default() -> SwLock30 {
        SwLock30(0)
    }
}
#[doc = "Software lock register for page 31. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock31(pub u32);
impl SwLock31 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock31sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock31sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock31sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock31nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock31nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock31nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock31 {
    #[inline(always)]
    fn default() -> SwLock31 {
        SwLock31(0)
    }
}
#[doc = "Software lock register for page 32. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock32(pub u32);
impl SwLock32 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock32sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock32sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock32sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock32nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock32nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock32nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock32 {
    #[inline(always)]
    fn default() -> SwLock32 {
        SwLock32(0)
    }
}
#[doc = "Software lock register for page 33. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock33(pub u32);
impl SwLock33 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock33sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock33sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock33sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock33nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock33nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock33nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock33 {
    #[inline(always)]
    fn default() -> SwLock33 {
        SwLock33(0)
    }
}
#[doc = "Software lock register for page 34. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock34(pub u32);
impl SwLock34 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock34sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock34sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock34sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock34nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock34nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock34nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock34 {
    #[inline(always)]
    fn default() -> SwLock34 {
        SwLock34(0)
    }
}
#[doc = "Software lock register for page 35. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock35(pub u32);
impl SwLock35 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock35sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock35sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock35sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock35nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock35nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock35nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock35 {
    #[inline(always)]
    fn default() -> SwLock35 {
        SwLock35(0)
    }
}
#[doc = "Software lock register for page 36. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock36(pub u32);
impl SwLock36 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock36sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock36sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock36sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock36nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock36nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock36nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock36 {
    #[inline(always)]
    fn default() -> SwLock36 {
        SwLock36(0)
    }
}
#[doc = "Software lock register for page 37. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock37(pub u32);
impl SwLock37 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock37sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock37sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock37sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock37nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock37nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock37nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock37 {
    #[inline(always)]
    fn default() -> SwLock37 {
        SwLock37(0)
    }
}
#[doc = "Software lock register for page 38. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock38(pub u32);
impl SwLock38 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock38sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock38sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock38sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock38nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock38nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock38nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock38 {
    #[inline(always)]
    fn default() -> SwLock38 {
        SwLock38(0)
    }
}
#[doc = "Software lock register for page 39. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock39(pub u32);
impl SwLock39 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock39sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock39sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock39sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock39nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock39nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock39nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock39 {
    #[inline(always)]
    fn default() -> SwLock39 {
        SwLock39(0)
    }
}
#[doc = "Software lock register for page 4. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock4(pub u32);
impl SwLock4 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock4sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock4sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock4sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock4nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock4nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock4nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock4 {
    #[inline(always)]
    fn default() -> SwLock4 {
        SwLock4(0)
    }
}
#[doc = "Software lock register for page 40. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock40(pub u32);
impl SwLock40 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock40sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock40sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock40sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock40nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock40nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock40nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock40 {
    #[inline(always)]
    fn default() -> SwLock40 {
        SwLock40(0)
    }
}
#[doc = "Software lock register for page 41. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock41(pub u32);
impl SwLock41 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock41sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock41sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock41sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock41nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock41nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock41nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock41 {
    #[inline(always)]
    fn default() -> SwLock41 {
        SwLock41(0)
    }
}
#[doc = "Software lock register for page 42. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock42(pub u32);
impl SwLock42 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock42sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock42sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock42sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock42nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock42nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock42nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock42 {
    #[inline(always)]
    fn default() -> SwLock42 {
        SwLock42(0)
    }
}
#[doc = "Software lock register for page 43. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock43(pub u32);
impl SwLock43 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock43sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock43sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock43sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock43nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock43nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock43nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock43 {
    #[inline(always)]
    fn default() -> SwLock43 {
        SwLock43(0)
    }
}
#[doc = "Software lock register for page 44. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock44(pub u32);
impl SwLock44 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock44sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock44sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock44sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock44nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock44nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock44nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock44 {
    #[inline(always)]
    fn default() -> SwLock44 {
        SwLock44(0)
    }
}
#[doc = "Software lock register for page 45. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock45(pub u32);
impl SwLock45 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock45sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock45sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock45sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock45nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock45nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock45nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock45 {
    #[inline(always)]
    fn default() -> SwLock45 {
        SwLock45(0)
    }
}
#[doc = "Software lock register for page 46. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock46(pub u32);
impl SwLock46 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock46sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock46sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock46sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock46nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock46nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock46nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock46 {
    #[inline(always)]
    fn default() -> SwLock46 {
        SwLock46(0)
    }
}
#[doc = "Software lock register for page 47. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock47(pub u32);
impl SwLock47 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock47sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock47sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock47sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock47nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock47nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock47nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock47 {
    #[inline(always)]
    fn default() -> SwLock47 {
        SwLock47(0)
    }
}
#[doc = "Software lock register for page 48. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock48(pub u32);
impl SwLock48 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock48sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock48sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock48sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock48nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock48nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock48nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock48 {
    #[inline(always)]
    fn default() -> SwLock48 {
        SwLock48(0)
    }
}
#[doc = "Software lock register for page 49. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock49(pub u32);
impl SwLock49 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock49sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock49sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock49sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock49nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock49nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock49nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock49 {
    #[inline(always)]
    fn default() -> SwLock49 {
        SwLock49(0)
    }
}
#[doc = "Software lock register for page 5. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock5(pub u32);
impl SwLock5 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock5sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock5sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock5sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock5nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock5nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock5nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock5 {
    #[inline(always)]
    fn default() -> SwLock5 {
        SwLock5(0)
    }
}
#[doc = "Software lock register for page 50. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock50(pub u32);
impl SwLock50 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock50sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock50sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock50sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock50nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock50nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock50nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock50 {
    #[inline(always)]
    fn default() -> SwLock50 {
        SwLock50(0)
    }
}
#[doc = "Software lock register for page 51. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock51(pub u32);
impl SwLock51 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock51sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock51sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock51sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock51nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock51nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock51nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock51 {
    #[inline(always)]
    fn default() -> SwLock51 {
        SwLock51(0)
    }
}
#[doc = "Software lock register for page 52. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock52(pub u32);
impl SwLock52 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock52sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock52sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock52sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock52nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock52nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock52nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock52 {
    #[inline(always)]
    fn default() -> SwLock52 {
        SwLock52(0)
    }
}
#[doc = "Software lock register for page 53. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock53(pub u32);
impl SwLock53 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock53sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock53sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock53sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock53nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock53nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock53nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock53 {
    #[inline(always)]
    fn default() -> SwLock53 {
        SwLock53(0)
    }
}
#[doc = "Software lock register for page 54. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock54(pub u32);
impl SwLock54 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock54sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock54sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock54sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock54nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock54nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock54nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock54 {
    #[inline(always)]
    fn default() -> SwLock54 {
        SwLock54(0)
    }
}
#[doc = "Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock55(pub u32);
impl SwLock55 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock55sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock55sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock55sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock55nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock55nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock55nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock55 {
    #[inline(always)]
    fn default() -> SwLock55 {
        SwLock55(0)
    }
}
#[doc = "Software lock register for page 56. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock56(pub u32);
impl SwLock56 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock56sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock56sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock56sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock56nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock56nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock56nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock56 {
    #[inline(always)]
    fn default() -> SwLock56 {
        SwLock56(0)
    }
}
#[doc = "Software lock register for page 57. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock57(pub u32);
impl SwLock57 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock57sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock57sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock57sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock57nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock57nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock57nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock57 {
    #[inline(always)]
    fn default() -> SwLock57 {
        SwLock57(0)
    }
}
#[doc = "Software lock register for page 58. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock58(pub u32);
impl SwLock58 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock58sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock58sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock58sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock58nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock58nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock58nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock58 {
    #[inline(always)]
    fn default() -> SwLock58 {
        SwLock58(0)
    }
}
#[doc = "Software lock register for page 59. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock59(pub u32);
impl SwLock59 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock59sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock59sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock59sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock59nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock59nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock59nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock59 {
    #[inline(always)]
    fn default() -> SwLock59 {
        SwLock59(0)
    }
}
#[doc = "Software lock register for page 6. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock6(pub u32);
impl SwLock6 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock6sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock6sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock6sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock6nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock6nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock6nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock6 {
    #[inline(always)]
    fn default() -> SwLock6 {
        SwLock6(0)
    }
}
#[doc = "Software lock register for page 60. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock60(pub u32);
impl SwLock60 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock60sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock60sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock60sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock60nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock60nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock60nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock60 {
    #[inline(always)]
    fn default() -> SwLock60 {
        SwLock60(0)
    }
}
#[doc = "Software lock register for page 61. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock61(pub u32);
impl SwLock61 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock61sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock61sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock61sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock61nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock61nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock61nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock61 {
    #[inline(always)]
    fn default() -> SwLock61 {
        SwLock61(0)
    }
}
#[doc = "Software lock register for page 62. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock62(pub u32);
impl SwLock62 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock62sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock62sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock62sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock62nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock62nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock62nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock62 {
    #[inline(always)]
    fn default() -> SwLock62 {
        SwLock62(0)
    }
}
#[doc = "Software lock register for page 63. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock63(pub u32);
impl SwLock63 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock63sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock63sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock63sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock63nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock63nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock63nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock63 {
    #[inline(always)]
    fn default() -> SwLock63 {
        SwLock63(0)
    }
}
#[doc = "Software lock register for page 7. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock7(pub u32);
impl SwLock7 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock7sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock7sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock7sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock7nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock7nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock7nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock7 {
    #[inline(always)]
    fn default() -> SwLock7 {
        SwLock7(0)
    }
}
#[doc = "Software lock register for page 8. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock8(pub u32);
impl SwLock8 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock8sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock8sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock8sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock8nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock8nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock8nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock8 {
    #[inline(always)]
    fn default() -> SwLock8 {
        SwLock8(0)
    }
}
#[doc = "Software lock register for page 9. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SwLock9(pub u32);
impl SwLock9 {
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub const fn sec(&self) -> super::vals::SwLock9sec {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::SwLock9sec::from_bits(val as u8)
    }
    #[doc = "Secure lock status. Writes are OR'd with the current value. This field is read-only to Non-secure code."]
    #[inline(always)]
    pub fn set_sec(&mut self, val: super::vals::SwLock9sec) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub const fn nsec(&self) -> super::vals::SwLock9nsec {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::SwLock9nsec::from_bits(val as u8)
    }
    #[doc = "Non-secure lock status. Writes are OR'd with the current value."]
    #[inline(always)]
    pub fn set_nsec(&mut self, val: super::vals::SwLock9nsec) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
}
impl Default for SwLock9 {
    #[inline(always)]
    fn default() -> SwLock9 {
        SwLock9(0)
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
