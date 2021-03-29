use crate::generic::*;
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioStatus(pub u32);
impl GpioStatus {
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for GpioStatus {
    fn default() -> GpioStatus {
        GpioStatus(0)
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Int(pub u32);
impl Int {
    pub fn edge_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3u32 + (n as u32) * 4u32;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    pub fn set_edge_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3u32 + (n as u32) * 4u32;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    pub fn edge_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 2u32 + (n as u32) * 4u32;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    pub fn set_edge_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 2u32 + (n as u32) * 4u32;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    pub fn level_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0u32 + (n as u32) * 4u32;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    pub fn set_level_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0u32 + (n as u32) * 4u32;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    pub fn level_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 1u32 + (n as u32) * 4u32;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    pub fn set_level_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 1u32 + (n as u32) * 4u32;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Int {
    fn default() -> Int {
        Int(0)
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct GpioCtrl(pub u32);
impl GpioCtrl {
    pub const fn irqover(&self) -> super::vals::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::vals::Irqover(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.0 as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::vals::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::vals::Inover(val as u8)
    }
    pub fn set_inover(&mut self, val: super::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.0 as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::vals::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::vals::Oeover(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.0 as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::vals::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::Outover(val as u8)
    }
    pub fn set_outover(&mut self, val: super::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for GpioCtrl {
    fn default() -> GpioCtrl {
        GpioCtrl(0)
    }
}
