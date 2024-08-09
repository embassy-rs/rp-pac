#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Psm {
    ptr: *mut u8,
}
unsafe impl Send for Psm {}
unsafe impl Sync for Psm {}
impl Psm {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Force block out of reset (i.e. power it on)"]
    #[inline(always)]
    pub const fn frce_on(self) -> crate::common::Reg<regs::FrceOn, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Force into reset (i.e. power it off)"]
    #[inline(always)]
    pub const fn frce_off(self) -> crate::common::Reg<regs::FrceOff, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Set to 1 if this peripheral should be reset when the watchdog fires."]
    #[inline(always)]
    pub const fn wdsel(self) -> crate::common::Reg<regs::Wdsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Indicates the peripheral's registers are ready to access."]
    #[inline(always)]
    pub const fn done(self) -> crate::common::Reg<regs::Done, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
}
pub mod regs;
