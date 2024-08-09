#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Resets {
    ptr: *mut u8,
}
unsafe impl Send for Resets {}
unsafe impl Sync for Resets {}
impl Resets {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Reset control. If a bit is set it means the peripheral is in reset. 0 means the peripheral's reset is deasserted."]
    #[inline(always)]
    pub const fn reset(self) -> crate::common::Reg<regs::Peripherals, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Watchdog select. If a bit is set then the watchdog will reset this peripheral when the watchdog fires."]
    #[inline(always)]
    pub const fn wdsel(self) -> crate::common::Reg<regs::Wdsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Reset done. If a bit is set then a reset done signal has been returned by the peripheral. This indicates that the peripheral's registers are ready to be accessed."]
    #[inline(always)]
    pub const fn reset_done(self) -> crate::common::Reg<regs::Peripherals, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
pub mod regs;
