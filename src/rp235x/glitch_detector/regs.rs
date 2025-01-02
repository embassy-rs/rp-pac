#[doc = "Forcibly arm the glitch detectors, if they are not already armed by OTP. When armed, any individual detector trigger will cause a restart of the switched core power domain's power-on reset state machine. Glitch detector triggers are recorded accumulatively in TRIG_STATUS. If the system is reset by a glitch detector trigger, this is recorded in POWMAN_CHIP_RESET. This register is Secure read/write only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Arm(pub u32);
impl Arm {
    #[inline(always)]
    pub const fn arm(&self) -> super::vals::Arm {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Arm::from_bits(val as u16)
    }
    #[inline(always)]
    pub fn set_arm(&mut self, val: super::vals::Arm) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Arm {
    #[inline(always)]
    fn default() -> Arm {
        Arm(0)
    }
}
impl core::fmt::Debug for Arm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Arm").field("arm", &self.arm()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Arm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Arm {
            arm: super::vals::Arm,
        }
        let proxy = Arm { arm: self.arm() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Disarm(pub u32);
impl Disarm {
    #[doc = "Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn disarm(&self) -> super::vals::Disarm {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::Disarm::from_bits(val as u16)
    }
    #[doc = "Forcibly disarm the glitch detectors, if they are armed by OTP. Ignored if ARM is YES. This register is Secure read/write only."]
    #[inline(always)]
    pub fn set_disarm(&mut self, val: super::vals::Disarm) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u32) & 0xffff) << 0usize);
    }
}
impl Default for Disarm {
    #[inline(always)]
    fn default() -> Disarm {
        Disarm(0)
    }
}
impl core::fmt::Debug for Disarm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Disarm")
            .field("disarm", &self.disarm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Disarm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Disarm {
            disarm: super::vals::Disarm,
        }
        let proxy = Disarm {
            disarm: self.disarm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lock(pub u32);
impl Lock {
    #[doc = "Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only."]
    #[inline(always)]
    pub const fn lock(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Write any nonzero value to disable writes to ARM, DISARM, SENSITIVITY and LOCK. This register is Secure read/write only."]
    #[inline(always)]
    pub fn set_lock(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Lock {
    #[inline(always)]
    fn default() -> Lock {
        Lock(0)
    }
}
impl core::fmt::Debug for Lock {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Lock").field("lock", &self.lock()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Lock {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Lock {
            lock: u8,
        }
        let proxy = Lock { lock: self.lock() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Adjust the sensitivity of glitch detectors to values other than their OTP-provided defaults. This register is Secure read/write only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sensitivity(pub u32);
impl Sensitivity {
    #[doc = "Set sensitivity for detector 0. Higher values are more sensitive."]
    #[inline(always)]
    pub const fn det0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "Set sensitivity for detector 0. Higher values are more sensitive."]
    #[inline(always)]
    pub fn set_det0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Set sensitivity for detector 1. Higher values are more sensitive."]
    #[inline(always)]
    pub const fn det1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "Set sensitivity for detector 1. Higher values are more sensitive."]
    #[inline(always)]
    pub fn set_det1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Set sensitivity for detector 2. Higher values are more sensitive."]
    #[inline(always)]
    pub const fn det2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Set sensitivity for detector 2. Higher values are more sensitive."]
    #[inline(always)]
    pub fn set_det2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "Set sensitivity for detector 3. Higher values are more sensitive."]
    #[inline(always)]
    pub const fn det3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[doc = "Set sensitivity for detector 3. Higher values are more sensitive."]
    #[inline(always)]
    pub fn set_det3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[doc = "Must be the inverse of DET0, else the default value is used."]
    #[inline(always)]
    pub const fn det0_inv(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Must be the inverse of DET0, else the default value is used."]
    #[inline(always)]
    pub fn set_det0_inv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "Must be the inverse of DET1, else the default value is used."]
    #[inline(always)]
    pub const fn det1_inv(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[doc = "Must be the inverse of DET1, else the default value is used."]
    #[inline(always)]
    pub fn set_det1_inv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[doc = "Must be the inverse of DET2, else the default value is used."]
    #[inline(always)]
    pub const fn det2_inv(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[doc = "Must be the inverse of DET2, else the default value is used."]
    #[inline(always)]
    pub fn set_det2_inv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[doc = "Must be the inverse of DET3, else the default value is used."]
    #[inline(always)]
    pub const fn det3_inv(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[doc = "Must be the inverse of DET3, else the default value is used."]
    #[inline(always)]
    pub fn set_det3_inv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[inline(always)]
    pub const fn default(&self) -> super::vals::Default {
        let val = (self.0 >> 24usize) & 0xff;
        super::vals::Default::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_default(&mut self, val: super::vals::Default) {
        self.0 = (self.0 & !(0xff << 24usize)) | (((val.to_bits() as u32) & 0xff) << 24usize);
    }
}
impl Default for Sensitivity {
    #[inline(always)]
    fn default() -> Sensitivity {
        Sensitivity(0)
    }
}
impl core::fmt::Debug for Sensitivity {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Sensitivity")
            .field("det0", &self.det0())
            .field("det1", &self.det1())
            .field("det2", &self.det2())
            .field("det3", &self.det3())
            .field("det0_inv", &self.det0_inv())
            .field("det1_inv", &self.det1_inv())
            .field("det2_inv", &self.det2_inv())
            .field("det3_inv", &self.det3_inv())
            .field("default", &self.default())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Sensitivity {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Sensitivity {
            det0: u8,
            det1: u8,
            det2: u8,
            det3: u8,
            det0_inv: u8,
            det1_inv: u8,
            det2_inv: u8,
            det3_inv: u8,
            default: super::vals::Default,
        }
        let proxy = Sensitivity {
            det0: self.det0(),
            det1: self.det1(),
            det2: self.det2(),
            det3: self.det3(),
            det0_inv: self.det0_inv(),
            det1_inv: self.det1_inv(),
            det2_inv: self.det2_inv(),
            det3_inv: self.det3_inv(),
            default: self.default(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Simulate the firing of one or more detectors. Writing ones to this register will set the matching bits in STATUS_TRIG. If the glitch detectors are currently armed, writing ones will also immediately reset the switched core power domain, and set the reset reason latches in POWMAN_CHIP_RESET to indicate a glitch detector resets. This register is Secure read/write only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigForce(pub u32);
impl TrigForce {
    #[inline(always)]
    pub const fn trig_force(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_trig_force(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for TrigForce {
    #[inline(always)]
    fn default() -> TrigForce {
        TrigForce(0)
    }
}
impl core::fmt::Debug for TrigForce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigForce")
            .field("trig_force", &self.trig_force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigForce {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrigForce {
            trig_force: u8,
        }
        let proxy = TrigForce {
            trig_force: self.trig_force(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set when a detector output triggers. Write-1-clear. (May immediately return high if the detector remains in a failed state. Detectors can only be cleared by a full reset of the switched core power domain.) This register is Secure read/write only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TrigStatus(pub u32);
impl TrigStatus {
    #[inline(always)]
    pub const fn det0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_det0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn det1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_det1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn det2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_det2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn det3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_det3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for TrigStatus {
    #[inline(always)]
    fn default() -> TrigStatus {
        TrigStatus(0)
    }
}
impl core::fmt::Debug for TrigStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TrigStatus")
            .field("det0", &self.det0())
            .field("det1", &self.det1())
            .field("det2", &self.det2())
            .field("det3", &self.det3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for TrigStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct TrigStatus {
            det0: bool,
            det1: bool,
            det2: bool,
            det3: bool,
        }
        let proxy = TrigStatus {
            det0: self.det0(),
            det1: self.det1(),
            det2: self.det2(),
            det3: self.det3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
