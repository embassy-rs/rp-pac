#[doc = "Auxiliary DMA access to XIP FIFOs, via fast AHB bus access"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipAux {
    ptr: *mut u8,
}
unsafe impl Send for XipAux {}
unsafe impl Sync for XipAux {}
impl XipAux {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Read the XIP stream FIFO (fast bus access to XIP_CTRL_STREAM_FIFO)"]
    #[inline(always)]
    pub const fn stream(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)"]
    #[inline(always)]
    pub const fn qmi_direct_tx(self) -> crate::common::Reg<regs::QmiDirectTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)"]
    #[inline(always)]
    pub const fn qmi_direct_rx(self) -> crate::common::Reg<regs::QmiDirectRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
