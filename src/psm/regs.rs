#[doc = "Force into reset (i.e. power it off)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrceOff(pub u32);
impl FrceOff {
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FrceOff {
    fn default() -> FrceOff {
        FrceOff(0)
    }
}
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdsel(pub u32);
impl Wdsel {
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Wdsel {
    fn default() -> Wdsel {
        Wdsel(0)
    }
}
#[doc = "Force block out of reset (i.e. power it on)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FrceOn(pub u32);
impl FrceOn {
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for FrceOn {
    fn default() -> FrceOn {
        FrceOn(0)
    }
}
#[doc = "Indicates the peripheral's registers are ready to access."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Done(pub u32);
impl Done {
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Done {
    fn default() -> Done {
        Done(0)
    }
}
