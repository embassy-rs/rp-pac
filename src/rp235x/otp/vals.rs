#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArchselCore0 {
    #[doc = "Switch core 0 to Arm (Cortex-M33)"]
    ARM = 0x0,
    #[doc = "Switch core 0 to RISC-V (Hazard3)"]
    RISCV = 0x01,
}
impl ArchselCore0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArchselCore0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArchselCore0 {
    #[inline(always)]
    fn from(val: u8) -> ArchselCore0 {
        ArchselCore0::from_bits(val)
    }
}
impl From<ArchselCore0> for u8 {
    #[inline(always)]
    fn from(val: ArchselCore0) -> u8 {
        ArchselCore0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArchselCore1 {
    #[doc = "Switch core 1 to Arm (Cortex-M33)"]
    ARM = 0x0,
    #[doc = "Switch core 1 to RISC-V (Hazard3)"]
    RISCV = 0x01,
}
impl ArchselCore1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArchselCore1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArchselCore1 {
    #[inline(always)]
    fn from(val: u8) -> ArchselCore1 {
        ArchselCore1::from_bits(val)
    }
}
impl From<ArchselCore1> for u8 {
    #[inline(always)]
    fn from(val: ArchselCore1) -> u8 {
        ArchselCore1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArchselStatusCore0 {
    #[doc = "Core 0 is currently Arm (Cortex-M33)"]
    ARM = 0x0,
    #[doc = "Core 0 is currently RISC-V (Hazard3)"]
    RISCV = 0x01,
}
impl ArchselStatusCore0 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArchselStatusCore0 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArchselStatusCore0 {
    #[inline(always)]
    fn from(val: u8) -> ArchselStatusCore0 {
        ArchselStatusCore0::from_bits(val)
    }
}
impl From<ArchselStatusCore0> for u8 {
    #[inline(always)]
    fn from(val: ArchselStatusCore0) -> u8 {
        ArchselStatusCore0::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum ArchselStatusCore1 {
    #[doc = "Core 1 is currently Arm (Cortex-M33)"]
    ARM = 0x0,
    #[doc = "Core 1 is currently RISC-V (Hazard3)"]
    RISCV = 0x01,
}
impl ArchselStatusCore1 {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> ArchselStatusCore1 {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for ArchselStatusCore1 {
    #[inline(always)]
    fn from(val: u8) -> ArchselStatusCore1 {
        ArchselStatusCore1::from_bits(val)
    }
}
impl From<ArchselStatusCore1> for u8 {
    #[inline(always)]
    fn from(val: ArchselStatusCore1) -> u8 {
        ArchselStatusCore1::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwLockNsec {
    READ_WRITE = 0x0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLockNsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLockNsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLockNsec {
    #[inline(always)]
    fn from(val: u8) -> SwLockNsec {
        SwLockNsec::from_bits(val)
    }
}
impl From<SwLockNsec> for u8 {
    #[inline(always)]
    fn from(val: SwLockNsec) -> u8 {
        SwLockNsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum SwLockSec {
    READ_WRITE = 0x0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLockSec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLockSec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLockSec {
    #[inline(always)]
    fn from(val: u8) -> SwLockSec {
        SwLockSec::from_bits(val)
    }
}
impl From<SwLockSec> for u8 {
    #[inline(always)]
    fn from(val: SwLockSec) -> u8 {
        SwLockSec::to_bits(val)
    }
}
