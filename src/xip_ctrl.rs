use crate::generic::*;
#[doc = "QSPI flash execute-in-place block"]
#[derive(Copy, Clone)]
pub struct XipCtrl(pub *mut u8);
unsafe impl Send for XipCtrl {}
unsafe impl Sync for XipCtrl {}
impl XipCtrl {
    #[doc = "Cache control"]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Cache Flush control"]
    pub fn flush(self) -> Reg<regs::Flush, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Cache Status"]
    pub fn stat(self) -> Reg<regs::Stat, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Cache Hit counter A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    pub fn ctr_hit(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Cache Access counter A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
    pub fn ctr_acc(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "FIFO stream address"]
    pub fn stream_addr(self) -> Reg<regs::StreamAddr, RW> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "FIFO stream control"]
    pub fn stream_ctr(self) -> Reg<regs::StreamCtr, RW> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "FIFO stream data Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic."]
    pub fn stream_fifo(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
}
pub mod regs;
