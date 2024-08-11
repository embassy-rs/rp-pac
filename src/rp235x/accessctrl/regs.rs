#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Adc0(pub u32);
impl Adc0 {
    #[doc = "If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, ADC0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, ADC0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, ADC0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, ADC0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ADC0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Adc0 {
    #[inline(always)]
    fn default() -> Adc0 {
        Adc0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busctrl(pub u32);
impl Busctrl {
    #[doc = "If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, BUSCTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, BUSCTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, BUSCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Busctrl {
    #[inline(always)]
    fn default() -> Busctrl {
        Busctrl(0)
    }
}
#[doc = "Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers. This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path. Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cfgreset(pub u32);
impl Cfgreset {
    #[inline(always)]
    pub const fn cfgreset(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_cfgreset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Cfgreset {
    #[inline(always)]
    fn default() -> Cfgreset {
        Cfgreset(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clocks(pub u32);
impl Clocks {
    #[doc = "If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, CLOCKS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, CLOCKS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, CLOCKS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, CLOCKS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CLOCKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Clocks {
    #[inline(always)]
    fn default() -> Clocks {
        Clocks(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoresightPeriph(pub u32);
impl CoresightPeriph {
    #[doc = "If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, CORESIGHT_PERIPH can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, CORESIGHT_PERIPH can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_PERIPH can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for CoresightPeriph {
    #[inline(always)]
    fn default() -> CoresightPeriph {
        CoresightPeriph(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoresightTrace(pub u32);
impl CoresightTrace {
    #[doc = "If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, CORESIGHT_TRACE can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, CORESIGHT_TRACE can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, CORESIGHT_TRACE can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for CoresightTrace {
    #[inline(always)]
    fn default() -> CoresightTrace {
        CoresightTrace(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma(pub u32);
impl Dma {
    #[doc = "If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, DMA can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, DMA can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, DMA can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, DMA can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, DMA can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Dma {
    #[inline(always)]
    fn default() -> Dma {
        Dma(0)
    }
}
#[doc = "Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ForceCoreNs(pub u32);
impl ForceCoreNs {
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for ForceCoreNs {
    #[inline(always)]
    fn default() -> ForceCoreNs {
        ForceCoreNs(0)
    }
}
#[doc = "Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioNsmask1(pub u32);
impl GpioNsmask1 {
    #[inline(always)]
    pub const fn gpio(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_gpio(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn usb_dp(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_usb_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn usb_dm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_usb_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn qspi_sck(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_qspi_sck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn qspi_csn(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_qspi_csn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn qspi_sd(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_qspi_sd(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for GpioNsmask1 {
    #[inline(always)]
    fn default() -> GpioNsmask1 {
        GpioNsmask1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Hstx(pub u32);
impl Hstx {
    #[doc = "If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, HSTX can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, HSTX can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, HSTX can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, HSTX can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, HSTX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Hstx {
    #[inline(always)]
    fn default() -> Hstx {
        Hstx(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c0(pub u32);
impl I2c0 {
    #[doc = "If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, I2C0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, I2C0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, I2C0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, I2C0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for I2c0 {
    #[inline(always)]
    fn default() -> I2c0 {
        I2c0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct I2c1(pub u32);
impl I2c1 {
    #[doc = "If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, I2C1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, I2C1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, I2C1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, I2C1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, I2C1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for I2c1 {
    #[inline(always)]
    fn default() -> I2c1 {
        I2c1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IoBank0(pub u32);
impl IoBank0 {
    #[doc = "If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, IO_BANK0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, IO_BANK0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IoBank0 {
    #[inline(always)]
    fn default() -> IoBank0 {
        IoBank0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IoBank1(pub u32);
impl IoBank1 {
    #[doc = "If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, IO_BANK1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, IO_BANK1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, IO_BANK1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for IoBank1 {
    #[inline(always)]
    fn default() -> IoBank1 {
        IoBank1(0)
    }
}
#[doc = "Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger. LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn debug(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_debug(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otp(pub u32);
impl Otp {
    #[doc = "If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, OTP can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, OTP can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, OTP can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, OTP can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, OTP can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Otp {
    #[inline(always)]
    fn default() -> Otp {
        Otp(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadsBank0(pub u32);
impl PadsBank0 {
    #[doc = "If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PADS_BANK0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PADS_BANK0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_BANK0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PadsBank0 {
    #[inline(always)]
    fn default() -> PadsBank0 {
        PadsBank0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PadsQspi(pub u32);
impl PadsQspi {
    #[doc = "If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PADS_QSPI can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PADS_QSPI can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PADS_QSPI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PadsQspi {
    #[inline(always)]
    fn default() -> PadsQspi {
        PadsQspi(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio0(pub u32);
impl Pio0 {
    #[doc = "If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PIO0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PIO0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PIO0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PIO0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pio0 {
    #[inline(always)]
    fn default() -> Pio0 {
        Pio0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio1(pub u32);
impl Pio1 {
    #[doc = "If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PIO1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PIO1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PIO1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PIO1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pio1 {
    #[inline(always)]
    fn default() -> Pio1 {
        Pio1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio2(pub u32);
impl Pio2 {
    #[doc = "If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PIO2 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PIO2 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PIO2 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PIO2 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PIO2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pio2 {
    #[inline(always)]
    fn default() -> Pio2 {
        Pio2(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllSys(pub u32);
impl PllSys {
    #[doc = "If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PLL_SYS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PLL_SYS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_SYS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PllSys {
    #[inline(always)]
    fn default() -> PllSys {
        PllSys(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PllUsb(pub u32);
impl PllUsb {
    #[doc = "If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PLL_USB can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PLL_USB can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PLL_USB can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PLL_USB can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PLL_USB can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for PllUsb {
    #[inline(always)]
    fn default() -> PllUsb {
        PllUsb(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Powman(pub u32);
impl Powman {
    #[doc = "If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, POWMAN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, POWMAN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, POWMAN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, POWMAN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, POWMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Powman {
    #[inline(always)]
    fn default() -> Powman {
        Powman(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwm(pub u32);
impl Pwm {
    #[doc = "If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, PWM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, PWM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, PWM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, PWM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, PWM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Pwm {
    #[inline(always)]
    fn default() -> Pwm {
        Pwm(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resets(pub u32);
impl Resets {
    #[doc = "If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, RESETS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, RESETS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, RESETS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, RESETS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RESETS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Resets {
    #[inline(always)]
    fn default() -> Resets {
        Resets(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rom(pub u32);
impl Rom {
    #[doc = "If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, ROM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, ROM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, ROM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, ROM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Rom {
    #[inline(always)]
    fn default() -> Rom {
        Rom(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosc(pub u32);
impl Rosc {
    #[doc = "If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, ROSC can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, ROSC can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, ROSC can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, ROSC can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, ROSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Rosc {
    #[inline(always)]
    fn default() -> Rosc {
        Rosc(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rsm(pub u32);
impl Rsm {
    #[doc = "If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, RSM can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, RSM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, RSM can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, RSM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, RSM can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Rsm {
    #[inline(always)]
    fn default() -> Rsm {
        Rsm(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sha256(pub u32);
impl Sha256 {
    #[doc = "If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SHA256 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SHA256 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SHA256 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SHA256 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SHA256 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sha256 {
    #[inline(always)]
    fn default() -> Sha256 {
        Sha256(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi0(pub u32);
impl Spi0 {
    #[doc = "If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SPI0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SPI0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SPI0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SPI0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Spi0 {
    #[inline(always)]
    fn default() -> Spi0 {
        Spi0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi1(pub u32);
impl Spi1 {
    #[doc = "If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SPI1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SPI1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SPI1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SPI1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SPI1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Spi1 {
    #[inline(always)]
    fn default() -> Spi1 {
        Spi1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram0(pub u32);
impl Sram0 {
    #[doc = "If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram0 {
    #[inline(always)]
    fn default() -> Sram0 {
        Sram0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram1(pub u32);
impl Sram1 {
    #[doc = "If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram1 {
    #[inline(always)]
    fn default() -> Sram1 {
        Sram1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram2(pub u32);
impl Sram2 {
    #[doc = "If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM2 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM2 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM2 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM2 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM2 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram2 {
    #[inline(always)]
    fn default() -> Sram2 {
        Sram2(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram3(pub u32);
impl Sram3 {
    #[doc = "If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM3 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM3 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM3 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM3 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM3 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram3 {
    #[inline(always)]
    fn default() -> Sram3 {
        Sram3(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram4(pub u32);
impl Sram4 {
    #[doc = "If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM4 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM4 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM4 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM4 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM4 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram4 {
    #[inline(always)]
    fn default() -> Sram4 {
        Sram4(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram5(pub u32);
impl Sram5 {
    #[doc = "If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM5 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM5 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM5 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM5 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM5 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram5 {
    #[inline(always)]
    fn default() -> Sram5 {
        Sram5(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram6(pub u32);
impl Sram6 {
    #[doc = "If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM6 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM6 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM6 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM6 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM6 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram6 {
    #[inline(always)]
    fn default() -> Sram6 {
        Sram6(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram7(pub u32);
impl Sram7 {
    #[doc = "If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM7 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM7 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM7 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM7 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM7 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram7 {
    #[inline(always)]
    fn default() -> Sram7 {
        Sram7(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram8(pub u32);
impl Sram8 {
    #[doc = "If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM8 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM8 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM8 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM8 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM8 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram8 {
    #[inline(always)]
    fn default() -> Sram8 {
        Sram8(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sram9(pub u32);
impl Sram9 {
    #[doc = "If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SRAM9 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SRAM9 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SRAM9 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SRAM9 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SRAM9 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sram9 {
    #[inline(always)]
    fn default() -> Sram9 {
        Sram9(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Syscfg(pub u32);
impl Syscfg {
    #[doc = "If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SYSCFG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SYSCFG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SYSCFG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SYSCFG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSCFG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Syscfg {
    #[inline(always)]
    fn default() -> Syscfg {
        Syscfg(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sysinfo(pub u32);
impl Sysinfo {
    #[doc = "If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, SYSINFO can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, SYSINFO can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, SYSINFO can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, SYSINFO can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, SYSINFO can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Sysinfo {
    #[inline(always)]
    fn default() -> Sysinfo {
        Sysinfo(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbman(pub u32);
impl Tbman {
    #[doc = "If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, TBMAN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, TBMAN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, TBMAN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, TBMAN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TBMAN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Tbman {
    #[inline(always)]
    fn default() -> Tbman {
        Tbman(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ticks(pub u32);
impl Ticks {
    #[doc = "If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, TICKS can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, TICKS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, TICKS can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, TICKS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TICKS can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ticks {
    #[inline(always)]
    fn default() -> Ticks {
        Ticks(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0(pub u32);
impl Timer0 {
    #[doc = "If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, TIMER0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, TIMER0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, TIMER0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, TIMER0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Timer0 {
    #[inline(always)]
    fn default() -> Timer0 {
        Timer0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1(pub u32);
impl Timer1 {
    #[doc = "If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, TIMER1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, TIMER1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, TIMER1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, TIMER1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TIMER1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Timer1 {
    #[inline(always)]
    fn default() -> Timer1 {
        Timer1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Trng(pub u32);
impl Trng {
    #[doc = "If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, TRNG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, TRNG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, TRNG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, TRNG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, TRNG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Trng {
    #[inline(always)]
    fn default() -> Trng {
        Trng(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart0(pub u32);
impl Uart0 {
    #[doc = "If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, UART0 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, UART0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, UART0 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, UART0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART0 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Uart0 {
    #[inline(always)]
    fn default() -> Uart0 {
        Uart0(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart1(pub u32);
impl Uart1 {
    #[doc = "If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, UART1 can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, UART1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, UART1 can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, UART1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, UART1 can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Uart1 {
    #[inline(always)]
    fn default() -> Uart1 {
        Uart1(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usbctrl(pub u32);
impl Usbctrl {
    #[doc = "If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, USBCTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, USBCTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, USBCTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, USBCTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, USBCTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Usbctrl {
    #[inline(always)]
    fn default() -> Usbctrl {
        Usbctrl(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Watchdog(pub u32);
impl Watchdog {
    #[doc = "If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, WATCHDOG can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, WATCHDOG can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, WATCHDOG can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Watchdog {
    #[inline(always)]
    fn default() -> Watchdog {
        Watchdog(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipAux(pub u32);
impl XipAux {
    #[doc = "If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, XIP_AUX can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, XIP_AUX can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_AUX can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for XipAux {
    #[inline(always)]
    fn default() -> XipAux {
        XipAux(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipCtrl(pub u32);
impl XipCtrl {
    #[doc = "If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, XIP_CTRL can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, XIP_CTRL can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_CTRL can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for XipCtrl {
    #[inline(always)]
    fn default() -> XipCtrl {
        XipCtrl(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipMain(pub u32);
impl XipMain {
    #[doc = "If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, XIP_MAIN can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, XIP_MAIN can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_MAIN can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for XipMain {
    #[inline(always)]
    fn default() -> XipMain {
        XipMain(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipQmi(pub u32);
impl XipQmi {
    #[doc = "If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, XIP_QMI can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, XIP_QMI can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XIP_QMI can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for XipQmi {
    #[inline(always)]
    fn default() -> XipQmi {
        XipQmi(0)
    }
}
#[doc = "Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Xosc(pub u32);
impl Xosc {
    #[doc = "If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub const fn nsu(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and NSP is also set, XOSC can be accessed from a Non-secure, Unprivileged context. This bit is writable from a Non-secure, Privileged context, if and only if the NSP bit is set."]
    #[inline(always)]
    pub fn set_nsu(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, XOSC can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub const fn nsp(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed from a Non-secure, Privileged context."]
    #[inline(always)]
    pub fn set_nsp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub const fn su(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and SP is also set, XOSC can be accessed from a Secure, Unprivileged context."]
    #[inline(always)]
    pub fn set_su(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, XOSC can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn sp(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_sp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed by core 0, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn core1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed by core 1, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dma(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed by the DMA, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub const fn dbg(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, XOSC can be accessed by the debugger, at security/privilege levels permitted by SP/NSP/SU/NSU in this register."]
    #[inline(always)]
    pub fn set_dbg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Xosc {
    #[inline(always)]
    fn default() -> Xosc {
        Xosc(0)
    }
}
