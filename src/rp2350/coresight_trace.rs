#[doc = "Coresight block - RP specific registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CoresightTrace {
    ptr: *mut u8,
}
unsafe impl Send for CoresightTrace {}
unsafe impl Sync for CoresightTrace {}
impl CoresightTrace {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub const fn ctrl_status(self) -> crate::common::Reg<regs::CtrlStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "FIFO for trace data captured from the TPIU"]
    #[inline(always)]
    pub const fn trace_capture_fifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
pub mod regs;
