#[doc = "GPIO control including function select and overrides."]
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
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::Outover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_outover(&mut self, val: super::vals::Outover) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[inline(always)]
    pub const fn oeover(&self) -> super::vals::Oeover {
        let val = (self.0 >> 12usize) & 0x03;
        super::vals::Oeover::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_oeover(&mut self, val: super::vals::Oeover) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val.to_bits() as u32) & 0x03) << 12usize);
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
impl core::fmt::Debug for GpioCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioCtrl")
            .field("funcsel", &self.funcsel())
            .field("outover", &self.outover())
            .field("oeover", &self.oeover())
            .field("inover", &self.inover())
            .field("irqover", &self.irqover())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpioCtrl {
            funcsel: u8,
            outover: super::vals::Outover,
            oeover: super::vals::Oeover,
            inover: super::vals::Inover,
            irqover: super::vals::Irqover,
        }
        let proxy = GpioCtrl {
            funcsel: self.funcsel(),
            outover: self.outover(),
            oeover: self.oeover(),
            inover: self.inover(),
            irqover: self.irqover(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GpioStatus(pub u32);
impl GpioStatus {
    #[doc = "output signal from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
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
    #[doc = "output enable from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    #[inline(always)]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
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
    #[doc = "input signal from pad, before override is applied"]
    #[inline(always)]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    #[inline(always)]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    #[inline(always)]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    #[inline(always)]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "interrupt from pad before override is applied"]
    #[inline(always)]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    #[inline(always)]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
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
impl core::fmt::Debug for GpioStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GpioStatus")
            .field("outfromperi", &self.outfromperi())
            .field("outtopad", &self.outtopad())
            .field("oefromperi", &self.oefromperi())
            .field("oetopad", &self.oetopad())
            .field("infrompad", &self.infrompad())
            .field("intoperi", &self.intoperi())
            .field("irqfrompad", &self.irqfrompad())
            .field("irqtoproc", &self.irqtoproc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GpioStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct GpioStatus {
            outfromperi: bool,
            outtopad: bool,
            oefromperi: bool,
            oetopad: bool,
            infrompad: bool,
            intoperi: bool,
            irqfrompad: bool,
            irqtoproc: bool,
        }
        let proxy = GpioStatus {
            outfromperi: self.outfromperi(),
            outtopad: self.outtopad(),
            oefromperi: self.oefromperi(),
            oetopad: self.oetopad(),
            infrompad: self.infrompad(),
            intoperi: self.intoperi(),
            irqfrompad: self.irqfrompad(),
            irqtoproc: self.irqtoproc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
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
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field(
                "level_low",
                &[
                    self.level_low(0usize),
                    self.level_low(1usize),
                    self.level_low(2usize),
                    self.level_low(3usize),
                    self.level_low(4usize),
                    self.level_low(5usize),
                    self.level_low(6usize),
                    self.level_low(7usize),
                ],
            )
            .field(
                "level_high",
                &[
                    self.level_high(0usize),
                    self.level_high(1usize),
                    self.level_high(2usize),
                    self.level_high(3usize),
                    self.level_high(4usize),
                    self.level_high(5usize),
                    self.level_high(6usize),
                    self.level_high(7usize),
                ],
            )
            .field(
                "edge_low",
                &[
                    self.edge_low(0usize),
                    self.edge_low(1usize),
                    self.edge_low(2usize),
                    self.edge_low(3usize),
                    self.edge_low(4usize),
                    self.edge_low(5usize),
                    self.edge_low(6usize),
                    self.edge_low(7usize),
                ],
            )
            .field(
                "edge_high",
                &[
                    self.edge_high(0usize),
                    self.edge_high(1usize),
                    self.edge_high(2usize),
                    self.edge_high(3usize),
                    self.edge_high(4usize),
                    self.edge_high(5usize),
                    self.edge_high(6usize),
                    self.edge_high(7usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            level_low: [bool; 8usize],
            level_high: [bool; 8usize],
            edge_low: [bool; 8usize],
            edge_high: [bool; 8usize],
        }
        let proxy = Int {
            level_low: [
                self.level_low(0usize),
                self.level_low(1usize),
                self.level_low(2usize),
                self.level_low(3usize),
                self.level_low(4usize),
                self.level_low(5usize),
                self.level_low(6usize),
                self.level_low(7usize),
            ],
            level_high: [
                self.level_high(0usize),
                self.level_high(1usize),
                self.level_high(2usize),
                self.level_high(3usize),
                self.level_high(4usize),
                self.level_high(5usize),
                self.level_high(6usize),
                self.level_high(7usize),
            ],
            edge_low: [
                self.edge_low(0usize),
                self.edge_low(1usize),
                self.edge_low(2usize),
                self.edge_low(3usize),
                self.edge_low(4usize),
                self.edge_low(5usize),
                self.edge_low(6usize),
                self.edge_low(7usize),
            ],
            edge_high: [
                self.edge_high(0usize),
                self.edge_high(1usize),
                self.edge_high(2usize),
                self.edge_high(3usize),
                self.edge_high(4usize),
                self.edge_high(5usize),
                self.edge_high(6usize),
                self.edge_high(7usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
