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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to approximately 16 seconds."]
    #[inline(always)]
    pub const fn load(self) -> crate::common::Reg<regs::Load, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset. Additionally, as of RP2350, a debugger warm reset of either core (SYSRESETREQ or hartreset) will also clear the watchdog reason register, so that software loaded under the debugger following a watchdog timeout will not continue to see the timeout condition."]
    #[inline(always)]
    pub const fn reason(self) -> crate::common::Reg<regs::Reason, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    #[inline(always)]
    pub const fn scratch7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
}
pub mod regs;
