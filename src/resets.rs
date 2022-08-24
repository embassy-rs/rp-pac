#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resets(pub *mut u8);
unsafe impl Send for Resets {}
unsafe impl Sync for Resets {}
impl Resets {
    #[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
    #[inline(always)]
    pub fn reset(self) -> crate::common::Reg<regs::Peripherals, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    #[inline(always)]
    pub fn wdsel(self) -> crate::common::Reg<regs::Wdsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    #[inline(always)]
    pub fn reset_done(self) -> crate::common::Reg<regs::Peripherals, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
