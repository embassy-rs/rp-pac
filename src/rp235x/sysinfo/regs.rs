#[doc = "JEDEC JEP-106 compliant chip identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId(pub u32);
impl ChipId {
    #[inline(always)]
    pub const fn stop_bit(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_stop_bit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn manufacturer(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_manufacturer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 1usize)) | (((val as u32) & 0x07ff) << 1usize);
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
            .field("stop_bit", &self.stop_bit())
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
            stop_bit: bool,
            manufacturer: u16,
            part: u16,
            revision: u8,
        }
        let proxy = ChipId {
            stop_bit: self.stop_bit(),
            manufacturer: self.manufacturer(),
            part: self.part(),
            revision: self.revision(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PackageSel(pub u32);
impl PackageSel {
    #[inline(always)]
    pub const fn package_sel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_package_sel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for PackageSel {
    #[inline(always)]
    fn default() -> PackageSel {
        PackageSel(0)
    }
}
impl core::fmt::Debug for PackageSel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PackageSel")
            .field("package_sel", &self.package_sel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PackageSel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PackageSel {
            package_sel: bool,
        }
        let proxy = PackageSel {
            package_sel: self.package_sel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in during pre-production development. Post-production, the PLATFORM is always ASIC, non-SIM."]
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
    #[inline(always)]
    pub const fn hdlsim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_hdlsim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn batchsim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_batchsim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn gatesim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gatesim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
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
            .field("hdlsim", &self.hdlsim())
            .field("batchsim", &self.batchsim())
            .field("gatesim", &self.gatesim())
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
            hdlsim: bool,
            batchsim: bool,
            gatesim: bool,
        }
        let proxy = Platform {
            fpga: self.fpga(),
            asic: self.asic(),
            hdlsim: self.hdlsim(),
            batchsim: self.batchsim(),
            gatesim: self.gatesim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
