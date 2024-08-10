#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ticks {
    ptr: *mut u8,
}
unsafe impl Send for Ticks {}
unsafe impl Sync for Ticks {}
impl Ticks {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn proc0_ctrl(self) -> crate::common::Reg<regs::Proc0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[inline(always)]
    pub const fn proc0_cycles(self) -> crate::common::Reg<regs::Proc0cycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[inline(always)]
    pub const fn proc0_count(self) -> crate::common::Reg<regs::Proc0count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn proc1_ctrl(self) -> crate::common::Reg<regs::Proc1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[inline(always)]
    pub const fn proc1_cycles(self) -> crate::common::Reg<regs::Proc1cycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[inline(always)]
    pub const fn proc1_count(self) -> crate::common::Reg<regs::Proc1count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn timer0_ctrl(self) -> crate::common::Reg<regs::Timer0ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[inline(always)]
    pub const fn timer0_cycles(self) -> crate::common::Reg<regs::Timer0cycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[inline(always)]
    pub const fn timer0_count(self) -> crate::common::Reg<regs::Timer0count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn timer1_ctrl(self) -> crate::common::Reg<regs::Timer1ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[inline(always)]
    pub const fn timer1_cycles(self) -> crate::common::Reg<regs::Timer1cycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[inline(always)]
    pub const fn timer1_count(self) -> crate::common::Reg<regs::Timer1count, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn watchdog_ctrl(self) -> crate::common::Reg<regs::WatchdogCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[inline(always)]
    pub const fn watchdog_cycles(
        self,
    ) -> crate::common::Reg<regs::WatchdogCycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[inline(always)]
    pub const fn watchdog_count(
        self,
    ) -> crate::common::Reg<regs::WatchdogCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn riscv_ctrl(self) -> crate::common::Reg<regs::RiscvCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[inline(always)]
    pub const fn riscv_cycles(self) -> crate::common::Reg<regs::RiscvCycles, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[inline(always)]
    pub const fn riscv_count(self) -> crate::common::Reg<regs::RiscvCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
}
pub mod regs;
