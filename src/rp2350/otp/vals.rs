#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArchselCore0 {
    #[doc = "Switch core 0 to Arm (Cortex-M33)"]
    ARM = 0,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArchselCore1 {
    #[doc = "Switch core 1 to Arm (Cortex-M33)"]
    ARM = 0,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArchselStatusCore0 {
    #[doc = "Core 0 is currently Arm (Cortex-M33)"]
    ARM = 0,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum ArchselStatusCore1 {
    #[doc = "Core 1 is currently Arm (Cortex-M33)"]
    ARM = 0,
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
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock0nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock0nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock0nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock0nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock0nsec {
        SwLock0nsec::from_bits(val)
    }
}
impl From<SwLock0nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock0nsec) -> u8 {
        SwLock0nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock0sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock0sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock0sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock0sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock0sec {
        SwLock0sec::from_bits(val)
    }
}
impl From<SwLock0sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock0sec) -> u8 {
        SwLock0sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock10nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock10nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock10nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock10nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock10nsec {
        SwLock10nsec::from_bits(val)
    }
}
impl From<SwLock10nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock10nsec) -> u8 {
        SwLock10nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock10sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock10sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock10sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock10sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock10sec {
        SwLock10sec::from_bits(val)
    }
}
impl From<SwLock10sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock10sec) -> u8 {
        SwLock10sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock11nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock11nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock11nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock11nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock11nsec {
        SwLock11nsec::from_bits(val)
    }
}
impl From<SwLock11nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock11nsec) -> u8 {
        SwLock11nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock11sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock11sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock11sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock11sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock11sec {
        SwLock11sec::from_bits(val)
    }
}
impl From<SwLock11sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock11sec) -> u8 {
        SwLock11sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock12nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock12nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock12nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock12nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock12nsec {
        SwLock12nsec::from_bits(val)
    }
}
impl From<SwLock12nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock12nsec) -> u8 {
        SwLock12nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock12sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock12sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock12sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock12sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock12sec {
        SwLock12sec::from_bits(val)
    }
}
impl From<SwLock12sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock12sec) -> u8 {
        SwLock12sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock13nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock13nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock13nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock13nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock13nsec {
        SwLock13nsec::from_bits(val)
    }
}
impl From<SwLock13nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock13nsec) -> u8 {
        SwLock13nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock13sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock13sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock13sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock13sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock13sec {
        SwLock13sec::from_bits(val)
    }
}
impl From<SwLock13sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock13sec) -> u8 {
        SwLock13sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock14nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock14nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock14nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock14nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock14nsec {
        SwLock14nsec::from_bits(val)
    }
}
impl From<SwLock14nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock14nsec) -> u8 {
        SwLock14nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock14sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock14sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock14sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock14sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock14sec {
        SwLock14sec::from_bits(val)
    }
}
impl From<SwLock14sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock14sec) -> u8 {
        SwLock14sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock15nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock15nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock15nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock15nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock15nsec {
        SwLock15nsec::from_bits(val)
    }
}
impl From<SwLock15nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock15nsec) -> u8 {
        SwLock15nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock15sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock15sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock15sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock15sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock15sec {
        SwLock15sec::from_bits(val)
    }
}
impl From<SwLock15sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock15sec) -> u8 {
        SwLock15sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock16nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock16nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock16nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock16nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock16nsec {
        SwLock16nsec::from_bits(val)
    }
}
impl From<SwLock16nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock16nsec) -> u8 {
        SwLock16nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock16sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock16sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock16sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock16sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock16sec {
        SwLock16sec::from_bits(val)
    }
}
impl From<SwLock16sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock16sec) -> u8 {
        SwLock16sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock17nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock17nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock17nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock17nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock17nsec {
        SwLock17nsec::from_bits(val)
    }
}
impl From<SwLock17nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock17nsec) -> u8 {
        SwLock17nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock17sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock17sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock17sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock17sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock17sec {
        SwLock17sec::from_bits(val)
    }
}
impl From<SwLock17sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock17sec) -> u8 {
        SwLock17sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock18nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock18nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock18nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock18nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock18nsec {
        SwLock18nsec::from_bits(val)
    }
}
impl From<SwLock18nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock18nsec) -> u8 {
        SwLock18nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock18sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock18sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock18sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock18sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock18sec {
        SwLock18sec::from_bits(val)
    }
}
impl From<SwLock18sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock18sec) -> u8 {
        SwLock18sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock19nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock19nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock19nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock19nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock19nsec {
        SwLock19nsec::from_bits(val)
    }
}
impl From<SwLock19nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock19nsec) -> u8 {
        SwLock19nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock19sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock19sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock19sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock19sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock19sec {
        SwLock19sec::from_bits(val)
    }
}
impl From<SwLock19sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock19sec) -> u8 {
        SwLock19sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock1nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock1nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock1nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock1nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock1nsec {
        SwLock1nsec::from_bits(val)
    }
}
impl From<SwLock1nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock1nsec) -> u8 {
        SwLock1nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock1sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock1sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock1sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock1sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock1sec {
        SwLock1sec::from_bits(val)
    }
}
impl From<SwLock1sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock1sec) -> u8 {
        SwLock1sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock20nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock20nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock20nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock20nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock20nsec {
        SwLock20nsec::from_bits(val)
    }
}
impl From<SwLock20nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock20nsec) -> u8 {
        SwLock20nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock20sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock20sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock20sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock20sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock20sec {
        SwLock20sec::from_bits(val)
    }
}
impl From<SwLock20sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock20sec) -> u8 {
        SwLock20sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock21nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock21nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock21nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock21nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock21nsec {
        SwLock21nsec::from_bits(val)
    }
}
impl From<SwLock21nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock21nsec) -> u8 {
        SwLock21nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock21sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock21sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock21sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock21sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock21sec {
        SwLock21sec::from_bits(val)
    }
}
impl From<SwLock21sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock21sec) -> u8 {
        SwLock21sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock22nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock22nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock22nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock22nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock22nsec {
        SwLock22nsec::from_bits(val)
    }
}
impl From<SwLock22nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock22nsec) -> u8 {
        SwLock22nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock22sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock22sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock22sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock22sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock22sec {
        SwLock22sec::from_bits(val)
    }
}
impl From<SwLock22sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock22sec) -> u8 {
        SwLock22sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock23nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock23nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock23nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock23nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock23nsec {
        SwLock23nsec::from_bits(val)
    }
}
impl From<SwLock23nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock23nsec) -> u8 {
        SwLock23nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock23sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock23sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock23sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock23sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock23sec {
        SwLock23sec::from_bits(val)
    }
}
impl From<SwLock23sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock23sec) -> u8 {
        SwLock23sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock24nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock24nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock24nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock24nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock24nsec {
        SwLock24nsec::from_bits(val)
    }
}
impl From<SwLock24nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock24nsec) -> u8 {
        SwLock24nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock24sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock24sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock24sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock24sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock24sec {
        SwLock24sec::from_bits(val)
    }
}
impl From<SwLock24sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock24sec) -> u8 {
        SwLock24sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock25nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock25nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock25nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock25nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock25nsec {
        SwLock25nsec::from_bits(val)
    }
}
impl From<SwLock25nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock25nsec) -> u8 {
        SwLock25nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock25sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock25sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock25sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock25sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock25sec {
        SwLock25sec::from_bits(val)
    }
}
impl From<SwLock25sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock25sec) -> u8 {
        SwLock25sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock26nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock26nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock26nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock26nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock26nsec {
        SwLock26nsec::from_bits(val)
    }
}
impl From<SwLock26nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock26nsec) -> u8 {
        SwLock26nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock26sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock26sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock26sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock26sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock26sec {
        SwLock26sec::from_bits(val)
    }
}
impl From<SwLock26sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock26sec) -> u8 {
        SwLock26sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock27nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock27nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock27nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock27nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock27nsec {
        SwLock27nsec::from_bits(val)
    }
}
impl From<SwLock27nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock27nsec) -> u8 {
        SwLock27nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock27sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock27sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock27sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock27sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock27sec {
        SwLock27sec::from_bits(val)
    }
}
impl From<SwLock27sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock27sec) -> u8 {
        SwLock27sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock28nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock28nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock28nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock28nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock28nsec {
        SwLock28nsec::from_bits(val)
    }
}
impl From<SwLock28nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock28nsec) -> u8 {
        SwLock28nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock28sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock28sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock28sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock28sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock28sec {
        SwLock28sec::from_bits(val)
    }
}
impl From<SwLock28sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock28sec) -> u8 {
        SwLock28sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock29nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock29nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock29nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock29nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock29nsec {
        SwLock29nsec::from_bits(val)
    }
}
impl From<SwLock29nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock29nsec) -> u8 {
        SwLock29nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock29sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock29sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock29sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock29sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock29sec {
        SwLock29sec::from_bits(val)
    }
}
impl From<SwLock29sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock29sec) -> u8 {
        SwLock29sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock2nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock2nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock2nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock2nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock2nsec {
        SwLock2nsec::from_bits(val)
    }
}
impl From<SwLock2nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock2nsec) -> u8 {
        SwLock2nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock2sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock2sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock2sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock2sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock2sec {
        SwLock2sec::from_bits(val)
    }
}
impl From<SwLock2sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock2sec) -> u8 {
        SwLock2sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock30nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock30nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock30nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock30nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock30nsec {
        SwLock30nsec::from_bits(val)
    }
}
impl From<SwLock30nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock30nsec) -> u8 {
        SwLock30nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock30sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock30sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock30sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock30sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock30sec {
        SwLock30sec::from_bits(val)
    }
}
impl From<SwLock30sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock30sec) -> u8 {
        SwLock30sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock31nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock31nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock31nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock31nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock31nsec {
        SwLock31nsec::from_bits(val)
    }
}
impl From<SwLock31nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock31nsec) -> u8 {
        SwLock31nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock31sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock31sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock31sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock31sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock31sec {
        SwLock31sec::from_bits(val)
    }
}
impl From<SwLock31sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock31sec) -> u8 {
        SwLock31sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock32nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock32nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock32nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock32nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock32nsec {
        SwLock32nsec::from_bits(val)
    }
}
impl From<SwLock32nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock32nsec) -> u8 {
        SwLock32nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock32sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock32sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock32sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock32sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock32sec {
        SwLock32sec::from_bits(val)
    }
}
impl From<SwLock32sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock32sec) -> u8 {
        SwLock32sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock33nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock33nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock33nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock33nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock33nsec {
        SwLock33nsec::from_bits(val)
    }
}
impl From<SwLock33nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock33nsec) -> u8 {
        SwLock33nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock33sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock33sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock33sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock33sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock33sec {
        SwLock33sec::from_bits(val)
    }
}
impl From<SwLock33sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock33sec) -> u8 {
        SwLock33sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock34nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock34nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock34nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock34nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock34nsec {
        SwLock34nsec::from_bits(val)
    }
}
impl From<SwLock34nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock34nsec) -> u8 {
        SwLock34nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock34sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock34sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock34sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock34sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock34sec {
        SwLock34sec::from_bits(val)
    }
}
impl From<SwLock34sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock34sec) -> u8 {
        SwLock34sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock35nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock35nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock35nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock35nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock35nsec {
        SwLock35nsec::from_bits(val)
    }
}
impl From<SwLock35nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock35nsec) -> u8 {
        SwLock35nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock35sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock35sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock35sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock35sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock35sec {
        SwLock35sec::from_bits(val)
    }
}
impl From<SwLock35sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock35sec) -> u8 {
        SwLock35sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock36nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock36nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock36nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock36nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock36nsec {
        SwLock36nsec::from_bits(val)
    }
}
impl From<SwLock36nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock36nsec) -> u8 {
        SwLock36nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock36sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock36sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock36sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock36sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock36sec {
        SwLock36sec::from_bits(val)
    }
}
impl From<SwLock36sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock36sec) -> u8 {
        SwLock36sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock37nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock37nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock37nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock37nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock37nsec {
        SwLock37nsec::from_bits(val)
    }
}
impl From<SwLock37nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock37nsec) -> u8 {
        SwLock37nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock37sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock37sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock37sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock37sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock37sec {
        SwLock37sec::from_bits(val)
    }
}
impl From<SwLock37sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock37sec) -> u8 {
        SwLock37sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock38nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock38nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock38nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock38nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock38nsec {
        SwLock38nsec::from_bits(val)
    }
}
impl From<SwLock38nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock38nsec) -> u8 {
        SwLock38nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock38sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock38sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock38sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock38sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock38sec {
        SwLock38sec::from_bits(val)
    }
}
impl From<SwLock38sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock38sec) -> u8 {
        SwLock38sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock39nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock39nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock39nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock39nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock39nsec {
        SwLock39nsec::from_bits(val)
    }
}
impl From<SwLock39nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock39nsec) -> u8 {
        SwLock39nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock39sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock39sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock39sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock39sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock39sec {
        SwLock39sec::from_bits(val)
    }
}
impl From<SwLock39sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock39sec) -> u8 {
        SwLock39sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock3nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock3nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock3nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock3nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock3nsec {
        SwLock3nsec::from_bits(val)
    }
}
impl From<SwLock3nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock3nsec) -> u8 {
        SwLock3nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock3sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock3sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock3sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock3sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock3sec {
        SwLock3sec::from_bits(val)
    }
}
impl From<SwLock3sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock3sec) -> u8 {
        SwLock3sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock40nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock40nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock40nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock40nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock40nsec {
        SwLock40nsec::from_bits(val)
    }
}
impl From<SwLock40nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock40nsec) -> u8 {
        SwLock40nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock40sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock40sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock40sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock40sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock40sec {
        SwLock40sec::from_bits(val)
    }
}
impl From<SwLock40sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock40sec) -> u8 {
        SwLock40sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock41nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock41nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock41nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock41nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock41nsec {
        SwLock41nsec::from_bits(val)
    }
}
impl From<SwLock41nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock41nsec) -> u8 {
        SwLock41nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock41sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock41sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock41sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock41sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock41sec {
        SwLock41sec::from_bits(val)
    }
}
impl From<SwLock41sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock41sec) -> u8 {
        SwLock41sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock42nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock42nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock42nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock42nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock42nsec {
        SwLock42nsec::from_bits(val)
    }
}
impl From<SwLock42nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock42nsec) -> u8 {
        SwLock42nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock42sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock42sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock42sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock42sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock42sec {
        SwLock42sec::from_bits(val)
    }
}
impl From<SwLock42sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock42sec) -> u8 {
        SwLock42sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock43nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock43nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock43nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock43nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock43nsec {
        SwLock43nsec::from_bits(val)
    }
}
impl From<SwLock43nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock43nsec) -> u8 {
        SwLock43nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock43sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock43sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock43sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock43sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock43sec {
        SwLock43sec::from_bits(val)
    }
}
impl From<SwLock43sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock43sec) -> u8 {
        SwLock43sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock44nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock44nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock44nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock44nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock44nsec {
        SwLock44nsec::from_bits(val)
    }
}
impl From<SwLock44nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock44nsec) -> u8 {
        SwLock44nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock44sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock44sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock44sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock44sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock44sec {
        SwLock44sec::from_bits(val)
    }
}
impl From<SwLock44sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock44sec) -> u8 {
        SwLock44sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock45nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock45nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock45nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock45nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock45nsec {
        SwLock45nsec::from_bits(val)
    }
}
impl From<SwLock45nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock45nsec) -> u8 {
        SwLock45nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock45sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock45sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock45sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock45sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock45sec {
        SwLock45sec::from_bits(val)
    }
}
impl From<SwLock45sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock45sec) -> u8 {
        SwLock45sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock46nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock46nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock46nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock46nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock46nsec {
        SwLock46nsec::from_bits(val)
    }
}
impl From<SwLock46nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock46nsec) -> u8 {
        SwLock46nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock46sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock46sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock46sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock46sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock46sec {
        SwLock46sec::from_bits(val)
    }
}
impl From<SwLock46sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock46sec) -> u8 {
        SwLock46sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock47nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock47nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock47nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock47nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock47nsec {
        SwLock47nsec::from_bits(val)
    }
}
impl From<SwLock47nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock47nsec) -> u8 {
        SwLock47nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock47sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock47sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock47sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock47sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock47sec {
        SwLock47sec::from_bits(val)
    }
}
impl From<SwLock47sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock47sec) -> u8 {
        SwLock47sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock48nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock48nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock48nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock48nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock48nsec {
        SwLock48nsec::from_bits(val)
    }
}
impl From<SwLock48nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock48nsec) -> u8 {
        SwLock48nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock48sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock48sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock48sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock48sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock48sec {
        SwLock48sec::from_bits(val)
    }
}
impl From<SwLock48sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock48sec) -> u8 {
        SwLock48sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock49nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock49nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock49nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock49nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock49nsec {
        SwLock49nsec::from_bits(val)
    }
}
impl From<SwLock49nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock49nsec) -> u8 {
        SwLock49nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock49sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock49sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock49sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock49sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock49sec {
        SwLock49sec::from_bits(val)
    }
}
impl From<SwLock49sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock49sec) -> u8 {
        SwLock49sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock4nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock4nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock4nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock4nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock4nsec {
        SwLock4nsec::from_bits(val)
    }
}
impl From<SwLock4nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock4nsec) -> u8 {
        SwLock4nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock4sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock4sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock4sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock4sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock4sec {
        SwLock4sec::from_bits(val)
    }
}
impl From<SwLock4sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock4sec) -> u8 {
        SwLock4sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock50nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock50nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock50nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock50nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock50nsec {
        SwLock50nsec::from_bits(val)
    }
}
impl From<SwLock50nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock50nsec) -> u8 {
        SwLock50nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock50sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock50sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock50sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock50sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock50sec {
        SwLock50sec::from_bits(val)
    }
}
impl From<SwLock50sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock50sec) -> u8 {
        SwLock50sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock51nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock51nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock51nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock51nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock51nsec {
        SwLock51nsec::from_bits(val)
    }
}
impl From<SwLock51nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock51nsec) -> u8 {
        SwLock51nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock51sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock51sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock51sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock51sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock51sec {
        SwLock51sec::from_bits(val)
    }
}
impl From<SwLock51sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock51sec) -> u8 {
        SwLock51sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock52nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock52nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock52nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock52nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock52nsec {
        SwLock52nsec::from_bits(val)
    }
}
impl From<SwLock52nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock52nsec) -> u8 {
        SwLock52nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock52sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock52sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock52sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock52sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock52sec {
        SwLock52sec::from_bits(val)
    }
}
impl From<SwLock52sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock52sec) -> u8 {
        SwLock52sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock53nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock53nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock53nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock53nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock53nsec {
        SwLock53nsec::from_bits(val)
    }
}
impl From<SwLock53nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock53nsec) -> u8 {
        SwLock53nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock53sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock53sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock53sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock53sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock53sec {
        SwLock53sec::from_bits(val)
    }
}
impl From<SwLock53sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock53sec) -> u8 {
        SwLock53sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock54nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock54nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock54nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock54nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock54nsec {
        SwLock54nsec::from_bits(val)
    }
}
impl From<SwLock54nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock54nsec) -> u8 {
        SwLock54nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock54sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock54sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock54sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock54sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock54sec {
        SwLock54sec::from_bits(val)
    }
}
impl From<SwLock54sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock54sec) -> u8 {
        SwLock54sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock55nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock55nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock55nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock55nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock55nsec {
        SwLock55nsec::from_bits(val)
    }
}
impl From<SwLock55nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock55nsec) -> u8 {
        SwLock55nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock55sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock55sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock55sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock55sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock55sec {
        SwLock55sec::from_bits(val)
    }
}
impl From<SwLock55sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock55sec) -> u8 {
        SwLock55sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock56nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock56nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock56nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock56nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock56nsec {
        SwLock56nsec::from_bits(val)
    }
}
impl From<SwLock56nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock56nsec) -> u8 {
        SwLock56nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock56sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock56sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock56sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock56sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock56sec {
        SwLock56sec::from_bits(val)
    }
}
impl From<SwLock56sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock56sec) -> u8 {
        SwLock56sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock57nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock57nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock57nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock57nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock57nsec {
        SwLock57nsec::from_bits(val)
    }
}
impl From<SwLock57nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock57nsec) -> u8 {
        SwLock57nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock57sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock57sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock57sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock57sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock57sec {
        SwLock57sec::from_bits(val)
    }
}
impl From<SwLock57sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock57sec) -> u8 {
        SwLock57sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock58nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock58nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock58nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock58nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock58nsec {
        SwLock58nsec::from_bits(val)
    }
}
impl From<SwLock58nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock58nsec) -> u8 {
        SwLock58nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock58sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock58sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock58sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock58sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock58sec {
        SwLock58sec::from_bits(val)
    }
}
impl From<SwLock58sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock58sec) -> u8 {
        SwLock58sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock59nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock59nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock59nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock59nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock59nsec {
        SwLock59nsec::from_bits(val)
    }
}
impl From<SwLock59nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock59nsec) -> u8 {
        SwLock59nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock59sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock59sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock59sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock59sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock59sec {
        SwLock59sec::from_bits(val)
    }
}
impl From<SwLock59sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock59sec) -> u8 {
        SwLock59sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock5nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock5nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock5nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock5nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock5nsec {
        SwLock5nsec::from_bits(val)
    }
}
impl From<SwLock5nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock5nsec) -> u8 {
        SwLock5nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock5sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock5sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock5sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock5sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock5sec {
        SwLock5sec::from_bits(val)
    }
}
impl From<SwLock5sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock5sec) -> u8 {
        SwLock5sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock60nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock60nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock60nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock60nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock60nsec {
        SwLock60nsec::from_bits(val)
    }
}
impl From<SwLock60nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock60nsec) -> u8 {
        SwLock60nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock60sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock60sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock60sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock60sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock60sec {
        SwLock60sec::from_bits(val)
    }
}
impl From<SwLock60sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock60sec) -> u8 {
        SwLock60sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock61nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock61nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock61nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock61nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock61nsec {
        SwLock61nsec::from_bits(val)
    }
}
impl From<SwLock61nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock61nsec) -> u8 {
        SwLock61nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock61sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock61sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock61sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock61sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock61sec {
        SwLock61sec::from_bits(val)
    }
}
impl From<SwLock61sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock61sec) -> u8 {
        SwLock61sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock62nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock62nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock62nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock62nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock62nsec {
        SwLock62nsec::from_bits(val)
    }
}
impl From<SwLock62nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock62nsec) -> u8 {
        SwLock62nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock62sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock62sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock62sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock62sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock62sec {
        SwLock62sec::from_bits(val)
    }
}
impl From<SwLock62sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock62sec) -> u8 {
        SwLock62sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock63nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock63nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock63nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock63nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock63nsec {
        SwLock63nsec::from_bits(val)
    }
}
impl From<SwLock63nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock63nsec) -> u8 {
        SwLock63nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock63sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock63sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock63sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock63sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock63sec {
        SwLock63sec::from_bits(val)
    }
}
impl From<SwLock63sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock63sec) -> u8 {
        SwLock63sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock6nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock6nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock6nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock6nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock6nsec {
        SwLock6nsec::from_bits(val)
    }
}
impl From<SwLock6nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock6nsec) -> u8 {
        SwLock6nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock6sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock6sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock6sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock6sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock6sec {
        SwLock6sec::from_bits(val)
    }
}
impl From<SwLock6sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock6sec) -> u8 {
        SwLock6sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock7nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock7nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock7nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock7nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock7nsec {
        SwLock7nsec::from_bits(val)
    }
}
impl From<SwLock7nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock7nsec) -> u8 {
        SwLock7nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock7sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock7sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock7sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock7sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock7sec {
        SwLock7sec::from_bits(val)
    }
}
impl From<SwLock7sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock7sec) -> u8 {
        SwLock7sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock8nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock8nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock8nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock8nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock8nsec {
        SwLock8nsec::from_bits(val)
    }
}
impl From<SwLock8nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock8nsec) -> u8 {
        SwLock8nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock8sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock8sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock8sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock8sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock8sec {
        SwLock8sec::from_bits(val)
    }
}
impl From<SwLock8sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock8sec) -> u8 {
        SwLock8sec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock9nsec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock9nsec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock9nsec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock9nsec {
    #[inline(always)]
    fn from(val: u8) -> SwLock9nsec {
        SwLock9nsec::from_bits(val)
    }
}
impl From<SwLock9nsec> for u8 {
    #[inline(always)]
    fn from(val: SwLock9nsec) -> u8 {
        SwLock9nsec::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SwLock9sec {
    READ_WRITE = 0,
    READ_ONLY = 0x01,
    _RESERVED_2 = 0x02,
    INACCESSIBLE = 0x03,
}
impl SwLock9sec {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SwLock9sec {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SwLock9sec {
    #[inline(always)]
    fn from(val: u8) -> SwLock9sec {
        SwLock9sec::from_bits(val)
    }
}
impl From<SwLock9sec> for u8 {
    #[inline(always)]
    fn from(val: SwLock9sec) -> u8 {
        SwLock9sec::to_bits(val)
    }
}
