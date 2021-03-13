use crate::generic::*;
#[doc = "Bus fabric performance event select for PERFCTR0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfsel0(pub u32);
impl Perfsel0 {
    #[doc = "Select a performance event for PERFCTR0"]
    pub const fn perfsel0(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Select a performance event for PERFCTR0"]
    pub fn set_perfsel0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Perfsel0 {
    fn default() -> Perfsel0 {
        Perfsel0(0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR3"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfsel3(pub u32);
impl Perfsel3 {
    #[doc = "Select a performance event for PERFCTR3"]
    pub const fn perfsel3(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Select a performance event for PERFCTR3"]
    pub fn set_perfsel3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Perfsel3 {
    fn default() -> Perfsel3 {
        Perfsel3(0)
    }
}
#[doc = "Bus fabric performance counter 3"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfctr3(pub u32);
impl Perfctr3 {
    #[doc = "Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    pub const fn perfctr3(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 3 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL3"]
    pub fn set_perfctr3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Perfctr3 {
    fn default() -> Perfctr3 {
        Perfctr3(0)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR2"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfsel2(pub u32);
impl Perfsel2 {
    #[doc = "Select a performance event for PERFCTR2"]
    pub const fn perfsel2(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Select a performance event for PERFCTR2"]
    pub fn set_perfsel2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Perfsel2 {
    fn default() -> Perfsel2 {
        Perfsel2(0)
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
#[doc = "Bus fabric performance event select for PERFCTR1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfsel1(pub u32);
impl Perfsel1 {
    #[doc = "Select a performance event for PERFCTR1"]
    pub const fn perfsel1(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Select a performance event for PERFCTR1"]
    pub fn set_perfsel1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Perfsel1 {
    fn default() -> Perfsel1 {
        Perfsel1(0)
    }
}
#[doc = "Bus fabric performance counter 2"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfctr2(pub u32);
impl Perfctr2 {
    #[doc = "Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
    pub const fn perfctr2(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 2 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL2"]
    pub fn set_perfctr2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Perfctr2 {
    fn default() -> Perfctr2 {
        Perfctr2(0)
    }
}
#[doc = "Bus fabric performance counter 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfctr0(pub u32);
impl Perfctr0 {
    #[doc = "Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    pub const fn perfctr0(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 0 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL0"]
    pub fn set_perfctr0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Perfctr0 {
    fn default() -> Perfctr0 {
        Perfctr0(0)
    }
}
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
#[doc = "Bus fabric performance counter 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Perfctr1(pub u32);
impl Perfctr1 {
    #[doc = "Busfabric saturating performance counter 1 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL1"]
    pub const fn perfctr1(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Busfabric saturating performance counter 1 Count some event signal from the busfabric arbiters. Write any value to clear. Select an event to count using PERFSEL1"]
    pub fn set_perfctr1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Perfctr1 {
    fn default() -> Perfctr1 {
        Perfctr1(0)
    }
}
