#[doc = "Indicates the type of platform in use"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Platform(pub u32);
impl Platform {
    #[doc = "Indicates the platform is an ASIC"]
    pub const fn asic(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the platform is an ASIC"]
    pub fn set_asic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Indicates the platform is an FPGA"]
    pub const fn fpga(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the platform is an FPGA"]
    pub fn set_fpga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Platform {
    fn default() -> Platform {
        Platform(0)
    }
}
