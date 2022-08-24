#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rosc(pub *mut u8);
unsafe impl Send for Rosc {}
unsafe impl Sync for Rosc {}
impl Rosc {
    #[doc = "Ring Oscillator control"]
    #[inline(always)]
    pub fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength"]
    #[inline(always)]
    pub fn freqa(self) -> crate::common::Reg<regs::Freqa, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "For a detailed description see freqa register"]
    #[inline(always)]
    pub fn freqb(self) -> crate::common::Reg<regs::Freqb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Ring Oscillator pause control This is used to save power by pausing the ROSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: setup the irq before selecting dormant mode"]
    #[inline(always)]
    pub fn dormant(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Controls the output divider"]
    #[inline(always)]
    pub fn div(self) -> crate::common::Reg<regs::Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Controls the phase shifted output"]
    #[inline(always)]
    pub fn phase(self) -> crate::common::Reg<regs::Phase, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Ring Oscillator Status"]
    #[inline(always)]
    pub fn status(self) -> crate::common::Reg<regs::Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
    #[inline(always)]
    pub fn randombit(self) -> crate::common::Reg<regs::Randombit, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
    #[inline(always)]
    pub fn count(self) -> crate::common::Reg<regs::Count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
}
pub mod regs;
pub mod vals;
