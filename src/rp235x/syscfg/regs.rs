#[doc = "Auxiliary system control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Auxctrl(pub u32);
impl Auxctrl {
    #[doc = "* Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
    #[inline(always)]
    pub const fn auxctrl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "* Bits 7:2: Reserved * Bit 1: When clear, the LPOSC output is XORed into the TRNG ROSC output as an additional, uncorrelated entropy source. When set, this behaviour is disabled. * Bit 0: Force POWMAN clock to switch to LPOSC, by asserting its WDRESET input. This must be set before initiating a watchdog reset of the RSM from a stage that includes CLOCKS, if POWMAN is running from clk_ref at the point that the watchdog reset takes place. Otherwise, the short pulse generated on clk_ref by the reset of the CLOCKS block may affect POWMAN register state."]
    #[inline(always)]
    pub fn set_auxctrl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Auxctrl {
    #[inline(always)]
    fn default() -> Auxctrl {
        Auxctrl(0)
    }
}
impl core::fmt::Debug for Auxctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Auxctrl")
            .field("auxctrl", &self.auxctrl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Auxctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Auxctrl {
            auxctrl: u8,
        }
        let proxy = Auxctrl {
            auxctrl: self.auxctrl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Directly control the chip SWD debug port"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgforce(pub u32);
impl Dbgforce {
    #[doc = "Observe the value of SWDIO output."]
    #[inline(always)]
    pub const fn swdo(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Observe the value of SWDIO output."]
    #[inline(always)]
    pub fn set_swdo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Directly drive SWDIO input, if ATTACH is set"]
    #[inline(always)]
    pub const fn swdi(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive SWDIO input, if ATTACH is set"]
    #[inline(always)]
    pub fn set_swdi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Directly drive SWCLK, if ATTACH is set"]
    #[inline(always)]
    pub const fn swclk(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Directly drive SWCLK, if ATTACH is set"]
    #[inline(always)]
    pub fn set_swclk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub const fn attach(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Attach chip debug port to syscfg controls, and disconnect it from external SWD pads."]
    #[inline(always)]
    pub fn set_attach(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Dbgforce {
    #[inline(always)]
    fn default() -> Dbgforce {
        Dbgforce(0)
    }
}
impl core::fmt::Debug for Dbgforce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgforce")
            .field("swdo", &self.swdo())
            .field("swdi", &self.swdi())
            .field("swclk", &self.swclk())
            .field("attach", &self.attach())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgforce {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbgforce {
            swdo: bool,
            swdi: bool,
            swclk: bool,
            attach: bool,
        }
        let proxy = Dbgforce {
            swdo: self.swdo(),
            swdi: self.swdi(),
            swclk: self.swclk(),
            attach: self.attach(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control PD pins to memories. Set high to put memories to a low power state. In this state the memories will retain contents but not be accessible Use with caution"]
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
    pub const fn sram6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sram7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sram8(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sram9(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn usb(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn bootram(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_bootram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Mempowerdown {
    #[inline(always)]
    fn default() -> Mempowerdown {
        Mempowerdown(0)
    }
}
impl core::fmt::Debug for Mempowerdown {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mempowerdown")
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("sram6", &self.sram6())
            .field("sram7", &self.sram7())
            .field("sram8", &self.sram8())
            .field("sram9", &self.sram9())
            .field("usb", &self.usb())
            .field("rom", &self.rom())
            .field("bootram", &self.bootram())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mempowerdown {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mempowerdown {
            sram0: bool,
            sram1: bool,
            sram2: bool,
            sram3: bool,
            sram4: bool,
            sram5: bool,
            sram6: bool,
            sram7: bool,
            sram8: bool,
            sram9: bool,
            usb: bool,
            rom: bool,
            bootram: bool,
        }
        let proxy = Mempowerdown {
            sram0: self.sram0(),
            sram1: self.sram1(),
            sram2: self.sram2(),
            sram3: self.sram3(),
            sram4: self.sram4(),
            sram5: self.sram5(),
            sram6: self.sram6(),
            sram7: self.sram7(),
            sram8: self.sram8(),
            sram9: self.sram9(),
            usb: self.usb(),
            rom: self.rom(),
            bootram: self.bootram(),
        };
        defmt::write!(f, "{}", proxy)
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
}
impl Default for ProcConfig {
    #[inline(always)]
    fn default() -> ProcConfig {
        ProcConfig(0)
    }
}
impl core::fmt::Debug for ProcConfig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ProcConfig")
            .field("proc0_halted", &self.proc0_halted())
            .field("proc1_halted", &self.proc1_halted())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ProcConfig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ProcConfig {
            proc0_halted: bool,
            proc1_halted: bool,
        }
        let proxy = ProcConfig {
            proc0_halted: self.proc0_halted(),
            proc1_halted: self.proc1_halted(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "For each bit, if 1, bypass the input synchronizer between that GPIO and the GPIO input register in the SIO. The input synchronizers should generally be unbypassed, to avoid injecting metastabilities into processors. If you're feeling brave, you can bypass to save two cycles of input latency. This register applies to GPIO 32...47. USB GPIO 56..57 QSPI GPIO 58..63"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ProcInSyncBypassHi(pub u32);
impl ProcInSyncBypassHi {
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
impl Default for ProcInSyncBypassHi {
    #[inline(always)]
    fn default() -> ProcInSyncBypassHi {
        ProcInSyncBypassHi(0)
    }
}
impl core::fmt::Debug for ProcInSyncBypassHi {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ProcInSyncBypassHi")
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
impl defmt::Format for ProcInSyncBypassHi {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ProcInSyncBypassHi {
            gpio: u16,
            usb_dp: bool,
            usb_dm: bool,
            qspi_sck: bool,
            qspi_csn: bool,
            qspi_sd: u8,
        }
        let proxy = ProcInSyncBypassHi {
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
