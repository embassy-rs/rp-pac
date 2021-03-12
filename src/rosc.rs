use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Rosc(*mut u8);
unsafe impl Send for Rosc {}
unsafe impl Sync for Rosc {}
impl Rosc {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Ring Oscillator control"]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "The FREQA & FREQB registers control the frequency by controlling the drive strength of each stage The drive strength has 4 levels determined by the number of bits set Increasing the number of bits set increases the drive strength and increases the oscillation frequency 0 bits set is the default drive strength 1 bit set doubles the drive strength 2 bits set triples drive strength 3 bits set quadruples drive strength"]
    pub fn freqa(self) -> Reg<regs::Freqa, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "For a detailed description see freqa register"]
    pub fn freqb(self) -> Reg<regs::Freqb, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Ring Oscillator pause control This is used to save power by pausing the ROSC On power-up this field is initialised to WAKE An invalid write will also select WAKE Warning: setup the irq before selecting dormant mode"]
    pub fn dormant(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Controls the output divider"]
    pub fn div(self) -> Reg<regs::Div, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Controls the phase shifted output"]
    pub fn phase(self) -> Reg<regs::Phase, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Ring Oscillator Status"]
    pub fn status(self) -> Reg<regs::Status, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "This just reads the state of the oscillator output so randomness is compromised if the ring oscillator is stopped or run at a harmonic of the bus frequency"]
    pub fn randombit(self) -> Reg<regs::Randombit, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "A down counter running at the ROSC frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
    pub fn count(self) -> Reg<regs::Count, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
}
pub mod regs;
pub mod vals;
