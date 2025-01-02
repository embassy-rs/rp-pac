#[doc = "FIFO status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[inline(always)]
    pub const fn level(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "FIFO was written when full. Write 1 to clear."]
    #[inline(always)]
    pub const fn wof(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "FIFO was written when full. Write 1 to clear."]
    #[inline(always)]
    pub fn set_wof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("level", &self.level())
            .field("full", &self.full())
            .field("empty", &self.empty())
            .field("wof", &self.wof())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat {
            level: u8,
            full: bool,
            empty: bool,
            wof: bool,
        }
        let proxy = Stat {
            level: self.level(),
            full: self.full(),
            empty: self.empty(),
            wof: self.wof(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
