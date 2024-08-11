#[doc = "FIFO status and write access for HSTX"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct HstxFifo {
    ptr: *mut u8,
}
unsafe impl Send for HstxFifo {}
unsafe impl Sync for HstxFifo {}
impl HstxFifo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "FIFO status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Write access to FIFO"]
    #[inline(always)]
    pub const fn fifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
pub mod regs;
