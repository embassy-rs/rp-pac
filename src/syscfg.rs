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
    #[doc = "Processor core 0 NMI source mask Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    pub const fn proc0_nmi_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Processor core 1 NMI source mask Set a bit high to enable NMI from that IRQ"]
    #[inline(always)]
    pub const fn proc1_nmi_mask(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Configuration for processors"]
    #[inline(always)]
    pub const fn proc_config(self) -> crate::common::Reg<regs::ProcConfig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass(
        self,
    ) -> crate::common::Reg<regs::ProcInSyncBypass, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    #[inline(always)]
    pub const fn proc_in_sync_bypass_hi(
        self,
    ) -> crate::common::Reg<regs::ProcInSyncBypassHi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Directly control the SWD debug port of either processor"]
    #[inline(always)]
    pub const fn dbgforce(self) -> crate::common::Reg<regs::Dbgforce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution"]
    #[inline(always)]
    pub const fn mempowerdown(self) -> crate::common::Reg<regs::Mempowerdown, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
}
pub mod regs;
