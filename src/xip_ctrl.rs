use crate::generic::*;
#[doc = "QSPI flash execute-in-place block"]
#[derive(Copy, Clone)]
pub struct XipCtrl(*mut u8);
unsafe impl Send for XipCtrl {}
unsafe impl Sync for XipCtrl {}
impl XipCtrl {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Cache control"]
    pub fn ctrl(self) -> Reg<fields::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Ctrl::from_bits(3)) }
    }
    #[doc = "Cache Flush control"]
    pub fn flush(self) -> Reg<fields::Flush, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Flush::from_bits(0)) }
    }
    #[doc = "Cache Status"]
    pub fn stat(self) -> Reg<fields::Stat, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Stat::from_bits(2)) }
    }
    #[doc = "Cache Hit counter A 32 bit saturating counter that increments upon each cache hit, i.e. when an XIP access is serviced directly from cached data. Write any value to clear."]
    pub fn ctr_hit(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(12usize), 0) }
    }
    #[doc = "Cache Access counter A 32 bit saturating counter that increments upon each XIP access, whether the cache is hit or not. This includes noncacheable accesses. Write any value to clear."]
    pub fn ctr_acc(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(16usize), 0) }
    }
    #[doc = "FIFO stream address"]
    pub fn stream_addr(self) -> Reg<fields::StreamAddr, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::StreamAddr::from_bits(0)) }
    }
    #[doc = "FIFO stream control"]
    pub fn stream_ctr(self) -> Reg<fields::StreamCtr, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::StreamCtr::from_bits(0)) }
    }
    #[doc = "FIFO stream data Streamed data is buffered here, for retrieval by the system DMA. This FIFO can also be accessed via the XIP_AUX slave, to avoid exposing the DMA to bus stalls caused by other XIP traffic."]
    pub fn stream_fifo(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(28usize), 0) }
    }
}
pub mod fields;
