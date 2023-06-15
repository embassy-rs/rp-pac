#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pads {
    ptr: *mut u8,
}
unsafe impl Send for Pads {}
unsafe impl Sync for Pads {}
impl Pads {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Voltage select. Per bank control"]
    #[inline(always)]
    pub const fn voltage_select(
        self,
    ) -> crate::common::Reg<regs::VoltageSelect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Pad control register"]
    #[inline(always)]
    pub const fn gpio(self, n: usize) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
