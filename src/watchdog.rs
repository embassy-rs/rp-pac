use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Watchdog(*mut u8);
unsafe impl Send for Watchdog {}
unsafe impl Sync for Watchdog {}
impl Watchdog {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    pub fn load(self) -> Reg<regs::Load, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    pub fn reason(self) -> Reg<regs::Reason, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch3(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch4(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch5(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch6(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch7(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Controls the tick generator"]
    pub fn tick(self) -> Reg<regs::Tick, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
}
pub mod regs;
