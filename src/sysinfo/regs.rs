#[doc = "Platform register. Allows software to know what environment it is running in."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Platform(pub u32);
impl Platform {
    pub const fn fpga(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    pub fn set_fpga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    pub const fn asic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    pub fn set_asic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Platform {
    fn default() -> Platform {
        Platform(0)
    }
}
#[doc = "JEDEC JEP-106 compliant chip identifier."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipId(pub u32);
impl ChipId {
    pub const fn manufacturer(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    pub fn set_manufacturer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    pub const fn part(&self) -> u16 {
        let val = (self.0 >> 12usize) & 0xffff;
        val as u16
    }
    pub fn set_part(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12usize)) | (((val as u32) & 0xffff) << 12usize);
    }
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for ChipId {
    fn default() -> ChipId {
        ChipId(0)
    }
}
