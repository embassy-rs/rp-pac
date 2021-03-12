use crate::generic::*;
#[doc = "Controls time and alarms time is a 64 bit value indicating the time in usec since power-on timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr An alarm is set by setting alarm_enable and writing to the corresponding alarm register When an alarm is pending, the corresponding alarm_running signal will be high An alarm can be cancelled before it has finished by clearing the alarm_enable When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared To clear the interrupt write a 1 to the corresponding alarm_irq"]
#[derive(Copy, Clone)]
pub struct Timer(*mut u8);
unsafe impl Send for Timer {}
unsafe impl Sync for Timer {}
impl Timer {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Write to bits 63:32 of time always write timelw before timehw"]
    pub fn timehw(self) -> Reg<u32, W> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Write to bits 31:0 of time writes do not get copied to time until timehw is written"]
    pub fn timelw(self) -> Reg<u32, W> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Read from bits 63:32 of time always read timelr before timehr"]
    pub fn timehr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Read from bits 31:0 of time"]
    pub fn timelr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm3(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
    pub fn armed(self) -> Reg<regs::Armed, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Raw read from bits 63:32 of time (no side effects)"]
    pub fn timerawh(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Raw read from bits 31:0 of time (no side effects)"]
    pub fn timerawl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
    pub fn dbgpause(self) -> Reg<regs::Dbgpause, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Set high to pause the timer"]
    pub fn pause(self) -> Reg<regs::Pause, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Inte, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Intf, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Ints, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
}
pub mod regs;
