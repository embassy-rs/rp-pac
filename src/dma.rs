use crate::generic::*;
#[doc = "DMA with separate read and write masters"]
#[derive(Copy, Clone)]
pub struct Dma(*mut u8);
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "DMA Channel 0 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch0_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "DMA Channel 0 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch0_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "DMA Channel 0 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch0_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "DMA Channel 0 Control and Status"]
    pub fn ch0_ctrl_trig(self) -> Reg<regs::Ch0CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register"]
    pub fn ch0_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register"]
    pub fn ch0_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register"]
    pub fn ch0_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register"]
    pub fn ch0_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register"]
    pub fn ch0_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register"]
    pub fn ch0_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "DMA Channel 1 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch1_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "DMA Channel 1 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch1_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "DMA Channel 1 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch1_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "DMA Channel 1 Control and Status"]
    pub fn ch1_ctrl_trig(self) -> Reg<regs::Ch1CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn ch1_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn ch1_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn ch1_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn ch1_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn ch1_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn ch1_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(124usize)) }
    }
    #[doc = "DMA Channel 2 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch2_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
    #[doc = "DMA Channel 2 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch2_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(132usize)) }
    }
    #[doc = "DMA Channel 2 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch2_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(136usize)) }
    }
    #[doc = "DMA Channel 2 Control and Status"]
    pub fn ch2_ctrl_trig(self) -> Reg<regs::Ch2CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(140usize)) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(144usize)) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register"]
    pub fn ch2_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(148usize)) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register"]
    pub fn ch2_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(152usize)) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(156usize)) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(160usize)) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register"]
    pub fn ch2_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(164usize)) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register"]
    pub fn ch2_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(168usize)) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(172usize)) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(176usize)) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register"]
    pub fn ch2_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(180usize)) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register"]
    pub fn ch2_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(184usize)) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(188usize)) }
    }
    #[doc = "DMA Channel 3 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch3_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(192usize)) }
    }
    #[doc = "DMA Channel 3 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch3_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(196usize)) }
    }
    #[doc = "DMA Channel 3 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch3_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(200usize)) }
    }
    #[doc = "DMA Channel 3 Control and Status"]
    pub fn ch3_ctrl_trig(self) -> Reg<regs::Ch3CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(204usize)) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(208usize)) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register"]
    pub fn ch3_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(212usize)) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register"]
    pub fn ch3_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(216usize)) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(220usize)) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(224usize)) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register"]
    pub fn ch3_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(228usize)) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register"]
    pub fn ch3_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(232usize)) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(236usize)) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(240usize)) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register"]
    pub fn ch3_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(244usize)) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register"]
    pub fn ch3_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(248usize)) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(252usize)) }
    }
    #[doc = "DMA Channel 4 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch4_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(256usize)) }
    }
    #[doc = "DMA Channel 4 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch4_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(260usize)) }
    }
    #[doc = "DMA Channel 4 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch4_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(264usize)) }
    }
    #[doc = "DMA Channel 4 Control and Status"]
    pub fn ch4_ctrl_trig(self) -> Reg<regs::Ch4CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(268usize)) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(272usize)) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register"]
    pub fn ch4_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(276usize)) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register"]
    pub fn ch4_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(280usize)) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(284usize)) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(288usize)) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register"]
    pub fn ch4_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(292usize)) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register"]
    pub fn ch4_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(296usize)) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(300usize)) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(304usize)) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register"]
    pub fn ch4_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(308usize)) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register"]
    pub fn ch4_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(312usize)) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(316usize)) }
    }
    #[doc = "DMA Channel 5 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch5_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(320usize)) }
    }
    #[doc = "DMA Channel 5 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch5_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(324usize)) }
    }
    #[doc = "DMA Channel 5 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch5_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(328usize)) }
    }
    #[doc = "DMA Channel 5 Control and Status"]
    pub fn ch5_ctrl_trig(self) -> Reg<regs::Ch5CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(332usize)) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(336usize)) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register"]
    pub fn ch5_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(340usize)) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register"]
    pub fn ch5_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(344usize)) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(348usize)) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(352usize)) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register"]
    pub fn ch5_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(356usize)) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register"]
    pub fn ch5_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(360usize)) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(364usize)) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(368usize)) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register"]
    pub fn ch5_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(372usize)) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register"]
    pub fn ch5_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(376usize)) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(380usize)) }
    }
    #[doc = "DMA Channel 6 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch6_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(384usize)) }
    }
    #[doc = "DMA Channel 6 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch6_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(388usize)) }
    }
    #[doc = "DMA Channel 6 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch6_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(392usize)) }
    }
    #[doc = "DMA Channel 6 Control and Status"]
    pub fn ch6_ctrl_trig(self) -> Reg<regs::Ch6CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(396usize)) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(400usize)) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register"]
    pub fn ch6_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(404usize)) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register"]
    pub fn ch6_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(408usize)) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(412usize)) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(416usize)) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register"]
    pub fn ch6_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(420usize)) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register"]
    pub fn ch6_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(424usize)) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(428usize)) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(432usize)) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register"]
    pub fn ch6_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(436usize)) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register"]
    pub fn ch6_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(440usize)) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(444usize)) }
    }
    #[doc = "DMA Channel 7 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch7_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(448usize)) }
    }
    #[doc = "DMA Channel 7 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch7_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(452usize)) }
    }
    #[doc = "DMA Channel 7 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch7_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(456usize)) }
    }
    #[doc = "DMA Channel 7 Control and Status"]
    pub fn ch7_ctrl_trig(self) -> Reg<regs::Ch7CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(460usize)) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(464usize)) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register"]
    pub fn ch7_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(468usize)) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register"]
    pub fn ch7_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(472usize)) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(476usize)) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(480usize)) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register"]
    pub fn ch7_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(484usize)) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register"]
    pub fn ch7_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(488usize)) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(492usize)) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(496usize)) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register"]
    pub fn ch7_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(500usize)) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register"]
    pub fn ch7_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(504usize)) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(508usize)) }
    }
    #[doc = "DMA Channel 8 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch8_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(512usize)) }
    }
    #[doc = "DMA Channel 8 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch8_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(516usize)) }
    }
    #[doc = "DMA Channel 8 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch8_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(520usize)) }
    }
    #[doc = "DMA Channel 8 Control and Status"]
    pub fn ch8_ctrl_trig(self) -> Reg<regs::Ch8CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(524usize)) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(528usize)) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register"]
    pub fn ch8_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(532usize)) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register"]
    pub fn ch8_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(536usize)) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(540usize)) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(544usize)) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register"]
    pub fn ch8_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(548usize)) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register"]
    pub fn ch8_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(552usize)) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(556usize)) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(560usize)) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register"]
    pub fn ch8_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(564usize)) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register"]
    pub fn ch8_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(568usize)) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(572usize)) }
    }
    #[doc = "DMA Channel 9 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch9_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(576usize)) }
    }
    #[doc = "DMA Channel 9 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch9_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(580usize)) }
    }
    #[doc = "DMA Channel 9 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch9_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(584usize)) }
    }
    #[doc = "DMA Channel 9 Control and Status"]
    pub fn ch9_ctrl_trig(self) -> Reg<regs::Ch9CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(588usize)) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(592usize)) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register"]
    pub fn ch9_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(596usize)) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register"]
    pub fn ch9_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(600usize)) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(604usize)) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(608usize)) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register"]
    pub fn ch9_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(612usize)) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register"]
    pub fn ch9_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(616usize)) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(620usize)) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(624usize)) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register"]
    pub fn ch9_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(628usize)) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register"]
    pub fn ch9_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(632usize)) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(636usize)) }
    }
    #[doc = "DMA Channel 10 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch10_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(640usize)) }
    }
    #[doc = "DMA Channel 10 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch10_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(644usize)) }
    }
    #[doc = "DMA Channel 10 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch10_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(648usize)) }
    }
    #[doc = "DMA Channel 10 Control and Status"]
    pub fn ch10_ctrl_trig(self) -> Reg<regs::Ch10CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(652usize)) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(656usize)) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register"]
    pub fn ch10_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(660usize)) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register"]
    pub fn ch10_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(664usize)) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(668usize)) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(672usize)) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register"]
    pub fn ch10_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(676usize)) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register"]
    pub fn ch10_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(680usize)) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(684usize)) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(688usize)) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register"]
    pub fn ch10_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(692usize)) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register"]
    pub fn ch10_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(696usize)) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(700usize)) }
    }
    #[doc = "DMA Channel 11 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch11_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(704usize)) }
    }
    #[doc = "DMA Channel 11 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch11_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(708usize)) }
    }
    #[doc = "DMA Channel 11 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch11_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(712usize)) }
    }
    #[doc = "DMA Channel 11 Control and Status"]
    pub fn ch11_ctrl_trig(self) -> Reg<regs::Ch11CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(716usize)) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(720usize)) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    pub fn ch11_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(724usize)) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    pub fn ch11_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(728usize)) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(732usize)) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(736usize)) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    pub fn ch11_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(740usize)) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    pub fn ch11_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(744usize)) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(748usize)) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(752usize)) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    pub fn ch11_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(756usize)) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    pub fn ch11_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(760usize)) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(764usize)) }
    }
    #[doc = "Interrupt Status (raw)"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(1024usize)) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    pub fn inte0(self) -> Reg<regs::Inte0, RW> {
        unsafe { Reg::new(self.0.add(1028usize)) }
    }
    #[doc = "Force Interrupts"]
    pub fn intf0(self) -> Reg<regs::Intf0, RW> {
        unsafe { Reg::new(self.0.add(1032usize)) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    pub fn ints0(self) -> Reg<regs::Ints0, RW> {
        unsafe { Reg::new(self.0.add(1036usize)) }
    }
    #[doc = "Interrupt Enables for IRQ 1"]
    pub fn inte1(self) -> Reg<regs::Inte1, RW> {
        unsafe { Reg::new(self.0.add(1044usize)) }
    }
    #[doc = "Force Interrupts for IRQ 1"]
    pub fn intf1(self) -> Reg<regs::Intf1, RW> {
        unsafe { Reg::new(self.0.add(1048usize)) }
    }
    #[doc = "Interrupt Status (masked) for IRQ 1"]
    pub fn ints1(self) -> Reg<regs::Ints1, RW> {
        unsafe { Reg::new(self.0.add(1052usize)) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub fn timer0(self) -> Reg<regs::Timer, RW> {
        unsafe { Reg::new(self.0.add(1056usize)) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub fn timer1(self) -> Reg<regs::Timer, RW> {
        unsafe { Reg::new(self.0.add(1060usize)) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    pub fn multi_chan_trigger(self) -> Reg<regs::MultiChanTrigger, RW> {
        unsafe { Reg::new(self.0.add(1072usize)) }
    }
    #[doc = "Sniffer Control"]
    pub fn sniff_ctrl(self) -> Reg<regs::SniffCtrl, RW> {
        unsafe { Reg::new(self.0.add(1076usize)) }
    }
    #[doc = "Data accumulator for sniff hardware Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub fn sniff_data(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(1080usize)) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    pub fn fifo_levels(self) -> Reg<regs::FifoLevels, RW> {
        unsafe { Reg::new(self.0.add(1088usize)) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    pub fn chan_abort(self) -> Reg<regs::ChanAbort, RW> {
        unsafe { Reg::new(self.0.add(1092usize)) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub fn n_channels(self) -> Reg<regs::NChannels, RW> {
        unsafe { Reg::new(self.0.add(1096usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch0_dbg_ctdreq(self) -> Reg<regs::Ch0DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2048usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch0_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2052usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch1_dbg_ctdreq(self) -> Reg<regs::Ch1DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2112usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch1_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2116usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch2_dbg_ctdreq(self) -> Reg<regs::Ch2DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2176usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch2_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2180usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch3_dbg_ctdreq(self) -> Reg<regs::Ch3DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2240usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch3_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2244usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch4_dbg_ctdreq(self) -> Reg<regs::Ch4DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2304usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch4_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2308usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch5_dbg_ctdreq(self) -> Reg<regs::Ch5DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2368usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch5_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2372usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch6_dbg_ctdreq(self) -> Reg<regs::Ch6DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2432usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch6_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2436usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch7_dbg_ctdreq(self) -> Reg<regs::Ch7DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2496usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch7_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2500usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch8_dbg_ctdreq(self) -> Reg<regs::Ch8DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2560usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch8_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2564usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch9_dbg_ctdreq(self) -> Reg<regs::Ch9DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2624usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch9_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2628usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch10_dbg_ctdreq(self) -> Reg<regs::Ch10DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2688usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch10_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2692usize)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch11_dbg_ctdreq(self) -> Reg<regs::Ch11DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2752usize)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch11_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2756usize)) }
    }
}
pub mod regs;
pub mod vals;
