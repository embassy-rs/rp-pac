use crate::generic::*;
#[doc = "Feedback divisor (note: this PLL does not support fractional division)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FbdivInt(pub u32);
impl FbdivInt {
    #[doc = "see ctrl reg description for constraints"]
    pub const fn fbdiv_int(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x0fff;
        val as u16
    }
    #[doc = "see ctrl reg description for constraints"]
    pub fn set_fbdiv_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val as u32) & 0x0fff) << 0u32);
    }
}
impl Default for FbdivInt {
    fn default() -> FbdivInt {
        FbdivInt(0)
    }
}
#[doc = "Controls the PLL power modes."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Pwr(pub u32);
impl Pwr {
    #[doc = "PLL VCO powerdown To save power set high when PLL output not required or bypass=1."]
    pub const fn vcopd(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "PLL VCO powerdown To save power set high when PLL output not required or bypass=1."]
    pub fn set_vcopd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "PLL post divider powerdown To save power set high when PLL output not required or bypass=1."]
    pub const fn postdivpd(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "PLL post divider powerdown To save power set high when PLL output not required or bypass=1."]
    pub fn set_postdivpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "PLL DSM powerdown Nothing is achieved by setting this low."]
    pub const fn dsmpd(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "PLL DSM powerdown Nothing is achieved by setting this low."]
    pub fn set_dsmpd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "PLL powerdown To save power set high when PLL output not required."]
    pub const fn pd(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "PLL powerdown To save power set high when PLL output not required."]
    pub fn set_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Pwr {
    fn default() -> Pwr {
        Pwr(0)
    }
}
#[doc = "Control and Status GENERAL CONSTRAINTS: Reference clock frequency min=5MHz, max=800MHz Feedback divider min=16, max=320 VCO frequency min=400MHz, max=1600MHz"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Cs(pub u32);
impl Cs {
    #[doc = "PLL is locked"]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "PLL is locked"]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    pub const fn bypass(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Passes the reference clock to the output instead of the divided VCO. The VCO continues to run so the user can switch between the reference clock and the divided VCO but the output will glitch when doing so."]
    pub fn set_bypass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Divides the PLL input reference clock. Behaviour is undefined for div=0. PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    pub const fn refdiv(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    #[doc = "Divides the PLL input reference clock. Behaviour is undefined for div=0. PLL output will be unpredictable during refdiv changes, wait for lock=1 before using it."]
    pub fn set_refdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Cs {
    fn default() -> Cs {
        Cs(0)
    }
}
#[doc = "Controls the PLL post dividers for the primary output (note: this PLL does not have a secondary output) the primary output is driven from VCO divided by postdiv1*postdiv2"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Prim(pub u32);
impl Prim {
    #[doc = "divide by 1-7"]
    pub const fn postdiv1(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x07;
        val as u8
    }
    #[doc = "divide by 1-7"]
    pub fn set_postdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16u32)) | (((val as u32) & 0x07) << 16u32);
    }
    #[doc = "divide by 1-7"]
    pub const fn postdiv2(&self) -> u8 {
        let val = (self.0 >> 12u32) & 0x07;
        val as u8
    }
    #[doc = "divide by 1-7"]
    pub fn set_postdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12u32)) | (((val as u32) & 0x07) << 12u32);
    }
}
impl Default for Prim {
    fn default() -> Prim {
        Prim(0)
    }
}
