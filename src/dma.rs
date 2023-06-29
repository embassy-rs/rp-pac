#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Channel 11 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    #[inline(always)]
    pub const fn read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DMA Channel 11 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    #[inline(always)]
    pub const fn write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "DMA Channel 11 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA Channel 11 Control and Status"]
    #[inline(always)]
    pub const fn ctrl_trig(self) -> crate::common::Reg<regs::CtrlTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    #[inline(always)]
    pub const fn al1_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    #[inline(always)]
    pub const fn al1_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al1_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al1_trans_count_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    #[inline(always)]
    pub const fn al2_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al2_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    #[inline(always)]
    pub const fn al2_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al2_write_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    #[inline(always)]
    pub const fn al3_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al3_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al3_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al3_read_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn dbg_ctdreq(self) -> crate::common::Reg<regs::DbgCtdreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize) as _) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn dbg_tcr(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2052usize) as _) }
    }
}
#[doc = "DMA with separate read and write masters"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Channel {
        assert!(n < 12usize);
        unsafe { Channel::from_ptr(self.ptr.add(0usize + n * 64usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    #[inline(always)]
    pub const fn inte0(self) -> crate::common::Reg<regs::Inte0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1028usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf0(self) -> crate::common::Reg<regs::Intf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1032usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    #[inline(always)]
    pub const fn ints0(self) -> crate::common::Reg<regs::Ints0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1036usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 1"]
    #[inline(always)]
    pub const fn inte1(self) -> crate::common::Reg<regs::Inte1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1044usize) as _) }
    }
    #[doc = "Force Interrupts for IRQ 1"]
    #[inline(always)]
    pub const fn intf1(self) -> crate::common::Reg<regs::Intf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1048usize) as _) }
    }
    #[doc = "Interrupt Status (masked) for IRQ 1"]
    #[inline(always)]
    pub const fn ints1(self) -> crate::common::Reg<regs::Ints1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1052usize) as _) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer(self, n: usize) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1056usize + n * 4usize) as _) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    #[inline(always)]
    pub const fn multi_chan_trigger(
        self,
    ) -> crate::common::Reg<regs::MultiChanTrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1072usize) as _) }
    }
    #[doc = "Sniffer Control"]
    #[inline(always)]
    pub const fn sniff_ctrl(self) -> crate::common::Reg<regs::SniffCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1076usize) as _) }
    }
    #[doc = "Data accumulator for sniff hardware Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    #[inline(always)]
    pub const fn sniff_data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1080usize) as _) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    #[inline(always)]
    pub const fn fifo_levels(self) -> crate::common::Reg<regs::FifoLevels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1088usize) as _) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    #[inline(always)]
    pub const fn chan_abort(self) -> crate::common::Reg<regs::ChanAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1092usize) as _) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    #[inline(always)]
    pub const fn n_channels(self) -> crate::common::Reg<regs::Nchannels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1096usize) as _) }
    }
}
pub mod regs;
pub mod vals;
