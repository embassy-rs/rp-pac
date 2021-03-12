use crate::generic::*;
#[doc = "Indicates the type of platform in use"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Platform(u32);
impl Platform {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Platform {
        Platform(val)
    }
    #[doc = "Indicates the platform is an FPGA"]
    pub const fn fpga(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates the platform is an FPGA"]
    pub fn set_fpga(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Indicates the platform is an ASIC"]
    pub const fn asic(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates the platform is an ASIC"]
    pub fn set_asic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Platform {
    fn default() -> Platform {
        Platform(0)
    }
}
