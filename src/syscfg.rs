use crate::generic::*;
#[doc = "Register block for various chip control signals"]
#[derive(Copy, Clone)]
pub struct Syscfg(*mut u8);
unsafe impl Send for Syscfg {}
unsafe impl Sync for Syscfg {}
impl Syscfg {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Processor core 0 NMI source mask Set a bit high to enable NMI from that IRQ"]
    pub fn proc0_nmi_mask(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Processor core 1 NMI source mask Set a bit high to enable NMI from that IRQ"]
    pub fn proc1_nmi_mask(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Configuration for processors"]
    pub fn proc_config(self) -> Reg<regs::ProcConfig, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
    pub fn proc_in_sync_bypass(self) -> Reg<regs::ProcInSyncBypass, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
    pub fn proc_in_sync_bypass_hi(self) -> Reg<regs::ProcInSyncBypassHi, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Directly control the SWD debug port of either processor"]
    pub fn dbgforce(self) -> Reg<regs::Dbgforce, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution"]
    pub fn mempowerdown(self) -> Reg<regs::Mempowerdown, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
}
pub mod regs;
