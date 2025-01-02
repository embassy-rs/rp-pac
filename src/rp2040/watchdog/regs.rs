#[doc = "Watchdog control The rst_wdsel register determines which subsystems are reset when the watchdog is triggered. The watchdog can be triggered in software."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    #[inline(always)]
    pub const fn time(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Indicates the number of ticks / 2 (see errata RP2040-E1) before a watchdog reset will be triggered"]
    #[inline(always)]
    pub fn set_time(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub const fn pause_jtag(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when JTAG is accessing the bus fabric"]
    #[inline(always)]
    pub fn set_pause_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub const fn pause_dbg0(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn set_pause_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub const fn pause_dbg1(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Pause the watchdog timer when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn set_pause_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When not enabled the watchdog timer is paused"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Trigger a watchdog reset"]
    #[inline(always)]
    pub const fn trigger(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger a watchdog reset"]
    #[inline(always)]
    pub fn set_trigger(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("time", &self.time())
            .field("pause_jtag", &self.pause_jtag())
            .field("pause_dbg0", &self.pause_dbg0())
            .field("pause_dbg1", &self.pause_dbg1())
            .field("enable", &self.enable())
            .field("trigger", &self.trigger())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            time: u32,
            pause_jtag: bool,
            pause_dbg0: bool,
            pause_dbg1: bool,
            enable: bool,
            trigger: bool,
        }
        let proxy = Ctrl {
            time: self.time(),
            pause_jtag: self.pause_jtag(),
            pause_dbg0: self.pause_dbg0(),
            pause_dbg1: self.pause_dbg1(),
            enable: self.enable(),
            trigger: self.trigger(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Load the watchdog timer. The maximum setting is 0xffffff which corresponds to 0xffffff / 2 ticks before triggering a watchdog reset (see errata RP2040-E1)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Load(pub u32);
impl Load {
    #[inline(always)]
    pub const fn load(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_load(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Load {
    #[inline(always)]
    fn default() -> Load {
        Load(0)
    }
}
impl core::fmt::Debug for Load {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Load").field("load", &self.load()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Load {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Load {
            load: u32,
        }
        let proxy = Load { load: self.load() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Logs the reason for the last reset. Both bits are zero for the case of a hardware reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Reason(pub u32);
impl Reason {
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn force(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_force(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Reason {
    #[inline(always)]
    fn default() -> Reason {
        Reason(0)
    }
}
impl core::fmt::Debug for Reason {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Reason")
            .field("timer", &self.timer())
            .field("force", &self.force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Reason {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Reason {
            timer: bool,
            force: bool,
        }
        let proxy = Reason {
            timer: self.timer(),
            force: self.force(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tick(pub u32);
impl Tick {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 11usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 11usize)) | (((val as u32) & 0x01ff) << 11usize);
    }
}
impl Default for Tick {
    #[inline(always)]
    fn default() -> Tick {
        Tick(0)
    }
}
impl core::fmt::Debug for Tick {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Tick")
            .field("cycles", &self.cycles())
            .field("enable", &self.enable())
            .field("running", &self.running())
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Tick {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Tick {
            cycles: u16,
            enable: bool,
            running: bool,
            count: u16,
        }
        let proxy = Tick {
            cycles: self.cycles(),
            enable: self.enable(),
            running: self.running(),
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
