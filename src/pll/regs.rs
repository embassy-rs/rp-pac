#[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=750MHz, max=1600MHz"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "Divides the PLL input reference clock. Behaviour is undefined for div=0. PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub const fn refdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Divides the PLL input reference clock. Behaviour is undefined for div=0. PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    #[inline(always)]
    pub fn set_refdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    #[inline(always)]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "PLL is locked"]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "PLL is locked"]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Cs {
    #[inline(always)]
    fn default() -> Cs {
        Cs(0)
    }
}
#[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FbdivInt(pub u32);
impl FbdivInt {
    #[doc = "see ctrl reg description for constraints"]
    #[inline(always)]
    pub const fn fbdiv_int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "see ctrl reg description for constraints"]
    #[inline(always)]
    pub fn set_fbdiv_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
}
impl Default for FbdivInt {
    #[inline(always)]
    fn default() -> FbdivInt {
        FbdivInt(0)
    }
}
#[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Prim(pub u32);
impl Prim {
    #[doc = "divide by 1-7"]
    #[inline(always)]
    pub const fn postdiv2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "divide by 1-7"]
    #[inline(always)]
    pub fn set_postdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "divide by 1-7"]
    #[inline(always)]
    pub const fn postdiv1(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "divide by 1-7"]
    #[inline(always)]
    pub fn set_postdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
}
impl Default for Prim {
    #[inline(always)]
    fn default() -> Prim {
        Prim(0)
    }
}
#[doc = "Controls the PLL power modes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwr(pub u32);
impl Pwr {
    #[doc = "PLL powerdown To save power set high when PLL output not required."]
    #[inline(always)]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "PLL powerdown To save power set high when PLL output not required."]
    #[inline(always)]
    pub fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "PLL DSM powerdown Nothing is achieved by setting this low."]
    #[inline(always)]
    pub const fn dsmpd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "PLL DSM powerdown Nothing is achieved by setting this low."]
    #[inline(always)]
    pub fn set_dsmpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "PLL post divider powerdown To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub const fn postdivpd(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "PLL post divider powerdown To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn set_postdivpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PLL VCO powerdown To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub const fn vcopd(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "PLL VCO powerdown To save power set high when PLL output not required or bypass=1."]
    #[inline(always)]
    pub fn set_vcopd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for Pwr {
    #[inline(always)]
    fn default() -> Pwr {
        Pwr(0)
    }
}
