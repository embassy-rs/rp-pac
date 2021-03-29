use crate::generic::*;
#[doc = "Chip reset control and status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ChipReset(pub u32);
impl ChipReset {
    #[doc = "This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    pub const fn psm_restart_flag(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "This is set by psm_restart from the debugger. Its purpose is to branch bootcode to a safe mode when the debugger has issued a psm_restart in order to recover from a boot lock-up. In the safe mode the debugger can repair the boot code, clear this flag then reboot the processor."]
    pub fn set_psm_restart_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Last reset was from the debug port"]
    pub const fn had_psm_restart(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the debug port"]
    pub fn set_had_psm_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "Last reset was from the RUN pin"]
    pub const fn had_run(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the RUN pin"]
    pub fn set_had_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Last reset was from the power-on reset or brown-out detection blocks"]
    pub const fn had_por(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the power-on reset or brown-out detection blocks"]
    pub fn set_had_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
impl Default for ChipReset {
    fn default() -> ChipReset {
        ChipReset(0)
    }
}
#[doc = "Voltage regulator control and status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Vreg(pub u32);
impl Vreg {
    #[doc = "regulation status 0=not in regulation, 1=in regulation"]
    pub const fn rok(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "regulation status 0=not in regulation, 1=in regulation"]
    pub fn set_rok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output voltage select 0000 to 0101 - 0.80V 0110 - 0.85V 0111 - 0.90V 1000 - 0.95V 1001 - 1.00V 1010 - 1.05V 1011 - 1.10V (default) 1100 - 1.15V 1101 - 1.20V 1110 - 1.25V 1111 - 1.30V"]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x0f;
        val as u8
    }
    #[doc = "output voltage select 0000 to 0101 - 0.80V 0110 - 0.85V 0111 - 0.90V 1000 - 0.95V 1001 - 1.00V 1010 - 1.05V 1011 - 1.10V (default) 1100 - 1.15V 1101 - 1.20V 1110 - 1.25V 1111 - 1.30V"]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4u32)) | (((val as u32) & 0x0f) << 4u32);
    }
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "enable 0=not enabled, 1=enabled"]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "enable 0=not enabled, 1=enabled"]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Vreg {
    fn default() -> Vreg {
        Vreg(0)
    }
}
#[doc = "brown-out detection control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Bod(pub u32);
impl Bod {
    #[doc = "threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x0f;
        val as u8
    }
    #[doc = "threshold select 0000 - 0.473V 0001 - 0.516V 0010 - 0.559V 0011 - 0.602V 0100 - 0.645V 0101 - 0.688V 0110 - 0.731V 0111 - 0.774V 1000 - 0.817V 1001 - 0.860V (default) 1010 - 0.903V 1011 - 0.946V 1100 - 0.989V 1101 - 1.032V 1110 - 1.075V 1111 - 1.118V"]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4u32)) | (((val as u32) & 0x0f) << 4u32);
    }
    #[doc = "enable 0=not enabled, 1=enabled"]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "enable 0=not enabled, 1=enabled"]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Bod {
    fn default() -> Bod {
        Bod(0)
    }
}
