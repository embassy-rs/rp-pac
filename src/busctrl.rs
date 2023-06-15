#[doc = "Register block for busfabric control signals and performance counters"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Busctrl {
    ptr: *mut u8,
}
unsafe impl Send for Busctrl {}
unsafe impl Sync for Busctrl {}
impl Busctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Set the priority of each master for bus arbitration."]
    #[inline(always)]
    pub const fn bus_priority(self) -> crate::common::Reg<regs::BusPriority, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Bus priority acknowledge"]
    #[inline(always)]
    pub const fn bus_priority_ack(
        self,
    ) -> crate::common::Reg<regs::BusPriorityAck, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Bus fabric performance counter 0"]
    #[inline(always)]
    pub const fn perfctr(self, n: usize) -> crate::common::Reg<regs::Perfctr, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize + n * 8usize) as _) }
    }
    #[doc = "Bus fabric performance event select for PERFCTR0"]
    #[inline(always)]
    pub const fn perfsel(self, n: usize) -> crate::common::Reg<regs::Perfsel, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize + n * 8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
