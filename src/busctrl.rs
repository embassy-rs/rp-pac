use crate::generic::*;
#[doc = "Register block for busfabric control signals and performance counters"]
#[derive(Copy, Clone)]
pub struct Busctrl(*mut u8);
unsafe impl Send for Busctrl {}
unsafe impl Sync for Busctrl {}
impl Busctrl {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Set the priority of each master for bus arbitration."]
    pub fn bus_priority(self) -> Reg<fields::BusPriority, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::BusPriority::from_bits(0)) }
    }
    #[doc = "Bus priority acknowledge"]
    pub fn bus_priority_ack(self) -> Reg<fields::BusPriorityAck, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::BusPriorityAck::from_bits(0)) }
    }
    #[doc = "Bus fabric performance counter 0"]
    pub fn perfctr0(self) -> Reg<fields::Perfctr0, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Perfctr0::from_bits(0)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR0"]
    pub fn perfsel0(self) -> Reg<fields::Perfsel0, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Perfsel0::from_bits(31)) }
    }
    #[doc = "Bus fabric performance counter 1"]
    pub fn perfctr1(self) -> Reg<fields::Perfctr1, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Perfctr1::from_bits(0)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR1"]
    pub fn perfsel1(self) -> Reg<fields::Perfsel1, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Perfsel1::from_bits(31)) }
    }
    #[doc = "Bus fabric performance counter 2"]
    pub fn perfctr2(self) -> Reg<fields::Perfctr2, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Perfctr2::from_bits(0)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR2"]
    pub fn perfsel2(self) -> Reg<fields::Perfsel2, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Perfsel2::from_bits(31)) }
    }
    #[doc = "Bus fabric performance counter 3"]
    pub fn perfctr3(self) -> Reg<fields::Perfctr3, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Perfctr3::from_bits(0)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR3"]
    pub fn perfsel3(self) -> Reg<fields::Perfsel3, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Perfsel3::from_bits(31)) }
    }
}
pub mod fields;
