#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Watchdog {
    ptr: *mut u8,
}
unsafe impl Send for Watchdog {}
unsafe impl Sync for Watchdog {}
impl Watchdog {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    #[inline(always)]
    pub const fn load(self) -> crate::common::Reg<regs::Load, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    #[inline(always)]
    pub const fn reason(self) -> crate::common::Reg<regs::Reason, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Controls the tick generator"]
    #[inline(always)]
    pub const fn tick(self) -> crate::common::Reg<regs::Tick, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
}
pub mod regs;
