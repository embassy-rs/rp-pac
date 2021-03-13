use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Psm(pub *mut u8);
unsafe impl Send for Psm {}
unsafe impl Sync for Psm {}
impl Psm {
    #[doc = "Force block out of reset (i.e. power it on)"]
    pub fn frce_on(self) -> Reg<regs::FrceOn, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Force into reset (i.e. power it off)"]
    pub fn frce_off(self) -> Reg<regs::FrceOff, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
    pub fn wdsel(self) -> Reg<regs::Wdsel, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Indicates the peripheral's registers are ready to access."]
    pub fn done(self) -> Reg<regs::Done, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
