use crate::generic::*;
#[doc = "Force block out of reset (i.e. power it on)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FrceOn(pub u32);
impl FrceOn {
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for FrceOn {
    fn default() -> FrceOn {
        FrceOn(0)
    }
}
#[doc = "Indicates the peripheral's registers are ready to access."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Done(pub u32);
impl Done {
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Done {
    fn default() -> Done {
        Done(0)
    }
}
#[doc = "Force into reset (i.e. power it off)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FrceOff(pub u32);
impl FrceOff {
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for FrceOff {
    fn default() -> FrceOff {
        FrceOff(0)
    }
}
#[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Wdsel(pub u32);
impl Wdsel {
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn sio(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn xip(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn sram5(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn sram4(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn sram3(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn sram2(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn sram1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn sram0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn rom(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn resets(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clocks(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn xosc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn rosc(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Wdsel {
    fn default() -> Wdsel {
        Wdsel(0)
    }
}
