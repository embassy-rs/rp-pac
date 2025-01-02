#[doc = "Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Access(pub u32);
impl Access {
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
impl Default for Access {
    #[inline(always)]
    fn default() -> Access {
        Access(0)
    }
}
impl core::fmt::Debug for Access {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Access")
            .field("nsu", &self.nsu())
            .field("nsp", &self.nsp())
            .field("su", &self.su())
            .field("sp", &self.sp())
            .field("core0", &self.core0())
            .field("core1", &self.core1())
            .field("dma", &self.dma())
            .field("dbg", &self.dbg())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Access {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Access {
            nsu: bool,
            nsp: bool,
            su: bool,
            sp: bool,
            core0: bool,
            core1: bool,
            dma: bool,
            dbg: bool,
        }
        let proxy = Access {
            nsu: self.nsu(),
            nsp: self.nsp(),
            su: self.su(),
            sp: self.sp(),
            core0: self.core0(),
            core1: self.core1(),
            dma: self.dma(),
            dbg: self.dbg(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Cfgreset {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Cfgreset")
            .field("cfgreset", &self.cfgreset())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Cfgreset {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Cfgreset {
            cfgreset: bool,
        }
        let proxy = Cfgreset {
            cfgreset: self.cfgreset(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for ForceCoreNs {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ForceCoreNs")
            .field("core1", &self.core1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ForceCoreNs {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ForceCoreNs {
            core1: bool,
        }
        let proxy = ForceCoreNs {
            core1: self.core1(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for GpioNsmask1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioNsmask1")
            .field("gpio", &self.gpio())
            .field("usb_dp", &self.usb_dp())
            .field("usb_dm", &self.usb_dm())
            .field("qspi_sck", &self.qspi_sck())
            .field("qspi_csn", &self.qspi_csn())
            .field("qspi_sd", &self.qspi_sd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioNsmask1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpioNsmask1 {
            gpio: u16,
            usb_dp: bool,
            usb_dm: bool,
            qspi_sck: bool,
            qspi_csn: bool,
            qspi_sd: u8,
        }
        let proxy = GpioNsmask1 {
            gpio: self.gpio(),
            usb_dp: self.usb_dp(),
            usb_dm: self.usb_dm(),
            qspi_sck: self.qspi_sck(),
            qspi_csn: self.qspi_csn(),
            qspi_sd: self.qspi_sd(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock")
            .field("core0", &self.core0())
            .field("core1", &self.core1())
            .field("dma", &self.dma())
            .field("debug", &self.debug())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lock {
            core0: bool,
            core1: bool,
            dma: bool,
            debug: bool,
        }
        let proxy = Lock {
            core0: self.core0(),
            core1: self.core1(),
            dma: self.dma(),
            debug: self.debug(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
