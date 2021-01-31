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
    pub fn reset(self) -> Reg<fields::Peripherals, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Peripherals::from_bits(33554431)) }
    }
    #[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    pub fn wdsel(self) -> Reg<fields::Wdsel, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Wdsel::from_bits(0)) }
    }
    #[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    pub fn reset_done(self) -> Reg<fields::Peripherals, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Peripherals::from_bits(0)) }
    }
}
pub mod fields;
