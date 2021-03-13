use crate::generic::*;
#[doc = "FIFO stream control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StreamCtr(pub u32);
impl StreamCtr {
    #[doc = "Write a nonzero value to start a streaming read. This will then progress in the background, using flash idle cycles to transfer a linear data block from flash to the streaming FIFO. Decrements automatically (1 at a time) as the stream progresses, and halts on reaching 0. Write 0 to halt an in-progress stream, and discard any in-flight read, so that a new stream can immediately be started (after draining the FIFO and reinitialising STREAM_ADDR)"]
    pub const fn stream_ctr(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x003f_ffff;
        val as u32
    }
    #[doc = "Write a nonzero value to start a streaming read. This will then progress in the background, using flash idle cycles to transfer a linear data block from flash to the streaming FIFO. Decrements automatically (1 at a time) as the stream progresses, and halts on reaching 0. Write 0 to halt an in-progress stream, and discard any in-flight read, so that a new stream can immediately be started (after draining the FIFO and reinitialising STREAM_ADDR)"]
    pub fn set_stream_ctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x003f_ffff << 0u32)) | (((val as u32) & 0x003f_ffff) << 0u32);
    }
}
impl Default for StreamCtr {
    fn default() -> StreamCtr {
        StreamCtr(0)
    }
}
#[doc = "Cache Flush control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Flush(pub u32);
impl Flush {
    #[doc = "Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
    pub const fn flush(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to flush the cache. This clears the tag memory, but the data memory retains its contents. (This means cache-as-SRAM contents is not affected by flush or reset.) Reading will hold the bus (stall the processor) until the flush completes. Alternatively STAT can be polled until completion."]
    pub fn set_flush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Flush {
    fn default() -> Flush {
        Flush(0)
    }
}
#[doc = "Cache Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Stat(pub u32);
impl Stat {
    #[doc = "When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
    pub const fn fifo_full(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely full. The streaming FIFO is 2 entries deep, so the full and empty flag allow its level to be ascertained."]
    pub fn set_fifo_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely empty."]
    pub const fn fifo_empty(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "When 1, indicates the XIP streaming FIFO is completely empty."]
    pub fn set_fifo_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
    pub const fn flush_ready(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Reads as 0 while a cache flush is in progress, and 1 otherwise. The cache is flushed whenever the XIP block is reset, and also when requested via the FLUSH register."]
    pub fn set_flush_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Stat {
    fn default() -> Stat {
        Stat(0)
    }
}
#[doc = "Cache control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot be enabled when powered down. Cache-as-SRAM accesses will produce a bus error response when the cache is powered down."]
    pub const fn power_down(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "When 1, the cache memories are powered down. They retain state, but can not be accessed. This reduces static power dissipation. Writing 1 to this bit forces CTRL_EN to 0, i.e. the cache cannot be enabled when powered down. Cache-as-SRAM accesses will produce a bus error response when the cache is powered down."]
    pub fn set_power_down(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "When 1, writes to any alias other than 0x0 (caching, allocating) will produce a bus fault. When 0, these writes are silently ignored. In either case, writes to the 0x0 alias will deallocate on tag match, as usual."]
    pub const fn err_badwrite(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "When 1, writes to any alias other than 0x0 (caching, allocating) will produce a bus fault. When 0, these writes are silently ignored. In either case, writes to the 0x0 alias will deallocate on tag match, as usual."]
    pub fn set_err_badwrite(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "When 1, enable the cache. When the cache is disabled, all XIP accesses will go straight to the flash, without querying the cache. When enabled, cacheable XIP accesses will query the cache, and the flash will not be accessed if the tag matches and the valid bit is set. If the cache is enabled, cache-as-SRAM accesses have no effect on the cache data RAM, and will produce a bus error response."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "When 1, enable the cache. When the cache is disabled, all XIP accesses will go straight to the flash, without querying the cache. When enabled, cacheable XIP accesses will query the cache, and the flash will not be accessed if the tag matches and the valid bit is set. If the cache is enabled, cache-as-SRAM accesses have no effect on the cache data RAM, and will produce a bus error response."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "FIFO stream address"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct StreamAddr(pub u32);
impl StreamAddr {
    #[doc = "The address of the next word to be streamed from flash to the streaming FIFO. Increments automatically after each flash access. Write the initial access address here before starting a streaming read."]
    pub const fn stream_addr(&self) -> u32 {
        let val = (self.0 >> 2u32) & 0x3fff_ffff;
        val as u32
    }
    #[doc = "The address of the next word to be streamed from flash to the streaming FIFO. Increments automatically after each flash access. Write the initial access address here before starting a streaming read."]
    pub fn set_stream_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x3fff_ffff << 2u32)) | (((val as u32) & 0x3fff_ffff) << 2u32);
    }
}
impl Default for StreamAddr {
    fn default() -> StreamAddr {
        StreamAddr(0)
    }
}
