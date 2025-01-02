#[doc = "JEDEC JEP-106 compliant chip identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId(pub u32);
impl ChipId {
    #[inline(always)]
    pub const fn manufacturer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[inline(always)]
    pub fn set_manufacturer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[inline(always)]
    pub const fn part(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_part(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ChipId {
    #[inline(always)]
    fn default() -> ChipId {
        ChipId(0)
    }
}
impl core::fmt::Debug for ChipId {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChipId")
            .field("manufacturer", &self.manufacturer())
            .field("part", &self.part())
            .field("revision", &self.revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChipId {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChipId {
            manufacturer: u16,
            part: u16,
            revision: u8,
        }
        let proxy = ChipId {
            manufacturer: self.manufacturer(),
            part: self.part(),
            revision: self.revision(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Platform(pub u32);
impl Platform {
    #[inline(always)]
    pub const fn fpga(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_fpga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn asic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_asic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Platform {
    #[inline(always)]
    fn default() -> Platform {
        Platform(0)
    }
}
impl core::fmt::Debug for Platform {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Platform")
            .field("fpga", &self.fpga())
            .field("asic", &self.asic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Platform {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Platform {
            fpga: bool,
            asic: bool,
        }
        let proxy = Platform {
            fpga: self.fpga(),
            asic: self.asic(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
