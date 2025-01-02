#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlStatus(pub u32);
impl CtrlStatus {
    #[doc = "Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow. Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register. Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU."]
    #[inline(always)]
    pub const fn trace_capture_fifo_flush(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to continuously hold the trace FIFO in a flushed state and prevent overflow. Before clearing this flag, configure and start a DMA channel with the correct DREQ for the TRACE_CAPTURE_FIFO register. Clear this flag to begin sampling trace data, and set once again once the trace capture buffer is full. You must configure the TPIU in order to generate trace packets to be captured, as well as components like the ETM further upstream to generate the event stream propagated to the TPIU."]
    #[inline(always)]
    pub fn set_trace_capture_fifo_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit."]
    #[inline(always)]
    pub const fn trace_capture_fifo_overflow(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This status flag is set high when trace data has been dropped due to the FIFO being full at the point trace data was sampled. Write 1 to acknowledge and clear the bit."]
    #[inline(always)]
    pub fn set_trace_capture_fifo_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for CtrlStatus {
    #[inline(always)]
    fn default() -> CtrlStatus {
        CtrlStatus(0)
    }
}
impl core::fmt::Debug for CtrlStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlStatus")
            .field("trace_capture_fifo_flush", &self.trace_capture_fifo_flush())
            .field(
                "trace_capture_fifo_overflow",
                &self.trace_capture_fifo_overflow(),
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CtrlStatus {
            trace_capture_fifo_flush: bool,
            trace_capture_fifo_overflow: bool,
        }
        let proxy = CtrlStatus {
            trace_capture_fifo_flush: self.trace_capture_fifo_flush(),
            trace_capture_fifo_overflow: self.trace_capture_fifo_overflow(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
