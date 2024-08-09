#[doc = "DMA Channel 0 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0transCount(pub u32);
impl Ch0transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch0transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch0transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch0transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch0transCount {
    #[inline(always)]
    fn default() -> Ch0transCount {
        Ch0transCount(0)
    }
}
#[doc = "DMA Channel 10 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch10transCount(pub u32);
impl Ch10transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch10transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch10transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch10transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch10transCount {
    #[inline(always)]
    fn default() -> Ch10transCount {
        Ch10transCount(0)
    }
}
#[doc = "DMA Channel 11 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch11transCount(pub u32);
impl Ch11transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch11transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch11transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch11transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch11transCount {
    #[inline(always)]
    fn default() -> Ch11transCount {
        Ch11transCount(0)
    }
}
#[doc = "DMA Channel 12 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch12transCount(pub u32);
impl Ch12transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch12transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch12transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch12transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch12transCount {
    #[inline(always)]
    fn default() -> Ch12transCount {
        Ch12transCount(0)
    }
}
#[doc = "DMA Channel 13 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch13transCount(pub u32);
impl Ch13transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch13transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch13transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch13transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch13transCount {
    #[inline(always)]
    fn default() -> Ch13transCount {
        Ch13transCount(0)
    }
}
#[doc = "DMA Channel 14 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch14transCount(pub u32);
impl Ch14transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch14transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch14transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch14transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch14transCount {
    #[inline(always)]
    fn default() -> Ch14transCount {
        Ch14transCount(0)
    }
}
#[doc = "DMA Channel 15 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch15transCount(pub u32);
impl Ch15transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch15transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch15transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch15transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch15transCount {
    #[inline(always)]
    fn default() -> Ch15transCount {
        Ch15transCount(0)
    }
}
#[doc = "DMA Channel 1 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1transCount(pub u32);
impl Ch1transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch1transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch1transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch1transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch1transCount {
    #[inline(always)]
    fn default() -> Ch1transCount {
        Ch1transCount(0)
    }
}
#[doc = "DMA Channel 2 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2transCount(pub u32);
impl Ch2transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch2transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch2transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch2transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch2transCount {
    #[inline(always)]
    fn default() -> Ch2transCount {
        Ch2transCount(0)
    }
}
#[doc = "DMA Channel 3 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3transCount(pub u32);
impl Ch3transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch3transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch3transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch3transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch3transCount {
    #[inline(always)]
    fn default() -> Ch3transCount {
        Ch3transCount(0)
    }
}
#[doc = "DMA Channel 4 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4transCount(pub u32);
impl Ch4transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch4transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch4transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch4transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch4transCount {
    #[inline(always)]
    fn default() -> Ch4transCount {
        Ch4transCount(0)
    }
}
#[doc = "DMA Channel 5 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5transCount(pub u32);
impl Ch5transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch5transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch5transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch5transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch5transCount {
    #[inline(always)]
    fn default() -> Ch5transCount {
        Ch5transCount(0)
    }
}
#[doc = "DMA Channel 6 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6transCount(pub u32);
impl Ch6transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch6transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch6transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch6transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch6transCount {
    #[inline(always)]
    fn default() -> Ch6transCount {
        Ch6transCount(0)
    }
}
#[doc = "DMA Channel 7 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7transCount(pub u32);
impl Ch7transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch7transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch7transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch7transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch7transCount {
    #[inline(always)]
    fn default() -> Ch7transCount {
        Ch7transCount(0)
    }
}
#[doc = "DMA Channel 8 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch8transCount(pub u32);
impl Ch8transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch8transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch8transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch8transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch8transCount {
    #[inline(always)]
    fn default() -> Ch8transCount {
        Ch8transCount(0)
    }
}
#[doc = "DMA Channel 9 Transfer Count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch9transCount(pub u32);
impl Ch9transCount {
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub const fn count(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0fff_ffff;
        val as u32
    }
    #[doc = "28-bit transfer count (256 million transfers maximum). Program the number of bus transfers a channel will perform before halting. Note that, if transfers are larger than one byte in size, this is not equal to the number of bytes transferred (see CTRL_DATA_SIZE). When the channel is active, reading this register shows the number of transfers remaining, updating automatically each time a write transfer completes. Writing this register sets the RELOAD value for the transfer counter. Each time this channel is triggered, the RELOAD value is copied into the live transfer counter. The channel can be started multiple times, and will perform the same number of transfers each time, as programmed by most recent write. The RELOAD value can be observed at CHx_DBG_TCR. If TRANS_COUNT is used as a trigger, the written value is used immediately as the length of the new transfer sequence, as well as being written to RELOAD."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0fff_ffff << 0usize)) | (((val as u32) & 0x0fff_ffff) << 0usize);
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Ch9transCountMode {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Ch9transCountMode::from_bits(val as u8)
    }
    #[doc = "When MODE is 0x0, the transfer count decrements with each transfer until 0, and then the channel triggers the next channel indicated by CTRL_CHAIN_TO. When MODE is 0x1, the transfer count decrements with each transfer until 0, and then the channel re-triggers itself, in addition to the trigger indicated by CTRL_CHAIN_TO. This is useful for e.g. an endless ring-buffer DMA with periodic interrupts. When MODE is 0xf, the transfer count does not decrement. The DMA channel performs an endless sequence of transfers, never triggering other channels or raising interrupts, until an ABORT is raised. All other values are reserved."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Ch9transCountMode) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for Ch9transCount {
    #[inline(always)]
    fn default() -> Ch9transCount {
        Ch9transCount(0)
    }
}
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
#[doc = "DMA Channel 13 Control and Status"]
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
    #[doc = "If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub const fn incr_read_rev(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and INCR_READ is 1, the read address is decremented rather than incremented with each transfer. If 1, and INCR_READ is 0, this otherwise-unused combination causes the read address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub fn set_incr_read_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub const fn incr_write(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the write address increments with each transfer. If 0, each write is directed to the same, initial address. Generally this should be disabled for memory-to-peripheral transfers."]
    #[inline(always)]
    pub fn set_incr_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub const fn incr_write_rev(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, and INCR_WRITE is 1, the write address is decremented rather than incremented with each transfer. If 1, and INCR_WRITE is 0, this otherwise-unused combination causes the write address to be incremented by twice the transfer size, i.e. skipping over alternate addresses."]
    #[inline(always)]
    pub fn set_incr_write_rev(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub const fn ring_size(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Size of address wrap region. If 0, don't wrap. For values n > 0, only the lower n bits of the address will change. This wraps the address on a (1 << n) byte boundary, facilitating access to naturally-aligned ring buffers. Ring sizes between 2 and 32768 bytes are possible. This can apply to either read or write addresses, based on value of RING_SEL."]
    #[inline(always)]
    pub fn set_ring_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub const fn ring_sel(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Select whether RING_SIZE applies to read or write addresses. If 0, read addresses are wrapped on a (1 << RING_SIZE) boundary. If 1, write addresses are wrapped."]
    #[inline(always)]
    pub fn set_ring_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Note this field resets to 0, so channels 1 and above will chain to channel 0 by default. Set this field to avoid this behaviour."]
    #[inline(always)]
    pub const fn chain_to(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x0f;
        val as u8
    }
    #[doc = "When this channel completes, it will trigger the channel indicated by CHAIN_TO. Disable by setting CHAIN_TO = _(this channel)_. Note this field resets to 0, so channels 1 and above will chain to channel 0 by default. Set this field to avoid this behaviour."]
    #[inline(always)]
    pub fn set_chain_to(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 13usize)) | (((val as u32) & 0x0f) << 13usize);
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub const fn treq_sel(&self) -> super::vals::TreqSel {
        let val = (self.0 >> 17usize) & 0x3f;
        super::vals::TreqSel::from_bits(val as u8)
    }
    #[doc = "Select a Transfer Request signal. The channel uses the transfer request signal to pace its data transfer rate. Sources for TREQ signals are internal (TIMERS) or external (DREQ, a Data Request from the system). 0x0 to 0x3a -> select DREQ n as TREQ"]
    #[inline(always)]
    pub fn set_treq_sel(&mut self, val: super::vals::TreqSel) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val.to_bits() as u32) & 0x3f) << 17usize);
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub const fn irq_quiet(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "In QUIET mode, the channel does not generate IRQs at the end of every transfer block. Instead, an IRQ is raised when NULL is written to a trigger register, indicating the end of a control block chain. This reduces the number of interrupts to be serviced by the CPU when transferring a DMA chain of many small control blocks."]
    #[inline(always)]
    pub fn set_irq_quiet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub const fn bswap(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Apply byte-swap transformation to DMA data. For byte data, this has no effect. For halfword data, the two bytes of each halfword are swapped. For word data, the four bytes of each word are swapped to reverse order."]
    #[inline(always)]
    pub fn set_bswap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub const fn sniff_en(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, this channel's data transfers are visible to the sniff hardware, and each transfer will advance the state of the checksum. This only applies if the sniff hardware is enabled, and has this channel selected. This allows checksum to be enabled or disabled on a per-control- block basis."]
    #[inline(always)]
    pub fn set_sniff_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "This flag goes high when the channel starts a new transfer sequence, and low when the last transfer of that sequence completes. Clearing EN while BUSY is high pauses the channel, and BUSY will stay high while paused. To terminate a sequence early (and clear the BUSY flag), see CHAN_ABORT."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
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
#[doc = "Interrupt Enables for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte0(pub u32);
impl Inte0 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 0. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ0."]
    #[inline(always)]
    pub const fn inte0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 0. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ0."]
    #[inline(always)]
    pub fn set_inte0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Inte0 {
    #[inline(always)]
    fn default() -> Inte0 {
        Inte0(0)
    }
}
#[doc = "Interrupt Enables for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte1(pub u32);
impl Inte1 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 1. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ1."]
    #[inline(always)]
    pub const fn inte1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 1. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ1."]
    #[inline(always)]
    pub fn set_inte1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Inte1 {
    #[inline(always)]
    fn default() -> Inte1 {
        Inte1(0)
    }
}
#[doc = "Interrupt Enables for IRQ 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte2(pub u32);
impl Inte2 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
    #[inline(always)]
    pub const fn inte2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 2. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ2."]
    #[inline(always)]
    pub fn set_inte2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Inte2 {
    #[inline(always)]
    fn default() -> Inte2 {
        Inte2(0)
    }
}
#[doc = "Interrupt Enables for IRQ 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte3(pub u32);
impl Inte3 {
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
    #[inline(always)]
    pub const fn inte3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Set bit n to pass interrupts from channel n to DMA IRQ 3. Note this bit has no effect if the channel security/privilege level, defined by SECCFG_CHx, is greater than the IRQ security/privilege defined by SECCFG_IRQ3."]
    #[inline(always)]
    pub fn set_inte3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Inte3 {
    #[inline(always)]
    fn default() -> Inte3 {
        Inte3(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf0(pub u32);
impl Intf0 {
    #[doc = "Write 1s to force the corresponding bits in INTS0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub const fn intf0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTS0. The interrupt remains asserted until INTF0 is cleared."]
    #[inline(always)]
    pub fn set_intf0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intf0 {
    #[inline(always)]
    fn default() -> Intf0 {
        Intf0(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf1(pub u32);
impl Intf1 {
    #[doc = "Write 1s to force the corresponding bits in INTS1. The interrupt remains asserted until INTF1 is cleared."]
    #[inline(always)]
    pub const fn intf1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTS1. The interrupt remains asserted until INTF1 is cleared."]
    #[inline(always)]
    pub fn set_intf1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intf1 {
    #[inline(always)]
    fn default() -> Intf1 {
        Intf1(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf2(pub u32);
impl Intf2 {
    #[doc = "Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
    #[inline(always)]
    pub const fn intf2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTS2. The interrupt remains asserted until INTF2 is cleared."]
    #[inline(always)]
    pub fn set_intf2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intf2 {
    #[inline(always)]
    fn default() -> Intf2 {
        Intf2(0)
    }
}
#[doc = "Force Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf3(pub u32);
impl Intf3 {
    #[doc = "Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
    #[inline(always)]
    pub const fn intf3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Write 1s to force the corresponding bits in INTS3. The interrupt remains asserted until INTF3 is cleared."]
    #[inline(always)]
    pub fn set_intf3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intf3 {
    #[inline(always)]
    fn default() -> Intf3 {
        Intf3(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub const fn intr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub fn set_intr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr1(pub u32);
impl Intr1 {
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub const fn intr1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub fn set_intr1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intr1 {
    #[inline(always)]
    fn default() -> Intr1 {
        Intr1(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr2(pub u32);
impl Intr2 {
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub const fn intr2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub fn set_intr2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intr2 {
    #[inline(always)]
    fn default() -> Intr2 {
        Intr2(0)
    }
}
#[doc = "Interrupt Status (raw)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr3(pub u32);
impl Intr3 {
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub const fn intr3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Raw interrupt status for DMA Channels 0..15. Bit n corresponds to channel n. Ignores any masking or forcing. Channel interrupts can be cleared by writing a bit mask to INTR or INTS0/1/2/3. Channel interrupts can be routed to either of four system-level IRQs based on INTE0, INTE1, INTE2 and INTE3. The multiple system-level interrupts might be used to allow NVIC IRQ preemption for more time-critical channels, to spread IRQ load across different cores, or to target IRQs to different security domains. It is also valid to ignore the multiple IRQs, and just use INTE0/INTS0/IRQ 0. If this register is accessed at a security/privilege level less than that of a given channel (as defined by that channel's SECCFG_CHx register), then that channel's interrupt status will read as 0, ignore writes."]
    #[inline(always)]
    pub fn set_intr3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Intr3 {
    #[inline(always)]
    fn default() -> Intr3 {
        Intr3(0)
    }
}
#[doc = "Interrupt Status for IRQ 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints0(pub u32);
impl Ints0 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ0) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub const fn ints0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 0 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ0) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn set_ints0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ints0 {
    #[inline(always)]
    fn default() -> Ints0 {
        Ints0(0)
    }
}
#[doc = "Interrupt Status for IRQ 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints1(pub u32);
impl Ints1 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ1) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub const fn ints1(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 1 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ1) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn set_ints1(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ints1 {
    #[inline(always)]
    fn default() -> Ints1 {
        Ints1(0)
    }
}
#[doc = "Interrupt Status for IRQ 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints2(pub u32);
impl Ints2 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub const fn ints2(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 2 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ2) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn set_ints2(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ints2 {
    #[inline(always)]
    fn default() -> Ints2 {
        Ints2(0)
    }
}
#[doc = "Interrupt Status for IRQ 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints3(pub u32);
impl Ints3 {
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub const fn ints3(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Indicates active channel interrupt requests which are currently causing IRQ 3 to be asserted. Channel interrupts can be cleared by writing a bit mask here. Channels with a security/privilege (SECCFG_CHx) greater SECCFG_IRQ3) read as 0 in this register, and ignore writes."]
    #[inline(always)]
    pub fn set_ints3(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ints3 {
    #[inline(always)]
    fn default() -> Ints3 {
        Ints3(0)
    }
}
#[doc = "Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar0(pub u32);
impl MpuBar0 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar0 {
    #[inline(always)]
    fn default() -> MpuBar0 {
        MpuBar0(0)
    }
}
#[doc = "Base address register for MPU region 1. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar1(pub u32);
impl MpuBar1 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar1 {
    #[inline(always)]
    fn default() -> MpuBar1 {
        MpuBar1(0)
    }
}
#[doc = "Base address register for MPU region 2. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar2(pub u32);
impl MpuBar2 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar2 {
    #[inline(always)]
    fn default() -> MpuBar2 {
        MpuBar2(0)
    }
}
#[doc = "Base address register for MPU region 3. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar3(pub u32);
impl MpuBar3 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar3 {
    #[inline(always)]
    fn default() -> MpuBar3 {
        MpuBar3(0)
    }
}
#[doc = "Base address register for MPU region 4. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar4(pub u32);
impl MpuBar4 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar4 {
    #[inline(always)]
    fn default() -> MpuBar4 {
        MpuBar4(0)
    }
}
#[doc = "Base address register for MPU region 5. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar5(pub u32);
impl MpuBar5 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar5 {
    #[inline(always)]
    fn default() -> MpuBar5 {
        MpuBar5(0)
    }
}
#[doc = "Base address register for MPU region 6. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar6(pub u32);
impl MpuBar6 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar6 {
    #[inline(always)]
    fn default() -> MpuBar6 {
        MpuBar6(0)
    }
}
#[doc = "Base address register for MPU region 7. Writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuBar7(pub u32);
impl MpuBar7 {
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "This MPU region matches addresses where addr\\[31:5\\] (the 27 most significant bits) are greater than or equal to BAR_ADDR, and less than or equal to LAR_ADDR. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuBar7 {
    #[inline(always)]
    fn default() -> MpuBar7 {
        MpuBar7(0)
    }
}
#[doc = "Control register for DMA MPU. Accessible only from a Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuCtrl(pub u32);
impl MpuCtrl {
    #[doc = "Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determine whether an address not covered by an active MPU region is Privileged (1) or Unprivileged (0)"]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determine whether an address not covered by an active MPU region is Secure (1) or Non-secure (0)"]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
    #[inline(always)]
    pub const fn ns_hide_addr(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "By default, when a region's S bit is clear, Non-secure-Privileged reads can see the region's base address and limit address. Set this bit to make the addresses appear as 0 to Non-secure reads, even when the region is Non-secure, to avoid leaking information about the processor SAU map."]
    #[inline(always)]
    pub fn set_ns_hide_addr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MpuCtrl {
    #[inline(always)]
    fn default() -> MpuCtrl {
        MpuCtrl(0)
    }
}
#[doc = "Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar0(pub u32);
impl MpuLar0 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar0 {
    #[inline(always)]
    fn default() -> MpuLar0 {
        MpuLar0(0)
    }
}
#[doc = "Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar1(pub u32);
impl MpuLar1 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar1 {
    #[inline(always)]
    fn default() -> MpuLar1 {
        MpuLar1(0)
    }
}
#[doc = "Limit address register for MPU region 2. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar2(pub u32);
impl MpuLar2 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar2 {
    #[inline(always)]
    fn default() -> MpuLar2 {
        MpuLar2(0)
    }
}
#[doc = "Limit address register for MPU region 3. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar3(pub u32);
impl MpuLar3 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar3 {
    #[inline(always)]
    fn default() -> MpuLar3 {
        MpuLar3(0)
    }
}
#[doc = "Limit address register for MPU region 4. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar4(pub u32);
impl MpuLar4 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar4 {
    #[inline(always)]
    fn default() -> MpuLar4 {
        MpuLar4(0)
    }
}
#[doc = "Limit address register for MPU region 5. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar5(pub u32);
impl MpuLar5 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar5 {
    #[inline(always)]
    fn default() -> MpuLar5 {
        MpuLar5(0)
    }
}
#[doc = "Limit address register for MPU region 6. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar6(pub u32);
impl MpuLar6 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar6 {
    #[inline(always)]
    fn default() -> MpuLar6 {
        MpuLar6(0)
    }
}
#[doc = "Limit address register for MPU region 7. Writable only from a Secure, Privileged context, with the exception of the P bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MpuLar7(pub u32);
impl MpuLar7 {
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Region enable. If 1, any address within range specified by the base address (BAR_ADDR) and limit address (LAR_ADDR) has the attributes specified by S and P."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Privileged/Unprivileged (=1/0) status of addresses matching this region, if this region is enabled. Writable from any Privileged context, if and only if the S bit is clear. Otherwise, writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Determines the Secure/Non-secure (=1/0) status of addresses matching this region, if this region is enabled."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x07ff_ffff;
        val as u32
    }
    #[doc = "Limit address bits 31:5. Readable from any Privileged context, if and only if this region's S bit is clear, and MPU_CTRL_NS_HIDE_ADDR is clear. Otherwise readable only from a Secure, Privileged context."]
    #[inline(always)]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x07ff_ffff << 5usize)) | (((val as u32) & 0x07ff_ffff) << 5usize);
    }
}
impl Default for MpuLar7 {
    #[inline(always)]
    fn default() -> MpuLar7 {
        MpuLar7(0)
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
#[doc = "Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh0(pub u32);
impl SeccfgCh0 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh0 {
    #[inline(always)]
    fn default() -> SeccfgCh0 {
        SeccfgCh0(0)
    }
}
#[doc = "Security configuration for channel 1. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh1(pub u32);
impl SeccfgCh1 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh1 {
    #[inline(always)]
    fn default() -> SeccfgCh1 {
        SeccfgCh1(0)
    }
}
#[doc = "Security configuration for channel 10. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh10(pub u32);
impl SeccfgCh10 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh10 {
    #[inline(always)]
    fn default() -> SeccfgCh10 {
        SeccfgCh10(0)
    }
}
#[doc = "Security configuration for channel 11. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh11(pub u32);
impl SeccfgCh11 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh11 {
    #[inline(always)]
    fn default() -> SeccfgCh11 {
        SeccfgCh11(0)
    }
}
#[doc = "Security configuration for channel 12. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh12(pub u32);
impl SeccfgCh12 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh12 {
    #[inline(always)]
    fn default() -> SeccfgCh12 {
        SeccfgCh12(0)
    }
}
#[doc = "Security configuration for channel 13. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh13(pub u32);
impl SeccfgCh13 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh13 {
    #[inline(always)]
    fn default() -> SeccfgCh13 {
        SeccfgCh13(0)
    }
}
#[doc = "Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh14(pub u32);
impl SeccfgCh14 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh14 {
    #[inline(always)]
    fn default() -> SeccfgCh14 {
        SeccfgCh14(0)
    }
}
#[doc = "Security configuration for channel 15. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh15(pub u32);
impl SeccfgCh15 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh15 {
    #[inline(always)]
    fn default() -> SeccfgCh15 {
        SeccfgCh15(0)
    }
}
#[doc = "Security configuration for channel 2. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh2(pub u32);
impl SeccfgCh2 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh2 {
    #[inline(always)]
    fn default() -> SeccfgCh2 {
        SeccfgCh2(0)
    }
}
#[doc = "Security configuration for channel 3. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh3(pub u32);
impl SeccfgCh3 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh3 {
    #[inline(always)]
    fn default() -> SeccfgCh3 {
        SeccfgCh3(0)
    }
}
#[doc = "Security configuration for channel 4. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh4(pub u32);
impl SeccfgCh4 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh4 {
    #[inline(always)]
    fn default() -> SeccfgCh4 {
        SeccfgCh4(0)
    }
}
#[doc = "Security configuration for channel 5. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh5(pub u32);
impl SeccfgCh5 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh5 {
    #[inline(always)]
    fn default() -> SeccfgCh5 {
        SeccfgCh5(0)
    }
}
#[doc = "Security configuration for channel 6. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh6(pub u32);
impl SeccfgCh6 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh6 {
    #[inline(always)]
    fn default() -> SeccfgCh6 {
        SeccfgCh6(0)
    }
}
#[doc = "Security configuration for channel 7. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh7(pub u32);
impl SeccfgCh7 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh7 {
    #[inline(always)]
    fn default() -> SeccfgCh7 {
        SeccfgCh7(0)
    }
}
#[doc = "Security configuration for channel 8. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh8(pub u32);
impl SeccfgCh8 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh8 {
    #[inline(always)]
    fn default() -> SeccfgCh8 {
        SeccfgCh8(0)
    }
}
#[doc = "Security configuration for channel 9. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgCh9(pub u32);
impl SeccfgCh9 {
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged channel. If 1, this channel performs Privileged bus accesses. If 0, it performs Unprivileged bus accesses. If 1, this channel is controllable only from a Privileged context of the same Secure/Non-secure level, or any context of a higher Secure/Non-secure level."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure channel. If 1, this channel performs Secure bus accesses. If 0, it performs Non-secure bus accesses. If 1, this channel is controllable only from a Secure context."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub const fn lock(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "LOCK is 0 at reset, and is set to 1 automatically upon a successful write to this channel's control registers. That is, a write to CTRL, READ_ADDR, WRITE_ADDR, TRANS_COUNT and their aliases. Once its LOCK bit is set, this register becomes read-only. A failed write, for example due to the write's privilege being lower than that specified in the channel's SECCFG register, will not set the LOCK bit."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for SeccfgCh9 {
    #[inline(always)]
    fn default() -> SeccfgCh9 {
        SeccfgCh9(0)
    }
}
#[doc = "Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgIrq0(pub u32);
impl SeccfgIrq0 {
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SeccfgIrq0 {
    #[inline(always)]
    fn default() -> SeccfgIrq0 {
        SeccfgIrq0(0)
    }
}
#[doc = "Security configuration for IRQ 1. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgIrq1(pub u32);
impl SeccfgIrq1 {
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SeccfgIrq1 {
    #[inline(always)]
    fn default() -> SeccfgIrq1 {
        SeccfgIrq1(0)
    }
}
#[doc = "Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgIrq2(pub u32);
impl SeccfgIrq2 {
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SeccfgIrq2 {
    #[inline(always)]
    fn default() -> SeccfgIrq2 {
        SeccfgIrq2(0)
    }
}
#[doc = "Security configuration for IRQ 3. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgIrq3(pub u32);
impl SeccfgIrq3 {
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub const fn p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Privileged IRQ. If 1, this IRQ's control registers can only be accessed from a Privileged context. If 0, this IRQ's control registers can be accessed from an Unprivileged context, but Privileged channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Privileged channels."]
    #[inline(always)]
    pub fn set_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub const fn s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Secure IRQ. If 1, this IRQ's control registers can only be accessed from a Secure context. If 0, this IRQ's control registers can be accessed from a Non-secure context, but Secure channels (as per SECCFG_CHx) are masked from the IRQ status, and this IRQ's registers can not be used to acknowledge the channel interrupts of Secure channels."]
    #[inline(always)]
    pub fn set_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for SeccfgIrq3 {
    #[inline(always)]
    fn default() -> SeccfgIrq3 {
        SeccfgIrq3(0)
    }
}
#[doc = "Miscellaneous security configuration"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeccfgMisc(pub u32);
impl SeccfgMisc {
    #[doc = "If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
    #[inline(always)]
    pub const fn sniff_p(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the sniffer can see data transfers from Privileged channels, and can itself only be accessed from a privileged context, or from a Secure context when SNIFF_S is 0. If 0, the sniffer can be accessed from either a Privileged or Unprivileged context (with sufficient security level) but can not see transfers from Privileged channels."]
    #[inline(always)]
    pub fn set_sniff_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
    #[inline(always)]
    pub const fn sniff_s(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the sniffer can see data transfers from Secure channels, and can itself only be accessed from a Secure context. If 0, the sniffer can be accessed from either a Secure or Non-secure context, but can not see data transfers of Secure channels."]
    #[inline(always)]
    pub fn set_sniff_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub const fn timer0_p(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER0 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 0 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn set_timer0_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
    #[inline(always)]
    pub const fn timer0_s(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER0 register is only accessible from a Secure context, and timer DREQ 0 is only visible to Secure channels."]
    #[inline(always)]
    pub fn set_timer0_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub const fn timer1_p(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER1 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 1 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn set_timer1_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
    #[inline(always)]
    pub const fn timer1_s(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER1 register is only accessible from a Secure context, and timer DREQ 1 is only visible to Secure channels."]
    #[inline(always)]
    pub fn set_timer1_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub const fn timer2_p(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER2 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 2 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn set_timer2_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
    #[inline(always)]
    pub const fn timer2_s(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER2 register is only accessible from a Secure context, and timer DREQ 2 is only visible to Secure channels."]
    #[inline(always)]
    pub fn set_timer2_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub const fn timer3_p(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER3 register is only accessible from a Privileged (or more Secure) context, and timer DREQ 3 is only visible to Privileged (or more Secure) channels."]
    #[inline(always)]
    pub fn set_timer3_p(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
    #[inline(always)]
    pub const fn timer3_s(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the TIMER3 register is only accessible from a Secure context, and timer DREQ 3 is only visible to Secure channels."]
    #[inline(always)]
    pub fn set_timer3_s(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for SeccfgMisc {
    #[inline(always)]
    fn default() -> SeccfgMisc {
        SeccfgMisc(0)
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
#[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
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
