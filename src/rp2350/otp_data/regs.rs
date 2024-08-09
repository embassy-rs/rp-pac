#[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey00(pub u32);
impl Bootkey00 {
    #[inline(always)]
    pub const fn bootkey0_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey00 {
    #[inline(always)]
    fn default() -> Bootkey00 {
        Bootkey00(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey01(pub u32);
impl Bootkey01 {
    #[inline(always)]
    pub const fn bootkey0_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey01 {
    #[inline(always)]
    fn default() -> Bootkey01 {
        Bootkey01(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey010(pub u32);
impl Bootkey010 {
    #[inline(always)]
    pub const fn bootkey0_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey010 {
    #[inline(always)]
    fn default() -> Bootkey010 {
        Bootkey010(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey011(pub u32);
impl Bootkey011 {
    #[inline(always)]
    pub const fn bootkey0_11(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey011 {
    #[inline(always)]
    fn default() -> Bootkey011 {
        Bootkey011(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey012(pub u32);
impl Bootkey012 {
    #[inline(always)]
    pub const fn bootkey0_12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey012 {
    #[inline(always)]
    fn default() -> Bootkey012 {
        Bootkey012(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey013(pub u32);
impl Bootkey013 {
    #[inline(always)]
    pub const fn bootkey0_13(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_13(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey013 {
    #[inline(always)]
    fn default() -> Bootkey013 {
        Bootkey013(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey014(pub u32);
impl Bootkey014 {
    #[inline(always)]
    pub const fn bootkey0_14(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_14(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey014 {
    #[inline(always)]
    fn default() -> Bootkey014 {
        Bootkey014(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey015(pub u32);
impl Bootkey015 {
    #[inline(always)]
    pub const fn bootkey0_15(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_15(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey015 {
    #[inline(always)]
    fn default() -> Bootkey015 {
        Bootkey015(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey02(pub u32);
impl Bootkey02 {
    #[inline(always)]
    pub const fn bootkey0_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey02 {
    #[inline(always)]
    fn default() -> Bootkey02 {
        Bootkey02(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey03(pub u32);
impl Bootkey03 {
    #[inline(always)]
    pub const fn bootkey0_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey03 {
    #[inline(always)]
    fn default() -> Bootkey03 {
        Bootkey03(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey04(pub u32);
impl Bootkey04 {
    #[inline(always)]
    pub const fn bootkey0_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey04 {
    #[inline(always)]
    fn default() -> Bootkey04 {
        Bootkey04(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey05(pub u32);
impl Bootkey05 {
    #[inline(always)]
    pub const fn bootkey0_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey05 {
    #[inline(always)]
    fn default() -> Bootkey05 {
        Bootkey05(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey06(pub u32);
impl Bootkey06 {
    #[inline(always)]
    pub const fn bootkey0_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey06 {
    #[inline(always)]
    fn default() -> Bootkey06 {
        Bootkey06(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey07(pub u32);
impl Bootkey07 {
    #[inline(always)]
    pub const fn bootkey0_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey07 {
    #[inline(always)]
    fn default() -> Bootkey07 {
        Bootkey07(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey08(pub u32);
impl Bootkey08 {
    #[inline(always)]
    pub const fn bootkey0_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey08 {
    #[inline(always)]
    fn default() -> Bootkey08 {
        Bootkey08(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey09(pub u32);
impl Bootkey09 {
    #[inline(always)]
    pub const fn bootkey0_9(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey0_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey09 {
    #[inline(always)]
    fn default() -> Bootkey09 {
        Bootkey09(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey10(pub u32);
impl Bootkey10 {
    #[inline(always)]
    pub const fn bootkey1_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey10 {
    #[inline(always)]
    fn default() -> Bootkey10 {
        Bootkey10(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey11(pub u32);
impl Bootkey11 {
    #[inline(always)]
    pub const fn bootkey1_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey11 {
    #[inline(always)]
    fn default() -> Bootkey11 {
        Bootkey11(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey110(pub u32);
impl Bootkey110 {
    #[inline(always)]
    pub const fn bootkey1_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey110 {
    #[inline(always)]
    fn default() -> Bootkey110 {
        Bootkey110(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey111(pub u32);
impl Bootkey111 {
    #[inline(always)]
    pub const fn bootkey1_11(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey111 {
    #[inline(always)]
    fn default() -> Bootkey111 {
        Bootkey111(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey112(pub u32);
impl Bootkey112 {
    #[inline(always)]
    pub const fn bootkey1_12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey112 {
    #[inline(always)]
    fn default() -> Bootkey112 {
        Bootkey112(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey113(pub u32);
impl Bootkey113 {
    #[inline(always)]
    pub const fn bootkey1_13(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_13(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey113 {
    #[inline(always)]
    fn default() -> Bootkey113 {
        Bootkey113(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey114(pub u32);
impl Bootkey114 {
    #[inline(always)]
    pub const fn bootkey1_14(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_14(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey114 {
    #[inline(always)]
    fn default() -> Bootkey114 {
        Bootkey114(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey115(pub u32);
impl Bootkey115 {
    #[inline(always)]
    pub const fn bootkey1_15(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_15(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey115 {
    #[inline(always)]
    fn default() -> Bootkey115 {
        Bootkey115(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey12(pub u32);
impl Bootkey12 {
    #[inline(always)]
    pub const fn bootkey1_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey12 {
    #[inline(always)]
    fn default() -> Bootkey12 {
        Bootkey12(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey13(pub u32);
impl Bootkey13 {
    #[inline(always)]
    pub const fn bootkey1_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey13 {
    #[inline(always)]
    fn default() -> Bootkey13 {
        Bootkey13(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey14(pub u32);
impl Bootkey14 {
    #[inline(always)]
    pub const fn bootkey1_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey14 {
    #[inline(always)]
    fn default() -> Bootkey14 {
        Bootkey14(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey15(pub u32);
impl Bootkey15 {
    #[inline(always)]
    pub const fn bootkey1_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey15 {
    #[inline(always)]
    fn default() -> Bootkey15 {
        Bootkey15(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey16(pub u32);
impl Bootkey16 {
    #[inline(always)]
    pub const fn bootkey1_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey16 {
    #[inline(always)]
    fn default() -> Bootkey16 {
        Bootkey16(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey17(pub u32);
impl Bootkey17 {
    #[inline(always)]
    pub const fn bootkey1_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey17 {
    #[inline(always)]
    fn default() -> Bootkey17 {
        Bootkey17(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey18(pub u32);
impl Bootkey18 {
    #[inline(always)]
    pub const fn bootkey1_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey18 {
    #[inline(always)]
    fn default() -> Bootkey18 {
        Bootkey18(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey19(pub u32);
impl Bootkey19 {
    #[inline(always)]
    pub const fn bootkey1_9(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey1_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey19 {
    #[inline(always)]
    fn default() -> Bootkey19 {
        Bootkey19(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey20(pub u32);
impl Bootkey20 {
    #[inline(always)]
    pub const fn bootkey2_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey20 {
    #[inline(always)]
    fn default() -> Bootkey20 {
        Bootkey20(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey21(pub u32);
impl Bootkey21 {
    #[inline(always)]
    pub const fn bootkey2_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey21 {
    #[inline(always)]
    fn default() -> Bootkey21 {
        Bootkey21(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey210(pub u32);
impl Bootkey210 {
    #[inline(always)]
    pub const fn bootkey2_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey210 {
    #[inline(always)]
    fn default() -> Bootkey210 {
        Bootkey210(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey211(pub u32);
impl Bootkey211 {
    #[inline(always)]
    pub const fn bootkey2_11(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey211 {
    #[inline(always)]
    fn default() -> Bootkey211 {
        Bootkey211(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey212(pub u32);
impl Bootkey212 {
    #[inline(always)]
    pub const fn bootkey2_12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey212 {
    #[inline(always)]
    fn default() -> Bootkey212 {
        Bootkey212(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey213(pub u32);
impl Bootkey213 {
    #[inline(always)]
    pub const fn bootkey2_13(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_13(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey213 {
    #[inline(always)]
    fn default() -> Bootkey213 {
        Bootkey213(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey214(pub u32);
impl Bootkey214 {
    #[inline(always)]
    pub const fn bootkey2_14(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_14(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey214 {
    #[inline(always)]
    fn default() -> Bootkey214 {
        Bootkey214(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey215(pub u32);
impl Bootkey215 {
    #[inline(always)]
    pub const fn bootkey2_15(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_15(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey215 {
    #[inline(always)]
    fn default() -> Bootkey215 {
        Bootkey215(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey22(pub u32);
impl Bootkey22 {
    #[inline(always)]
    pub const fn bootkey2_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey22 {
    #[inline(always)]
    fn default() -> Bootkey22 {
        Bootkey22(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey23(pub u32);
impl Bootkey23 {
    #[inline(always)]
    pub const fn bootkey2_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey23 {
    #[inline(always)]
    fn default() -> Bootkey23 {
        Bootkey23(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey24(pub u32);
impl Bootkey24 {
    #[inline(always)]
    pub const fn bootkey2_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey24 {
    #[inline(always)]
    fn default() -> Bootkey24 {
        Bootkey24(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey25(pub u32);
impl Bootkey25 {
    #[inline(always)]
    pub const fn bootkey2_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey25 {
    #[inline(always)]
    fn default() -> Bootkey25 {
        Bootkey25(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey26(pub u32);
impl Bootkey26 {
    #[inline(always)]
    pub const fn bootkey2_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey26 {
    #[inline(always)]
    fn default() -> Bootkey26 {
        Bootkey26(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey27(pub u32);
impl Bootkey27 {
    #[inline(always)]
    pub const fn bootkey2_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey27 {
    #[inline(always)]
    fn default() -> Bootkey27 {
        Bootkey27(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey28(pub u32);
impl Bootkey28 {
    #[inline(always)]
    pub const fn bootkey2_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey28 {
    #[inline(always)]
    fn default() -> Bootkey28 {
        Bootkey28(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey29(pub u32);
impl Bootkey29 {
    #[inline(always)]
    pub const fn bootkey2_9(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey2_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey29 {
    #[inline(always)]
    fn default() -> Bootkey29 {
        Bootkey29(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey30(pub u32);
impl Bootkey30 {
    #[inline(always)]
    pub const fn bootkey3_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey30 {
    #[inline(always)]
    fn default() -> Bootkey30 {
        Bootkey30(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey31(pub u32);
impl Bootkey31 {
    #[inline(always)]
    pub const fn bootkey3_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey31 {
    #[inline(always)]
    fn default() -> Bootkey31 {
        Bootkey31(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey310(pub u32);
impl Bootkey310 {
    #[inline(always)]
    pub const fn bootkey3_10(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_10(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey310 {
    #[inline(always)]
    fn default() -> Bootkey310 {
        Bootkey310(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey311(pub u32);
impl Bootkey311 {
    #[inline(always)]
    pub const fn bootkey3_11(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_11(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey311 {
    #[inline(always)]
    fn default() -> Bootkey311 {
        Bootkey311(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey312(pub u32);
impl Bootkey312 {
    #[inline(always)]
    pub const fn bootkey3_12(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_12(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey312 {
    #[inline(always)]
    fn default() -> Bootkey312 {
        Bootkey312(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey313(pub u32);
impl Bootkey313 {
    #[inline(always)]
    pub const fn bootkey3_13(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_13(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey313 {
    #[inline(always)]
    fn default() -> Bootkey313 {
        Bootkey313(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey314(pub u32);
impl Bootkey314 {
    #[inline(always)]
    pub const fn bootkey3_14(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_14(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey314 {
    #[inline(always)]
    fn default() -> Bootkey314 {
        Bootkey314(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey315(pub u32);
impl Bootkey315 {
    #[inline(always)]
    pub const fn bootkey3_15(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_15(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey315 {
    #[inline(always)]
    fn default() -> Bootkey315 {
        Bootkey315(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey32(pub u32);
impl Bootkey32 {
    #[inline(always)]
    pub const fn bootkey3_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey32 {
    #[inline(always)]
    fn default() -> Bootkey32 {
        Bootkey32(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey33(pub u32);
impl Bootkey33 {
    #[inline(always)]
    pub const fn bootkey3_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey33 {
    #[inline(always)]
    fn default() -> Bootkey33 {
        Bootkey33(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey34(pub u32);
impl Bootkey34 {
    #[inline(always)]
    pub const fn bootkey3_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey34 {
    #[inline(always)]
    fn default() -> Bootkey34 {
        Bootkey34(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey35(pub u32);
impl Bootkey35 {
    #[inline(always)]
    pub const fn bootkey3_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey35 {
    #[inline(always)]
    fn default() -> Bootkey35 {
        Bootkey35(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey36(pub u32);
impl Bootkey36 {
    #[inline(always)]
    pub const fn bootkey3_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey36 {
    #[inline(always)]
    fn default() -> Bootkey36 {
        Bootkey36(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey37(pub u32);
impl Bootkey37 {
    #[inline(always)]
    pub const fn bootkey3_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey37 {
    #[inline(always)]
    fn default() -> Bootkey37 {
        Bootkey37(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey38(pub u32);
impl Bootkey38 {
    #[inline(always)]
    pub const fn bootkey3_8(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_8(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey38 {
    #[inline(always)]
    fn default() -> Bootkey38 {
        Bootkey38(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey39(pub u32);
impl Bootkey39 {
    #[inline(always)]
    pub const fn bootkey3_9(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_bootkey3_9(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Bootkey39 {
    #[inline(always)]
    fn default() -> Bootkey39 {
        Bootkey39(0)
    }
}
#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselLedCfg(pub u32);
impl BootselLedCfg {
    #[doc = "GPIO index to use for bootloader activity LED."]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "GPIO index to use for bootloader activity LED."]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub const fn activelow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub fn set_activelow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for BootselLedCfg {
    #[inline(always)]
    fn default() -> BootselLedCfg {
        BootselLedCfg(0)
    }
}
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselPllCfg(pub u32);
impl BootselPllCfg {
    #[doc = "PLL feedback divisor, in the range 16..320 inclusive."]
    #[inline(always)]
    pub const fn fbdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "PLL feedback divisor, in the range 16..320 inclusive."]
    #[inline(always)]
    pub fn set_fbdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "PLL post-divide 1 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub const fn postdiv1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "PLL post-divide 1 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn set_postdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
    }
    #[doc = "PLL post-divide 2 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub const fn postdiv2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "PLL post-divide 2 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn set_postdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub const fn refdiv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub fn set_refdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for BootselPllCfg {
    #[inline(always)]
    fn default() -> BootselPllCfg {
        BootselPllCfg(0)
    }
}
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselXoscCfg(pub u32);
impl BootselXoscCfg {
    #[doc = "Value of the XOSC_STARTUP register"]
    #[inline(always)]
    pub const fn startup(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Value of the XOSC_STARTUP register"]
    #[inline(always)]
    pub fn set_startup(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
}
impl Default for BootselXoscCfg {
    #[inline(always)]
    fn default() -> BootselXoscCfg {
        BootselXoscCfg(0)
    }
}
#[doc = "Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid0(pub u32);
impl Chipid0 {
    #[inline(always)]
    pub const fn chipid0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_chipid0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Chipid0 {
    #[inline(always)]
    fn default() -> Chipid0 {
        Chipid0(0)
    }
}
#[doc = "Bits 31:16 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid1(pub u32);
impl Chipid1 {
    #[inline(always)]
    pub const fn chipid1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_chipid1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Chipid1 {
    #[inline(always)]
    fn default() -> Chipid1 {
        Chipid1(0)
    }
}
#[doc = "Bits 47:32 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid2(pub u32);
impl Chipid2 {
    #[inline(always)]
    pub const fn chipid2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_chipid2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Chipid2 {
    #[inline(always)]
    fn default() -> Chipid2 {
        Chipid2(0)
    }
}
#[doc = "Bits 63:48 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid3(pub u32);
impl Chipid3 {
    #[inline(always)]
    pub const fn chipid3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_chipid3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Chipid3 {
    #[inline(always)]
    fn default() -> Chipid3 {
        Chipid3(0)
    }
}
#[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashDevinfo(pub u32);
impl FlashDevinfo {
    #[doc = "Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
    #[inline(always)]
    pub const fn cs1_gpio(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
    #[inline(always)]
    pub fn set_cs1_gpio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
    #[inline(always)]
    pub const fn d8h_erase_supported(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
    #[inline(always)]
    pub fn set_d8h_erase_supported(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
    #[inline(always)]
    pub const fn cs0_size(&self) -> super::vals::Cs0size {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Cs0size::from_bits(val as u8)
    }
    #[doc = "The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
    #[inline(always)]
    pub fn set_cs0_size(&mut self, val: super::vals::Cs0size) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub const fn cs1_size(&self) -> super::vals::Cs1size {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cs1size::from_bits(val as u8)
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub fn set_cs1_size(&mut self, val: super::vals::Cs1size) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u32) & 0x0f) << 12usize);
    }
}
impl Default for FlashDevinfo {
    #[inline(always)]
    fn default() -> FlashDevinfo {
        FlashDevinfo(0)
    }
}
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashPartitionSlotSize(pub u32);
impl FlashPartitionSlotSize {
    #[inline(always)]
    pub const fn flash_partition_slot_size(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_flash_partition_slot_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for FlashPartitionSlotSize {
    #[inline(always)]
    fn default() -> FlashPartitionSlotSize {
        FlashPartitionSlotSize(0)
    }
}
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc0(pub u32);
impl InfoCrc0 {
    #[inline(always)]
    pub const fn info_crc0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_info_crc0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for InfoCrc0 {
    #[inline(always)]
    fn default() -> InfoCrc0 {
        InfoCrc0(0)
    }
}
#[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc1(pub u32);
impl InfoCrc1 {
    #[inline(always)]
    pub const fn info_crc1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_info_crc1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for InfoCrc1 {
    #[inline(always)]
    fn default() -> InfoCrc1 {
        InfoCrc1(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key10(pub u32);
impl Key10 {
    #[inline(always)]
    pub const fn key1_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key10 {
    #[inline(always)]
    fn default() -> Key10 {
        Key10(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key11(pub u32);
impl Key11 {
    #[inline(always)]
    pub const fn key1_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key11 {
    #[inline(always)]
    fn default() -> Key11 {
        Key11(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key12(pub u32);
impl Key12 {
    #[inline(always)]
    pub const fn key1_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key12 {
    #[inline(always)]
    fn default() -> Key12 {
        Key12(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key13(pub u32);
impl Key13 {
    #[inline(always)]
    pub const fn key1_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key13 {
    #[inline(always)]
    fn default() -> Key13 {
        Key13(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key14(pub u32);
impl Key14 {
    #[inline(always)]
    pub const fn key1_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key14 {
    #[inline(always)]
    fn default() -> Key14 {
        Key14(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key15(pub u32);
impl Key15 {
    #[inline(always)]
    pub const fn key1_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key15 {
    #[inline(always)]
    fn default() -> Key15 {
        Key15(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key16(pub u32);
impl Key16 {
    #[inline(always)]
    pub const fn key1_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key16 {
    #[inline(always)]
    fn default() -> Key16 {
        Key16(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key17(pub u32);
impl Key17 {
    #[inline(always)]
    pub const fn key1_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key1_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key17 {
    #[inline(always)]
    fn default() -> Key17 {
        Key17(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key20(pub u32);
impl Key20 {
    #[inline(always)]
    pub const fn key2_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key20 {
    #[inline(always)]
    fn default() -> Key20 {
        Key20(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key21(pub u32);
impl Key21 {
    #[inline(always)]
    pub const fn key2_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key21 {
    #[inline(always)]
    fn default() -> Key21 {
        Key21(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key22(pub u32);
impl Key22 {
    #[inline(always)]
    pub const fn key2_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key22 {
    #[inline(always)]
    fn default() -> Key22 {
        Key22(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key23(pub u32);
impl Key23 {
    #[inline(always)]
    pub const fn key2_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key23 {
    #[inline(always)]
    fn default() -> Key23 {
        Key23(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key24(pub u32);
impl Key24 {
    #[inline(always)]
    pub const fn key2_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key24 {
    #[inline(always)]
    fn default() -> Key24 {
        Key24(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key25(pub u32);
impl Key25 {
    #[inline(always)]
    pub const fn key2_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key25 {
    #[inline(always)]
    fn default() -> Key25 {
        Key25(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key26(pub u32);
impl Key26 {
    #[inline(always)]
    pub const fn key2_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key26 {
    #[inline(always)]
    fn default() -> Key26 {
        Key26(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key27(pub u32);
impl Key27 {
    #[inline(always)]
    pub const fn key2_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key2_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key27 {
    #[inline(always)]
    fn default() -> Key27 {
        Key27(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key30(pub u32);
impl Key30 {
    #[inline(always)]
    pub const fn key3_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key30 {
    #[inline(always)]
    fn default() -> Key30 {
        Key30(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key31(pub u32);
impl Key31 {
    #[inline(always)]
    pub const fn key3_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key31 {
    #[inline(always)]
    fn default() -> Key31 {
        Key31(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key32(pub u32);
impl Key32 {
    #[inline(always)]
    pub const fn key3_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key32 {
    #[inline(always)]
    fn default() -> Key32 {
        Key32(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key33(pub u32);
impl Key33 {
    #[inline(always)]
    pub const fn key3_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key33 {
    #[inline(always)]
    fn default() -> Key33 {
        Key33(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key34(pub u32);
impl Key34 {
    #[inline(always)]
    pub const fn key3_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key34 {
    #[inline(always)]
    fn default() -> Key34 {
        Key34(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key35(pub u32);
impl Key35 {
    #[inline(always)]
    pub const fn key3_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key35 {
    #[inline(always)]
    fn default() -> Key35 {
        Key35(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key36(pub u32);
impl Key36 {
    #[inline(always)]
    pub const fn key3_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key36 {
    #[inline(always)]
    fn default() -> Key36 {
        Key36(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key37(pub u32);
impl Key37 {
    #[inline(always)]
    pub const fn key3_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key3_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key37 {
    #[inline(always)]
    fn default() -> Key37 {
        Key37(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key40(pub u32);
impl Key40 {
    #[inline(always)]
    pub const fn key4_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key40 {
    #[inline(always)]
    fn default() -> Key40 {
        Key40(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key41(pub u32);
impl Key41 {
    #[inline(always)]
    pub const fn key4_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key41 {
    #[inline(always)]
    fn default() -> Key41 {
        Key41(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key42(pub u32);
impl Key42 {
    #[inline(always)]
    pub const fn key4_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key42 {
    #[inline(always)]
    fn default() -> Key42 {
        Key42(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key43(pub u32);
impl Key43 {
    #[inline(always)]
    pub const fn key4_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key43 {
    #[inline(always)]
    fn default() -> Key43 {
        Key43(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key44(pub u32);
impl Key44 {
    #[inline(always)]
    pub const fn key4_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key44 {
    #[inline(always)]
    fn default() -> Key44 {
        Key44(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key45(pub u32);
impl Key45 {
    #[inline(always)]
    pub const fn key4_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key45 {
    #[inline(always)]
    fn default() -> Key45 {
        Key45(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key46(pub u32);
impl Key46 {
    #[inline(always)]
    pub const fn key4_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key46 {
    #[inline(always)]
    fn default() -> Key46 {
        Key46(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key47(pub u32);
impl Key47 {
    #[inline(always)]
    pub const fn key4_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key4_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key47 {
    #[inline(always)]
    fn default() -> Key47 {
        Key47(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key50(pub u32);
impl Key50 {
    #[inline(always)]
    pub const fn key5_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key50 {
    #[inline(always)]
    fn default() -> Key50 {
        Key50(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key51(pub u32);
impl Key51 {
    #[inline(always)]
    pub const fn key5_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key51 {
    #[inline(always)]
    fn default() -> Key51 {
        Key51(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key52(pub u32);
impl Key52 {
    #[inline(always)]
    pub const fn key5_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key52 {
    #[inline(always)]
    fn default() -> Key52 {
        Key52(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key53(pub u32);
impl Key53 {
    #[inline(always)]
    pub const fn key5_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key53 {
    #[inline(always)]
    fn default() -> Key53 {
        Key53(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key54(pub u32);
impl Key54 {
    #[inline(always)]
    pub const fn key5_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key54 {
    #[inline(always)]
    fn default() -> Key54 {
        Key54(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key55(pub u32);
impl Key55 {
    #[inline(always)]
    pub const fn key5_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key55 {
    #[inline(always)]
    fn default() -> Key55 {
        Key55(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key56(pub u32);
impl Key56 {
    #[inline(always)]
    pub const fn key5_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key56 {
    #[inline(always)]
    fn default() -> Key56 {
        Key56(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key57(pub u32);
impl Key57 {
    #[inline(always)]
    pub const fn key5_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key5_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key57 {
    #[inline(always)]
    fn default() -> Key57 {
        Key57(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key60(pub u32);
impl Key60 {
    #[inline(always)]
    pub const fn key6_0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key60 {
    #[inline(always)]
    fn default() -> Key60 {
        Key60(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key61(pub u32);
impl Key61 {
    #[inline(always)]
    pub const fn key6_1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key61 {
    #[inline(always)]
    fn default() -> Key61 {
        Key61(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key62(pub u32);
impl Key62 {
    #[inline(always)]
    pub const fn key6_2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key62 {
    #[inline(always)]
    fn default() -> Key62 {
        Key62(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key63(pub u32);
impl Key63 {
    #[inline(always)]
    pub const fn key6_3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key63 {
    #[inline(always)]
    fn default() -> Key63 {
        Key63(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key64(pub u32);
impl Key64 {
    #[inline(always)]
    pub const fn key6_4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key64 {
    #[inline(always)]
    fn default() -> Key64 {
        Key64(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key65(pub u32);
impl Key65 {
    #[inline(always)]
    pub const fn key6_5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key65 {
    #[inline(always)]
    fn default() -> Key65 {
        Key65(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key66(pub u32);
impl Key66 {
    #[inline(always)]
    pub const fn key6_6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key66 {
    #[inline(always)]
    fn default() -> Key66 {
        Key66(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key67(pub u32);
impl Key67 {
    #[inline(always)]
    pub const fn key6_7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_key6_7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Key67 {
    #[inline(always)]
    fn default() -> Key67 {
        Key67(0)
    }
}
#[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LposcCalib(pub u32);
impl LposcCalib {
    #[inline(always)]
    pub const fn lposc_calib(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_lposc_calib(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LposcCalib {
    #[inline(always)]
    fn default() -> LposcCalib {
        LposcCalib(0)
    }
}
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumGpios(pub u32);
impl NumGpios {
    #[inline(always)]
    pub const fn num_gpios(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_num_gpios(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for NumGpios {
    #[inline(always)]
    fn default() -> NumGpios {
        NumGpios(0)
    }
}
#[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst0(pub u32);
impl OtpbootDst0 {
    #[inline(always)]
    pub const fn otpboot_dst0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_otpboot_dst0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OtpbootDst0 {
    #[inline(always)]
    fn default() -> OtpbootDst0 {
        OtpbootDst0(0)
    }
}
#[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst1(pub u32);
impl OtpbootDst1 {
    #[inline(always)]
    pub const fn otpboot_dst1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_otpboot_dst1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OtpbootDst1 {
    #[inline(always)]
    fn default() -> OtpbootDst1 {
        OtpbootDst1(0)
    }
}
#[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootLen(pub u32);
impl OtpbootLen {
    #[inline(always)]
    pub const fn otpboot_len(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_otpboot_len(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OtpbootLen {
    #[inline(always)]
    fn default() -> OtpbootLen {
        OtpbootLen(0)
    }
}
#[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootSrc(pub u32);
impl OtpbootSrc {
    #[inline(always)]
    pub const fn otpboot_src(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_otpboot_src(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for OtpbootSrc {
    #[inline(always)]
    fn default() -> OtpbootSrc {
        OtpbootSrc(0)
    }
}
#[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid0(pub u32);
impl Randid0 {
    #[inline(always)]
    pub const fn randid0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid0 {
    #[inline(always)]
    fn default() -> Randid0 {
        Randid0(0)
    }
}
#[doc = "Bits 31:16 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid1(pub u32);
impl Randid1 {
    #[inline(always)]
    pub const fn randid1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid1 {
    #[inline(always)]
    fn default() -> Randid1 {
        Randid1(0)
    }
}
#[doc = "Bits 47:32 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid2(pub u32);
impl Randid2 {
    #[inline(always)]
    pub const fn randid2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid2 {
    #[inline(always)]
    fn default() -> Randid2 {
        Randid2(0)
    }
}
#[doc = "Bits 63:48 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid3(pub u32);
impl Randid3 {
    #[inline(always)]
    pub const fn randid3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid3 {
    #[inline(always)]
    fn default() -> Randid3 {
        Randid3(0)
    }
}
#[doc = "Bits 79:64 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid4(pub u32);
impl Randid4 {
    #[inline(always)]
    pub const fn randid4(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid4(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid4 {
    #[inline(always)]
    fn default() -> Randid4 {
        Randid4(0)
    }
}
#[doc = "Bits 95:80 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid5(pub u32);
impl Randid5 {
    #[inline(always)]
    pub const fn randid5(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid5(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid5 {
    #[inline(always)]
    fn default() -> Randid5 {
        Randid5(0)
    }
}
#[doc = "Bits 111:96 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid6(pub u32);
impl Randid6 {
    #[inline(always)]
    pub const fn randid6(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid6(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid6 {
    #[inline(always)]
    fn default() -> Randid6 {
        Randid6(0)
    }
}
#[doc = "Bits 127:112 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid7(pub u32);
impl Randid7 {
    #[inline(always)]
    pub const fn randid7(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_randid7(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Randid7 {
    #[inline(always)]
    fn default() -> Randid7 {
        Randid7(0)
    }
}
#[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RoscCalib(pub u32);
impl RoscCalib {
    #[inline(always)]
    pub const fn rosc_calib(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_rosc_calib(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for RoscCalib {
    #[inline(always)]
    fn default() -> RoscCalib {
        RoscCalib(0)
    }
}
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbWhiteLabelAddr(pub u32);
impl UsbWhiteLabelAddr {
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> super::vals::UsbWhiteLabelAddr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::UsbWhiteLabelAddr::from_bits(val as u16)
    }
    #[inline(always)]
    pub fn set_usb_white_label_addr(&mut self, val: super::vals::UsbWhiteLabelAddr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for UsbWhiteLabelAddr {
    #[inline(always)]
    fn default() -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr(0)
    }
}
