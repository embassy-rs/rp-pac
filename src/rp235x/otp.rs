#[doc = "SNPS OTP control IF (SBPI and RPi wrapper control)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otp {
    ptr: *mut u8,
}
unsafe impl Send for Otp {}
unsafe impl Sync for Otp {}
impl Otp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock(self, n: usize) -> crate::common::Reg<regs::SwLock, crate::common::RW> {
        assert!(n < 64usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Dispatch instructions to the SBPI interface, used for programming the OTP fuses."]
    #[inline(always)]
    pub const fn sbpi_instr(self) -> crate::common::Reg<regs::SbpiInstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "SBPI write payload bytes 3..0"]
    #[inline(always)]
    pub const fn sbpi_wdata(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize + n * 4usize) as _) }
    }
    #[doc = "Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn sbpi_status(self) -> crate::common::Reg<regs::SbpiStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Controls for APB data read interface (USER interface)"]
    #[inline(always)]
    pub const fn usr(self) -> crate::common::Reg<regs::Usr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Debug for OTP power-on state machine"]
    #[inline(always)]
    pub const fn dbg(self) -> crate::common::Reg<regs::Dbg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "During BIST, count address locations that have at least one leaky bit"]
    #[inline(always)]
    pub const fn bist(self) -> crate::common::Reg<regs::Bist, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Word 0 (bits 31..0) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize + n * 4usize) as _) }
    }
    #[doc = "Quickly check values of critical flags read during boot up"]
    #[inline(always)]
    pub const fn critical(self) -> crate::common::Reg<regs::Critical, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "Which keys were valid (enrolled) at boot time"]
    #[inline(always)]
    pub const fn key_valid(self) -> crate::common::Reg<regs::KeyValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags."]
    #[inline(always)]
    pub const fn debugen(self) -> crate::common::Reg<regs::Debugen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset."]
    #[inline(always)]
    pub const fn debugen_lock(self) -> crate::common::Reg<regs::DebugenLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module."]
    #[inline(always)]
    pub const fn archsel(self) -> crate::common::Reg<regs::Archsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update."]
    #[inline(always)]
    pub const fn archsel_status(
        self,
    ) -> crate::common::Reg<regs::ArchselStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register."]
    #[inline(always)]
    pub const fn bootdis(self) -> crate::common::Reg<regs::Bootdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
}
pub mod regs;
pub mod vals;
