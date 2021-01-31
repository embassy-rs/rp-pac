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
        unsafe { Reg::new(self.0.add(0usize), 0) }
    }
    #[doc = "Write to bits 31:0 of time writes do not get copied to time until timehw is written"]
    pub fn timelw(self) -> Reg<u32, W> {
        unsafe { Reg::new(self.0.add(4usize), 0) }
    }
    #[doc = "Read from bits 63:32 of time always read timelr before timehr"]
    pub fn timehr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(8usize), 0) }
    }
    #[doc = "Read from bits 31:0 of time"]
    pub fn timelr(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(12usize), 0) }
    }
    #[doc = "Arm alarm 0, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM0 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(16usize), 0) }
    }
    #[doc = "Arm alarm 1, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM1 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(20usize), 0) }
    }
    #[doc = "Arm alarm 2, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM2 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(24usize), 0) }
    }
    #[doc = "Arm alarm 3, and configure the time it will fire. Once armed, the alarm fires when TIMER_ALARM3 == TIMELR. The alarm will disarm itself once it fires, and can be disarmed early using the ARMED status register."]
    pub fn alarm3(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(28usize), 0) }
    }
    #[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
    pub fn armed(self) -> Reg<fields::Armed, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Armed::from_bits(0)) }
    }
    #[doc = "Raw read from bits 63:32 of time (no side effects)"]
    pub fn timerawh(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(36usize), 0) }
    }
    #[doc = "Raw read from bits 31:0 of time (no side effects)"]
    pub fn timerawl(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(40usize), 0) }
    }
    #[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
    pub fn dbgpause(self) -> Reg<fields::Dbgpause, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Dbgpause::from_bits(7)) }
    }
    #[doc = "Set high to pause the timer"]
    pub fn pause(self) -> Reg<fields::Pause, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Pause::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
