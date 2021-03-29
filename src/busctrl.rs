use crate::generic::*;
#[doc = "Register block for busfabric control signals and performance counters"]
#[derive(Copy, Clone)]
pub struct Busctrl(pub *mut u8);
unsafe impl Send for Busctrl {}
unsafe impl Sync for Busctrl {}
impl Busctrl {
    #[doc = "Set the priority of each master for bus arbitration."]
    pub fn bus_priority(self) -> Reg<regs::BusPriority, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Bus priority acknowledge"]
    pub fn bus_priority_ack(self) -> Reg<regs::BusPriorityAck, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR0"]
    pub fn perfsel(self, n: usize) -> Reg<regs::Perfsel, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(12usize + n * 8usize)) }
    }
    #[doc = "Bus fabric performance counter 0"]
    pub fn perfctr(self, n: usize) -> Reg<regs::Perfctr, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(8usize + n * 8usize)) }
    }
}
pub mod regs;
