#[doc = "Set the priority of each master for bus arbitration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusPriority(pub u32);
impl BusPriority {
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub const fn dma_r(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn set_dma_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub const fn dma_w(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    #[inline(always)]
    pub fn set_dma_w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for BusPriority {
    #[inline(always)]
    fn default() -> BusPriority {
        BusPriority(0)
    }
}
#[doc = "Bus priority acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BusPriorityAck(pub u32);
impl BusPriorityAck {
    #[doc = "Goes to 1 once all arbiters have registered the new global priority levels. Arbiters update their local priority when servicing a new nonsequential access. In normal circumstances this will happen almost immediately."]
    #[inline(always)]
    pub const fn bus_priority_ack(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Goes to 1 once all arbiters have registered the new global priority levels. Arbiters update their local priority when servicing a new nonsequential access. In normal circumstances this will happen almost immediately."]
    #[inline(always)]
    pub fn set_bus_priority_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for BusPriorityAck {
    #[inline(always)]
    fn default() -> BusPriorityAck {
        BusPriorityAck(0)
    }
}
#[doc = "Bus fabric performance counter 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perfctr(pub u32);
impl Perfctr {
    #[doc = "Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub const fn perfctr(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    #[inline(always)]
    pub fn set_perfctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Perfctr {
    #[inline(always)]
    fn default() -> Perfctr {
        Perfctr(0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perfsel(pub u32);
impl Perfsel {
    #[doc = "Select an event for PERFCTR1. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub const fn perfsel(&self) -> super::vals::Perfsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Perfsel::from_bits(val as u8)
    }
    #[doc = "Select an event for PERFCTR1. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub fn set_perfsel(&mut self, val: super::vals::Perfsel) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
}
impl Default for Perfsel {
    #[inline(always)]
    fn default() -> Perfsel {
        Perfsel(0)
    }
}
