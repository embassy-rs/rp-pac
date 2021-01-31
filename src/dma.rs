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
        unsafe { Reg::new(self.0.add(0usize), 0) }
    }
    #[doc = "DMA Channel 0 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch0_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(4usize), 0) }
    }
    #[doc = "DMA Channel 0 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch0_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(8usize), 0) }
    }
    #[doc = "DMA Channel 0 Control and Status"]
    pub fn ch0_ctrl_trig(self) -> Reg<fields::Ch0CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Ch0CtrlTrig::from_bits(0)) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(16usize), 0) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register"]
    pub fn ch0_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(20usize), 0) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register"]
    pub fn ch0_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(24usize), 0) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(28usize), 0) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(32usize), 0) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register"]
    pub fn ch0_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(36usize), 0) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register"]
    pub fn ch0_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(40usize), 0) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(44usize), 0) }
    }
    #[doc = "Alias for channel 0 CTRL register"]
    pub fn ch0_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(48usize), 0) }
    }
    #[doc = "Alias for channel 0 WRITE_ADDR register"]
    pub fn ch0_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(52usize), 0) }
    }
    #[doc = "Alias for channel 0 TRANS_COUNT register"]
    pub fn ch0_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(56usize), 0) }
    }
    #[doc = "Alias for channel 0 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch0_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(60usize), 0) }
    }
    #[doc = "DMA Channel 1 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch1_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(64usize), 0) }
    }
    #[doc = "DMA Channel 1 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch1_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(68usize), 0) }
    }
    #[doc = "DMA Channel 1 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch1_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(72usize), 0) }
    }
    #[doc = "DMA Channel 1 Control and Status"]
    pub fn ch1_ctrl_trig(self) -> Reg<fields::Ch1CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::Ch1CtrlTrig::from_bits(2048)) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(80usize), 0) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn ch1_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(84usize), 0) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn ch1_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(88usize), 0) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(92usize), 0) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(96usize), 0) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn ch1_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(100usize), 0) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    pub fn ch1_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(104usize), 0) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(108usize), 0) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    pub fn ch1_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(112usize), 0) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    pub fn ch1_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(116usize), 0) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    pub fn ch1_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(120usize), 0) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch1_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(124usize), 0) }
    }
    #[doc = "DMA Channel 2 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch2_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(128usize), 0) }
    }
    #[doc = "DMA Channel 2 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch2_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(132usize), 0) }
    }
    #[doc = "DMA Channel 2 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch2_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(136usize), 0) }
    }
    #[doc = "DMA Channel 2 Control and Status"]
    pub fn ch2_ctrl_trig(self) -> Reg<fields::Ch2CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::Ch2CtrlTrig::from_bits(4096)) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(144usize), 0) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register"]
    pub fn ch2_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(148usize), 0) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register"]
    pub fn ch2_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(152usize), 0) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(156usize), 0) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(160usize), 0) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register"]
    pub fn ch2_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(164usize), 0) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register"]
    pub fn ch2_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(168usize), 0) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(172usize), 0) }
    }
    #[doc = "Alias for channel 2 CTRL register"]
    pub fn ch2_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(176usize), 0) }
    }
    #[doc = "Alias for channel 2 WRITE_ADDR register"]
    pub fn ch2_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(180usize), 0) }
    }
    #[doc = "Alias for channel 2 TRANS_COUNT register"]
    pub fn ch2_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(184usize), 0) }
    }
    #[doc = "Alias for channel 2 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch2_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(188usize), 0) }
    }
    #[doc = "DMA Channel 3 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch3_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(192usize), 0) }
    }
    #[doc = "DMA Channel 3 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch3_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(196usize), 0) }
    }
    #[doc = "DMA Channel 3 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch3_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(200usize), 0) }
    }
    #[doc = "DMA Channel 3 Control and Status"]
    pub fn ch3_ctrl_trig(self) -> Reg<fields::Ch3CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(204usize), fields::Ch3CtrlTrig::from_bits(6144)) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(208usize), 0) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register"]
    pub fn ch3_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(212usize), 0) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register"]
    pub fn ch3_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(216usize), 0) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(220usize), 0) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(224usize), 0) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register"]
    pub fn ch3_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(228usize), 0) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register"]
    pub fn ch3_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(232usize), 0) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(236usize), 0) }
    }
    #[doc = "Alias for channel 3 CTRL register"]
    pub fn ch3_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(240usize), 0) }
    }
    #[doc = "Alias for channel 3 WRITE_ADDR register"]
    pub fn ch3_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(244usize), 0) }
    }
    #[doc = "Alias for channel 3 TRANS_COUNT register"]
    pub fn ch3_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(248usize), 0) }
    }
    #[doc = "Alias for channel 3 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch3_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(252usize), 0) }
    }
    #[doc = "DMA Channel 4 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch4_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(256usize), 0) }
    }
    #[doc = "DMA Channel 4 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch4_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(260usize), 0) }
    }
    #[doc = "DMA Channel 4 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch4_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(264usize), 0) }
    }
    #[doc = "DMA Channel 4 Control and Status"]
    pub fn ch4_ctrl_trig(self) -> Reg<fields::Ch4CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(268usize), fields::Ch4CtrlTrig::from_bits(8192)) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(272usize), 0) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register"]
    pub fn ch4_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(276usize), 0) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register"]
    pub fn ch4_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(280usize), 0) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(284usize), 0) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(288usize), 0) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register"]
    pub fn ch4_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(292usize), 0) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register"]
    pub fn ch4_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(296usize), 0) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(300usize), 0) }
    }
    #[doc = "Alias for channel 4 CTRL register"]
    pub fn ch4_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(304usize), 0) }
    }
    #[doc = "Alias for channel 4 WRITE_ADDR register"]
    pub fn ch4_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(308usize), 0) }
    }
    #[doc = "Alias for channel 4 TRANS_COUNT register"]
    pub fn ch4_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(312usize), 0) }
    }
    #[doc = "Alias for channel 4 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch4_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(316usize), 0) }
    }
    #[doc = "DMA Channel 5 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch5_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(320usize), 0) }
    }
    #[doc = "DMA Channel 5 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch5_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(324usize), 0) }
    }
    #[doc = "DMA Channel 5 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch5_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(328usize), 0) }
    }
    #[doc = "DMA Channel 5 Control and Status"]
    pub fn ch5_ctrl_trig(self) -> Reg<fields::Ch5CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(332usize), fields::Ch5CtrlTrig::from_bits(10240)) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(336usize), 0) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register"]
    pub fn ch5_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(340usize), 0) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register"]
    pub fn ch5_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(344usize), 0) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(348usize), 0) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(352usize), 0) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register"]
    pub fn ch5_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(356usize), 0) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register"]
    pub fn ch5_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(360usize), 0) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(364usize), 0) }
    }
    #[doc = "Alias for channel 5 CTRL register"]
    pub fn ch5_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(368usize), 0) }
    }
    #[doc = "Alias for channel 5 WRITE_ADDR register"]
    pub fn ch5_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(372usize), 0) }
    }
    #[doc = "Alias for channel 5 TRANS_COUNT register"]
    pub fn ch5_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(376usize), 0) }
    }
    #[doc = "Alias for channel 5 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch5_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(380usize), 0) }
    }
    #[doc = "DMA Channel 6 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch6_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(384usize), 0) }
    }
    #[doc = "DMA Channel 6 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch6_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(388usize), 0) }
    }
    #[doc = "DMA Channel 6 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch6_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(392usize), 0) }
    }
    #[doc = "DMA Channel 6 Control and Status"]
    pub fn ch6_ctrl_trig(self) -> Reg<fields::Ch6CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(396usize), fields::Ch6CtrlTrig::from_bits(12288)) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(400usize), 0) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register"]
    pub fn ch6_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(404usize), 0) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register"]
    pub fn ch6_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(408usize), 0) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(412usize), 0) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(416usize), 0) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register"]
    pub fn ch6_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(420usize), 0) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register"]
    pub fn ch6_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(424usize), 0) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(428usize), 0) }
    }
    #[doc = "Alias for channel 6 CTRL register"]
    pub fn ch6_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(432usize), 0) }
    }
    #[doc = "Alias for channel 6 WRITE_ADDR register"]
    pub fn ch6_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(436usize), 0) }
    }
    #[doc = "Alias for channel 6 TRANS_COUNT register"]
    pub fn ch6_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(440usize), 0) }
    }
    #[doc = "Alias for channel 6 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch6_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(444usize), 0) }
    }
    #[doc = "DMA Channel 7 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch7_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(448usize), 0) }
    }
    #[doc = "DMA Channel 7 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch7_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(452usize), 0) }
    }
    #[doc = "DMA Channel 7 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch7_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(456usize), 0) }
    }
    #[doc = "DMA Channel 7 Control and Status"]
    pub fn ch7_ctrl_trig(self) -> Reg<fields::Ch7CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(460usize), fields::Ch7CtrlTrig::from_bits(14336)) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(464usize), 0) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register"]
    pub fn ch7_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(468usize), 0) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register"]
    pub fn ch7_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(472usize), 0) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(476usize), 0) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(480usize), 0) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register"]
    pub fn ch7_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(484usize), 0) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register"]
    pub fn ch7_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(488usize), 0) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(492usize), 0) }
    }
    #[doc = "Alias for channel 7 CTRL register"]
    pub fn ch7_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(496usize), 0) }
    }
    #[doc = "Alias for channel 7 WRITE_ADDR register"]
    pub fn ch7_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(500usize), 0) }
    }
    #[doc = "Alias for channel 7 TRANS_COUNT register"]
    pub fn ch7_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(504usize), 0) }
    }
    #[doc = "Alias for channel 7 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch7_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(508usize), 0) }
    }
    #[doc = "DMA Channel 8 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch8_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(512usize), 0) }
    }
    #[doc = "DMA Channel 8 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch8_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(516usize), 0) }
    }
    #[doc = "DMA Channel 8 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch8_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(520usize), 0) }
    }
    #[doc = "DMA Channel 8 Control and Status"]
    pub fn ch8_ctrl_trig(self) -> Reg<fields::Ch8CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(524usize), fields::Ch8CtrlTrig::from_bits(16384)) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(528usize), 0) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register"]
    pub fn ch8_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(532usize), 0) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register"]
    pub fn ch8_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(536usize), 0) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(540usize), 0) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(544usize), 0) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register"]
    pub fn ch8_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(548usize), 0) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register"]
    pub fn ch8_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(552usize), 0) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(556usize), 0) }
    }
    #[doc = "Alias for channel 8 CTRL register"]
    pub fn ch8_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(560usize), 0) }
    }
    #[doc = "Alias for channel 8 WRITE_ADDR register"]
    pub fn ch8_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(564usize), 0) }
    }
    #[doc = "Alias for channel 8 TRANS_COUNT register"]
    pub fn ch8_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(568usize), 0) }
    }
    #[doc = "Alias for channel 8 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch8_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(572usize), 0) }
    }
    #[doc = "DMA Channel 9 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch9_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(576usize), 0) }
    }
    #[doc = "DMA Channel 9 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch9_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(580usize), 0) }
    }
    #[doc = "DMA Channel 9 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch9_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(584usize), 0) }
    }
    #[doc = "DMA Channel 9 Control and Status"]
    pub fn ch9_ctrl_trig(self) -> Reg<fields::Ch9CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(588usize), fields::Ch9CtrlTrig::from_bits(18432)) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(592usize), 0) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register"]
    pub fn ch9_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(596usize), 0) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register"]
    pub fn ch9_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(600usize), 0) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(604usize), 0) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(608usize), 0) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register"]
    pub fn ch9_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(612usize), 0) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register"]
    pub fn ch9_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(616usize), 0) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(620usize), 0) }
    }
    #[doc = "Alias for channel 9 CTRL register"]
    pub fn ch9_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(624usize), 0) }
    }
    #[doc = "Alias for channel 9 WRITE_ADDR register"]
    pub fn ch9_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(628usize), 0) }
    }
    #[doc = "Alias for channel 9 TRANS_COUNT register"]
    pub fn ch9_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(632usize), 0) }
    }
    #[doc = "Alias for channel 9 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch9_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(636usize), 0) }
    }
    #[doc = "DMA Channel 10 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch10_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(640usize), 0) }
    }
    #[doc = "DMA Channel 10 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch10_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(644usize), 0) }
    }
    #[doc = "DMA Channel 10 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch10_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(648usize), 0) }
    }
    #[doc = "DMA Channel 10 Control and Status"]
    pub fn ch10_ctrl_trig(self) -> Reg<fields::Ch10CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(652usize), fields::Ch10CtrlTrig::from_bits(20480)) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(656usize), 0) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register"]
    pub fn ch10_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(660usize), 0) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register"]
    pub fn ch10_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(664usize), 0) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(668usize), 0) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(672usize), 0) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register"]
    pub fn ch10_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(676usize), 0) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register"]
    pub fn ch10_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(680usize), 0) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(684usize), 0) }
    }
    #[doc = "Alias for channel 10 CTRL register"]
    pub fn ch10_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(688usize), 0) }
    }
    #[doc = "Alias for channel 10 WRITE_ADDR register"]
    pub fn ch10_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(692usize), 0) }
    }
    #[doc = "Alias for channel 10 TRANS_COUNT register"]
    pub fn ch10_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(696usize), 0) }
    }
    #[doc = "Alias for channel 10 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch10_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(700usize), 0) }
    }
    #[doc = "DMA Channel 11 Read Address pointer This register updates automatically each time a read completes. The current value is the next address to be read by this channel."]
    pub fn ch11_read_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(704usize), 0) }
    }
    #[doc = "DMA Channel 11 Write Address pointer This register updates automatically each time a write completes. The current value is the next address to be written by this channel."]
    pub fn ch11_write_addr(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(708usize), 0) }
    }
    #[doc = "DMA Channel 11 Transfer Count Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    pub fn ch11_trans_count(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(712usize), 0) }
    }
    #[doc = "DMA Channel 11 Control and Status"]
    pub fn ch11_ctrl_trig(self) -> Reg<fields::Ch11CtrlTrig, RW> {
        unsafe { Reg::new(self.0.add(716usize), fields::Ch11CtrlTrig::from_bits(22528)) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al1_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(720usize), 0) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    pub fn ch11_al1_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(724usize), 0) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    pub fn ch11_al1_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(728usize), 0) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al1_trans_count_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(732usize), 0) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al2_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(736usize), 0) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    pub fn ch11_al2_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(740usize), 0) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register"]
    pub fn ch11_al2_read_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(744usize), 0) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al2_write_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(748usize), 0) }
    }
    #[doc = "Alias for channel 11 CTRL register"]
    pub fn ch11_al3_ctrl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(752usize), 0) }
    }
    #[doc = "Alias for channel 11 WRITE_ADDR register"]
    pub fn ch11_al3_write_addr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(756usize), 0) }
    }
    #[doc = "Alias for channel 11 TRANS_COUNT register"]
    pub fn ch11_al3_trans_count(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(760usize), 0) }
    }
    #[doc = "Alias for channel 11 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    pub fn ch11_al3_read_addr_trig(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(764usize), 0) }
    }
    #[doc = "Interrupt Status (raw)"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(1024usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    pub fn inte0(self) -> Reg<fields::Inte0, RW> {
        unsafe { Reg::new(self.0.add(1028usize), fields::Inte0::from_bits(0)) }
    }
    #[doc = "Force Interrupts"]
    pub fn intf0(self) -> Reg<fields::Intf0, RW> {
        unsafe { Reg::new(self.0.add(1032usize), fields::Intf0::from_bits(0)) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    pub fn ints0(self) -> Reg<fields::Ints0, RW> {
        unsafe { Reg::new(self.0.add(1036usize), fields::Ints0::from_bits(0)) }
    }
    #[doc = "Interrupt Enables for IRQ 1"]
    pub fn inte1(self) -> Reg<fields::Inte1, RW> {
        unsafe { Reg::new(self.0.add(1044usize), fields::Inte1::from_bits(0)) }
    }
    #[doc = "Force Interrupts for IRQ 1"]
    pub fn intf1(self) -> Reg<fields::Intf1, RW> {
        unsafe { Reg::new(self.0.add(1048usize), fields::Intf1::from_bits(0)) }
    }
    #[doc = "Interrupt Status (masked) for IRQ 1"]
    pub fn ints1(self) -> Reg<fields::Ints1, RW> {
        unsafe { Reg::new(self.0.add(1052usize), fields::Ints1::from_bits(0)) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub fn timer0(self) -> Reg<fields::Timer0, RW> {
        unsafe { Reg::new(self.0.add(1056usize), fields::Timer0::from_bits(0)) }
    }
    #[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    pub fn timer1(self) -> Reg<fields::Timer1, RW> {
        unsafe { Reg::new(self.0.add(1060usize), fields::Timer1::from_bits(0)) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    pub fn multi_chan_trigger(self) -> Reg<fields::MultiChanTrigger, RW> {
        unsafe {
            Reg::new(
                self.0.add(1072usize),
                fields::MultiChanTrigger::from_bits(0),
            )
        }
    }
    #[doc = "Sniffer Control"]
    pub fn sniff_ctrl(self) -> Reg<fields::SniffCtrl, RW> {
        unsafe { Reg::new(self.0.add(1076usize), fields::SniffCtrl::from_bits(0)) }
    }
    #[doc = "Data accumulator for sniff hardware Write an initial seed value here before starting a DMA transfer on the channel indicated by SNIFF_CTRL_DMACH. The hardware will update this register each time it observes a read from the indicated channel. Once the channel completes, the final result can be read from this register."]
    pub fn sniff_data(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(1080usize), 0) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    pub fn fifo_levels(self) -> Reg<fields::FifoLevels, RW> {
        unsafe { Reg::new(self.0.add(1088usize), fields::FifoLevels::from_bits(0)) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    pub fn chan_abort(self) -> Reg<fields::ChanAbort, RW> {
        unsafe { Reg::new(self.0.add(1092usize), fields::ChanAbort::from_bits(0)) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    pub fn n_channels(self) -> Reg<fields::NChannels, RW> {
        unsafe { Reg::new(self.0.add(1096usize), fields::NChannels::from_bits(0)) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch0_dbg_ctdreq(self) -> Reg<fields::Ch0DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2048usize), fields::Ch0DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch0_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2052usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch1_dbg_ctdreq(self) -> Reg<fields::Ch1DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2112usize), fields::Ch1DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch1_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2116usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch2_dbg_ctdreq(self) -> Reg<fields::Ch2DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2176usize), fields::Ch2DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch2_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2180usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch3_dbg_ctdreq(self) -> Reg<fields::Ch3DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2240usize), fields::Ch3DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch3_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2244usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch4_dbg_ctdreq(self) -> Reg<fields::Ch4DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2304usize), fields::Ch4DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch4_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2308usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch5_dbg_ctdreq(self) -> Reg<fields::Ch5DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2368usize), fields::Ch5DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch5_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2372usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch6_dbg_ctdreq(self) -> Reg<fields::Ch6DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2432usize), fields::Ch6DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch6_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2436usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch7_dbg_ctdreq(self) -> Reg<fields::Ch7DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2496usize), fields::Ch7DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch7_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2500usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch8_dbg_ctdreq(self) -> Reg<fields::Ch8DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2560usize), fields::Ch8DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch8_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2564usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch9_dbg_ctdreq(self) -> Reg<fields::Ch9DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2624usize), fields::Ch9DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch9_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2628usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch10_dbg_ctdreq(self) -> Reg<fields::Ch10DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2688usize), fields::Ch10DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch10_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2692usize), 0) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    pub fn ch11_dbg_ctdreq(self) -> Reg<fields::Ch11DbgCtdreq, RW> {
        unsafe { Reg::new(self.0.add(2752usize), fields::Ch11DbgCtdreq::from_bits(0)) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    pub fn ch11_dbg_tcr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(2756usize), 0) }
    }
}
pub mod fields;
pub mod values;
