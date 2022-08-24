#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psm(pub *mut u8);
unsafe impl Send for Psm {}
unsafe impl Sync for Psm {}
impl Psm {
    #[doc = "Force block out of reset (i.e. power it on)"]
    #[inline(always)]
    pub fn frce_on(self) -> crate::common::Reg<regs::FrceOn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Force into reset (i.e. power it off)"]
    #[inline(always)]
    pub fn frce_off(self) -> crate::common::Reg<regs::FrceOff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
    #[inline(always)]
    pub fn wdsel(self) -> crate::common::Reg<regs::Wdsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Indicates the peripheral's registers are ready to access."]
    #[inline(always)]
    pub fn done(self) -> crate::common::Reg<regs::Done, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
pub mod regs;
