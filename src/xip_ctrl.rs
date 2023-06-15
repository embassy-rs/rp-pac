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
    #[doc = "Cache control"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Cache Flush control"]
    #[inline(always)]
    pub const fn flush(self) -> crate::common::Reg<regs::Flush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Cache Status"]
    #[inline(always)]
    pub const fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Cache Hit counter A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    #[inline(always)]
    pub const fn ctr_hit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Cache Access counter A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
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
    #[doc = "FIFO stream data Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic."]
    #[inline(always)]
    pub const fn stream_fifo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
}
pub mod regs;
