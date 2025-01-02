#[doc = "Baud rate"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Baudr(pub u32);
impl Baudr {
    #[doc = "SSI clock divider"]
    #[inline(always)]
    pub const fn sckdv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "SSI clock divider"]
    #[inline(always)]
    pub fn set_sckdv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Baudr {
    #[inline(always)]
    fn default() -> Baudr {
        Baudr(0)
    }
}
impl core::fmt::Debug for Baudr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Baudr")
            .field("sckdv", &self.sckdv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Baudr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Baudr {
            sckdv: u16,
        }
        let proxy = Baudr {
            sckdv: self.sckdv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlr0(pub u32);
impl Ctrlr0 {
    #[doc = "Data frame size"]
    #[inline(always)]
    pub const fn dfs(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data frame size"]
    #[inline(always)]
    pub fn set_dfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Frame format"]
    #[inline(always)]
    pub fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Serial clock phase"]
    #[inline(always)]
    pub const fn scph(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Serial clock phase"]
    #[inline(always)]
    pub fn set_scph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Serial clock polarity"]
    #[inline(always)]
    pub const fn scpol(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Serial clock polarity"]
    #[inline(always)]
    pub fn set_scpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transfer mode"]
    #[inline(always)]
    pub const fn tmod(&self) -> super::vals::Tmod {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Tmod::from_bits(val as u8)
    }
    #[doc = "Transfer mode"]
    #[inline(always)]
    pub fn set_tmod(&mut self, val: super::vals::Tmod) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Slave output enable"]
    #[inline(always)]
    pub const fn slv_oe(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Slave output enable"]
    #[inline(always)]
    pub fn set_slv_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Shift register loop (test mode)"]
    #[inline(always)]
    pub const fn srl(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Shift register loop (test mode)"]
    #[inline(always)]
    pub fn set_srl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Control frame size Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub const fn cfs(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[doc = "Control frame size Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn set_cfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[doc = "Data frame size in 32b transfer mode Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub const fn dfs_32(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Data frame size in 32b transfer mode Value of n -> n+1 clocks per frame."]
    #[inline(always)]
    pub fn set_dfs_32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "SPI frame format"]
    #[inline(always)]
    pub const fn spi_frf(&self) -> super::vals::SpiFrf {
        let val = (self.0 >> 21usize) & 0x03;
        super::vals::SpiFrf::from_bits(val as u8)
    }
    #[doc = "SPI frame format"]
    #[inline(always)]
    pub fn set_spi_frf(&mut self, val: super::vals::SpiFrf) {
        self.0 = (self.0 & !(0x03 << 21usize)) | (((val.to_bits() as u32) & 0x03) << 21usize);
    }
    #[doc = "Slave select toggle enable"]
    #[inline(always)]
    pub const fn sste(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Slave select toggle enable"]
    #[inline(always)]
    pub fn set_sste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
}
impl Default for Ctrlr0 {
    #[inline(always)]
    fn default() -> Ctrlr0 {
        Ctrlr0(0)
    }
}
impl core::fmt::Debug for Ctrlr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrlr0")
            .field("dfs", &self.dfs())
            .field("frf", &self.frf())
            .field("scph", &self.scph())
            .field("scpol", &self.scpol())
            .field("tmod", &self.tmod())
            .field("slv_oe", &self.slv_oe())
            .field("srl", &self.srl())
            .field("cfs", &self.cfs())
            .field("dfs_32", &self.dfs_32())
            .field("spi_frf", &self.spi_frf())
            .field("sste", &self.sste())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrlr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrlr0 {
            dfs: u8,
            frf: u8,
            scph: bool,
            scpol: bool,
            tmod: super::vals::Tmod,
            slv_oe: bool,
            srl: bool,
            cfs: u8,
            dfs_32: u8,
            spi_frf: super::vals::SpiFrf,
            sste: bool,
        }
        let proxy = Ctrlr0 {
            dfs: self.dfs(),
            frf: self.frf(),
            scph: self.scph(),
            scpol: self.scpol(),
            tmod: self.tmod(),
            slv_oe: self.slv_oe(),
            srl: self.srl(),
            cfs: self.cfs(),
            dfs_32: self.dfs_32(),
            spi_frf: self.spi_frf(),
            sste: self.sste(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Master Control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrlr1(pub u32);
impl Ctrlr1 {
    #[doc = "Number of data frames"]
    #[inline(always)]
    pub const fn ndf(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Number of data frames"]
    #[inline(always)]
    pub fn set_ndf(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ctrlr1 {
    #[inline(always)]
    fn default() -> Ctrlr1 {
        Ctrlr1(0)
    }
}
impl core::fmt::Debug for Ctrlr1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrlr1").field("ndf", &self.ndf()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrlr1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrlr1 {
            ndf: u16,
        }
        let proxy = Ctrlr1 { ndf: self.ndf() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmacr(pub u32);
impl Dmacr {
    #[doc = "Receive DMA enable"]
    #[inline(always)]
    pub const fn rdmae(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA enable"]
    #[inline(always)]
    pub fn set_rdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit DMA enable"]
    #[inline(always)]
    pub const fn tdmae(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA enable"]
    #[inline(always)]
    pub fn set_tdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Dmacr {
    #[inline(always)]
    fn default() -> Dmacr {
        Dmacr(0)
    }
}
impl core::fmt::Debug for Dmacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmacr")
            .field("rdmae", &self.rdmae())
            .field("tdmae", &self.tdmae())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmacr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dmacr {
            rdmae: bool,
            tdmae: bool,
        }
        let proxy = Dmacr {
            rdmae: self.rdmae(),
            tdmae: self.tdmae(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA RX data level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmardlr(pub u32);
impl Dmardlr {
    #[doc = "Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub const fn dmardl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive data watermark level (DMARDLR+1)"]
    #[inline(always)]
    pub fn set_dmardl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dmardlr {
    #[inline(always)]
    fn default() -> Dmardlr {
        Dmardlr(0)
    }
}
impl core::fmt::Debug for Dmardlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmardlr")
            .field("dmardl", &self.dmardl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmardlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dmardlr {
            dmardl: u8,
        }
        let proxy = Dmardlr {
            dmardl: self.dmardl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA TX data level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dmatdlr(pub u32);
impl Dmatdlr {
    #[doc = "Transmit data watermark level"]
    #[inline(always)]
    pub const fn dmatdl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit data watermark level"]
    #[inline(always)]
    pub fn set_dmatdl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Dmatdlr {
    #[inline(always)]
    fn default() -> Dmatdlr {
        Dmatdlr(0)
    }
}
impl core::fmt::Debug for Dmatdlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dmatdlr")
            .field("dmatdl", &self.dmatdl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dmatdlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dmatdlr {
            dmatdl: u8,
        }
        let proxy = Dmatdlr {
            dmatdl: self.dmatdl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Clear-on-read all active interrupts"]
    #[inline(always)]
    pub const fn icr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read all active interrupts"]
    #[inline(always)]
    pub fn set_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Icr {
    #[inline(always)]
    fn default() -> Icr {
        Icr(0)
    }
}
impl core::fmt::Debug for Icr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Icr").field("icr", &self.icr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Icr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Icr {
            icr: bool,
        }
        let proxy = Icr { icr: self.icr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt mask"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub const fn txeim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty interrupt mask"]
    #[inline(always)]
    pub fn set_txeim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub const fn txoim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn set_txoim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub const fn rxuim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow interrupt mask"]
    #[inline(always)]
    pub fn set_rxuim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub const fn rxoim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow interrupt mask"]
    #[inline(always)]
    pub fn set_rxoim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub const fn rxfim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full interrupt mask"]
    #[inline(always)]
    pub fn set_rxfim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi-master contention interrupt mask"]
    #[inline(always)]
    pub const fn mstim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention interrupt mask"]
    #[inline(always)]
    pub fn set_mstim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Imr {
    #[inline(always)]
    fn default() -> Imr {
        Imr(0)
    }
}
impl core::fmt::Debug for Imr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Imr")
            .field("txeim", &self.txeim())
            .field("txoim", &self.txoim())
            .field("rxuim", &self.rxuim())
            .field("rxoim", &self.rxoim())
            .field("rxfim", &self.rxfim())
            .field("mstim", &self.mstim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Imr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Imr {
            txeim: bool,
            txoim: bool,
            rxuim: bool,
            rxoim: bool,
            rxfim: bool,
            mstim: bool,
        }
        let proxy = Imr {
            txeim: self.txeim(),
            txoim: self.txoim(),
            rxuim: self.rxuim(),
            rxoim: self.rxoim(),
            rxfim: self.rxfim(),
            mstim: self.mstim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Transmit FIFO empty interrupt status"]
    #[inline(always)]
    pub const fn txeis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty interrupt status"]
    #[inline(always)]
    pub fn set_txeis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO overflow interrupt status"]
    #[inline(always)]
    pub const fn txois(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn set_txois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO underflow interrupt status"]
    #[inline(always)]
    pub const fn rxuis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow interrupt status"]
    #[inline(always)]
    pub fn set_rxuis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO overflow interrupt status"]
    #[inline(always)]
    pub const fn rxois(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow interrupt status"]
    #[inline(always)]
    pub fn set_rxois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO full interrupt status"]
    #[inline(always)]
    pub const fn rxfis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full interrupt status"]
    #[inline(always)]
    pub fn set_rxfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi-master contention interrupt status"]
    #[inline(always)]
    pub const fn mstis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention interrupt status"]
    #[inline(always)]
    pub fn set_mstis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Isr {
    #[inline(always)]
    fn default() -> Isr {
        Isr(0)
    }
}
impl core::fmt::Debug for Isr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Isr")
            .field("txeis", &self.txeis())
            .field("txois", &self.txois())
            .field("rxuis", &self.rxuis())
            .field("rxois", &self.rxois())
            .field("rxfis", &self.rxfis())
            .field("mstis", &self.mstis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Isr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Isr {
            txeis: bool,
            txois: bool,
            rxuis: bool,
            rxois: bool,
            rxfis: bool,
            mstis: bool,
        }
        let proxy = Isr {
            txeis: self.txeis(),
            txois: self.txois(),
            rxuis: self.rxuis(),
            rxois: self.rxois(),
            rxfis: self.rxfis(),
            mstis: self.mstis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Multi-master interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Msticr(pub u32);
impl Msticr {
    #[doc = "Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub const fn msticr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read multi-master contention interrupt"]
    #[inline(always)]
    pub fn set_msticr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Msticr {
    #[inline(always)]
    fn default() -> Msticr {
        Msticr(0)
    }
}
impl core::fmt::Debug for Msticr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Msticr")
            .field("msticr", &self.msticr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Msticr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Msticr {
            msticr: bool,
        }
        let proxy = Msticr {
            msticr: self.msticr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Microwire Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mwcr(pub u32);
impl Mwcr {
    #[doc = "Microwire transfer mode"]
    #[inline(always)]
    pub const fn mwmod(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Microwire transfer mode"]
    #[inline(always)]
    pub fn set_mwmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Microwire control"]
    #[inline(always)]
    pub const fn mdd(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Microwire control"]
    #[inline(always)]
    pub fn set_mdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Microwire handshaking"]
    #[inline(always)]
    pub const fn mhs(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Microwire handshaking"]
    #[inline(always)]
    pub fn set_mhs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Mwcr {
    #[inline(always)]
    fn default() -> Mwcr {
        Mwcr(0)
    }
}
impl core::fmt::Debug for Mwcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Mwcr")
            .field("mwmod", &self.mwmod())
            .field("mdd", &self.mdd())
            .field("mhs", &self.mhs())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Mwcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Mwcr {
            mwmod: bool,
            mdd: bool,
            mhs: bool,
        }
        let proxy = Mwcr {
            mwmod: self.mwmod(),
            mdd: self.mdd(),
            mhs: self.mhs(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Raw interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Risr(pub u32);
impl Risr {
    #[doc = "Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub const fn txeir(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty raw interrupt status"]
    #[inline(always)]
    pub fn set_txeir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub const fn txoir(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn set_txoir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO underflow raw interrupt status"]
    #[inline(always)]
    pub const fn rxuir(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow raw interrupt status"]
    #[inline(always)]
    pub fn set_rxuir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub const fn rxoir(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow raw interrupt status"]
    #[inline(always)]
    pub fn set_rxoir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub const fn rxfir(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full raw interrupt status"]
    #[inline(always)]
    pub fn set_rxfir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Multi-master contention raw interrupt status"]
    #[inline(always)]
    pub const fn mstir(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention raw interrupt status"]
    #[inline(always)]
    pub fn set_mstir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Risr {
    #[inline(always)]
    fn default() -> Risr {
        Risr(0)
    }
}
impl core::fmt::Debug for Risr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Risr")
            .field("txeir", &self.txeir())
            .field("txoir", &self.txoir())
            .field("rxuir", &self.rxuir())
            .field("rxoir", &self.rxoir())
            .field("rxfir", &self.rxfir())
            .field("mstir", &self.mstir())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Risr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Risr {
            txeir: bool,
            txoir: bool,
            rxuir: bool,
            rxoir: bool,
            rxfir: bool,
            mstir: bool,
        }
        let proxy = Risr {
            txeir: self.txeir(),
            txoir: self.txoir(),
            rxuir: self.rxuir(),
            rxoir: self.rxoir(),
            rxfir: self.rxfir(),
            mstir: self.mstir(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX sample delay"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RxSampleDly(pub u32);
impl RxSampleDly {
    #[doc = "RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub const fn rsd(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "RXD sample delay (in SCLK cycles)"]
    #[inline(always)]
    pub fn set_rsd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for RxSampleDly {
    #[inline(always)]
    fn default() -> RxSampleDly {
        RxSampleDly(0)
    }
}
impl core::fmt::Debug for RxSampleDly {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RxSampleDly")
            .field("rsd", &self.rsd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RxSampleDly {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RxSampleDly {
            rsd: u8,
        }
        let proxy = RxSampleDly { rsd: self.rsd() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX FIFO level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxflr(pub u32);
impl Rxflr {
    #[doc = "Receive FIFO level"]
    #[inline(always)]
    pub const fn rxtfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO level"]
    #[inline(always)]
    pub fn set_rxtfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxflr {
    #[inline(always)]
    fn default() -> Rxflr {
        Rxflr(0)
    }
}
impl core::fmt::Debug for Rxflr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxflr")
            .field("rxtfl", &self.rxtfl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxflr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rxflr {
            rxtfl: u8,
        }
        let proxy = Rxflr {
            rxtfl: self.rxtfl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX FIFO threshold level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxftlr(pub u32);
impl Rxftlr {
    #[doc = "Receive FIFO threshold"]
    #[inline(always)]
    pub const fn rft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO threshold"]
    #[inline(always)]
    pub fn set_rft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Rxftlr {
    #[inline(always)]
    fn default() -> Rxftlr {
        Rxftlr(0)
    }
}
impl core::fmt::Debug for Rxftlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxftlr").field("rft", &self.rft()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxftlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rxftlr {
            rft: u8,
        }
        let proxy = Rxftlr { rft: self.rft() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX FIFO overflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxoicr(pub u32);
impl Rxoicr {
    #[doc = "Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub const fn rxoicr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read receive FIFO overflow interrupt"]
    #[inline(always)]
    pub fn set_rxoicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxoicr {
    #[inline(always)]
    fn default() -> Rxoicr {
        Rxoicr(0)
    }
}
impl core::fmt::Debug for Rxoicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxoicr")
            .field("rxoicr", &self.rxoicr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxoicr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rxoicr {
            rxoicr: bool,
        }
        let proxy = Rxoicr {
            rxoicr: self.rxoicr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX FIFO underflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rxuicr(pub u32);
impl Rxuicr {
    #[doc = "Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub const fn rxuicr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read receive FIFO underflow interrupt"]
    #[inline(always)]
    pub fn set_rxuicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Rxuicr {
    #[inline(always)]
    fn default() -> Rxuicr {
        Rxuicr(0)
    }
}
impl core::fmt::Debug for Rxuicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Rxuicr")
            .field("rxuicr", &self.rxuicr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Rxuicr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Rxuicr {
            rxuicr: bool,
        }
        let proxy = Rxuicr {
            rxuicr: self.rxuicr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Slave enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ser(pub u32);
impl Ser {
    #[doc = "For each bit: 0 -> slave not selected 1 -> slave selected"]
    #[inline(always)]
    pub const fn ser(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "For each bit: 0 -> slave not selected 1 -> slave selected"]
    #[inline(always)]
    pub fn set_ser(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ser {
    #[inline(always)]
    fn default() -> Ser {
        Ser(0)
    }
}
impl core::fmt::Debug for Ser {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ser").field("ser", &self.ser()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ser {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ser {
            ser: bool,
        }
        let proxy = Ser { ser: self.ser() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SPI control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SpiCtrlr0(pub u32);
impl SpiCtrlr0 {
    #[doc = "Address and instruction transfer format"]
    #[inline(always)]
    pub const fn trans_type(&self) -> super::vals::TransType {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::TransType::from_bits(val as u8)
    }
    #[doc = "Address and instruction transfer format"]
    #[inline(always)]
    pub fn set_trans_type(&mut self, val: super::vals::TransType) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    pub const fn addr_l(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x0f;
        val as u8
    }
    #[doc = "Address length (0b-60b in 4b increments)"]
    #[inline(always)]
    pub fn set_addr_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2usize)) | (((val as u32) & 0x0f) << 2usize);
    }
    #[doc = "Instruction length (0/4/8/16b)"]
    #[inline(always)]
    pub const fn inst_l(&self) -> super::vals::InstL {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::InstL::from_bits(val as u8)
    }
    #[doc = "Instruction length (0/4/8/16b)"]
    #[inline(always)]
    pub fn set_inst_l(&mut self, val: super::vals::InstL) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    pub const fn wait_cycles(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x1f;
        val as u8
    }
    #[doc = "Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    #[inline(always)]
    pub fn set_wait_cycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11usize)) | (((val as u32) & 0x1f) << 11usize);
    }
    #[doc = "SPI DDR transfer enable"]
    #[inline(always)]
    pub const fn spi_ddr_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "SPI DDR transfer enable"]
    #[inline(always)]
    pub fn set_spi_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Instruction DDR transfer enable"]
    #[inline(always)]
    pub const fn inst_ddr_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Instruction DDR transfer enable"]
    #[inline(always)]
    pub fn set_inst_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Read data strobe enable"]
    #[inline(always)]
    pub const fn spi_rxds_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Read data strobe enable"]
    #[inline(always)]
    pub fn set_spi_rxds_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    pub const fn xip_cmd(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0xff;
        val as u8
    }
    #[doc = "SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    #[inline(always)]
    pub fn set_xip_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val as u32) & 0xff) << 24usize);
    }
}
impl Default for SpiCtrlr0 {
    #[inline(always)]
    fn default() -> SpiCtrlr0 {
        SpiCtrlr0(0)
    }
}
impl core::fmt::Debug for SpiCtrlr0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SpiCtrlr0")
            .field("trans_type", &self.trans_type())
            .field("addr_l", &self.addr_l())
            .field("inst_l", &self.inst_l())
            .field("wait_cycles", &self.wait_cycles())
            .field("spi_ddr_en", &self.spi_ddr_en())
            .field("inst_ddr_en", &self.inst_ddr_en())
            .field("spi_rxds_en", &self.spi_rxds_en())
            .field("xip_cmd", &self.xip_cmd())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpiCtrlr0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SpiCtrlr0 {
            trans_type: super::vals::TransType,
            addr_l: u8,
            inst_l: super::vals::InstL,
            wait_cycles: u8,
            spi_ddr_en: bool,
            inst_ddr_en: bool,
            spi_rxds_en: bool,
            xip_cmd: u8,
        }
        let proxy = SpiCtrlr0 {
            trans_type: self.trans_type(),
            addr_l: self.addr_l(),
            inst_l: self.inst_l(),
            wait_cycles: self.wait_cycles(),
            spi_ddr_en: self.spi_ddr_en(),
            inst_ddr_en: self.inst_ddr_en(),
            spi_rxds_en: self.spi_rxds_en(),
            xip_cmd: self.xip_cmd(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "SSI busy flag"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSI busy flag"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO not full"]
    #[inline(always)]
    pub const fn tfnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full"]
    #[inline(always)]
    pub fn set_tfnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Transmit FIFO empty"]
    #[inline(always)]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty"]
    #[inline(always)]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO not empty"]
    #[inline(always)]
    pub const fn rfne(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty"]
    #[inline(always)]
    pub fn set_rfne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO full"]
    #[inline(always)]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full"]
    #[inline(always)]
    pub fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmission error"]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmission error"]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Data collision error"]
    #[inline(always)]
    pub const fn dcol(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Data collision error"]
    #[inline(always)]
    pub fn set_dcol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
}
impl Default for Sr {
    #[inline(always)]
    fn default() -> Sr {
        Sr(0)
    }
}
impl core::fmt::Debug for Sr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sr")
            .field("busy", &self.busy())
            .field("tfnf", &self.tfnf())
            .field("tfe", &self.tfe())
            .field("rfne", &self.rfne())
            .field("rff", &self.rff())
            .field("txe", &self.txe())
            .field("dcol", &self.dcol())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sr {
            busy: bool,
            tfnf: bool,
            tfe: bool,
            rfne: bool,
            rff: bool,
            txe: bool,
            dcol: bool,
        }
        let proxy = Sr {
            busy: self.busy(),
            tfnf: self.tfnf(),
            tfe: self.tfe(),
            rfne: self.rfne(),
            rff: self.rff(),
            txe: self.txe(),
            dcol: self.dcol(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SSI Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ssienr(pub u32);
impl Ssienr {
    #[doc = "SSI enable"]
    #[inline(always)]
    pub const fn ssi_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "SSI enable"]
    #[inline(always)]
    pub fn set_ssi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Ssienr {
    #[inline(always)]
    fn default() -> Ssienr {
        Ssienr(0)
    }
}
impl core::fmt::Debug for Ssienr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ssienr")
            .field("ssi_en", &self.ssi_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ssienr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ssienr {
            ssi_en: bool,
        }
        let proxy = Ssienr {
            ssi_en: self.ssi_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TX drive edge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TxdDriveEdge(pub u32);
impl TxdDriveEdge {
    #[doc = "TXD drive edge"]
    #[inline(always)]
    pub const fn tde(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "TXD drive edge"]
    #[inline(always)]
    pub fn set_tde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for TxdDriveEdge {
    #[inline(always)]
    fn default() -> TxdDriveEdge {
        TxdDriveEdge(0)
    }
}
impl core::fmt::Debug for TxdDriveEdge {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TxdDriveEdge")
            .field("tde", &self.tde())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TxdDriveEdge {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TxdDriveEdge {
            tde: u8,
        }
        let proxy = TxdDriveEdge { tde: self.tde() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TX FIFO level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txflr(pub u32);
impl Txflr {
    #[doc = "Transmit FIFO level"]
    #[inline(always)]
    pub const fn tftfl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO level"]
    #[inline(always)]
    pub fn set_tftfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txflr {
    #[inline(always)]
    fn default() -> Txflr {
        Txflr(0)
    }
}
impl core::fmt::Debug for Txflr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txflr")
            .field("tftfl", &self.tftfl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txflr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Txflr {
            tftfl: u8,
        }
        let proxy = Txflr {
            tftfl: self.tftfl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TX FIFO threshold level"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txftlr(pub u32);
impl Txftlr {
    #[doc = "Transmit FIFO threshold"]
    #[inline(always)]
    pub const fn tft(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO threshold"]
    #[inline(always)]
    pub fn set_tft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Txftlr {
    #[inline(always)]
    fn default() -> Txftlr {
        Txftlr(0)
    }
}
impl core::fmt::Debug for Txftlr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txftlr").field("tft", &self.tft()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txftlr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Txftlr {
            tft: u8,
        }
        let proxy = Txftlr { tft: self.tft() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TX FIFO overflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Txoicr(pub u32);
impl Txoicr {
    #[doc = "Clear-on-read transmit FIFO overflow interrupt"]
    #[inline(always)]
    pub const fn txoicr(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read transmit FIFO overflow interrupt"]
    #[inline(always)]
    pub fn set_txoicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Txoicr {
    #[inline(always)]
    fn default() -> Txoicr {
        Txoicr(0)
    }
}
impl core::fmt::Debug for Txoicr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Txoicr")
            .field("txoicr", &self.txoicr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Txoicr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Txoicr {
            txoicr: bool,
        }
        let proxy = Txoicr {
            txoicr: self.txoicr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
