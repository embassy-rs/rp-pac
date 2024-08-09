#[doc = "NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NmiMask1(pub u32);
impl NmiMask1 {
    #[inline(always)]
    pub const fn nmi_mask1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_nmi_mask1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for NmiMask1 {
    #[inline(always)]
    fn default() -> NmiMask1 {
        NmiMask1(0)
    }
}
#[doc = "Nonstandard sleep control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sleepctrl(pub u32);
impl Sleepctrl {
    #[doc = "By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
    #[inline(always)]
    pub const fn light_sleep(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "By default, any processor sleep will deassert the system-level clock request. Reenabling the clocks incurs 5 cycles of additional latency on wakeup. Setting LIGHT_SLEEP to 1 keeps the clock request asserted during a normal sleep (Arm SCR.SLEEPDEEP = 0), for faster wakeup. Processor deep sleep (Arm SCR.SLEEPDEEP = 1) is not affected, and will always deassert the system-level clock request."]
    #[inline(always)]
    pub fn set_light_sleep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
    #[inline(always)]
    pub const fn wicenreq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Request that the next processor deep sleep is a WIC sleep. After setting this bit, before sleeping, poll WICENACK to ensure the processor interrupt controller has acknowledged the change."]
    #[inline(always)]
    pub fn set_wicenreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK."]
    #[inline(always)]
    pub const fn wicenack(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Status signal from the processor's interrupt controller. Changes to WICENREQ are eventually reflected in WICENACK."]
    #[inline(always)]
    pub fn set_wicenack(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Sleepctrl {
    #[inline(always)]
    fn default() -> Sleepctrl {
        Sleepctrl(0)
    }
}
