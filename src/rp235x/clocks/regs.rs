#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAdcCtrl(pub u32);
impl ClkAdcCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkAdcCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ClkAdcCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkAdcCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ClkAdcCtrl {
    #[inline(always)]
    fn default() -> ClkAdcCtrl {
        ClkAdcCtrl(0)
    }
}
impl core::fmt::Debug for ClkAdcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkAdcCtrl")
            .field("auxsrc", &self.auxsrc())
            .field("kill", &self.kill())
            .field("enable", &self.enable())
            .field("phase", &self.phase())
            .field("nudge", &self.nudge())
            .field("enabled", &self.enabled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkAdcCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkAdcCtrl {
            auxsrc: super::vals::ClkAdcCtrlAuxsrc,
            kill: bool,
            enable: bool,
            phase: u8,
            nudge: bool,
            enabled: bool,
        }
        let proxy = ClkAdcCtrl {
            auxsrc: self.auxsrc(),
            kill: self.kill(),
            enable: self.enable(),
            phase: self.phase(),
            nudge: self.nudge(),
            enabled: self.enabled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAdcDiv(pub u32);
impl ClkAdcDiv {
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for ClkAdcDiv {
    #[inline(always)]
    fn default() -> ClkAdcDiv {
        ClkAdcDiv(0)
    }
}
impl core::fmt::Debug for ClkAdcDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkAdcDiv")
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkAdcDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkAdcDiv {
            int: u8,
        }
        let proxy = ClkAdcDiv { int: self.int() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkAdcSelected(pub u32);
impl ClkAdcSelected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_adc_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_adc_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkAdcSelected {
    #[inline(always)]
    fn default() -> ClkAdcSelected {
        ClkAdcSelected(0)
    }
}
impl core::fmt::Debug for ClkAdcSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkAdcSelected")
            .field("clk_adc_selected", &self.clk_adc_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkAdcSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkAdcSelected {
            clk_adc_selected: bool,
        }
        let proxy = ClkAdcSelected {
            clk_adc_selected: self.clk_adc_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpout0selected(pub u32);
impl ClkGpout0selected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_gpout0_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_gpout0_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkGpout0selected {
    #[inline(always)]
    fn default() -> ClkGpout0selected {
        ClkGpout0selected(0)
    }
}
impl core::fmt::Debug for ClkGpout0selected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpout0selected")
            .field("clk_gpout0_selected", &self.clk_gpout0_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpout0selected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpout0selected {
            clk_gpout0_selected: bool,
        }
        let proxy = ClkGpout0selected {
            clk_gpout0_selected: self.clk_gpout0_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpout1selected(pub u32);
impl ClkGpout1selected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_gpout1_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_gpout1_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkGpout1selected {
    #[inline(always)]
    fn default() -> ClkGpout1selected {
        ClkGpout1selected(0)
    }
}
impl core::fmt::Debug for ClkGpout1selected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpout1selected")
            .field("clk_gpout1_selected", &self.clk_gpout1_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpout1selected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpout1selected {
            clk_gpout1_selected: bool,
        }
        let proxy = ClkGpout1selected {
            clk_gpout1_selected: self.clk_gpout1_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpout2selected(pub u32);
impl ClkGpout2selected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_gpout2_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_gpout2_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkGpout2selected {
    #[inline(always)]
    fn default() -> ClkGpout2selected {
        ClkGpout2selected(0)
    }
}
impl core::fmt::Debug for ClkGpout2selected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpout2selected")
            .field("clk_gpout2_selected", &self.clk_gpout2_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpout2selected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpout2selected {
            clk_gpout2_selected: bool,
        }
        let proxy = ClkGpout2selected {
            clk_gpout2_selected: self.clk_gpout2_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpout3selected(pub u32);
impl ClkGpout3selected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_gpout3_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_gpout3_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkGpout3selected {
    #[inline(always)]
    fn default() -> ClkGpout3selected {
        ClkGpout3selected(0)
    }
}
impl core::fmt::Debug for ClkGpout3selected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpout3selected")
            .field("clk_gpout3_selected", &self.clk_gpout3_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpout3selected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpout3selected {
            clk_gpout3_selected: bool,
        }
        let proxy = ClkGpout3selected {
            clk_gpout3_selected: self.clk_gpout3_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpoutCtrl(pub u32);
impl ClkGpoutCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkGpoutCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x0f;
        super::vals::ClkGpoutCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkGpoutCtrlAuxsrc) {
        self.0 = (self.0 & !(0x0f << 5usize)) | (((val.to_bits() as u32) & 0x0f) << 5usize);
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Enables duty cycle correction for odd divisors, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn dc50(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Enables duty cycle correction for odd divisors, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_dc50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ClkGpoutCtrl {
    #[inline(always)]
    fn default() -> ClkGpoutCtrl {
        ClkGpoutCtrl(0)
    }
}
impl core::fmt::Debug for ClkGpoutCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpoutCtrl")
            .field("auxsrc", &self.auxsrc())
            .field("kill", &self.kill())
            .field("enable", &self.enable())
            .field("dc50", &self.dc50())
            .field("phase", &self.phase())
            .field("nudge", &self.nudge())
            .field("enabled", &self.enabled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpoutCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpoutCtrl {
            auxsrc: super::vals::ClkGpoutCtrlAuxsrc,
            kill: bool,
            enable: bool,
            dc50: bool,
            phase: u8,
            nudge: bool,
            enabled: bool,
        }
        let proxy = ClkGpoutCtrl {
            auxsrc: self.auxsrc(),
            kill: self.kill(),
            enable: self.enable(),
            dc50: self.dc50(),
            phase: self.phase(),
            nudge: self.nudge(),
            enabled: self.enabled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkGpoutDiv(pub u32);
impl ClkGpoutDiv {
    #[doc = "Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn frac(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_frac(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ClkGpoutDiv {
    #[inline(always)]
    fn default() -> ClkGpoutDiv {
        ClkGpoutDiv(0)
    }
}
impl core::fmt::Debug for ClkGpoutDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkGpoutDiv")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkGpoutDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkGpoutDiv {
            frac: u16,
            int: u16,
        }
        let proxy = ClkGpoutDiv {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkHstxCtrl(pub u32);
impl ClkHstxCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkHstxCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ClkHstxCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkHstxCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ClkHstxCtrl {
    #[inline(always)]
    fn default() -> ClkHstxCtrl {
        ClkHstxCtrl(0)
    }
}
impl core::fmt::Debug for ClkHstxCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkHstxCtrl")
            .field("auxsrc", &self.auxsrc())
            .field("kill", &self.kill())
            .field("enable", &self.enable())
            .field("phase", &self.phase())
            .field("nudge", &self.nudge())
            .field("enabled", &self.enabled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkHstxCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkHstxCtrl {
            auxsrc: super::vals::ClkHstxCtrlAuxsrc,
            kill: bool,
            enable: bool,
            phase: u8,
            nudge: bool,
            enabled: bool,
        }
        let proxy = ClkHstxCtrl {
            auxsrc: self.auxsrc(),
            kill: self.kill(),
            enable: self.enable(),
            phase: self.phase(),
            nudge: self.nudge(),
            enabled: self.enabled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkHstxDiv(pub u32);
impl ClkHstxDiv {
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for ClkHstxDiv {
    #[inline(always)]
    fn default() -> ClkHstxDiv {
        ClkHstxDiv(0)
    }
}
impl core::fmt::Debug for ClkHstxDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkHstxDiv")
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkHstxDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkHstxDiv {
            int: u8,
        }
        let proxy = ClkHstxDiv { int: self.int() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkHstxSelected(pub u32);
impl ClkHstxSelected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_hstx_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_hstx_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkHstxSelected {
    #[inline(always)]
    fn default() -> ClkHstxSelected {
        ClkHstxSelected(0)
    }
}
impl core::fmt::Debug for ClkHstxSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkHstxSelected")
            .field("clk_hstx_selected", &self.clk_hstx_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkHstxSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkHstxSelected {
            clk_hstx_selected: bool,
        }
        let proxy = ClkHstxSelected {
            clk_hstx_selected: self.clk_hstx_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPeriCtrl(pub u32);
impl ClkPeriCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkPeriCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ClkPeriCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkPeriCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ClkPeriCtrl {
    #[inline(always)]
    fn default() -> ClkPeriCtrl {
        ClkPeriCtrl(0)
    }
}
impl core::fmt::Debug for ClkPeriCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkPeriCtrl")
            .field("auxsrc", &self.auxsrc())
            .field("kill", &self.kill())
            .field("enable", &self.enable())
            .field("enabled", &self.enabled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkPeriCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkPeriCtrl {
            auxsrc: super::vals::ClkPeriCtrlAuxsrc,
            kill: bool,
            enable: bool,
            enabled: bool,
        }
        let proxy = ClkPeriCtrl {
            auxsrc: self.auxsrc(),
            kill: self.kill(),
            enable: self.enable(),
            enabled: self.enabled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPeriDiv(pub u32);
impl ClkPeriDiv {
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
}
impl Default for ClkPeriDiv {
    #[inline(always)]
    fn default() -> ClkPeriDiv {
        ClkPeriDiv(0)
    }
}
impl core::fmt::Debug for ClkPeriDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkPeriDiv")
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkPeriDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkPeriDiv {
            int: u8,
        }
        let proxy = ClkPeriDiv { int: self.int() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkPeriSelected(pub u32);
impl ClkPeriSelected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_peri_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_peri_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkPeriSelected {
    #[inline(always)]
    fn default() -> ClkPeriSelected {
        ClkPeriSelected(0)
    }
}
impl core::fmt::Debug for ClkPeriSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkPeriSelected")
            .field("clk_peri_selected", &self.clk_peri_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkPeriSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkPeriSelected {
            clk_peri_selected: bool,
        }
        let proxy = ClkPeriSelected {
            clk_peri_selected: self.clk_peri_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefCtrl(pub u32);
impl ClkRefCtrl {
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::ClkRefCtrlSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::ClkRefCtrlSrc::from_bits(val as u8)
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::ClkRefCtrlSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkRefCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::ClkRefCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkRefCtrlAuxsrc) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
}
impl Default for ClkRefCtrl {
    #[inline(always)]
    fn default() -> ClkRefCtrl {
        ClkRefCtrl(0)
    }
}
impl core::fmt::Debug for ClkRefCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRefCtrl")
            .field("src", &self.src())
            .field("auxsrc", &self.auxsrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRefCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkRefCtrl {
            src: super::vals::ClkRefCtrlSrc,
            auxsrc: super::vals::ClkRefCtrlAuxsrc,
        }
        let proxy = ClkRefCtrl {
            src: self.src(),
            auxsrc: self.auxsrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefDiv(pub u32);
impl ClkRefDiv {
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for ClkRefDiv {
    #[inline(always)]
    fn default() -> ClkRefDiv {
        ClkRefDiv(0)
    }
}
impl core::fmt::Debug for ClkRefDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRefDiv")
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRefDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkRefDiv {
            int: u8,
        }
        let proxy = ClkRefDiv { int: self.int() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkRefSelected(pub u32);
impl ClkRefSelected {
    #[doc = "The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub const fn clk_ref_selected(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub fn set_clk_ref_selected(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for ClkRefSelected {
    #[inline(always)]
    fn default() -> ClkRefSelected {
        ClkRefSelected(0)
    }
}
impl core::fmt::Debug for ClkRefSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkRefSelected")
            .field("clk_ref_selected", &self.clk_ref_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkRefSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkRefSelected {
            clk_ref_selected: u8,
        }
        let proxy = ClkRefSelected {
            clk_ref_selected: self.clk_ref_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysCtrl(pub u32);
impl ClkSysCtrl {
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn src(&self) -> super::vals::ClkSysCtrlSrc {
        let val = (self.0 >> 0usize) & 0x01;
        super::vals::ClkSysCtrlSrc::from_bits(val as u8)
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::ClkSysCtrlSrc) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val.to_bits() as u32) & 0x01) << 0usize);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkSysCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ClkSysCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkSysCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
}
impl Default for ClkSysCtrl {
    #[inline(always)]
    fn default() -> ClkSysCtrl {
        ClkSysCtrl(0)
    }
}
impl core::fmt::Debug for ClkSysCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkSysCtrl")
            .field("src", &self.src())
            .field("auxsrc", &self.auxsrc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkSysCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkSysCtrl {
            src: super::vals::ClkSysCtrlSrc,
            auxsrc: super::vals::ClkSysCtrlAuxsrc,
        }
        let proxy = ClkSysCtrl {
            src: self.src(),
            auxsrc: self.auxsrc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysDiv(pub u32);
impl ClkSysDiv {
    #[doc = "Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn frac(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fractional component of the divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_frac(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for ClkSysDiv {
    #[inline(always)]
    fn default() -> ClkSysDiv {
        ClkSysDiv(0)
    }
}
impl core::fmt::Debug for ClkSysDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkSysDiv")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkSysDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkSysDiv {
            frac: u16,
            int: u16,
        }
        let proxy = ClkSysDiv {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysResusCtrl(pub u32);
impl ClkSysResusCtrl {
    #[doc = "This is expressed as a number of clk_ref cycles and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "This is expressed as a number of clk_ref cycles and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    #[inline(always)]
    pub fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Enable resus"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Enable resus"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Force a resus, for test purposes only"]
    #[inline(always)]
    pub const fn frce(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Force a resus, for test purposes only"]
    #[inline(always)]
    pub fn set_frce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "For clearing the resus after the fault that triggered it has been corrected"]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for ClkSysResusCtrl {
    #[inline(always)]
    fn default() -> ClkSysResusCtrl {
        ClkSysResusCtrl(0)
    }
}
impl core::fmt::Debug for ClkSysResusCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkSysResusCtrl")
            .field("timeout", &self.timeout())
            .field("enable", &self.enable())
            .field("frce", &self.frce())
            .field("clear", &self.clear())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkSysResusCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkSysResusCtrl {
            timeout: u8,
            enable: bool,
            frce: bool,
            clear: bool,
        }
        let proxy = ClkSysResusCtrl {
            timeout: self.timeout(),
            enable: self.enable(),
            frce: self.frce(),
            clear: self.clear(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysResusStatus(pub u32);
impl ClkSysResusStatus {
    #[doc = "Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub const fn resussed(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    #[inline(always)]
    pub fn set_resussed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkSysResusStatus {
    #[inline(always)]
    fn default() -> ClkSysResusStatus {
        ClkSysResusStatus(0)
    }
}
impl core::fmt::Debug for ClkSysResusStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkSysResusStatus")
            .field("resussed", &self.resussed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkSysResusStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkSysResusStatus {
            resussed: bool,
        }
        let proxy = ClkSysResusStatus {
            resussed: self.resussed(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkSysSelected(pub u32);
impl ClkSysSelected {
    #[doc = "The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub const fn clk_sys_selected(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "The glitchless multiplexer does not switch instantaneously (to avoid glitches), so software should poll this register to wait for the switch to complete. This register contains one decoded bit for each of the clock sources enumerated in the CTRL SRC field. At most one of these bits will be set at any time, indicating that clock is currently present at the output of the glitchless mux. Whilst switching is in progress, this register may briefly show all-0s."]
    #[inline(always)]
    pub fn set_clk_sys_selected(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
}
impl Default for ClkSysSelected {
    #[inline(always)]
    fn default() -> ClkSysSelected {
        ClkSysSelected(0)
    }
}
impl core::fmt::Debug for ClkSysSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkSysSelected")
            .field("clk_sys_selected", &self.clk_sys_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkSysSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkSysSelected {
            clk_sys_selected: u8,
        }
        let proxy = ClkSysSelected {
            clk_sys_selected: self.clk_sys_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkUsbCtrl(pub u32);
impl ClkUsbCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub const fn auxsrc(&self) -> super::vals::ClkUsbCtrlAuxsrc {
        let val = (self.0 >> 5usize) & 0x07;
        super::vals::ClkUsbCtrlAuxsrc::from_bits(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    #[inline(always)]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkUsbCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val.to_bits() as u32) & 0x07) << 5usize);
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator, enable must be set low before deasserting kill"]
    #[inline(always)]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    #[inline(always)]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    #[inline(always)]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub const fn enabled(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "clock generator is enabled"]
    #[inline(always)]
    pub fn set_enabled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ClkUsbCtrl {
    #[inline(always)]
    fn default() -> ClkUsbCtrl {
        ClkUsbCtrl(0)
    }
}
impl core::fmt::Debug for ClkUsbCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkUsbCtrl")
            .field("auxsrc", &self.auxsrc())
            .field("kill", &self.kill())
            .field("enable", &self.enable())
            .field("phase", &self.phase())
            .field("nudge", &self.nudge())
            .field("enabled", &self.enabled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkUsbCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkUsbCtrl {
            auxsrc: super::vals::ClkUsbCtrlAuxsrc,
            kill: bool,
            enable: bool,
            phase: u8,
            nudge: bool,
            enabled: bool,
        }
        let proxy = ClkUsbCtrl {
            auxsrc: self.auxsrc(),
            kill: self.kill(),
            enable: self.enable(),
            phase: self.phase(),
            nudge: self.nudge(),
            enabled: self.enabled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkUsbDiv(pub u32);
impl ClkUsbDiv {
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Integer part of clock divisor, 0 -> max+1, can be changed on-the-fly"]
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for ClkUsbDiv {
    #[inline(always)]
    fn default() -> ClkUsbDiv {
        ClkUsbDiv(0)
    }
}
impl core::fmt::Debug for ClkUsbDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkUsbDiv")
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkUsbDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkUsbDiv {
            int: u8,
        }
        let proxy = ClkUsbDiv { int: self.int() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Indicates which src is currently selected (one-hot)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ClkUsbSelected(pub u32);
impl ClkUsbSelected {
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub const fn clk_usb_selected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This slice does not have a glitchless mux (only the AUX_SRC field is present, not SRC) so this register is hardwired to 0x1."]
    #[inline(always)]
    pub fn set_clk_usb_selected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for ClkUsbSelected {
    #[inline(always)]
    fn default() -> ClkUsbSelected {
        ClkUsbSelected(0)
    }
}
impl core::fmt::Debug for ClkUsbSelected {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ClkUsbSelected")
            .field("clk_usb_selected", &self.clk_usb_selected())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ClkUsbSelected {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ClkUsbSelected {
            clk_usb_selected: bool,
        }
        let proxy = ClkUsbSelected {
            clk_usb_selected: self.clk_usb_selected(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftclkLposcCtrl(pub u32);
impl DftclkLposcCtrl {
    #[inline(always)]
    pub const fn src(&self) -> super::vals::DftclkLposcCtrlSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DftclkLposcCtrlSrc::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::DftclkLposcCtrlSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for DftclkLposcCtrl {
    #[inline(always)]
    fn default() -> DftclkLposcCtrl {
        DftclkLposcCtrl(0)
    }
}
impl core::fmt::Debug for DftclkLposcCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DftclkLposcCtrl")
            .field("src", &self.src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DftclkLposcCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DftclkLposcCtrl {
            src: super::vals::DftclkLposcCtrlSrc,
        }
        let proxy = DftclkLposcCtrl { src: self.src() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftclkRoscCtrl(pub u32);
impl DftclkRoscCtrl {
    #[inline(always)]
    pub const fn src(&self) -> super::vals::DftclkRoscCtrlSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DftclkRoscCtrlSrc::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::DftclkRoscCtrlSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for DftclkRoscCtrl {
    #[inline(always)]
    fn default() -> DftclkRoscCtrl {
        DftclkRoscCtrl(0)
    }
}
impl core::fmt::Debug for DftclkRoscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DftclkRoscCtrl")
            .field("src", &self.src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DftclkRoscCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DftclkRoscCtrl {
            src: super::vals::DftclkRoscCtrlSrc,
        }
        let proxy = DftclkRoscCtrl { src: self.src() };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DftclkXoscCtrl(pub u32);
impl DftclkXoscCtrl {
    #[inline(always)]
    pub const fn src(&self) -> super::vals::DftclkXoscCtrlSrc {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::DftclkXoscCtrlSrc::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_src(&mut self, val: super::vals::DftclkXoscCtrlSrc) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
}
impl Default for DftclkXoscCtrl {
    #[inline(always)]
    fn default() -> DftclkXoscCtrl {
        DftclkXoscCtrl(0)
    }
}
impl core::fmt::Debug for DftclkXoscCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DftclkXoscCtrl")
            .field("src", &self.src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DftclkXoscCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DftclkXoscCtrl {
            src: super::vals::DftclkXoscCtrlSrc,
        }
        let proxy = DftclkXoscCtrl { src: self.src() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "indicates the state of the clock enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enabled0(pub u32);
impl Enabled0 {
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_accessctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_bootram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_glitch_detector(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sha256(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for Enabled0 {
    #[inline(always)]
    fn default() -> Enabled0 {
        Enabled0(0)
    }
}
impl core::fmt::Debug for Enabled0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enabled0")
            .field("clk_sys_clocks", &self.clk_sys_clocks())
            .field("clk_sys_accessctrl", &self.clk_sys_accessctrl())
            .field("clk_adc", &self.clk_adc())
            .field("clk_sys_adc", &self.clk_sys_adc())
            .field("clk_sys_bootram", &self.clk_sys_bootram())
            .field("clk_sys_busctrl", &self.clk_sys_busctrl())
            .field("clk_sys_busfabric", &self.clk_sys_busfabric())
            .field("clk_sys_dma", &self.clk_sys_dma())
            .field("clk_sys_glitch_detector", &self.clk_sys_glitch_detector())
            .field("clk_hstx", &self.clk_hstx())
            .field("clk_sys_hstx", &self.clk_sys_hstx())
            .field("clk_sys_i2c0", &self.clk_sys_i2c0())
            .field("clk_sys_i2c1", &self.clk_sys_i2c1())
            .field("clk_sys_io", &self.clk_sys_io())
            .field("clk_sys_jtag", &self.clk_sys_jtag())
            .field("clk_ref_otp", &self.clk_ref_otp())
            .field("clk_sys_otp", &self.clk_sys_otp())
            .field("clk_sys_pads", &self.clk_sys_pads())
            .field("clk_sys_pio0", &self.clk_sys_pio0())
            .field("clk_sys_pio1", &self.clk_sys_pio1())
            .field("clk_sys_pio2", &self.clk_sys_pio2())
            .field("clk_sys_pll_sys", &self.clk_sys_pll_sys())
            .field("clk_sys_pll_usb", &self.clk_sys_pll_usb())
            .field("clk_ref_powman", &self.clk_ref_powman())
            .field("clk_sys_powman", &self.clk_sys_powman())
            .field("clk_sys_pwm", &self.clk_sys_pwm())
            .field("clk_sys_resets", &self.clk_sys_resets())
            .field("clk_sys_rom", &self.clk_sys_rom())
            .field("clk_sys_rosc", &self.clk_sys_rosc())
            .field("clk_sys_psm", &self.clk_sys_psm())
            .field("clk_sys_sha256", &self.clk_sys_sha256())
            .field("clk_sys_sio", &self.clk_sys_sio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enabled0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Enabled0 {
            clk_sys_clocks: bool,
            clk_sys_accessctrl: bool,
            clk_adc: bool,
            clk_sys_adc: bool,
            clk_sys_bootram: bool,
            clk_sys_busctrl: bool,
            clk_sys_busfabric: bool,
            clk_sys_dma: bool,
            clk_sys_glitch_detector: bool,
            clk_hstx: bool,
            clk_sys_hstx: bool,
            clk_sys_i2c0: bool,
            clk_sys_i2c1: bool,
            clk_sys_io: bool,
            clk_sys_jtag: bool,
            clk_ref_otp: bool,
            clk_sys_otp: bool,
            clk_sys_pads: bool,
            clk_sys_pio0: bool,
            clk_sys_pio1: bool,
            clk_sys_pio2: bool,
            clk_sys_pll_sys: bool,
            clk_sys_pll_usb: bool,
            clk_ref_powman: bool,
            clk_sys_powman: bool,
            clk_sys_pwm: bool,
            clk_sys_resets: bool,
            clk_sys_rom: bool,
            clk_sys_rosc: bool,
            clk_sys_psm: bool,
            clk_sys_sha256: bool,
            clk_sys_sio: bool,
        }
        let proxy = Enabled0 {
            clk_sys_clocks: self.clk_sys_clocks(),
            clk_sys_accessctrl: self.clk_sys_accessctrl(),
            clk_adc: self.clk_adc(),
            clk_sys_adc: self.clk_sys_adc(),
            clk_sys_bootram: self.clk_sys_bootram(),
            clk_sys_busctrl: self.clk_sys_busctrl(),
            clk_sys_busfabric: self.clk_sys_busfabric(),
            clk_sys_dma: self.clk_sys_dma(),
            clk_sys_glitch_detector: self.clk_sys_glitch_detector(),
            clk_hstx: self.clk_hstx(),
            clk_sys_hstx: self.clk_sys_hstx(),
            clk_sys_i2c0: self.clk_sys_i2c0(),
            clk_sys_i2c1: self.clk_sys_i2c1(),
            clk_sys_io: self.clk_sys_io(),
            clk_sys_jtag: self.clk_sys_jtag(),
            clk_ref_otp: self.clk_ref_otp(),
            clk_sys_otp: self.clk_sys_otp(),
            clk_sys_pads: self.clk_sys_pads(),
            clk_sys_pio0: self.clk_sys_pio0(),
            clk_sys_pio1: self.clk_sys_pio1(),
            clk_sys_pio2: self.clk_sys_pio2(),
            clk_sys_pll_sys: self.clk_sys_pll_sys(),
            clk_sys_pll_usb: self.clk_sys_pll_usb(),
            clk_ref_powman: self.clk_ref_powman(),
            clk_sys_powman: self.clk_sys_powman(),
            clk_sys_pwm: self.clk_sys_pwm(),
            clk_sys_resets: self.clk_sys_resets(),
            clk_sys_rom: self.clk_sys_rom(),
            clk_sys_rosc: self.clk_sys_rosc(),
            clk_sys_psm: self.clk_sys_psm(),
            clk_sys_sha256: self.clk_sys_sha256(),
            clk_sys_sio: self.clk_sys_sio(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "indicates the state of the clock enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Enabled1(pub u32);
impl Enabled1 {
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for Enabled1 {
    #[inline(always)]
    fn default() -> Enabled1 {
        Enabled1(0)
    }
}
impl core::fmt::Debug for Enabled1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Enabled1")
            .field("clk_peri_spi0", &self.clk_peri_spi0())
            .field("clk_sys_spi0", &self.clk_sys_spi0())
            .field("clk_peri_spi1", &self.clk_peri_spi1())
            .field("clk_sys_spi1", &self.clk_sys_spi1())
            .field("clk_sys_sram0", &self.clk_sys_sram0())
            .field("clk_sys_sram1", &self.clk_sys_sram1())
            .field("clk_sys_sram2", &self.clk_sys_sram2())
            .field("clk_sys_sram3", &self.clk_sys_sram3())
            .field("clk_sys_sram4", &self.clk_sys_sram4())
            .field("clk_sys_sram5", &self.clk_sys_sram5())
            .field("clk_sys_sram6", &self.clk_sys_sram6())
            .field("clk_sys_sram7", &self.clk_sys_sram7())
            .field("clk_sys_sram8", &self.clk_sys_sram8())
            .field("clk_sys_sram9", &self.clk_sys_sram9())
            .field("clk_sys_syscfg", &self.clk_sys_syscfg())
            .field("clk_sys_sysinfo", &self.clk_sys_sysinfo())
            .field("clk_sys_tbman", &self.clk_sys_tbman())
            .field("clk_ref_ticks", &self.clk_ref_ticks())
            .field("clk_sys_ticks", &self.clk_sys_ticks())
            .field("clk_sys_timer0", &self.clk_sys_timer0())
            .field("clk_sys_timer1", &self.clk_sys_timer1())
            .field("clk_sys_trng", &self.clk_sys_trng())
            .field("clk_peri_uart0", &self.clk_peri_uart0())
            .field("clk_sys_uart0", &self.clk_sys_uart0())
            .field("clk_peri_uart1", &self.clk_peri_uart1())
            .field("clk_sys_uart1", &self.clk_sys_uart1())
            .field("clk_sys_usbctrl", &self.clk_sys_usbctrl())
            .field("clk_usb", &self.clk_usb())
            .field("clk_sys_watchdog", &self.clk_sys_watchdog())
            .field("clk_sys_xip", &self.clk_sys_xip())
            .field("clk_sys_xosc", &self.clk_sys_xosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Enabled1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Enabled1 {
            clk_peri_spi0: bool,
            clk_sys_spi0: bool,
            clk_peri_spi1: bool,
            clk_sys_spi1: bool,
            clk_sys_sram0: bool,
            clk_sys_sram1: bool,
            clk_sys_sram2: bool,
            clk_sys_sram3: bool,
            clk_sys_sram4: bool,
            clk_sys_sram5: bool,
            clk_sys_sram6: bool,
            clk_sys_sram7: bool,
            clk_sys_sram8: bool,
            clk_sys_sram9: bool,
            clk_sys_syscfg: bool,
            clk_sys_sysinfo: bool,
            clk_sys_tbman: bool,
            clk_ref_ticks: bool,
            clk_sys_ticks: bool,
            clk_sys_timer0: bool,
            clk_sys_timer1: bool,
            clk_sys_trng: bool,
            clk_peri_uart0: bool,
            clk_sys_uart0: bool,
            clk_peri_uart1: bool,
            clk_sys_uart1: bool,
            clk_sys_usbctrl: bool,
            clk_usb: bool,
            clk_sys_watchdog: bool,
            clk_sys_xip: bool,
            clk_sys_xosc: bool,
        }
        let proxy = Enabled1 {
            clk_peri_spi0: self.clk_peri_spi0(),
            clk_sys_spi0: self.clk_sys_spi0(),
            clk_peri_spi1: self.clk_peri_spi1(),
            clk_sys_spi1: self.clk_sys_spi1(),
            clk_sys_sram0: self.clk_sys_sram0(),
            clk_sys_sram1: self.clk_sys_sram1(),
            clk_sys_sram2: self.clk_sys_sram2(),
            clk_sys_sram3: self.clk_sys_sram3(),
            clk_sys_sram4: self.clk_sys_sram4(),
            clk_sys_sram5: self.clk_sys_sram5(),
            clk_sys_sram6: self.clk_sys_sram6(),
            clk_sys_sram7: self.clk_sys_sram7(),
            clk_sys_sram8: self.clk_sys_sram8(),
            clk_sys_sram9: self.clk_sys_sram9(),
            clk_sys_syscfg: self.clk_sys_syscfg(),
            clk_sys_sysinfo: self.clk_sys_sysinfo(),
            clk_sys_tbman: self.clk_sys_tbman(),
            clk_ref_ticks: self.clk_ref_ticks(),
            clk_sys_ticks: self.clk_sys_ticks(),
            clk_sys_timer0: self.clk_sys_timer0(),
            clk_sys_timer1: self.clk_sys_timer1(),
            clk_sys_trng: self.clk_sys_trng(),
            clk_peri_uart0: self.clk_peri_uart0(),
            clk_sys_uart0: self.clk_sys_uart0(),
            clk_peri_uart1: self.clk_peri_uart1(),
            clk_sys_uart1: self.clk_sys_uart1(),
            clk_sys_usbctrl: self.clk_sys_usbctrl(),
            clk_usb: self.clk_usb(),
            clk_sys_watchdog: self.clk_sys_watchdog(),
            clk_sys_xip: self.clk_sys_xip(),
            clk_sys_xosc: self.clk_sys_xosc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0delay(pub u32);
impl Fc0delay {
    #[inline(always)]
    pub const fn fc0_delay(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_fc0_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
}
impl Default for Fc0delay {
    #[inline(always)]
    fn default() -> Fc0delay {
        Fc0delay(0)
    }
}
impl core::fmt::Debug for Fc0delay {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0delay")
            .field("fc0_delay", &self.fc0_delay())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0delay {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0delay {
            fc0_delay: u8,
        }
        let proxy = Fc0delay {
            fc0_delay: self.fc0_delay(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0interval(pub u32);
impl Fc0interval {
    #[inline(always)]
    pub const fn fc0_interval(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_fc0_interval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Fc0interval {
    #[inline(always)]
    fn default() -> Fc0interval {
        Fc0interval(0)
    }
}
impl core::fmt::Debug for Fc0interval {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0interval")
            .field("fc0_interval", &self.fc0_interval())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0interval {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0interval {
            fc0_interval: u8,
        }
        let proxy = Fc0interval {
            fc0_interval: self.fc0_interval(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0maxKhz(pub u32);
impl Fc0maxKhz {
    #[inline(always)]
    pub const fn fc0_max_khz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_fc0_max_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
}
impl Default for Fc0maxKhz {
    #[inline(always)]
    fn default() -> Fc0maxKhz {
        Fc0maxKhz(0)
    }
}
impl core::fmt::Debug for Fc0maxKhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0maxKhz")
            .field("fc0_max_khz", &self.fc0_max_khz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0maxKhz {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0maxKhz {
            fc0_max_khz: u32,
        }
        let proxy = Fc0maxKhz {
            fc0_max_khz: self.fc0_max_khz(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0minKhz(pub u32);
impl Fc0minKhz {
    #[inline(always)]
    pub const fn fc0_min_khz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x01ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_fc0_min_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0usize)) | (((val as u32) & 0x01ff_ffff) << 0usize);
    }
}
impl Default for Fc0minKhz {
    #[inline(always)]
    fn default() -> Fc0minKhz {
        Fc0minKhz(0)
    }
}
impl core::fmt::Debug for Fc0minKhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0minKhz")
            .field("fc0_min_khz", &self.fc0_min_khz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0minKhz {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0minKhz {
            fc0_min_khz: u32,
        }
        let proxy = Fc0minKhz {
            fc0_min_khz: self.fc0_min_khz(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Reference clock frequency in kHz"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0refKhz(pub u32);
impl Fc0refKhz {
    #[inline(always)]
    pub const fn fc0_ref_khz(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x000f_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_fc0_ref_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0usize)) | (((val as u32) & 0x000f_ffff) << 0usize);
    }
}
impl Default for Fc0refKhz {
    #[inline(always)]
    fn default() -> Fc0refKhz {
        Fc0refKhz(0)
    }
}
impl core::fmt::Debug for Fc0refKhz {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0refKhz")
            .field("fc0_ref_khz", &self.fc0_ref_khz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0refKhz {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0refKhz {
            fc0_ref_khz: u32,
        }
        let proxy = Fc0refKhz {
            fc0_ref_khz: self.fc0_ref_khz(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Result of frequency measurement, only valid when status_done=1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0result(pub u32);
impl Fc0result {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[inline(always)]
    pub const fn khz(&self) -> u32 {
        let val = (self.0 >> 5usize) & 0x01ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 5usize)) | (((val as u32) & 0x01ff_ffff) << 5usize);
    }
}
impl Default for Fc0result {
    #[inline(always)]
    fn default() -> Fc0result {
        Fc0result(0)
    }
}
impl core::fmt::Debug for Fc0result {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0result")
            .field("frac", &self.frac())
            .field("khz", &self.khz())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0result {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0result {
            frac: u8,
            khz: u32,
        }
        let proxy = Fc0result {
            frac: self.frac(),
            khz: self.khz(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0src(pub u32);
impl Fc0src {
    #[inline(always)]
    pub const fn fc0_src(&self) -> super::vals::Fc0src {
        let val = (self.0 >> 0usize) & 0xff;
        super::vals::Fc0src::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_fc0_src(&mut self, val: super::vals::Fc0src) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val.to_bits() as u32) & 0xff) << 0usize);
    }
}
impl Default for Fc0src {
    #[inline(always)]
    fn default() -> Fc0src {
        Fc0src(0)
    }
}
impl core::fmt::Debug for Fc0src {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0src")
            .field("fc0_src", &self.fc0_src())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0src {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0src {
            fc0_src: super::vals::Fc0src,
        }
        let proxy = Fc0src {
            fc0_src: self.fc0_src(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Frequency counter status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fc0status(pub u32);
impl Fc0status {
    #[doc = "Test passed"]
    #[inline(always)]
    pub const fn pass(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Test passed"]
    #[inline(always)]
    pub fn set_pass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Test complete"]
    #[inline(always)]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Test complete"]
    #[inline(always)]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Test running"]
    #[inline(always)]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Test running"]
    #[inline(always)]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Waiting for test clock to start"]
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Waiting for test clock to start"]
    #[inline(always)]
    pub fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Test failed"]
    #[inline(always)]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Test failed"]
    #[inline(always)]
    pub fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Test clock slower than expected, only valid when status_done=1"]
    #[inline(always)]
    pub const fn slow(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Test clock slower than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn set_slow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Test clock faster than expected, only valid when status_done=1"]
    #[inline(always)]
    pub const fn fast(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Test clock faster than expected, only valid when status_done=1"]
    #[inline(always)]
    pub fn set_fast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Test clock stopped during test"]
    #[inline(always)]
    pub const fn died(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Test clock stopped during test"]
    #[inline(always)]
    pub fn set_died(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Fc0status {
    #[inline(always)]
    fn default() -> Fc0status {
        Fc0status(0)
    }
}
impl core::fmt::Debug for Fc0status {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fc0status")
            .field("pass", &self.pass())
            .field("done", &self.done())
            .field("running", &self.running())
            .field("waiting", &self.waiting())
            .field("fail", &self.fail())
            .field("slow", &self.slow())
            .field("fast", &self.fast())
            .field("died", &self.died())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fc0status {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fc0status {
            pass: bool,
            done: bool,
            running: bool,
            waiting: bool,
            fail: bool,
            slow: bool,
            fast: bool,
            died: bool,
        }
        let proxy = Fc0status {
            pass: self.pass(),
            done: self.done(),
            running: self.running(),
            waiting: self.waiting(),
            fail: self.fail(),
            slow: self.slow(),
            fast: self.fast(),
            died: self.died(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[inline(always)]
    pub const fn clk_sys_resus(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_resus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("clk_sys_resus", &self.clk_sys_resus())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            clk_sys_resus: bool,
        }
        let proxy = Int {
            clk_sys_resus: self.clk_sys_resus(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "enable clock in sleep mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SleepEn0(pub u32);
impl SleepEn0 {
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_accessctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_bootram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_glitch_detector(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sha256(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SleepEn0 {
    #[inline(always)]
    fn default() -> SleepEn0 {
        SleepEn0(0)
    }
}
impl core::fmt::Debug for SleepEn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SleepEn0")
            .field("clk_sys_clocks", &self.clk_sys_clocks())
            .field("clk_sys_accessctrl", &self.clk_sys_accessctrl())
            .field("clk_adc", &self.clk_adc())
            .field("clk_sys_adc", &self.clk_sys_adc())
            .field("clk_sys_bootram", &self.clk_sys_bootram())
            .field("clk_sys_busctrl", &self.clk_sys_busctrl())
            .field("clk_sys_busfabric", &self.clk_sys_busfabric())
            .field("clk_sys_dma", &self.clk_sys_dma())
            .field("clk_sys_glitch_detector", &self.clk_sys_glitch_detector())
            .field("clk_hstx", &self.clk_hstx())
            .field("clk_sys_hstx", &self.clk_sys_hstx())
            .field("clk_sys_i2c0", &self.clk_sys_i2c0())
            .field("clk_sys_i2c1", &self.clk_sys_i2c1())
            .field("clk_sys_io", &self.clk_sys_io())
            .field("clk_sys_jtag", &self.clk_sys_jtag())
            .field("clk_ref_otp", &self.clk_ref_otp())
            .field("clk_sys_otp", &self.clk_sys_otp())
            .field("clk_sys_pads", &self.clk_sys_pads())
            .field("clk_sys_pio0", &self.clk_sys_pio0())
            .field("clk_sys_pio1", &self.clk_sys_pio1())
            .field("clk_sys_pio2", &self.clk_sys_pio2())
            .field("clk_sys_pll_sys", &self.clk_sys_pll_sys())
            .field("clk_sys_pll_usb", &self.clk_sys_pll_usb())
            .field("clk_ref_powman", &self.clk_ref_powman())
            .field("clk_sys_powman", &self.clk_sys_powman())
            .field("clk_sys_pwm", &self.clk_sys_pwm())
            .field("clk_sys_resets", &self.clk_sys_resets())
            .field("clk_sys_rom", &self.clk_sys_rom())
            .field("clk_sys_rosc", &self.clk_sys_rosc())
            .field("clk_sys_psm", &self.clk_sys_psm())
            .field("clk_sys_sha256", &self.clk_sys_sha256())
            .field("clk_sys_sio", &self.clk_sys_sio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SleepEn0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SleepEn0 {
            clk_sys_clocks: bool,
            clk_sys_accessctrl: bool,
            clk_adc: bool,
            clk_sys_adc: bool,
            clk_sys_bootram: bool,
            clk_sys_busctrl: bool,
            clk_sys_busfabric: bool,
            clk_sys_dma: bool,
            clk_sys_glitch_detector: bool,
            clk_hstx: bool,
            clk_sys_hstx: bool,
            clk_sys_i2c0: bool,
            clk_sys_i2c1: bool,
            clk_sys_io: bool,
            clk_sys_jtag: bool,
            clk_ref_otp: bool,
            clk_sys_otp: bool,
            clk_sys_pads: bool,
            clk_sys_pio0: bool,
            clk_sys_pio1: bool,
            clk_sys_pio2: bool,
            clk_sys_pll_sys: bool,
            clk_sys_pll_usb: bool,
            clk_ref_powman: bool,
            clk_sys_powman: bool,
            clk_sys_pwm: bool,
            clk_sys_resets: bool,
            clk_sys_rom: bool,
            clk_sys_rosc: bool,
            clk_sys_psm: bool,
            clk_sys_sha256: bool,
            clk_sys_sio: bool,
        }
        let proxy = SleepEn0 {
            clk_sys_clocks: self.clk_sys_clocks(),
            clk_sys_accessctrl: self.clk_sys_accessctrl(),
            clk_adc: self.clk_adc(),
            clk_sys_adc: self.clk_sys_adc(),
            clk_sys_bootram: self.clk_sys_bootram(),
            clk_sys_busctrl: self.clk_sys_busctrl(),
            clk_sys_busfabric: self.clk_sys_busfabric(),
            clk_sys_dma: self.clk_sys_dma(),
            clk_sys_glitch_detector: self.clk_sys_glitch_detector(),
            clk_hstx: self.clk_hstx(),
            clk_sys_hstx: self.clk_sys_hstx(),
            clk_sys_i2c0: self.clk_sys_i2c0(),
            clk_sys_i2c1: self.clk_sys_i2c1(),
            clk_sys_io: self.clk_sys_io(),
            clk_sys_jtag: self.clk_sys_jtag(),
            clk_ref_otp: self.clk_ref_otp(),
            clk_sys_otp: self.clk_sys_otp(),
            clk_sys_pads: self.clk_sys_pads(),
            clk_sys_pio0: self.clk_sys_pio0(),
            clk_sys_pio1: self.clk_sys_pio1(),
            clk_sys_pio2: self.clk_sys_pio2(),
            clk_sys_pll_sys: self.clk_sys_pll_sys(),
            clk_sys_pll_usb: self.clk_sys_pll_usb(),
            clk_ref_powman: self.clk_ref_powman(),
            clk_sys_powman: self.clk_sys_powman(),
            clk_sys_pwm: self.clk_sys_pwm(),
            clk_sys_resets: self.clk_sys_resets(),
            clk_sys_rom: self.clk_sys_rom(),
            clk_sys_rosc: self.clk_sys_rosc(),
            clk_sys_psm: self.clk_sys_psm(),
            clk_sys_sha256: self.clk_sys_sha256(),
            clk_sys_sio: self.clk_sys_sio(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "enable clock in sleep mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SleepEn1(pub u32);
impl SleepEn1 {
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for SleepEn1 {
    #[inline(always)]
    fn default() -> SleepEn1 {
        SleepEn1(0)
    }
}
impl core::fmt::Debug for SleepEn1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SleepEn1")
            .field("clk_peri_spi0", &self.clk_peri_spi0())
            .field("clk_sys_spi0", &self.clk_sys_spi0())
            .field("clk_peri_spi1", &self.clk_peri_spi1())
            .field("clk_sys_spi1", &self.clk_sys_spi1())
            .field("clk_sys_sram0", &self.clk_sys_sram0())
            .field("clk_sys_sram1", &self.clk_sys_sram1())
            .field("clk_sys_sram2", &self.clk_sys_sram2())
            .field("clk_sys_sram3", &self.clk_sys_sram3())
            .field("clk_sys_sram4", &self.clk_sys_sram4())
            .field("clk_sys_sram5", &self.clk_sys_sram5())
            .field("clk_sys_sram6", &self.clk_sys_sram6())
            .field("clk_sys_sram7", &self.clk_sys_sram7())
            .field("clk_sys_sram8", &self.clk_sys_sram8())
            .field("clk_sys_sram9", &self.clk_sys_sram9())
            .field("clk_sys_syscfg", &self.clk_sys_syscfg())
            .field("clk_sys_sysinfo", &self.clk_sys_sysinfo())
            .field("clk_sys_tbman", &self.clk_sys_tbman())
            .field("clk_ref_ticks", &self.clk_ref_ticks())
            .field("clk_sys_ticks", &self.clk_sys_ticks())
            .field("clk_sys_timer0", &self.clk_sys_timer0())
            .field("clk_sys_timer1", &self.clk_sys_timer1())
            .field("clk_sys_trng", &self.clk_sys_trng())
            .field("clk_peri_uart0", &self.clk_peri_uart0())
            .field("clk_sys_uart0", &self.clk_sys_uart0())
            .field("clk_peri_uart1", &self.clk_peri_uart1())
            .field("clk_sys_uart1", &self.clk_sys_uart1())
            .field("clk_sys_usbctrl", &self.clk_sys_usbctrl())
            .field("clk_usb", &self.clk_usb())
            .field("clk_sys_watchdog", &self.clk_sys_watchdog())
            .field("clk_sys_xip", &self.clk_sys_xip())
            .field("clk_sys_xosc", &self.clk_sys_xosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SleepEn1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SleepEn1 {
            clk_peri_spi0: bool,
            clk_sys_spi0: bool,
            clk_peri_spi1: bool,
            clk_sys_spi1: bool,
            clk_sys_sram0: bool,
            clk_sys_sram1: bool,
            clk_sys_sram2: bool,
            clk_sys_sram3: bool,
            clk_sys_sram4: bool,
            clk_sys_sram5: bool,
            clk_sys_sram6: bool,
            clk_sys_sram7: bool,
            clk_sys_sram8: bool,
            clk_sys_sram9: bool,
            clk_sys_syscfg: bool,
            clk_sys_sysinfo: bool,
            clk_sys_tbman: bool,
            clk_ref_ticks: bool,
            clk_sys_ticks: bool,
            clk_sys_timer0: bool,
            clk_sys_timer1: bool,
            clk_sys_trng: bool,
            clk_peri_uart0: bool,
            clk_sys_uart0: bool,
            clk_peri_uart1: bool,
            clk_sys_uart1: bool,
            clk_sys_usbctrl: bool,
            clk_usb: bool,
            clk_sys_watchdog: bool,
            clk_sys_xip: bool,
            clk_sys_xosc: bool,
        }
        let proxy = SleepEn1 {
            clk_peri_spi0: self.clk_peri_spi0(),
            clk_sys_spi0: self.clk_sys_spi0(),
            clk_peri_spi1: self.clk_peri_spi1(),
            clk_sys_spi1: self.clk_sys_spi1(),
            clk_sys_sram0: self.clk_sys_sram0(),
            clk_sys_sram1: self.clk_sys_sram1(),
            clk_sys_sram2: self.clk_sys_sram2(),
            clk_sys_sram3: self.clk_sys_sram3(),
            clk_sys_sram4: self.clk_sys_sram4(),
            clk_sys_sram5: self.clk_sys_sram5(),
            clk_sys_sram6: self.clk_sys_sram6(),
            clk_sys_sram7: self.clk_sys_sram7(),
            clk_sys_sram8: self.clk_sys_sram8(),
            clk_sys_sram9: self.clk_sys_sram9(),
            clk_sys_syscfg: self.clk_sys_syscfg(),
            clk_sys_sysinfo: self.clk_sys_sysinfo(),
            clk_sys_tbman: self.clk_sys_tbman(),
            clk_ref_ticks: self.clk_ref_ticks(),
            clk_sys_ticks: self.clk_sys_ticks(),
            clk_sys_timer0: self.clk_sys_timer0(),
            clk_sys_timer1: self.clk_sys_timer1(),
            clk_sys_trng: self.clk_sys_trng(),
            clk_peri_uart0: self.clk_peri_uart0(),
            clk_sys_uart0: self.clk_sys_uart0(),
            clk_peri_uart1: self.clk_peri_uart1(),
            clk_sys_uart1: self.clk_sys_uart1(),
            clk_sys_usbctrl: self.clk_sys_usbctrl(),
            clk_usb: self.clk_usb(),
            clk_sys_watchdog: self.clk_sys_watchdog(),
            clk_sys_xip: self.clk_sys_xip(),
            clk_sys_xosc: self.clk_sys_xosc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "enable clock in wake mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeEn0(pub u32);
impl WakeEn0 {
    #[inline(always)]
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_accessctrl(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_accessctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_adc(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_bootram(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_bootram(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_glitch_detector(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_glitch_detector(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_hstx(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_hstx(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_hstx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_ref_otp(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_otp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_otp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pio2(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pio2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_ref_powman(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_sys_powman(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sha256(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sha256(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for WakeEn0 {
    #[inline(always)]
    fn default() -> WakeEn0 {
        WakeEn0(0)
    }
}
impl core::fmt::Debug for WakeEn0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeEn0")
            .field("clk_sys_clocks", &self.clk_sys_clocks())
            .field("clk_sys_accessctrl", &self.clk_sys_accessctrl())
            .field("clk_adc", &self.clk_adc())
            .field("clk_sys_adc", &self.clk_sys_adc())
            .field("clk_sys_bootram", &self.clk_sys_bootram())
            .field("clk_sys_busctrl", &self.clk_sys_busctrl())
            .field("clk_sys_busfabric", &self.clk_sys_busfabric())
            .field("clk_sys_dma", &self.clk_sys_dma())
            .field("clk_sys_glitch_detector", &self.clk_sys_glitch_detector())
            .field("clk_hstx", &self.clk_hstx())
            .field("clk_sys_hstx", &self.clk_sys_hstx())
            .field("clk_sys_i2c0", &self.clk_sys_i2c0())
            .field("clk_sys_i2c1", &self.clk_sys_i2c1())
            .field("clk_sys_io", &self.clk_sys_io())
            .field("clk_sys_jtag", &self.clk_sys_jtag())
            .field("clk_ref_otp", &self.clk_ref_otp())
            .field("clk_sys_otp", &self.clk_sys_otp())
            .field("clk_sys_pads", &self.clk_sys_pads())
            .field("clk_sys_pio0", &self.clk_sys_pio0())
            .field("clk_sys_pio1", &self.clk_sys_pio1())
            .field("clk_sys_pio2", &self.clk_sys_pio2())
            .field("clk_sys_pll_sys", &self.clk_sys_pll_sys())
            .field("clk_sys_pll_usb", &self.clk_sys_pll_usb())
            .field("clk_ref_powman", &self.clk_ref_powman())
            .field("clk_sys_powman", &self.clk_sys_powman())
            .field("clk_sys_pwm", &self.clk_sys_pwm())
            .field("clk_sys_resets", &self.clk_sys_resets())
            .field("clk_sys_rom", &self.clk_sys_rom())
            .field("clk_sys_rosc", &self.clk_sys_rosc())
            .field("clk_sys_psm", &self.clk_sys_psm())
            .field("clk_sys_sha256", &self.clk_sys_sha256())
            .field("clk_sys_sio", &self.clk_sys_sio())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeEn0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WakeEn0 {
            clk_sys_clocks: bool,
            clk_sys_accessctrl: bool,
            clk_adc: bool,
            clk_sys_adc: bool,
            clk_sys_bootram: bool,
            clk_sys_busctrl: bool,
            clk_sys_busfabric: bool,
            clk_sys_dma: bool,
            clk_sys_glitch_detector: bool,
            clk_hstx: bool,
            clk_sys_hstx: bool,
            clk_sys_i2c0: bool,
            clk_sys_i2c1: bool,
            clk_sys_io: bool,
            clk_sys_jtag: bool,
            clk_ref_otp: bool,
            clk_sys_otp: bool,
            clk_sys_pads: bool,
            clk_sys_pio0: bool,
            clk_sys_pio1: bool,
            clk_sys_pio2: bool,
            clk_sys_pll_sys: bool,
            clk_sys_pll_usb: bool,
            clk_ref_powman: bool,
            clk_sys_powman: bool,
            clk_sys_pwm: bool,
            clk_sys_resets: bool,
            clk_sys_rom: bool,
            clk_sys_rosc: bool,
            clk_sys_psm: bool,
            clk_sys_sha256: bool,
            clk_sys_sio: bool,
        }
        let proxy = WakeEn0 {
            clk_sys_clocks: self.clk_sys_clocks(),
            clk_sys_accessctrl: self.clk_sys_accessctrl(),
            clk_adc: self.clk_adc(),
            clk_sys_adc: self.clk_sys_adc(),
            clk_sys_bootram: self.clk_sys_bootram(),
            clk_sys_busctrl: self.clk_sys_busctrl(),
            clk_sys_busfabric: self.clk_sys_busfabric(),
            clk_sys_dma: self.clk_sys_dma(),
            clk_sys_glitch_detector: self.clk_sys_glitch_detector(),
            clk_hstx: self.clk_hstx(),
            clk_sys_hstx: self.clk_sys_hstx(),
            clk_sys_i2c0: self.clk_sys_i2c0(),
            clk_sys_i2c1: self.clk_sys_i2c1(),
            clk_sys_io: self.clk_sys_io(),
            clk_sys_jtag: self.clk_sys_jtag(),
            clk_ref_otp: self.clk_ref_otp(),
            clk_sys_otp: self.clk_sys_otp(),
            clk_sys_pads: self.clk_sys_pads(),
            clk_sys_pio0: self.clk_sys_pio0(),
            clk_sys_pio1: self.clk_sys_pio1(),
            clk_sys_pio2: self.clk_sys_pio2(),
            clk_sys_pll_sys: self.clk_sys_pll_sys(),
            clk_sys_pll_usb: self.clk_sys_pll_usb(),
            clk_ref_powman: self.clk_ref_powman(),
            clk_sys_powman: self.clk_sys_powman(),
            clk_sys_pwm: self.clk_sys_pwm(),
            clk_sys_resets: self.clk_sys_resets(),
            clk_sys_rom: self.clk_sys_rom(),
            clk_sys_rosc: self.clk_sys_rosc(),
            clk_sys_psm: self.clk_sys_psm(),
            clk_sys_sha256: self.clk_sys_sha256(),
            clk_sys_sio: self.clk_sys_sio(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "enable clock in wake mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct WakeEn1(pub u32);
impl WakeEn1 {
    #[inline(always)]
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram6(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram7(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram8(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sram9(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sram9(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn clk_ref_ticks(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_ref_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn clk_sys_ticks(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_ticks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer0(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn clk_sys_timer1(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_timer1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn clk_sys_trng(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_trng(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn clk_usb(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
}
impl Default for WakeEn1 {
    #[inline(always)]
    fn default() -> WakeEn1 {
        WakeEn1(0)
    }
}
impl core::fmt::Debug for WakeEn1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WakeEn1")
            .field("clk_peri_spi0", &self.clk_peri_spi0())
            .field("clk_sys_spi0", &self.clk_sys_spi0())
            .field("clk_peri_spi1", &self.clk_peri_spi1())
            .field("clk_sys_spi1", &self.clk_sys_spi1())
            .field("clk_sys_sram0", &self.clk_sys_sram0())
            .field("clk_sys_sram1", &self.clk_sys_sram1())
            .field("clk_sys_sram2", &self.clk_sys_sram2())
            .field("clk_sys_sram3", &self.clk_sys_sram3())
            .field("clk_sys_sram4", &self.clk_sys_sram4())
            .field("clk_sys_sram5", &self.clk_sys_sram5())
            .field("clk_sys_sram6", &self.clk_sys_sram6())
            .field("clk_sys_sram7", &self.clk_sys_sram7())
            .field("clk_sys_sram8", &self.clk_sys_sram8())
            .field("clk_sys_sram9", &self.clk_sys_sram9())
            .field("clk_sys_syscfg", &self.clk_sys_syscfg())
            .field("clk_sys_sysinfo", &self.clk_sys_sysinfo())
            .field("clk_sys_tbman", &self.clk_sys_tbman())
            .field("clk_ref_ticks", &self.clk_ref_ticks())
            .field("clk_sys_ticks", &self.clk_sys_ticks())
            .field("clk_sys_timer0", &self.clk_sys_timer0())
            .field("clk_sys_timer1", &self.clk_sys_timer1())
            .field("clk_sys_trng", &self.clk_sys_trng())
            .field("clk_peri_uart0", &self.clk_peri_uart0())
            .field("clk_sys_uart0", &self.clk_sys_uart0())
            .field("clk_peri_uart1", &self.clk_peri_uart1())
            .field("clk_sys_uart1", &self.clk_sys_uart1())
            .field("clk_sys_usbctrl", &self.clk_sys_usbctrl())
            .field("clk_usb", &self.clk_usb())
            .field("clk_sys_watchdog", &self.clk_sys_watchdog())
            .field("clk_sys_xip", &self.clk_sys_xip())
            .field("clk_sys_xosc", &self.clk_sys_xosc())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for WakeEn1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct WakeEn1 {
            clk_peri_spi0: bool,
            clk_sys_spi0: bool,
            clk_peri_spi1: bool,
            clk_sys_spi1: bool,
            clk_sys_sram0: bool,
            clk_sys_sram1: bool,
            clk_sys_sram2: bool,
            clk_sys_sram3: bool,
            clk_sys_sram4: bool,
            clk_sys_sram5: bool,
            clk_sys_sram6: bool,
            clk_sys_sram7: bool,
            clk_sys_sram8: bool,
            clk_sys_sram9: bool,
            clk_sys_syscfg: bool,
            clk_sys_sysinfo: bool,
            clk_sys_tbman: bool,
            clk_ref_ticks: bool,
            clk_sys_ticks: bool,
            clk_sys_timer0: bool,
            clk_sys_timer1: bool,
            clk_sys_trng: bool,
            clk_peri_uart0: bool,
            clk_sys_uart0: bool,
            clk_peri_uart1: bool,
            clk_sys_uart1: bool,
            clk_sys_usbctrl: bool,
            clk_usb: bool,
            clk_sys_watchdog: bool,
            clk_sys_xip: bool,
            clk_sys_xosc: bool,
        }
        let proxy = WakeEn1 {
            clk_peri_spi0: self.clk_peri_spi0(),
            clk_sys_spi0: self.clk_sys_spi0(),
            clk_peri_spi1: self.clk_peri_spi1(),
            clk_sys_spi1: self.clk_sys_spi1(),
            clk_sys_sram0: self.clk_sys_sram0(),
            clk_sys_sram1: self.clk_sys_sram1(),
            clk_sys_sram2: self.clk_sys_sram2(),
            clk_sys_sram3: self.clk_sys_sram3(),
            clk_sys_sram4: self.clk_sys_sram4(),
            clk_sys_sram5: self.clk_sys_sram5(),
            clk_sys_sram6: self.clk_sys_sram6(),
            clk_sys_sram7: self.clk_sys_sram7(),
            clk_sys_sram8: self.clk_sys_sram8(),
            clk_sys_sram9: self.clk_sys_sram9(),
            clk_sys_syscfg: self.clk_sys_syscfg(),
            clk_sys_sysinfo: self.clk_sys_sysinfo(),
            clk_sys_tbman: self.clk_sys_tbman(),
            clk_ref_ticks: self.clk_ref_ticks(),
            clk_sys_ticks: self.clk_sys_ticks(),
            clk_sys_timer0: self.clk_sys_timer0(),
            clk_sys_timer1: self.clk_sys_timer1(),
            clk_sys_trng: self.clk_sys_trng(),
            clk_peri_uart0: self.clk_peri_uart0(),
            clk_sys_uart0: self.clk_sys_uart0(),
            clk_peri_uart1: self.clk_peri_uart1(),
            clk_sys_uart1: self.clk_sys_uart1(),
            clk_sys_usbctrl: self.clk_sys_usbctrl(),
            clk_usb: self.clk_usb(),
            clk_sys_watchdog: self.clk_sys_watchdog(),
            clk_sys_xip: self.clk_sys_xip(),
            clk_sys_xosc: self.clk_sys_xosc(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
