#[doc = "QSPI flash execute-in-place block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipCtrl {
    ptr: *mut u8,
}
unsafe impl Send for XipCtrl {}
unsafe impl Sync for XipCtrl {}
impl XipCtrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Cache control register. Read-only from a Non-secure context."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Cache Hit counter"]
    #[inline(always)]
    pub const fn ctr_hit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Cache Access counter"]
    #[inline(always)]
    pub const fn ctr_acc(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "FIFO stream address"]
    #[inline(always)]
    pub const fn stream_addr(self) -> crate::common::Reg<regs::StreamAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "FIFO stream control"]
    #[inline(always)]
    pub const fn stream_ctr(self) -> crate::common::Reg<regs::StreamCtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "FIFO stream data"]
    #[inline(always)]
    pub const fn stream_fifo(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
}
pub mod regs;
