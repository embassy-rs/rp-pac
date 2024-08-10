#[doc = "Glitch detector controls"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GlitchDetector {
    ptr: *mut u8,
}
unsafe impl Send for GlitchDetector {}
unsafe impl Sync for GlitchDetector {}
impl GlitchDetector {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn arm(self) -> crate::common::Reg<regs::Arm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[inline(always)]
    pub const fn disarm(self) -> crate::common::Reg<regs::Disarm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn sensitivity(self) -> crate::common::Reg<regs::Sensitivity, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only."]
    #[inline(always)]
    pub const fn trig_status(self) -> crate::common::Reg<regs::TrigStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn trig_force(self) -> crate::common::Reg<regs::TrigForce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
}
pub mod regs;
pub mod vals;
