#[doc = "Directly control the SWD debug port of either processor"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgforce(pub u32);
impl Dbgforce {
    #[doc = "Observe the value of processor 0 SWDIO output."]
    #[inline(always)]
    pub const fn proc0_swdo(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Observe the value of processor 0 SWDIO output."]
    #[inline(always)]
    pub fn set_proc0_swdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub const fn proc0_swdi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive processor 0 SWDIO input, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn set_proc0_swdi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub const fn proc0_swclk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive processor 0 SWCLK, if PROC0_ATTACH is set"]
    #[inline(always)]
    pub fn set_proc0_swclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub const fn proc0_attach(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Attach processor 0 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn set_proc0_attach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Observe the value of processor 1 SWDIO output."]
    #[inline(always)]
    pub const fn proc1_swdo(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Observe the value of processor 1 SWDIO output."]
    #[inline(always)]
    pub fn set_proc1_swdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub const fn proc1_swdi(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive processor 1 SWDIO input, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn set_proc1_swdi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub const fn proc1_swclk(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive processor 1 SWCLK, if PROC1_ATTACH is set"]
    #[inline(always)]
    pub fn set_proc1_swclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub const fn proc1_attach(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Attach processor 1 debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn set_proc1_attach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Dbgforce {
    #[inline(always)]
    fn default() -> Dbgforce {
        Dbgforce(0)
    }
}
#[doc = "Control power downs to memories. Set high to power down memories. Use with extreme caution"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mempowerdown(pub u32);
impl Mempowerdown {
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Mempowerdown {
    #[inline(always)]
    fn default() -> Mempowerdown {
        Mempowerdown(0)
    }
}
#[doc = "Configuration for processors"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcConfig(pub u32);
impl ProcConfig {
    #[doc = "Indication that proc0 has halted"]
    #[inline(always)]
    pub const fn proc0_halted(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that proc0 has halted"]
    #[inline(always)]
    pub fn set_proc0_halted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indication that proc1 has halted"]
    #[inline(always)]
    pub const fn proc1_halted(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indication that proc1 has halted"]
    #[inline(always)]
    pub fn set_proc1_halted(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Configure proc0 DAP instance ID. Recommend that this is NOT changed until you require debug access in multi-chip environment WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub const fn proc0_dap_instid(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Configure proc0 DAP instance ID. Recommend that this is NOT changed until you require debug access in multi-chip environment WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn set_proc0_dap_instid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Configure proc1 DAP instance ID. Recommend that this is NOT changed until you require debug access in multi-chip environment WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub const fn proc1_dap_instid(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Configure proc1 DAP instance ID. Recommend that this is NOT changed until you require debug access in multi-chip environment WARNING: do not set to 15 as this is reserved for RescueDP"]
    #[inline(always)]
    pub fn set_proc1_dap_instid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ProcConfig {
    #[inline(always)]
    fn default() -> ProcConfig {
        ProcConfig(0)
    }
}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 0...29."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcInSyncBypass(pub u32);
impl ProcInSyncBypass {
    #[inline(always)]
    pub const fn proc_in_sync_bypass(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x3fff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_proc_in_sync_bypass(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 0usize)) | (((val as u32) & 0x3fff_ffff) << 0usize);
    }
}
impl Default for ProcInSyncBypass {
    #[inline(always)]
    fn default() -> ProcInSyncBypass {
        ProcInSyncBypass(0)
    }
}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 30...35 (the QSPI IOs)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcInSyncBypassHi(pub u32);
impl ProcInSyncBypassHi {
    #[inline(always)]
    pub const fn proc_in_sync_bypass_hi(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_proc_in_sync_bypass_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for ProcInSyncBypassHi {
    #[inline(always)]
    fn default() -> ProcInSyncBypassHi {
        ProcInSyncBypassHi(0)
    }
}
