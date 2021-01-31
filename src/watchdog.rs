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
    pub fn ctrl(self) -> Reg<fields::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Ctrl::from_bits(117440512)) }
    }
    #[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
    pub fn load(self) -> Reg<fields::Load, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Load::from_bits(0)) }
    }
    #[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
    pub fn reason(self) -> Reg<fields::Reason, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Reason::from_bits(0)) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(12usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(16usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(20usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch3(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(24usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch4(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(28usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch5(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(32usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch6(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(36usize), 0) }
    }
    #[doc = "Scratch register. Information persists through soft reset of the chip."]
    pub fn scratch7(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(40usize), 0) }
    }
    #[doc = "Controls the tick generator"]
    pub fn tick(self) -> Reg<fields::Tick, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Tick::from_bits(512)) }
    }
}
pub mod fields;
