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
impl core::fmt::Debug for BusPriority {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BusPriority")
            .field("proc0", &self.proc0())
            .field("proc1", &self.proc1())
            .field("dma_r", &self.dma_r())
            .field("dma_w", &self.dma_w())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusPriority {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BusPriority {
            proc0: bool,
            proc1: bool,
            dma_r: bool,
            dma_w: bool,
        }
        let proxy = BusPriority {
            proc0: self.proc0(),
            proc1: self.proc1(),
            dma_r: self.dma_r(),
            dma_w: self.dma_w(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for BusPriorityAck {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BusPriorityAck")
            .field("bus_priority_ack", &self.bus_priority_ack())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BusPriorityAck {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BusPriorityAck {
            bus_priority_ack: bool,
        }
        let proxy = BusPriorityAck {
            bus_priority_ack: self.bus_priority_ack(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Perfctr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Perfctr")
            .field("perfctr", &self.perfctr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Perfctr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Perfctr {
            perfctr: u32,
        }
        let proxy = Perfctr {
            perfctr: self.perfctr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Bus fabric performance event select for PERFCTR0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Perfsel(pub u32);
impl Perfsel {
    #[doc = "Select an event for PERFCTR0. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
    #[inline(always)]
    pub const fn perfsel(&self) -> super::vals::Perfsel {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::Perfsel::from_bits(val as u8)
    }
    #[doc = "Select an event for PERFCTR0. Count either contested accesses, or all accesses, on a downstream port of the main crossbar."]
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
impl core::fmt::Debug for Perfsel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Perfsel")
            .field("perfsel", &self.perfsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Perfsel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Perfsel {
            perfsel: super::vals::Perfsel,
        }
        let proxy = Perfsel {
            perfsel: self.perfsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
