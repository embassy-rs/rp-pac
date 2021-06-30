#[doc = "QSPI flash execute-in-place block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipCtrl(pub *mut u8);
unsafe impl Send for XipCtrl {}
unsafe impl Sync for XipCtrl {}
impl XipCtrl {
    #[doc = "Cache control"]
    pub fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Cache Flush control"]
    pub fn flush(self) -> crate::common::Reg<regs::Flush, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Cache Status"]
    pub fn stat(self) -> crate::common::Reg<regs::Stat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Cache Hit counter A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    pub fn ctr_hit(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Cache Access counter A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
    pub fn ctr_acc(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "FIFO stream address"]
    pub fn stream_addr(self) -> crate::common::Reg<regs::StreamAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "FIFO stream control"]
    pub fn stream_ctr(self) -> crate::common::Reg<regs::StreamCtr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "FIFO stream data Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic."]
    pub fn stream_fifo(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
}
pub mod regs;
