#[doc = "Register block for various chip control signals"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg {
    ptr: *mut u8,
}
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Configuration for processors"]
    #[inline(always)]
    pub const fn proc_config(self) -> crate::common::Reg<regs::ProcConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...31."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63"]
    #[inline(always)]
    pub const fn proc_in_sync_bypass_hi(
        self,
    ) -> crate::common::Reg<regs::ProcInSyncBypassHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Directly control the chip SWD debug port"]
    #[inline(always)]
    pub const fn dbgforce(self) -> crate::common::Reg<regs::Dbgforce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Control PD pins to memories. Set high to put memories to a low power state. In this state the memories will retain contents but not be accessible Use with caution"]
    #[inline(always)]
    pub const fn mempowerdown(self) -> crate::common::Reg<regs::Mempowerdown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Auxiliary system control register"]
    #[inline(always)]
    pub const fn auxctrl(self) -> crate::common::Reg<regs::Auxctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
}
pub mod regs;
