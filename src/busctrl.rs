#[doc = "Register block for busfabric control signals and performance counters"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busctrl(pub *mut u8);
unsafe impl Send for Busctrl {}
unsafe impl Sync for Busctrl {}
impl Busctrl {
    #[doc = "Set the priority of each master for bus arbitration."]
    #[inline(always)]
    pub fn bus_priority(self) -> crate::common::Reg<regs::BusPriority, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Bus priority acknowledge"]
    #[inline(always)]
    pub fn bus_priority_ack(self) -> crate::common::Reg<regs::BusPriorityAck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Bus fabric performance counter 0"]
    #[inline(always)]
    pub fn perfctr(self, n: usize) -> crate::common::Reg<regs::Perfctr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize + n * 8usize)) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR0"]
    #[inline(always)]
    pub fn perfsel(self, n: usize) -> crate::common::Reg<regs::Perfsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize + n * 8usize)) }
    }
}
pub mod regs;
pub mod vals;
