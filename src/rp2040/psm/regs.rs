#[doc = "Indicates the peripheral's registers are ready to access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done(pub u32);
impl Done {
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Done {
    #[inline(always)]
    fn default() -> Done {
        Done(0)
    }
}
impl core::fmt::Debug for Done {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Done")
            .field("rosc", &self.rosc())
            .field("xosc", &self.xosc())
            .field("clocks", &self.clocks())
            .field("resets", &self.resets())
            .field("busfabric", &self.busfabric())
            .field("rom", &self.rom())
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("xip", &self.xip())
            .field("vreg_and_chip_reset", &self.vreg_and_chip_reset())
            .field("sio", &self.sio())
            .field("proc0", &self.proc0())
            .field("proc1", &self.proc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Done {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Done {
            rosc: bool,
            xosc: bool,
            clocks: bool,
            resets: bool,
            busfabric: bool,
            rom: bool,
            sram0: bool,
            sram1: bool,
            sram2: bool,
            sram3: bool,
            sram4: bool,
            sram5: bool,
            xip: bool,
            vreg_and_chip_reset: bool,
            sio: bool,
            proc0: bool,
            proc1: bool,
        }
        let proxy = Done {
            rosc: self.rosc(),
            xosc: self.xosc(),
            clocks: self.clocks(),
            resets: self.resets(),
            busfabric: self.busfabric(),
            rom: self.rom(),
            sram0: self.sram0(),
            sram1: self.sram1(),
            sram2: self.sram2(),
            sram3: self.sram3(),
            sram4: self.sram4(),
            sram5: self.sram5(),
            xip: self.xip(),
            vreg_and_chip_reset: self.vreg_and_chip_reset(),
            sio: self.sio(),
            proc0: self.proc0(),
            proc1: self.proc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Force into reset (i.e. power it off)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrceOff(pub u32);
impl FrceOff {
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FrceOff {
    #[inline(always)]
    fn default() -> FrceOff {
        FrceOff(0)
    }
}
impl core::fmt::Debug for FrceOff {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrceOff")
            .field("rosc", &self.rosc())
            .field("xosc", &self.xosc())
            .field("clocks", &self.clocks())
            .field("resets", &self.resets())
            .field("busfabric", &self.busfabric())
            .field("rom", &self.rom())
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("xip", &self.xip())
            .field("vreg_and_chip_reset", &self.vreg_and_chip_reset())
            .field("sio", &self.sio())
            .field("proc0", &self.proc0())
            .field("proc1", &self.proc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrceOff {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrceOff {
            rosc: bool,
            xosc: bool,
            clocks: bool,
            resets: bool,
            busfabric: bool,
            rom: bool,
            sram0: bool,
            sram1: bool,
            sram2: bool,
            sram3: bool,
            sram4: bool,
            sram5: bool,
            xip: bool,
            vreg_and_chip_reset: bool,
            sio: bool,
            proc0: bool,
            proc1: bool,
        }
        let proxy = FrceOff {
            rosc: self.rosc(),
            xosc: self.xosc(),
            clocks: self.clocks(),
            resets: self.resets(),
            busfabric: self.busfabric(),
            rom: self.rom(),
            sram0: self.sram0(),
            sram1: self.sram1(),
            sram2: self.sram2(),
            sram3: self.sram3(),
            sram4: self.sram4(),
            sram5: self.sram5(),
            xip: self.xip(),
            vreg_and_chip_reset: self.vreg_and_chip_reset(),
            sio: self.sio(),
            proc0: self.proc0(),
            proc1: self.proc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Force block out of reset (i.e. power it on)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrceOn(pub u32);
impl FrceOn {
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FrceOn {
    #[inline(always)]
    fn default() -> FrceOn {
        FrceOn(0)
    }
}
impl core::fmt::Debug for FrceOn {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FrceOn")
            .field("rosc", &self.rosc())
            .field("xosc", &self.xosc())
            .field("clocks", &self.clocks())
            .field("resets", &self.resets())
            .field("busfabric", &self.busfabric())
            .field("rom", &self.rom())
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("xip", &self.xip())
            .field("vreg_and_chip_reset", &self.vreg_and_chip_reset())
            .field("sio", &self.sio())
            .field("proc0", &self.proc0())
            .field("proc1", &self.proc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FrceOn {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FrceOn {
            rosc: bool,
            xosc: bool,
            clocks: bool,
            resets: bool,
            busfabric: bool,
            rom: bool,
            sram0: bool,
            sram1: bool,
            sram2: bool,
            sram3: bool,
            sram4: bool,
            sram5: bool,
            xip: bool,
            vreg_and_chip_reset: bool,
            sio: bool,
            proc0: bool,
            proc1: bool,
        }
        let proxy = FrceOn {
            rosc: self.rosc(),
            xosc: self.xosc(),
            clocks: self.clocks(),
            resets: self.resets(),
            busfabric: self.busfabric(),
            rom: self.rom(),
            sram0: self.sram0(),
            sram1: self.sram1(),
            sram2: self.sram2(),
            sram3: self.sram3(),
            sram4: self.sram4(),
            sram5: self.sram5(),
            xip: self.xip(),
            vreg_and_chip_reset: self.vreg_and_chip_reset(),
            sio: self.sio(),
            proc0: self.proc0(),
            proc1: self.proc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdsel(pub u32);
impl Wdsel {
    #[inline(always)]
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Wdsel {
    #[inline(always)]
    fn default() -> Wdsel {
        Wdsel(0)
    }
}
impl core::fmt::Debug for Wdsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Wdsel")
            .field("rosc", &self.rosc())
            .field("xosc", &self.xosc())
            .field("clocks", &self.clocks())
            .field("resets", &self.resets())
            .field("busfabric", &self.busfabric())
            .field("rom", &self.rom())
            .field("sram0", &self.sram0())
            .field("sram1", &self.sram1())
            .field("sram2", &self.sram2())
            .field("sram3", &self.sram3())
            .field("sram4", &self.sram4())
            .field("sram5", &self.sram5())
            .field("xip", &self.xip())
            .field("vreg_and_chip_reset", &self.vreg_and_chip_reset())
            .field("sio", &self.sio())
            .field("proc0", &self.proc0())
            .field("proc1", &self.proc1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Wdsel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Wdsel {
            rosc: bool,
            xosc: bool,
            clocks: bool,
            resets: bool,
            busfabric: bool,
            rom: bool,
            sram0: bool,
            sram1: bool,
            sram2: bool,
            sram3: bool,
            sram4: bool,
            sram5: bool,
            xip: bool,
            vreg_and_chip_reset: bool,
            sio: bool,
            proc0: bool,
            proc1: bool,
        }
        let proxy = Wdsel {
            rosc: self.rosc(),
            xosc: self.xosc(),
            clocks: self.clocks(),
            resets: self.resets(),
            busfabric: self.busfabric(),
            rom: self.rom(),
            sram0: self.sram0(),
            sram1: self.sram1(),
            sram2: self.sram2(),
            sram3: self.sram3(),
            sram4: self.sram4(),
            sram5: self.sram5(),
            xip: self.xip(),
            vreg_and_chip_reset: self.vreg_and_chip_reset(),
            sio: self.sio(),
            proc0: self.proc0(),
            proc1: self.proc1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
