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
