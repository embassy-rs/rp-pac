use crate::generic::*;
#[doc = "JEDEC JEP-106 compliant chip identifier."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ChipId(pub u32);
impl ChipId {
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 28u32) & 0x0f;
        val as u8
    }
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28u32)) | (((val as u32) & 0x0f) << 28u32);
    }
    pub const fn part(&self) -> u16 {
        let val = (self.0 >> 12u32) & 0xffff;
        val as u16
    }
    pub fn set_part(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 12u32)) | (((val as u32) & 0xffff) << 12u32);
    }
    pub const fn manufacturer(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x0fff;
        val as u16
    }
    pub fn set_manufacturer(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val as u32) & 0x0fff) << 0u32);
    }
}
impl Default for ChipId {
    fn default() -> ChipId {
        ChipId(0)
    }
}
#[doc = "Platform register. Allows software to know what environment it is running in."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Platform(pub u32);
impl Platform {
    pub const fn asic(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_asic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn fpga(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_fpga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Platform {
    fn default() -> Platform {
        Platform(0)
    }
}
