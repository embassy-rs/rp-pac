#[doc = "Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootlockStat(pub u32);
impl BootlockStat {
    #[inline(always)]
    pub const fn bootlock_stat(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_bootlock_stat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for BootlockStat {
    #[inline(always)]
    fn default() -> BootlockStat {
        BootlockStat(0)
    }
}
