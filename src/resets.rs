use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Resets(*mut u8);
unsafe impl Send for Resets {}
unsafe impl Sync for Resets {}
impl Resets {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
    pub fn reset(self) -> Reg<regs::Peripherals, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    pub fn wdsel(self) -> Reg<regs::Wdsel, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    pub fn reset_done(self) -> Reg<regs::Peripherals, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
}
pub mod regs;
