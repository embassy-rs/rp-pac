use crate::generic::*;
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch10DbgCtdreq(u32);
impl Ch10DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch10DbgCtdreq {
        Ch10DbgCtdreq(val)
    }
    pub const fn ch10_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch10_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch10DbgCtdreq {
    fn default() -> Ch10DbgCtdreq {
        Ch10DbgCtdreq(0)
    }
}
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Timer(u32);
impl Timer {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Timer {
        Timer(val)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1DbgCtdreq(u32);
impl Ch1DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch1DbgCtdreq {
        Ch1DbgCtdreq(val)
    }
    pub const fn ch1_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch1_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch1DbgCtdreq {
    fn default() -> Ch1DbgCtdreq {
        Ch1DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 4 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4CtrlTrig(u32);
impl Ch4CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch4CtrlTrig {
        Ch4CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (4)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (4)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch4CtrlTrig {
    fn default() -> Ch4CtrlTrig {
        Ch4CtrlTrig(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf0(u32);
impl Intf0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intf0 {
        Intf0(val)
    }
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
#[doc = "Trigger one or more channels simultaneously"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MultiChanTrigger(u32);
impl MultiChanTrigger {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MultiChanTrigger {
        MultiChanTrigger(val)
    }
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
#[doc = "DMA Channel 5 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5CtrlTrig(u32);
impl Ch5CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch5CtrlTrig {
        Ch5CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (5)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (5)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch5CtrlTrig {
    fn default() -> Ch5CtrlTrig {
        Ch5CtrlTrig(0)
    }
}
#[doc = "Interrupt Enables for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte0(u32);
impl Inte0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Inte0 {
        Inte0(val)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5DbgCtdreq(u32);
impl Ch5DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch5DbgCtdreq {
        Ch5DbgCtdreq(val)
    }
    pub const fn ch5_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch5_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch5DbgCtdreq {
    fn default() -> Ch5DbgCtdreq {
        Ch5DbgCtdreq(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2DbgCtdreq(u32);
impl Ch2DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch2DbgCtdreq {
        Ch2DbgCtdreq(val)
    }
    pub const fn ch2_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch2_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch2DbgCtdreq {
    fn default() -> Ch2DbgCtdreq {
        Ch2DbgCtdreq(0)
    }
}
#[doc = "Interrupt Enables for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte1(u32);
impl Inte1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Inte1 {
        Inte1(val)
    }
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
#[doc = "Force Interrupts for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf1(u32);
impl Intf1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intf1 {
        Intf1(val)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch11DbgCtdreq(u32);
impl Ch11DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch11DbgCtdreq {
        Ch11DbgCtdreq(val)
    }
    pub const fn ch11_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch11_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch11DbgCtdreq {
    fn default() -> Ch11DbgCtdreq {
        Ch11DbgCtdreq(0)
    }
}
#[doc = "Interrupt Status (masked) for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints1(u32);
impl Ints1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ints1 {
        Ints1(val)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch9DbgCtdreq(u32);
impl Ch9DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch9DbgCtdreq {
        Ch9DbgCtdreq(val)
    }
    pub const fn ch9_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch9_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch9DbgCtdreq {
    fn default() -> Ch9DbgCtdreq {
        Ch9DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 2 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2CtrlTrig(u32);
impl Ch2CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch2CtrlTrig {
        Ch2CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (2)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (2)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch2CtrlTrig {
    fn default() -> Ch2CtrlTrig {
        Ch2CtrlTrig(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0DbgCtdreq(u32);
impl Ch0DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch0DbgCtdreq {
        Ch0DbgCtdreq(val)
    }
    pub const fn ch0_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch0_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch0DbgCtdreq {
    fn default() -> Ch0DbgCtdreq {
        Ch0DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 0 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0CtrlTrig(u32);
impl Ch0CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch0CtrlTrig {
        Ch0CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (0)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (0)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch0CtrlTrig {
    fn default() -> Ch0CtrlTrig {
        Ch0CtrlTrig(0)
    }
}
#[doc = "DMA Channel 8 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch8CtrlTrig(u32);
impl Ch8CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch8CtrlTrig {
        Ch8CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (8)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (8)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch8CtrlTrig {
    fn default() -> Ch8CtrlTrig {
        Ch8CtrlTrig(0)
    }
}
#[doc = "DMA Channel 3 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3CtrlTrig(u32);
impl Ch3CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch3CtrlTrig {
        Ch3CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch3CtrlTrig {
    fn default() -> Ch3CtrlTrig {
        Ch3CtrlTrig(0)
    }
}
#[doc = "DMA Channel 7 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7CtrlTrig(u32);
impl Ch7CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch7CtrlTrig {
        Ch7CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (7)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (7)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch7CtrlTrig {
    fn default() -> Ch7CtrlTrig {
        Ch7CtrlTrig(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(u32);
impl Intr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr {
        Intr(val)
    }
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch8DbgCtdreq(u32);
impl Ch8DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch8DbgCtdreq {
        Ch8DbgCtdreq(val)
    }
    pub const fn ch8_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch8_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch8DbgCtdreq {
    fn default() -> Ch8DbgCtdreq {
        Ch8DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 9 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch9CtrlTrig(u32);
impl Ch9CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch9CtrlTrig {
        Ch9CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (9)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (9)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch9CtrlTrig {
    fn default() -> Ch9CtrlTrig {
        Ch9CtrlTrig(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7DbgCtdreq(u32);
impl Ch7DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch7DbgCtdreq {
        Ch7DbgCtdreq(val)
    }
    pub const fn ch7_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch7_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch7DbgCtdreq {
    fn default() -> Ch7DbgCtdreq {
        Ch7DbgCtdreq(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4DbgCtdreq(u32);
impl Ch4DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch4DbgCtdreq {
        Ch4DbgCtdreq(val)
    }
    pub const fn ch4_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch4_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch4DbgCtdreq {
    fn default() -> Ch4DbgCtdreq {
        Ch4DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 1 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1CtrlTrig(u32);
impl Ch1CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch1CtrlTrig {
        Ch1CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (1)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (1)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch1CtrlTrig {
    fn default() -> Ch1CtrlTrig {
        Ch1CtrlTrig(0)
    }
}
#[doc = "Sniffer Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SniffCtrl(u32);
impl SniffCtrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> SniffCtrl {
        SniffCtrl(val)
    }
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
        super::vals::SniffCtrlCalc::from_bits(val as u8)
    }
    pub fn set_calc(&mut self, val: super::vals::SniffCtrlCalc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.to_bits() as u32) & 0x0f) << 5u32);
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
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6DbgCtdreq(u32);
impl Ch6DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch6DbgCtdreq {
        Ch6DbgCtdreq(val)
    }
    pub const fn ch6_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch6_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch6DbgCtdreq {
    fn default() -> Ch6DbgCtdreq {
        Ch6DbgCtdreq(0)
    }
}
#[doc = "Interrupt Status for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints0(u32);
impl Ints0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ints0 {
        Ints0(val)
    }
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
#[doc = "DMA Channel 10 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch10CtrlTrig(u32);
impl Ch10CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch10CtrlTrig {
        Ch10CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (10)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (10)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch10CtrlTrig {
    fn default() -> Ch10CtrlTrig {
        Ch10CtrlTrig(0)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3DbgCtdreq(u32);
impl Ch3DbgCtdreq {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch3DbgCtdreq {
        Ch3DbgCtdreq(val)
    }
    pub const fn ch3_dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x3f;
        val as u8
    }
    pub fn set_ch3_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0u32)) | (((val as u32) & 0x3f) << 0u32);
    }
}
impl Default for Ch3DbgCtdreq {
    fn default() -> Ch3DbgCtdreq {
        Ch3DbgCtdreq(0)
    }
}
#[doc = "DMA Channel 6 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6CtrlTrig(u32);
impl Ch6CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch6CtrlTrig {
        Ch6CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (6)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (6)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch6CtrlTrig {
    fn default() -> Ch6CtrlTrig {
        Ch6CtrlTrig(0)
    }
}
#[doc = "DMA Channel 11 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch11CtrlTrig(u32);
impl Ch11CtrlTrig {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ch11CtrlTrig {
        Ch11CtrlTrig(val)
    }
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
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15u32)) | (((val.to_bits() as u32) & 0x3f) << 15u32);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (11)."]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Reset value is equal to channel number (11)."]
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
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val.to_bits() as u32) & 0x03) << 2u32);
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
impl Default for Ch11CtrlTrig {
    fn default() -> Ch11CtrlTrig {
        Ch11CtrlTrig(0)
    }
}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NChannels(u32);
impl NChannels {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NChannels {
        NChannels(val)
    }
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
#[doc = "Debug RAF, WAF, TDF levels"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FifoLevels(u32);
impl FifoLevels {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> FifoLevels {
        FifoLevels(val)
    }
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
#[doc = "Abort an in-progress transfer sequence on one or more channels"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ChanAbort(u32);
impl ChanAbort {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> ChanAbort {
        ChanAbort(val)
    }
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
