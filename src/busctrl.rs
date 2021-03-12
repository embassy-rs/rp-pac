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
    pub fn bus_priority(self) -> Reg<regs::BusPriority, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Bus priority acknowledge"]
    pub fn bus_priority_ack(self) -> Reg<regs::BusPriorityAck, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Bus fabric performance counter 0"]
    pub fn perfctr0(self) -> Reg<regs::Perfctr0, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR0"]
    pub fn perfsel0(self) -> Reg<regs::Perfsel0, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Bus fabric performance counter 1"]
    pub fn perfctr1(self) -> Reg<regs::Perfctr1, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR1"]
    pub fn perfsel1(self) -> Reg<regs::Perfsel1, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Bus fabric performance counter 2"]
    pub fn perfctr2(self) -> Reg<regs::Perfctr2, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR2"]
    pub fn perfsel2(self) -> Reg<regs::Perfsel2, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Bus fabric performance counter 3"]
    pub fn perfctr3(self) -> Reg<regs::Perfctr3, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR3"]
    pub fn perfsel3(self) -> Reg<regs::Perfsel3, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
}
pub mod regs;
