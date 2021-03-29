use crate::generic::*;
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    #[doc = "Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    pub fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    #[doc = "Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    pub fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Timer {
    fn default() -> Timer {
        Timer(0)
    }
}
#[doc = "Trigger one or more channels simultaneously"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MultiChanTrigger(pub u32);
impl MultiChanTrigger {
    #[doc = "Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    pub const fn multi_chan_trigger(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    pub fn set_multi_chan_trigger(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for MultiChanTrigger {
    fn default() -> MultiChanTrigger {
        MultiChanTrigger(0)
    }
}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NChannels(pub u32);
impl NChannels {
    pub const fn n_channels(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    pub fn set_n_channels(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for NChannels {
    fn default() -> NChannels {
        NChannels(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DbgCtdreq(pub u32);
impl DbgCtdreq {
    pub const fn ch2_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch2_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for DbgCtdreq {
    fn default() -> DbgCtdreq {
        DbgCtdreq(0)
    }
}
#[doc = "Interrupt Enables for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte1(pub u32);
impl Inte1 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    pub const fn inte1(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 1."]
    pub fn set_inte1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Inte1 {
    fn default() -> Inte1 {
        Inte1(0)
    }
}
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ChanAbort(pub u32);
impl ChanAbort {
    #[doc = "Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs. After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    pub const fn chan_abort(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs. After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    pub fn set_chan_abort(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for ChanAbort {
    fn default() -> ChanAbort {
        ChanAbort(0)
    }
}
#[doc = "DMA Channel 3 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct CtrlTrig(pub u32);
impl CtrlTrig {
    #[doc = "Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    pub const fn ahb_error(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    pub fn set_ahb_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 3 transfers later)"]
    pub const fn read_error(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 3 transfers later)"]
    pub fn set_read_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 5 transfers later)"]
    pub const fn write_error(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not to be earlier, or more than 5 transfers later)"]
    pub fn set_write_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    pub const fn sniff_en(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    pub fn set_sniff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    pub const fn bswap(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    pub fn set_bswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    pub const fn irq_quiet(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    pub fn set_irq_quiet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub const fn treq_sel(&self) -> super::vals::TreqSel {
        let val = (self.0 >> 15u32) & 0x3f;
        super::vals::TreqSel(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.0 as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (3)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (3)."]
    pub fn set_chain_to(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11u32)) | (((val as u32) & 0x0f) << 11u32);
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    pub const fn ring_sel(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    pub fn set_ring_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    pub const fn ring_size(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x0f;
        val as u8
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    pub fn set_ring_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6u32)) | (((val as u32) & 0x0f) << 6u32);
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    pub const fn incr_write(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    pub fn set_incr_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    pub const fn incr_read(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    pub fn set_incr_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub const fn data_size(&self) -> super::vals::DataSize {
        let val = (self.0 >> 2u32) & 0x03;
        super::vals::DataSize(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.0 as u32) & 0x03) << 2u32);
    }
    #[doc = "HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    pub const fn high_priority(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    pub fn set_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for CtrlTrig {
    fn default() -> CtrlTrig {
        CtrlTrig(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    pub const fn intr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR, INTS0 or INTS1. Channel interrupts can be routed to either of two system-level IRQs based on INTE0 and INTE1. This can be used vector different channel interrupts to different ISRs: this might be done to allow NVIC IRQ preemption for more time-critical channels, or to spread IRQ load across different cores. It is also valid to ignore this behaviour and just use INTE0/INTS0/IRQ 0."]
    pub fn set_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Sniffer Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SniffCtrl(pub u32);
impl SniffCtrl {
    #[doc = "If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    pub const fn out_inv(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    pub fn set_out_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    pub const fn out_rev(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    pub fn set_out_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Locally perform a byte reverse on the sniffed data, before feeding into checksum. Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    pub const fn bswap(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "Locally perform a byte reverse on the sniffed data, before feeding into checksum. Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    pub fn set_bswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn calc(&self) -> super::vals::SniffCtrlCalc {
        let val = (self.0 >> 5u32) & 0x0f;
        super::vals::SniffCtrlCalc(val as u8)
    }
    pub fn set_calc(&mut self, val: super::vals::SniffCtrlCalc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.0 as u32) & 0x0f) << 5u32);
    }
    #[doc = "DMA channel for Sniffer to observe"]
    pub const fn dmach(&self) -> u8 {
        let val = (self.0 >> 1u32) & 0x0f;
        val as u8
    }
    #[doc = "DMA channel for Sniffer to observe"]
    pub fn set_dmach(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1u32)) | (((val as u32) & 0x0f) << 1u32);
    }
    #[doc = "Enable sniffer"]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable sniffer"]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SniffCtrl {
    fn default() -> SniffCtrl {
        SniffCtrl(0)
    }
}
#[doc = "Interrupt Enables for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte0(pub u32);
impl Inte0 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    pub const fn inte0(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 0."]
    pub fn set_inte0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Inte0 {
    fn default() -> Inte0 {
        Inte0(0)
    }
}
#[doc = "Interrupt Status (masked) for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints1(pub u32);
impl Ints1 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    pub const fn ints1(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    pub fn set_ints1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ints1 {
    fn default() -> Ints1 {
        Ints1(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf0(pub u32);
impl Intf0 {
    #[doc = "Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    pub const fn intf0(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    pub fn set_intf0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Intf0 {
    fn default() -> Intf0 {
        Intf0(0)
    }
}
#[doc = "Interrupt Status for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints0(pub u32);
impl Ints0 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    pub const fn ints0(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here."]
    pub fn set_ints0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ints0 {
    fn default() -> Ints0 {
        Ints0(0)
    }
}
#[doc = "Debug RAF, WAF, TDF levels"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FifoLevels(pub u32);
impl FifoLevels {
    #[doc = "Current Read-Address-FIFO fill level"]
    pub const fn raf_lvl(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0xff;
        val as u8
    }
    #[doc = "Current Read-Address-FIFO fill level"]
    pub fn set_raf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16u32)) | (((val as u32) & 0xff) << 16u32);
    }
    #[doc = "Current Write-Address-FIFO fill level"]
    pub const fn waf_lvl(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0xff;
        val as u8
    }
    #[doc = "Current Write-Address-FIFO fill level"]
    pub fn set_waf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
    }
    #[doc = "Current Transfer-Data-FIFO fill level"]
    pub const fn tdf_lvl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Current Transfer-Data-FIFO fill level"]
    pub fn set_tdf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for FifoLevels {
    fn default() -> FifoLevels {
        FifoLevels(0)
    }
}
#[doc = "Force Interrupts for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf1(pub u32);
impl Intf1 {
    #[doc = "Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    pub const fn intf1(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTE0. The interrupt remains asserted until INTF0 is cleared."]
    pub fn set_intf1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Intf1 {
    fn default() -> Intf1 {
        Intf1(0)
    }
}
