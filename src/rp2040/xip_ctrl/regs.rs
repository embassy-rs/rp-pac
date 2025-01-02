#[doc = "Cache control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "When 1, enable the cache. When the cache is disabled, all XIP accesses will go straight to the flash, without querying the cache. When enabled, cacheable XIP accesses will query the cache, and the flash will not be accessed if the tag matches and the valid bit is set. If the cache is enabled, cache-as-SRAM accesses have no effect on the cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, enable the cache. When the cache is disabled, all XIP accesses will go straight to the flash, without querying the cache. When enabled, cacheable XIP accesses will query the cache, and the flash will not be accessed if the tag matches and the valid bit is set. If the cache is enabled, cache-as-SRAM accesses have no effect on the cache data RAM, and will produce a bus error response."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When 1, writes to any alias other than 0x0 (caching, allocating) will produce a bus fault. When 0, these writes are silently ignored. In either case, writes to the 0x0 alias will deallocate on tag match, as usual."]
    #[inline(always)]
    pub const fn err_badwrite(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, writes to any alias other than 0x0 (caching, allocating) will produce a bus fault. When 0, these writes are silently ignored. In either case, writes to the 0x0 alias will deallocate on tag match, as usual."]
    #[inline(always)]
    pub fn set_err_badwrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot be enabled when powered down. Cache-as-SRAM accesses will produce a bus error response when the cache is powered down."]
    #[inline(always)]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot be enabled when powered down. Cache-as-SRAM accesses will produce a bus error response when the cache is powered down."]
    #[inline(always)]
    pub fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("en", &self.en())
            .field("err_badwrite", &self.err_badwrite())
            .field("power_down", &self.power_down())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            en: bool,
            err_badwrite: bool,
            power_down: bool,
        }
        let proxy = Ctrl {
            en: self.en(),
            err_badwrite: self.err_badwrite(),
            power_down: self.power_down(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Cache Flush control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flush(pub u32);
impl Flush {
    #[doc = "Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub const fn flush(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
    #[inline(always)]
    pub fn set_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Flush {
    #[inline(always)]
    fn default() -> Flush {
        Flush(0)
    }
}
impl core::fmt::Debug for Flush {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flush")
            .field("flush", &self.flush())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flush {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flush {
            flush: bool,
        }
        let proxy = Flush {
            flush: self.flush(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Cache Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
    #[inline(always)]
    pub const fn flush_ready(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
    #[inline(always)]
    pub fn set_flush_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub const fn fifo_empty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely empty."]
    #[inline(always)]
    pub fn set_fifo_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
    #[inline(always)]
    pub const fn fifo_full(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
    #[inline(always)]
    pub fn set_fifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Stat {
    #[inline(always)]
    fn default() -> Stat {
        Stat(0)
    }
}
impl core::fmt::Debug for Stat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Stat")
            .field("flush_ready", &self.flush_ready())
            .field("fifo_empty", &self.fifo_empty())
            .field("fifo_full", &self.fifo_full())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Stat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Stat {
            flush_ready: bool,
            fifo_empty: bool,
            fifo_full: bool,
        }
        let proxy = Stat {
            flush_ready: self.flush_ready(),
            fifo_empty: self.fifo_empty(),
            fifo_full: self.fifo_full(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO stream address"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StreamAddr(pub u32);
impl StreamAddr {
    #[doc = "The address of the next word to be streamed from flash to the streaming FIFO. Increments automatically after each flash access. Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub const fn stream_addr(&self) -> u32 {
        let val = (self.0 >> 2usize) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "The address of the next word to be streamed from flash to the streaming FIFO. Increments automatically after each flash access. Write the initial access address here before starting a streaming read."]
    #[inline(always)]
    pub fn set_stream_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2usize)) | (((val as u32) & 0x3fff_ffff) << 2usize);
    }
}
impl Default for StreamAddr {
    #[inline(always)]
    fn default() -> StreamAddr {
        StreamAddr(0)
    }
}
impl core::fmt::Debug for StreamAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StreamAddr")
            .field("stream_addr", &self.stream_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StreamAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct StreamAddr {
            stream_addr: u32,
        }
        let proxy = StreamAddr {
            stream_addr: self.stream_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO stream control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StreamCtr(pub u32);
impl StreamCtr {
    #[doc = "Write a nonzero value to start a streaming read. This will then progress in the background, using flash idle cycles to transfer a linear data block from flash to the streaming FIFO. Decrements automatically (1 at a time) as the stream progresses, and halts on reaching 0. Write 0 to halt an in-progress stream, and discard any in-flight read, so that a new stream can immediately be started (after draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub const fn stream_ctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write a nonzero value to start a streaming read. This will then progress in the background, using flash idle cycles to transfer a linear data block from flash to the streaming FIFO. Decrements automatically (1 at a time) as the stream progresses, and halts on reaching 0. Write 0 to halt an in-progress stream, and discard any in-flight read, so that a new stream can immediately be started (after draining the FIFO and reinitialising STREAM_ADDR)"]
    #[inline(always)]
    pub fn set_stream_ctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0usize)) | (((val as u32) & 0x003f_ffff) << 0usize);
    }
}
impl Default for StreamCtr {
    #[inline(always)]
    fn default() -> StreamCtr {
        StreamCtr(0)
    }
}
impl core::fmt::Debug for StreamCtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("StreamCtr")
            .field("stream_ctr", &self.stream_ctr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for StreamCtr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct StreamCtr {
            stream_ctr: u32,
        }
        let proxy = StreamCtr {
            stream_ctr: self.stream_ctr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
