use crate::generic::*;
#[doc = "Controls the crystal oscillator"]
#[derive(Copy, Clone)]
pub struct Xosc(pub *mut u8);
unsafe impl Send for Xosc {}
unsafe impl Sync for Xosc {}
impl Xosc {
    #[doc = "Crystal Oscillator Control"]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Crystal Oscillator Status"]
    pub fn status(self) -> Reg<regs::Status, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Crystal Oscillator pause control This is used to save power by pausing the XOSC On power-up this field is initialised to WAKE An invalid write will also select WAKE WARNING: stop the PLLs before selecting dormant mode WARNING: setup the irq before selecting dormant mode"]
    pub fn dormant(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Controls the startup delay"]
    pub fn startup(self) -> Reg<regs::Startup, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "A down counter running at the xosc frequency which counts to zero and stops. To start the counter write a non-zero value. Can be used for short software pauses when setting up time sensitive hardware."]
    pub fn count(self) -> Reg<regs::Count, RW> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
}
pub mod regs;
pub mod vals;
