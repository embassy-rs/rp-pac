use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Channel(pub *mut u8);
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[doc = "DMA Channel 1 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "DMA Channel 1 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "DMA Channel 1 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "DMA Channel 1 Control and Status"]
    pub fn ctrl_trig(self) -> Reg<regs::CtrlTrig, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn dbg_ctdreq(self) -> Reg<regs::DbgCtdreq, RW> {
        unsafe { Reg::from_ptr(self.0.add(2048usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(2052usize)) }
    }
}
#[doc = "DMA with separate read and write masters"]
#[derive(Copy, Clone)]
pub struct Dma(pub *mut u8);
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[doc = "Interrupt Status (raw)"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(1024usize)) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    pub fn inte0(self) -> Reg<regs::Inte0, RW> {
        unsafe { Reg::from_ptr(self.0.add(1028usize)) }
    }
    #[doc = "Force Interrupts"]
    pub fn intf0(self) -> Reg<regs::Intf0, RW> {
        unsafe { Reg::from_ptr(self.0.add(1032usize)) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    pub fn ints0(self) -> Reg<regs::Ints0, RW> {
        unsafe { Reg::from_ptr(self.0.add(1036usize)) }
    }
    #[doc = "Interrupt Enables for IRQ 1"]
    pub fn inte1(self) -> Reg<regs::Inte1, RW> {
        unsafe { Reg::from_ptr(self.0.add(1044usize)) }
    }
    #[doc = "Force Interrupts for IRQ 1"]
    pub fn intf1(self) -> Reg<regs::Intf1, RW> {
        unsafe { Reg::from_ptr(self.0.add(1048usize)) }
    }
    #[doc = "Interrupt Status (masked) for IRQ 1"]
    pub fn ints1(self) -> Reg<regs::Ints1, RW> {
        unsafe { Reg::from_ptr(self.0.add(1052usize)) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    pub fn multi_chan_trigger(self) -> Reg<regs::MultiChanTrigger, RW> {
        unsafe { Reg::from_ptr(self.0.add(1072usize)) }
    }
    #[doc = "Sniffer Control"]
    pub fn sniff_ctrl(self) -> Reg<regs::SniffCtrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(1076usize)) }
    }
    #[doc = "Data accumulator for sniff hardware Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub fn sniff_data(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(1080usize)) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    pub fn fifo_levels(self) -> Reg<regs::FifoLevels, RW> {
        unsafe { Reg::from_ptr(self.0.add(1088usize)) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    pub fn chan_abort(self) -> Reg<regs::ChanAbort, RW> {
        unsafe { Reg::from_ptr(self.0.add(1092usize)) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub fn n_channels(self) -> Reg<regs::NChannels, RW> {
        unsafe { Reg::from_ptr(self.0.add(1096usize)) }
    }
    pub fn ch(self, n: usize) -> Channel {
        assert!(n < 12usize);
        unsafe { Channel(self.0.add(0usize + n * 64usize)) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub fn timer(self, n: usize) -> Reg<regs::Timer, RW> {
        assert!(n < 2usize);
        unsafe { Reg::from_ptr(self.0.add(1056usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
