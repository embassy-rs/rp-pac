#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioCtrl(pub u32);
impl GpioCtrl {
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[inline(always)]
    pub const fn funcsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    #[inline(always)]
    pub fn set_funcsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[inline(always)]
    pub const fn outover(&self) -> super::vals::Outover {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Outover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_outover(&mut self, val: super::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
    }
    #[inline(always)]
    pub const fn oeover(&self) -> super::vals::Oeover {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Oeover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_oeover(&mut self, val: super::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[inline(always)]
    pub const fn inover(&self) -> super::vals::Inover {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Inover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_inover(&mut self, val: super::vals::Inover) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[inline(always)]
    pub const fn irqover(&self) -> super::vals::Irqover {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Irqover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_irqover(&mut self, val: super::vals::Irqover) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
}
impl Default for GpioCtrl {
    #[inline(always)]
    fn default() -> GpioCtrl {
        GpioCtrl(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioStatus(pub u32);
impl GpioStatus {
    #[doc = "output signal to pad after register override is applied"]
    #[inline(always)]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    #[inline(always)]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "output enable to pad after register override is applied"]
    #[inline(always)]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    #[inline(always)]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "input signal from pad, before filtering and override are applied"]
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before filtering and override are applied"]
    #[inline(always)]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "interrupt to processors, after override is applied"]
    #[inline(always)]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    #[inline(always)]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for GpioStatus {
    #[inline(always)]
    fn default() -> GpioStatus {
        GpioStatus(0)
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[inline(always)]
    pub const fn level_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_level_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 0usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn level_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_level_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 1usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn edge_low(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_edge_low(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 2usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn edge_high(&self, n: usize) -> bool {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_edge_high(&mut self, n: usize, val: bool) {
        assert!(n < 8usize);
        let offs = 3usize + n * 4usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
