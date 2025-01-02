#[doc = "Abort an in-progress transfer sequence on one or more channels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChanAbort(pub u32);
impl ChanAbort {
    #[doc = "Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs. After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub const fn chan_abort(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit corresponds to a channel. Writing a 1 aborts whatever transfer sequence is in progress on that channel. The bit will remain high until any in-flight transfers have been flushed through the address and data FIFOs. After writing, this register must be polled until it returns all-zero. Until this point, it is unsafe to restart the channel."]
    #[inline(always)]
    pub fn set_chan_abort(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChanAbort {
    #[inline(always)]
    fn default() -> ChanAbort {
        ChanAbort(0)
    }
}
impl core::fmt::Debug for ChanAbort {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChanAbort")
            .field("chan_abort", &self.chan_abort())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChanAbort {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChanAbort {
            chan_abort: u16,
        }
        let proxy = ChanAbort {
            chan_abort: self.chan_abort(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Channel 0 Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CtrlTrig(pub u32);
impl CtrlTrig {
    #[doc = "DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "DMA Channel Enable. When 1, the channel will respond to triggering events, which will cause it to become BUSY and start transferring data. When 0, the channel will ignore triggers, stop issuing transfers, and pause the current transfer sequence (i.e. BUSY will remain high if already high)"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    pub const fn high_priority(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "HIGH_PRIORITY gives a channel preferential treatment in issue scheduling: in each scheduling round, all high priority channels are considered first, and then only a single low priority channel, before returning to the high priority channels. This only affects the order in which the DMA schedules channels. The DMA's bus priority is not changed. If the DMA is not saturated then a low priority channel will see no loss of throughput."]
    #[inline(always)]
    pub fn set_high_priority(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    pub const fn data_size(&self) -> super::vals::DataSize {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::DataSize::from_bits(val as u8)
    }
    #[doc = "Set the size of each bus transfer (byte/halfword/word). READ_ADDR and WRITE_ADDR advance by this amount (1/2/4 bytes) with each transfer."]
    #[inline(always)]
    pub fn set_data_size(&mut self, val: super::vals::DataSize) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    pub const fn incr_read(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the read address increments with each transfer. If 0, each read is directed to the same, initial address. Generally this should be disabled for peripheral-to-memory transfers."]
    #[inline(always)]
    pub fn set_incr_read(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub const fn incr_write(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub fn set_incr_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub const fn ring_size(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x0f;
        val as u8
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub fn set_ring_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 6usize)) | (((val as u32) & 0x0f) << 6usize);
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub const fn ring_sel(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub fn set_ring_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_."]
    #[inline(always)]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 11usize) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_."]
    #[inline(always)]
    pub fn set_chain_to(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 11usize)) | (((val as u32) & 0x0f) << 11usize);
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub const fn treq_sel(&self) -> super::vals::TreqSel {
        let val = (self.0 >> 15usize) & 0x3f;
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 15usize)) | (((val.to_bits() as u32) & 0x3f) << 15usize);
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub const fn irq_quiet(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub fn set_irq_quiet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub const fn bswap(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub fn set_bswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub const fn sniff_en(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub fn set_sniff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    pub const fn write_error(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the channel received a write bus error. Write one to clear. WRITE_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 5 transfers later)"]
    #[inline(always)]
    pub fn set_write_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    pub const fn read_error(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the channel received a read bus error. Write one to clear. READ_ADDR shows the approximate address where the bus error was encountered (will not be earlier, or more than 3 transfers later)"]
    #[inline(always)]
    pub fn set_read_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    #[inline(always)]
    pub const fn ahb_error(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Logical OR of the READ_ERROR and WRITE_ERROR flags. The channel halts when it encounters any bus error, and always raises its channel IRQ flag."]
    #[inline(always)]
    pub fn set_ahb_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for CtrlTrig {
    #[inline(always)]
    fn default() -> CtrlTrig {
        CtrlTrig(0)
    }
}
impl core::fmt::Debug for CtrlTrig {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CtrlTrig")
            .field("en", &self.en())
            .field("high_priority", &self.high_priority())
            .field("data_size", &self.data_size())
            .field("incr_read", &self.incr_read())
            .field("incr_write", &self.incr_write())
            .field("ring_size", &self.ring_size())
            .field("ring_sel", &self.ring_sel())
            .field("chain_to", &self.chain_to())
            .field("treq_sel", &self.treq_sel())
            .field("irq_quiet", &self.irq_quiet())
            .field("bswap", &self.bswap())
            .field("sniff_en", &self.sniff_en())
            .field("busy", &self.busy())
            .field("write_error", &self.write_error())
            .field("read_error", &self.read_error())
            .field("ahb_error", &self.ahb_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for CtrlTrig {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct CtrlTrig {
            en: bool,
            high_priority: bool,
            data_size: super::vals::DataSize,
            incr_read: bool,
            incr_write: bool,
            ring_size: u8,
            ring_sel: bool,
            chain_to: u8,
            treq_sel: super::vals::TreqSel,
            irq_quiet: bool,
            bswap: bool,
            sniff_en: bool,
            busy: bool,
            write_error: bool,
            read_error: bool,
            ahb_error: bool,
        }
        let proxy = CtrlTrig {
            en: self.en(),
            high_priority: self.high_priority(),
            data_size: self.data_size(),
            incr_read: self.incr_read(),
            incr_write: self.incr_write(),
            ring_size: self.ring_size(),
            ring_sel: self.ring_sel(),
            chain_to: self.chain_to(),
            treq_sel: self.treq_sel(),
            irq_quiet: self.irq_quiet(),
            bswap: self.bswap(),
            sniff_en: self.sniff_en(),
            busy: self.busy(),
            write_error: self.write_error(),
            read_error: self.read_error(),
            ahb_error: self.ahb_error(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgCtdreq(pub u32);
impl DbgCtdreq {
    #[inline(always)]
    pub const fn dbg_ctdreq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_dbg_ctdreq(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for DbgCtdreq {
    #[inline(always)]
    fn default() -> DbgCtdreq {
        DbgCtdreq(0)
    }
}
impl core::fmt::Debug for DbgCtdreq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgCtdreq")
            .field("dbg_ctdreq", &self.dbg_ctdreq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgCtdreq {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbgCtdreq {
            dbg_ctdreq: u8,
        }
        let proxy = DbgCtdreq {
            dbg_ctdreq: self.dbg_ctdreq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Debug RAF, WAF, TDF levels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoLevels(pub u32);
impl FifoLevels {
    #[doc = "Current Transfer-Data-FIFO fill level"]
    #[inline(always)]
    pub const fn tdf_lvl(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Current Transfer-Data-FIFO fill level"]
    #[inline(always)]
    pub fn set_tdf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Current Write-Address-FIFO fill level"]
    #[inline(always)]
    pub const fn waf_lvl(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Current Write-Address-FIFO fill level"]
    #[inline(always)]
    pub fn set_waf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Current Read-Address-FIFO fill level"]
    #[inline(always)]
    pub const fn raf_lvl(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Current Read-Address-FIFO fill level"]
    #[inline(always)]
    pub fn set_raf_lvl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for FifoLevels {
    #[inline(always)]
    fn default() -> FifoLevels {
        FifoLevels(0)
    }
}
impl core::fmt::Debug for FifoLevels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FifoLevels")
            .field("tdf_lvl", &self.tdf_lvl())
            .field("waf_lvl", &self.waf_lvl())
            .field("raf_lvl", &self.raf_lvl())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FifoLevels {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FifoLevels {
            tdf_lvl: u8,
            waf_lvl: u8,
            raf_lvl: u8,
        }
        let proxy = FifoLevels {
            tdf_lvl: self.tdf_lvl(),
            waf_lvl: self.waf_lvl(),
            raf_lvl: self.raf_lvl(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Trigger one or more channels simultaneously"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MultiChanTrigger(pub u32);
impl MultiChanTrigger {
    #[doc = "Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    pub const fn multi_chan_trigger(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Each bit in this register corresponds to a DMA channel. Writing a 1 to the relevant bit is the same as writing to that channel's trigger register; the channel will start if it is currently enabled and not already busy."]
    #[inline(always)]
    pub fn set_multi_chan_trigger(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for MultiChanTrigger {
    #[inline(always)]
    fn default() -> MultiChanTrigger {
        MultiChanTrigger(0)
    }
}
impl core::fmt::Debug for MultiChanTrigger {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MultiChanTrigger")
            .field("multi_chan_trigger", &self.multi_chan_trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MultiChanTrigger {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MultiChanTrigger {
            multi_chan_trigger: u16,
        }
        let proxy = MultiChanTrigger {
            multi_chan_trigger: self.multi_chan_trigger(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Nchannels(pub u32);
impl Nchannels {
    #[inline(always)]
    pub const fn n_channels(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub fn set_n_channels(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for Nchannels {
    #[inline(always)]
    fn default() -> Nchannels {
        Nchannels(0)
    }
}
impl core::fmt::Debug for Nchannels {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Nchannels")
            .field("n_channels", &self.n_channels())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Nchannels {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Nchannels {
            n_channels: u8,
        }
        let proxy = Nchannels {
            n_channels: self.n_channels(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Sniffer Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SniffCtrl(pub u32);
impl SniffCtrl {
    #[doc = "Enable sniffer"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable sniffer"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DMA channel for Sniffer to observe"]
    #[inline(always)]
    pub const fn dmach(&self) -> u8 {
        let val = (self.0 >> 1usize) & 0x0f;
        val as u8
    }
    #[doc = "DMA channel for Sniffer to observe"]
    #[inline(always)]
    pub fn set_dmach(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 1usize)) | (((val as u32) & 0x0f) << 1usize);
    }
    #[inline(always)]
    pub const fn calc(&self) -> super::vals::Calc {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::Calc::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_calc(&mut self, val: super::vals::Calc) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "Locally perform a byte reverse on the sniffed data, before feeding into checksum. Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    pub const fn bswap(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Locally perform a byte reverse on the sniffed data, before feeding into checksum. Note that the sniff hardware is downstream of the DMA channel byteswap performed in the read master: if channel CTRL_BSWAP and SNIFF_CTRL_BSWAP are both enabled, their effects cancel from the sniffer's point of view."]
    #[inline(always)]
    pub fn set_bswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub const fn out_rev(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "If set, the result appears bit-reversed when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn set_out_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub const fn out_inv(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "If set, the result appears inverted (bitwise complement) when read. This does not affect the way the checksum is calculated; the result is transformed on-the-fly between the result register and the bus."]
    #[inline(always)]
    pub fn set_out_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for SniffCtrl {
    #[inline(always)]
    fn default() -> SniffCtrl {
        SniffCtrl(0)
    }
}
impl core::fmt::Debug for SniffCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SniffCtrl")
            .field("en", &self.en())
            .field("dmach", &self.dmach())
            .field("calc", &self.calc())
            .field("bswap", &self.bswap())
            .field("out_rev", &self.out_rev())
            .field("out_inv", &self.out_inv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SniffCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SniffCtrl {
            en: bool,
            dmach: u8,
            calc: super::vals::Calc,
            bswap: bool,
            out_rev: bool,
            out_inv: bool,
        }
        let proxy = SniffCtrl {
            en: self.en(),
            dmach: self.dmach(),
            calc: self.calc(),
            bswap: self.bswap(),
            out_rev: self.out_rev(),
            out_inv: self.out_inv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Pacing (X/Y) Fractional Timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub const fn y(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Pacing Timer Divisor. Specifies the Y value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn set_y(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub const fn x(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Pacing Timer Dividend. Specifies the X value for the (X/Y) fractional timer."]
    #[inline(always)]
    pub fn set_x(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}
impl core::fmt::Debug for Timer {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer")
            .field("y", &self.y())
            .field("x", &self.x())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer {
            y: u16,
            x: u16,
        }
        let proxy = Timer {
            y: self.y(),
            x: self.x(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
