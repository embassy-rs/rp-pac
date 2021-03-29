use crate::generic::*;
#[doc = "Set the priority of each master for bus arbitration."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BusPriority(pub u32);
impl BusPriority {
    #[doc = "0 - low priority, 1 - high priority"]
    pub const fn dma_w(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub fn set_dma_w(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub const fn dma_r(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub fn set_dma_r(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub const fn proc1(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub fn set_proc1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub const fn proc0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "0 - low priority, 1 - high priority"]
    pub fn set_proc0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for BusPriority {
    fn default() -> BusPriority {
        BusPriority(0)
    }
}
#[doc = "Bus priority acknowledge"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BusPriorityAck(pub u32);
impl BusPriorityAck {
    #[doc = "Goes to 1 once all arbiters have registered the new global priority levels. Arbiters update their local priority when servicing a new nonsequential access. In normal circumstances this will happen almost immediately."]
    pub const fn bus_priority_ack(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Goes to 1 once all arbiters have registered the new global priority levels. Arbiters update their local priority when servicing a new nonsequential access. In normal circumstances this will happen almost immediately."]
    pub fn set_bus_priority_ack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for BusPriorityAck {
    fn default() -> BusPriorityAck {
        BusPriorityAck(0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR3"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfsel(pub u32);
impl Perfsel {
    #[doc = "Select a performance event for PERFCTR3"]
    pub const fn perfsel(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Select a performance event for PERFCTR3"]
    pub fn set_perfsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Perfsel {
    fn default() -> Perfsel {
        Perfsel(0)
    }
}
#[doc = "Bus fabric performance counter 3"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfctr(pub u32);
impl Perfctr {
    #[doc = "Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    pub const fn perfctr(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    pub fn set_perfctr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Perfctr {
    fn default() -> Perfctr {
        Perfctr(0)
    }
}
