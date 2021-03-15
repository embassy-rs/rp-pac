use crate::generic::*;
#[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Trigger a watchdog reset"]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Trigger a watchdog reset"]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    pub const fn pause_dbg1(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    pub fn set_pause_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    pub const fn pause_dbg0(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    pub fn set_pause_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    pub const fn pause_jtag(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    pub fn set_pause_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    pub const fn time(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    pub fn set_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Ctrl {
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Tick(pub u32);
impl Tick {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 11u32) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 11u32)) | (((val as u32) & 0x01ff) << 11u32);
    }
    #[doc = "Is the tick generator running?"]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "start / stop tick generation"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0u32)) | (((val as u32) & 0x01ff) << 0u32);
    }
}
impl Default for Tick {
    fn default() -> Tick {
        Tick(0)
    }
}
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Load(pub u32);
impl Load {
    pub const fn load(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    pub fn set_load(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Load {
    fn default() -> Load {
        Load(0)
    }
}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Reason(pub u32);
impl Reason {
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Reason {
    fn default() -> Reason {
        Reason(0)
    }
}
