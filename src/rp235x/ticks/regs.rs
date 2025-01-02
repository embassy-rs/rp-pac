#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0count(pub u32);
impl Proc0count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn proc0_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_proc0_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc0count {
    #[inline(always)]
    fn default() -> Proc0count {
        Proc0count(0)
    }
}
impl core::fmt::Debug for Proc0count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0count")
            .field("proc0_count", &self.proc0_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0count {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc0count {
            proc0_count: u16,
        }
        let proxy = Proc0count {
            proc0_count: self.proc0_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0ctrl(pub u32);
impl Proc0ctrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Proc0ctrl {
    #[inline(always)]
    fn default() -> Proc0ctrl {
        Proc0ctrl(0)
    }
}
impl core::fmt::Debug for Proc0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc0ctrl {
            enable: bool,
            running: bool,
        }
        let proxy = Proc0ctrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc0cycles(pub u32);
impl Proc0cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn proc0_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_proc0_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc0cycles {
    #[inline(always)]
    fn default() -> Proc0cycles {
        Proc0cycles(0)
    }
}
impl core::fmt::Debug for Proc0cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc0cycles")
            .field("proc0_cycles", &self.proc0_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc0cycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc0cycles {
            proc0_cycles: u16,
        }
        let proxy = Proc0cycles {
            proc0_cycles: self.proc0_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1count(pub u32);
impl Proc1count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn proc1_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_proc1_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc1count {
    #[inline(always)]
    fn default() -> Proc1count {
        Proc1count(0)
    }
}
impl core::fmt::Debug for Proc1count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1count")
            .field("proc1_count", &self.proc1_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1count {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc1count {
            proc1_count: u16,
        }
        let proxy = Proc1count {
            proc1_count: self.proc1_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1ctrl(pub u32);
impl Proc1ctrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Proc1ctrl {
    #[inline(always)]
    fn default() -> Proc1ctrl {
        Proc1ctrl(0)
    }
}
impl core::fmt::Debug for Proc1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc1ctrl {
            enable: bool,
            running: bool,
        }
        let proxy = Proc1ctrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Proc1cycles(pub u32);
impl Proc1cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn proc1_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_proc1_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Proc1cycles {
    #[inline(always)]
    fn default() -> Proc1cycles {
        Proc1cycles(0)
    }
}
impl core::fmt::Debug for Proc1cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Proc1cycles")
            .field("proc1_cycles", &self.proc1_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Proc1cycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Proc1cycles {
            proc1_cycles: u16,
        }
        let proxy = Proc1cycles {
            proc1_cycles: self.proc1_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCount(pub u32);
impl RiscvCount {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn riscv_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_riscv_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for RiscvCount {
    #[inline(always)]
    fn default() -> RiscvCount {
        RiscvCount(0)
    }
}
impl core::fmt::Debug for RiscvCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCount")
            .field("riscv_count", &self.riscv_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCount {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RiscvCount {
            riscv_count: u16,
        }
        let proxy = RiscvCount {
            riscv_count: self.riscv_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCtrl(pub u32);
impl RiscvCtrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for RiscvCtrl {
    #[inline(always)]
    fn default() -> RiscvCtrl {
        RiscvCtrl(0)
    }
}
impl core::fmt::Debug for RiscvCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCtrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RiscvCtrl {
            enable: bool,
            running: bool,
        }
        let proxy = RiscvCtrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvCycles(pub u32);
impl RiscvCycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn riscv_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_riscv_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for RiscvCycles {
    #[inline(always)]
    fn default() -> RiscvCycles {
        RiscvCycles(0)
    }
}
impl core::fmt::Debug for RiscvCycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RiscvCycles")
            .field("riscv_cycles", &self.riscv_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for RiscvCycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct RiscvCycles {
            riscv_cycles: u16,
        }
        let proxy = RiscvCycles {
            riscv_cycles: self.riscv_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0count(pub u32);
impl Timer0count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn timer0_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_timer0_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer0count {
    #[inline(always)]
    fn default() -> Timer0count {
        Timer0count(0)
    }
}
impl core::fmt::Debug for Timer0count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0count")
            .field("timer0_count", &self.timer0_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0count {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer0count {
            timer0_count: u16,
        }
        let proxy = Timer0count {
            timer0_count: self.timer0_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0ctrl(pub u32);
impl Timer0ctrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Timer0ctrl {
    #[inline(always)]
    fn default() -> Timer0ctrl {
        Timer0ctrl(0)
    }
}
impl core::fmt::Debug for Timer0ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer0ctrl {
            enable: bool,
            running: bool,
        }
        let proxy = Timer0ctrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer0cycles(pub u32);
impl Timer0cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn timer0_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_timer0_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer0cycles {
    #[inline(always)]
    fn default() -> Timer0cycles {
        Timer0cycles(0)
    }
}
impl core::fmt::Debug for Timer0cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer0cycles")
            .field("timer0_cycles", &self.timer0_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer0cycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer0cycles {
            timer0_cycles: u16,
        }
        let proxy = Timer0cycles {
            timer0_cycles: self.timer0_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1count(pub u32);
impl Timer1count {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn timer1_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_timer1_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer1count {
    #[inline(always)]
    fn default() -> Timer1count {
        Timer1count(0)
    }
}
impl core::fmt::Debug for Timer1count {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1count")
            .field("timer1_count", &self.timer1_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1count {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer1count {
            timer1_count: u16,
        }
        let proxy = Timer1count {
            timer1_count: self.timer1_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1ctrl(pub u32);
impl Timer1ctrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Timer1ctrl {
    #[inline(always)]
    fn default() -> Timer1ctrl {
        Timer1ctrl(0)
    }
}
impl core::fmt::Debug for Timer1ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1ctrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer1ctrl {
            enable: bool,
            running: bool,
        }
        let proxy = Timer1ctrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer1cycles(pub u32);
impl Timer1cycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn timer1_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_timer1_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for Timer1cycles {
    #[inline(always)]
    fn default() -> Timer1cycles {
        Timer1cycles(0)
    }
}
impl core::fmt::Debug for Timer1cycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Timer1cycles")
            .field("timer1_cycles", &self.timer1_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Timer1cycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Timer1cycles {
            timer1_cycles: u16,
        }
        let proxy = Timer1cycles {
            timer1_cycles: self.timer1_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCount(pub u32);
impl WatchdogCount {
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub const fn watchdog_count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Count down timer: the remaining number clk_tick cycles before the next tick is generated."]
    #[inline(always)]
    pub fn set_watchdog_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for WatchdogCount {
    #[inline(always)]
    fn default() -> WatchdogCount {
        WatchdogCount(0)
    }
}
impl core::fmt::Debug for WatchdogCount {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCount")
            .field("watchdog_count", &self.watchdog_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCount {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WatchdogCount {
            watchdog_count: u16,
        }
        let proxy = WatchdogCount {
            watchdog_count: self.watchdog_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Controls the tick generator"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCtrl(pub u32);
impl WatchdogCtrl {
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "start / stop tick generation"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Is the tick generator running?"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for WatchdogCtrl {
    #[inline(always)]
    fn default() -> WatchdogCtrl {
        WatchdogCtrl(0)
    }
}
impl core::fmt::Debug for WatchdogCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCtrl")
            .field("enable", &self.enable())
            .field("running", &self.running())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WatchdogCtrl {
            enable: bool,
            running: bool,
        }
        let proxy = WatchdogCtrl {
            enable: self.enable(),
            running: self.running(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WatchdogCycles(pub u32);
impl WatchdogCycles {
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub const fn watchdog_cycles(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "Total number of clk_tick cycles before the next tick."]
    #[inline(always)]
    pub fn set_watchdog_cycles(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
    }
}
impl Default for WatchdogCycles {
    #[inline(always)]
    fn default() -> WatchdogCycles {
        WatchdogCycles(0)
    }
}
impl core::fmt::Debug for WatchdogCycles {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WatchdogCycles")
            .field("watchdog_cycles", &self.watchdog_cycles())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WatchdogCycles {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WatchdogCycles {
            watchdog_cycles: u16,
        }
        let proxy = WatchdogCycles {
            watchdog_cycles: self.watchdog_cycles(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
