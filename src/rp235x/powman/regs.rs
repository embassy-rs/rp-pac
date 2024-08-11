#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlarmTime15to0(pub u32);
impl AlarmTime15to0 {
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub const fn alarm_time_15to0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub fn set_alarm_time_15to0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AlarmTime15to0 {
    #[inline(always)]
    fn default() -> AlarmTime15to0 {
        AlarmTime15to0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlarmTime31to16(pub u32);
impl AlarmTime31to16 {
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub const fn alarm_time_31to16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub fn set_alarm_time_31to16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AlarmTime31to16 {
    #[inline(always)]
    fn default() -> AlarmTime31to16 {
        AlarmTime31to16(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlarmTime47to32(pub u32);
impl AlarmTime47to32 {
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub const fn alarm_time_47to32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub fn set_alarm_time_47to32(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AlarmTime47to32 {
    #[inline(always)]
    fn default() -> AlarmTime47to32 {
        AlarmTime47to32(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AlarmTime63to48(pub u32);
impl AlarmTime63to48 {
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub const fn alarm_time_63to48(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "This field must only be written when POWMAN_ALARM_ENAB=0"]
    #[inline(always)]
    pub fn set_alarm_time_63to48(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for AlarmTime63to48 {
    #[inline(always)]
    fn default() -> AlarmTime63to48 {
        AlarmTime63to48(0)
    }
}
#[doc = "Indicates a bad password has been used"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Badpasswd(pub u32);
impl Badpasswd {
    #[inline(always)]
    pub const fn badpasswd(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_badpasswd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Badpasswd {
    #[inline(always)]
    fn default() -> Badpasswd {
        Badpasswd(0)
    }
}
#[doc = "Brown-out Detection Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bod(pub u32);
impl Bod {
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
}
impl Default for Bod {
    #[inline(always)]
    fn default() -> Bod {
        Bod(0)
    }
}
#[doc = "Brown-out Detection Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodCtrl(pub u32);
impl BodCtrl {
    #[doc = "isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub const fn isolate(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "isolates the brown-out detection control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub fn set_isolate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for BodCtrl {
    #[inline(always)]
    fn default() -> BodCtrl {
        BodCtrl(0)
    }
}
#[doc = "Brown-out Detection Low Power Entry Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLpEntry(pub u32);
impl BodLpEntry {
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
}
impl Default for BodLpEntry {
    #[inline(always)]
    fn default() -> BodLpEntry {
        BodLpEntry(0)
    }
}
#[doc = "Brown-out Detection Low Power Exit Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BodLpExit(pub u32);
impl BodLpExit {
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "enable brown-out detection 0=not enabled, 1=enabled"]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "threshold select 00000 - 0.473V 00001 - 0.516V 00010 - 0.559V 00011 - 0.602V 00100 - 0.645VS 00101 - 0.688V 00110 - 0.731V 00111 - 0.774V 01000 - 0.817V 01001 - 0.860V (default) 01010 - 0.903V 01011 - 0.946V 01100 - 0.989V 01101 - 1.032V 01110 - 1.075V 01111 - 1.118V 10000 - 1.161 10001 - 1.204V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
}
impl Default for BodLpExit {
    #[inline(always)]
    fn default() -> BodLpExit {
        BodLpExit(0)
    }
}
#[doc = "Tell the bootrom to ignore the BOOT0..3 registers following the next RSM reset (e.g. the next core power down/up). If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by powering the core up and down. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the OTP BOOTDIS register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootdis(pub u32);
impl Bootdis {
    #[doc = "When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    pub const fn now(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When powman resets the RSM, the current value of BOOTDIS_NEXT is OR'd into BOOTDIS_NOW, and BOOTDIS_NEXT is cleared. The bootrom checks this flag before reading the BOOT0..3 registers. If it is set, the bootrom clears it, and ignores the BOOT registers. This prevents Secure software from diverting the boot path before a bootloader has had the chance to soft lock OTP pages containing sensitive data."]
    #[inline(always)]
    pub fn set_now(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
    #[inline(always)]
    pub const fn next(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "This flag always ORs writes into its current contents. It can be set but not cleared by software. The BOOTDIS_NEXT bit is OR'd into the BOOTDIS_NOW bit when the core is powered down. Simultaneously, the BOOTDIS_NEXT bit is cleared. Setting this bit means that the BOOT0..3 registers will be ignored following the next reset of the RSM by powman. This flag should be set by an early boot stage that has soft-locked OTP pages, to prevent later stages from unlocking it by power cycling."]
    #[inline(always)]
    pub fn set_next(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Bootdis {
    #[inline(always)]
    fn default() -> Bootdis {
        Bootdis(0)
    }
}
#[doc = "Chip reset control and status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChipReset(pub u32);
impl ChipReset {
    #[doc = "This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
    #[inline(always)]
    pub const fn double_tap(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "This flag is set by double-tapping RUN. It tells bootcode to go into the bootloader."]
    #[inline(always)]
    pub fn set_double_tap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
    #[inline(always)]
    pub const fn rescue_flag(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "This is set by a rescue reset from the RP-AP. Its purpose is to halt before the bootrom before booting from flash in order to recover from a boot lock-up. The debugger can then attach once the bootrom has been halted and flash some working code that does not lock up."]
    #[inline(always)]
    pub fn set_rescue_flag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Last reset was from the power-on reset This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_por(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the power-on reset This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_por(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Last reset was from the brown-out detection block This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_bor(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the brown-out detection block This resets: double_tap flag yes DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_bor(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Last reset was from the RUN pin This resets: double_tap flag no DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_run_low(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was from the RUN pin This resets: double_tap flag no DP yes RPAP yes rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_run_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Last reset was an reset request from the arm debugger This resets: double_tap flag no DP no RPAP no rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_dp_reset_req(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was an reset request from the arm debugger This resets: double_tap flag no DP no RPAP no rescue_flag yes timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_dp_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Last reset was a rescue reset from the debugger This resets: double_tap flag no DP no RPAP no rescue_flag no, it sets this flag timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_rescue(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a rescue reset from the debugger This resets: double_tap flag no DP no RPAP no rescue_flag no, it sets this flag timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_rescue(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power manager asynchronously This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_watchdog_reset_powman_async(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power manager asynchronously This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_watchdog_reset_powman_async(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power manager This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_watchdog_reset_powman(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power manager This resets: double_tap flag no DP no RPAP no rescue_flag no timer yes powman yes swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_watchdog_reset_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the switched-core This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_watchdog_reset_swcore(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the switched-core This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_watchdog_reset_swcore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Last reset was a switched core powerdown This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub const fn had_swcore_pd(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a switched core powerdown This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore yes psm yes then starts the power sequencer"]
    #[inline(always)]
    pub fn set_had_swcore_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Last reset was due to a power supply glitch This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub const fn had_glitch_detect(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was due to a power supply glitch This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn set_had_glitch_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Last reset was a system reset from the hazard debugger This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub const fn had_hzd_sys_reset_req(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a system reset from the hazard debugger This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn set_had_hzd_sys_reset_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power-on state machine This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub const fn had_watchdog_reset_rsm(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Last reset was a watchdog timeout which was configured to reset the power-on state machine This resets: double_tap flag no DP no RPAP no rescue_flag no timer no powman no swcore no psm yes and does not change the power state"]
    #[inline(always)]
    pub fn set_had_watchdog_reset_rsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for ChipReset {
    #[inline(always)]
    fn default() -> ChipReset {
        ChipReset(0)
    }
}
#[doc = "Indicates current powerup request state pwrup events can be cleared by removing the enable from the pwrup register. The alarm pwrup req can be cleared by clearing timer.alarm_enab 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct CurrentPwrupReq(pub u32);
impl CurrentPwrupReq {
    #[inline(always)]
    pub const fn current_pwrup_req(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub fn set_current_pwrup_req(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for CurrentPwrupReq {
    #[inline(always)]
    fn default() -> CurrentPwrupReq {
        CurrentPwrupReq(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgPwrcfg(pub u32);
impl DbgPwrcfg {
    #[doc = "Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
    #[inline(always)]
    pub const fn ignore(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Ignore pwrup req from debugger. If pwrup req is asserted then this will prevent power down and set powerdown blocked. Set ignore to stop paying attention to pwrup_req"]
    #[inline(always)]
    pub fn set_ignore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for DbgPwrcfg {
    #[inline(always)]
    fn default() -> DbgPwrcfg {
        DbgPwrcfg(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgconfig(pub u32);
impl Dbgconfig {
    #[doc = "Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
    #[inline(always)]
    pub const fn dp_instid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Configure DP instance ID for SWD multidrop selection. Recommend that this is NOT changed until you require debug access in multi-chip environment"]
    #[inline(always)]
    pub fn set_dp_instid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Dbgconfig {
    #[inline(always)]
    fn default() -> Dbgconfig {
        Dbgconfig(0)
    }
}
#[doc = "Configures a gpio as a power mode aware control output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtCtrl0(pub u32);
impl ExtCtrl0 {
    #[doc = "selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    pub const fn gpio_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    pub fn set_gpio_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn init_state(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_init_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "output level when entering the low power state"]
    #[inline(always)]
    pub const fn lp_entry_state(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "output level when entering the low power state"]
    #[inline(always)]
    pub fn set_lp_entry_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "output level when exiting the low power state"]
    #[inline(always)]
    pub const fn lp_exit_state(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "output level when exiting the low power state"]
    #[inline(always)]
    pub fn set_lp_exit_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for ExtCtrl0 {
    #[inline(always)]
    fn default() -> ExtCtrl0 {
        ExtCtrl0(0)
    }
}
#[doc = "Configures a gpio as a power mode aware control output"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtCtrl1(pub u32);
impl ExtCtrl1 {
    #[doc = "selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    pub const fn gpio_select(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "selects from gpio 0->30 set to 31 to disable this feature"]
    #[inline(always)]
    pub fn set_gpio_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[inline(always)]
    pub const fn init(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_init(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn init_state(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_init_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "output level when entering the low power state"]
    #[inline(always)]
    pub const fn lp_entry_state(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "output level when entering the low power state"]
    #[inline(always)]
    pub fn set_lp_entry_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "output level when exiting the low power state"]
    #[inline(always)]
    pub const fn lp_exit_state(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "output level when exiting the low power state"]
    #[inline(always)]
    pub fn set_lp_exit_state(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
}
impl Default for ExtCtrl1 {
    #[inline(always)]
    fn default() -> ExtCtrl1 {
        ExtCtrl1(0)
    }
}
#[doc = "Select a GPIO to use as a time reference, the source can be used to drive the low power clock at 32kHz, or to provide a 1ms tick to the timer, or provide a 1Hz tick to the timer. The tick selection is controlled by the POWMAN_TIMER register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExtTimeRef(pub u32);
impl ExtTimeRef {
    #[doc = "0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
    #[inline(always)]
    pub const fn source_sel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "0 -> gpio12 1 -> gpio20 2 -> gpio14 3 -> gpio22"]
    #[inline(always)]
    pub fn set_source_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub const fn drive_lpck(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Use the selected GPIO to drive the 32kHz low power clock, in place of LPOSC. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_drive_lpck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for ExtTimeRef {
    #[inline(always)]
    fn default() -> ExtTimeRef {
        ExtTimeRef(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte(pub u32);
impl Inte {
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_output_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub fn set_state_req_ignored(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Inte {
    #[inline(always)]
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf(pub u32);
impl Intf {
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_output_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub fn set_state_req_ignored(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Intf {
    #[inline(always)]
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_output_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub fn set_state_req_ignored(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints(pub u32);
impl Ints {
    #[inline(always)]
    pub const fn vreg_output_low(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vreg_output_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn timer(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub const fn state_req_ignored(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.req_ignored"]
    #[inline(always)]
    pub fn set_state_req_ignored(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Source is state.pwrup_while_waiting"]
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Ints {
    #[inline(always)]
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "Indicates which pwrup source triggered the last switched-core power up 0 = chip reset, for the source of the last reset see POWMAN_CHIP_RESET 1 = pwrup0 2 = pwrup1 3 = pwrup2 4 = pwrup3 5 = coresight_pwrup 6 = alarm_pwrup"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LastSwcorePwrup(pub u32);
impl LastSwcorePwrup {
    #[inline(always)]
    pub const fn last_swcore_pwrup(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[inline(always)]
    pub fn set_last_swcore_pwrup(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
}
impl Default for LastSwcorePwrup {
    #[inline(always)]
    fn default() -> LastSwcorePwrup {
        LastSwcorePwrup(0)
    }
}
#[doc = "Low power oscillator control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Lposc(pub u32);
impl Lposc {
    #[doc = "This feature has been removed"]
    #[inline(always)]
    pub const fn mode(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[doc = "This feature has been removed"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[doc = "Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
    #[inline(always)]
    pub const fn trim(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x3f;
        val as u8
    }
    #[doc = "Frequency trim - the trim step is typically 1% of the reset frequency, but can be up to 3%"]
    #[inline(always)]
    pub fn set_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 4usize)) | (((val as u32) & 0x3f) << 4usize);
    }
}
impl Default for Lposc {
    #[inline(always)]
    fn default() -> Lposc {
        Lposc(0)
    }
}
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the LPOSC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LposcFreqKhzFrac(pub u32);
impl LposcFreqKhzFrac {
    #[doc = "Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub const fn lposc_freq_khz_frac(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fractional component of the LPOSC or GPIO clock source frequency in kHz. Default = 0.768 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub fn set_lposc_freq_khz_frac(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for LposcFreqKhzFrac {
    #[inline(always)]
    fn default() -> LposcFreqKhzFrac {
        LposcFreqKhzFrac(0)
    }
}
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the LPOSC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LposcFreqKhzInt(pub u32);
impl LposcFreqKhzInt {
    #[doc = "Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub const fn lposc_freq_khz_int(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Integer component of the LPOSC or GPIO clock source frequency in kHz. Default = 32 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=1"]
    #[inline(always)]
    pub fn set_lposc_freq_khz_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for LposcFreqKhzInt {
    #[inline(always)]
    fn default() -> LposcFreqKhzInt {
        LposcFreqKhzInt(0)
    }
}
#[doc = "power state machine delays"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowDelay(pub u32);
impl PowDelay {
    #[doc = "timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub const fn swcore_step(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "timing between the swcore power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn set_swcore_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub const fn xip_step(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "timing between the xip power state machine steps measured in units of the lposc period, 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn set_xip_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub const fn sram_step(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "timing between the sram0 and sram1 power state machine steps measured in units of the powman tick period (>=1us), 0 gives a delay of 1 unit"]
    #[inline(always)]
    pub fn set_sram_step(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for PowDelay {
    #[inline(always)]
    fn default() -> PowDelay {
        PowDelay(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PowFastdiv(pub u32);
impl PowFastdiv {
    #[doc = "divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
    #[inline(always)]
    pub const fn pow_fastdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[doc = "divides the POWMAN clock to provide a tick for the delay module and state machines when clk_pow is running from the slow clock it is not divided when clk_pow is running from the fast clock it is divided by tick_div"]
    #[inline(always)]
    pub fn set_pow_fastdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for PowFastdiv {
    #[inline(always)]
    fn default() -> PowFastdiv {
        PowFastdiv(0)
    }
}
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrup0(pub u32);
impl Pwrup0 {
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Pwrup0direction {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pwrup0direction::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_direction(&mut self, val: super::vals::Pwrup0direction) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pwrup0mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pwrup0mode::from_bits(val as u8)
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Pwrup0mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub fn set_raw_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Pwrup0 {
    #[inline(always)]
    fn default() -> Pwrup0 {
        Pwrup0(0)
    }
}
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrup1(pub u32);
impl Pwrup1 {
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Pwrup1direction {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pwrup1direction::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_direction(&mut self, val: super::vals::Pwrup1direction) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pwrup1mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pwrup1mode::from_bits(val as u8)
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Pwrup1mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub fn set_raw_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Pwrup1 {
    #[inline(always)]
    fn default() -> Pwrup1 {
        Pwrup1(0)
    }
}
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrup2(pub u32);
impl Pwrup2 {
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Pwrup2direction {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pwrup2direction::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_direction(&mut self, val: super::vals::Pwrup2direction) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pwrup2mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pwrup2mode::from_bits(val as u8)
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Pwrup2mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub fn set_raw_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Pwrup2 {
    #[inline(always)]
    fn default() -> Pwrup2 {
        Pwrup2(0)
    }
}
#[doc = "4 GPIO powerup events can be configured to wake the chip up from a low power state. The pwrups are level/edge sensitive and can be set to trigger on a high/rising or low/falling event The number of gpios available depends on the package option. An invalid selection will be ignored source = 0 selects gpio0 . . source = 47 selects gpio47 source = 48 selects qspi_ss source = 49 selects qspi_sd0 source = 50 selects qspi_sd1 source = 51 selects qspi_sd2 source = 52 selects qspi_sd3 source = 53 selects qspi_sclk level = 0 triggers the pwrup when the source is low level = 1 triggers the pwrup when the source is high"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pwrup3(pub u32);
impl Pwrup3 {
    #[inline(always)]
    pub const fn source(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[inline(always)]
    pub fn set_source(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to enable the wakeup source. Set to 0 to disable the wakeup source and clear a pending wakeup event. If using edge detect a latched edge needs to be cleared by writing 1 to the status register also."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn direction(&self) -> super::vals::Pwrup3direction {
        let val = (self.0 >> 7usize) & 0x01;
        super::vals::Pwrup3direction::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_direction(&mut self, val: super::vals::Pwrup3direction) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val.to_bits() as u32) & 0x01) << 7usize);
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub const fn mode(&self) -> super::vals::Pwrup3mode {
        let val = (self.0 >> 8usize) & 0x01;
        super::vals::Pwrup3mode::from_bits(val as u8)
    }
    #[doc = "Edge or level detect. Edge will detect a 0 to 1 transition (or 1 to 0 transition). Level will detect a 1 or 0. Both types of event get latched into the current_pwrup_req register."]
    #[inline(always)]
    pub fn set_mode(&mut self, val: super::vals::Pwrup3mode) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val.to_bits() as u32) & 0x01) << 8usize);
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub const fn status(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Status of gpio wakeup. Write to 1 to clear a latched edge detect."]
    #[inline(always)]
    pub fn set_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub const fn raw_status(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Value of selected gpio pin (only if enable == 1)"]
    #[inline(always)]
    pub fn set_raw_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Pwrup3 {
    #[inline(always)]
    fn default() -> Pwrup3 {
        Pwrup3(0)
    }
}
#[doc = "For configuration of the power sequencer Writes are ignored while POWMAN_STATE_CHANGING=1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SeqCfg(pub u32);
impl SeqCfg {
    #[doc = "Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub const fn hw_pwrup_sram1(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the power state of SRAM1 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub fn set_hw_pwrup_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub const fn hw_pwrup_sram0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Specifies the power state of SRAM0 when powering up swcore from a low power state (P1.xxx) to a high power state (P0.0xx). 0=power-up 1=no change"]
    #[inline(always)]
    pub fn set_hw_pwrup_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub const fn use_vreg_lp(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 0 to prevent automatic switching to vreg low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn set_use_vreg_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub const fn use_vreg_hp(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 0 to prevent automatic switching to vreg high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub fn set_use_vreg_hp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub const fn use_bod_lp(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 0 to prevent automatic switching to bod low power mode when switched-core is powered down This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn set_use_bod_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub const fn use_bod_hp(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 0 to prevent automatic switching to bod high power mode when switched-core is powered up This setting takes effect when the swcore is next powered up"]
    #[inline(always)]
    pub fn set_use_bod_hp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub const fn run_lposc_in_lp(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 0 to stop the low power osc when the switched-core is powered down, which is unwise if using it to clock the timer This setting takes effect when the swcore is next powered down"]
    #[inline(always)]
    pub fn set_run_lposc_in_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
    #[inline(always)]
    pub const fn use_fast_powck(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "selects the reference clock (clk_ref) as the source of the POWMAN clock when switched-core is powered. The POWMAN clock always switches to the slow clock (lposc) when switched-core is powered down because the fast clock stops running. 0 always run the POWMAN clock from the slow clock (lposc) 1 run the POWMAN clock from the fast clock when available This setting takes effect when a power up sequence is next run"]
    #[inline(always)]
    pub fn set_use_fast_powck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Indicates the voltage regulator (VREG) mode 0 = VREG high power mode which is the default 1 = VREG low power mode"]
    #[inline(always)]
    pub const fn using_vreg_lp(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the voltage regulator (VREG) mode 0 = VREG high power mode which is the default 1 = VREG low power mode"]
    #[inline(always)]
    pub fn set_using_vreg_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Indicates the brown-out detector (BOD) mode 0 = BOD high power mode which is the default 1 = BOD low power mode"]
    #[inline(always)]
    pub const fn using_bod_lp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates the brown-out detector (BOD) mode 0 = BOD high power mode which is the default 1 = BOD low power mode"]
    #[inline(always)]
    pub fn set_using_bod_lp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "0 indicates the POWMAN clock is running from the low power oscillator (32kHz) 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)"]
    #[inline(always)]
    pub const fn using_fast_powck(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "0 indicates the POWMAN clock is running from the low power oscillator (32kHz) 1 indicates the POWMAN clock is running from the reference clock (2-50MHz)"]
    #[inline(always)]
    pub fn set_using_fast_powck(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for SeqCfg {
    #[inline(always)]
    fn default() -> SeqCfg {
        SeqCfg(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetTime15to0(pub u32);
impl SetTime15to0 {
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub const fn set_time_15to0(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_set_time_15to0(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SetTime15to0 {
    #[inline(always)]
    fn default() -> SetTime15to0 {
        SetTime15to0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetTime31to16(pub u32);
impl SetTime31to16 {
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub const fn set_time_31to16(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_set_time_31to16(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SetTime31to16 {
    #[inline(always)]
    fn default() -> SetTime31to16 {
        SetTime31to16(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetTime47to32(pub u32);
impl SetTime47to32 {
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub const fn set_time_47to32(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_set_time_47to32(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SetTime47to32 {
    #[inline(always)]
    fn default() -> SetTime47to32 {
        SetTime47to32(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetTime63to48(pub u32);
impl SetTime63to48 {
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub const fn set_time_63to48(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "For setting the time, do not use for reading the time, use POWMAN_READ_TIME_UPPER and POWMAN_READ_TIME_LOWER. This field must only be written when POWMAN_TIMER_RUN=0"]
    #[inline(always)]
    pub fn set_set_time_63to48(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SetTime63to48 {
    #[inline(always)]
    fn default() -> SetTime63to48 {
        SetTime63to48(0)
    }
}
#[doc = "This register controls the power state of the 4 power domains. The current power state is indicated in POWMAN_STATE_CURRENT which is read-only. To change the state, write to POWMAN_STATE_REQ. The coding of POWMAN_STATE_CURRENT & POWMAN_STATE_REQ corresponds to the power states defined in the datasheet: bit 3 = SWCORE bit 2 = XIP cache bit 1 = SRAM0 bit 0 = SRAM1 0 = powered up 1 = powered down When POWMAN_STATE_REQ is written, the POWMAN_STATE_WAITING flag is set while the Power Manager determines what is required. If an invalid transition is requested the Power Manager will still register the request in POWMAN_STATE_REQ but will also set the POWMAN_BAD_REQ flag. It will then implement the power-up requests and ignore the power down requests. To do nothing would risk entering an unrecoverable lock-up state. Invalid requests are: any combination of power up and power down requests any request that results in swcore boing powered and xip unpowered If the request is to power down the switched-core domain then POWMAN_STATE_WAITING stays active until the processors halt. During this time the POWMAN_STATE_REQ field can be re-written to change or cancel the request. When the power state transition begins the POWMAN_STATE_WAITING_flag is cleared, the POWMAN_STATE_CHANGING flag is set and POWMAN register writes are ignored until the transition completes."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct State(pub u32);
impl State {
    #[inline(always)]
    pub const fn current(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_current(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn req(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_req(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[inline(always)]
    pub const fn req_ignored(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_req_ignored(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
    #[inline(always)]
    pub const fn pwrup_while_waiting(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Request ignored because of a pending pwrup request. See current_pwrup_req. Note this blocks powering up AND powering down."]
    #[inline(always)]
    pub fn set_pwrup_while_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Bad software initiated state request. No action taken."]
    #[inline(always)]
    pub const fn bad_sw_req(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Bad software initiated state request. No action taken."]
    #[inline(always)]
    pub fn set_bad_sw_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Bad hardware initiated state request. Went back to state 0 (i.e. everything powered up)"]
    #[inline(always)]
    pub const fn bad_hw_req(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Bad hardware initiated state request. Went back to state 0 (i.e. everything powered up)"]
    #[inline(always)]
    pub fn set_bad_hw_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn changing(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_changing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
}
impl Default for State {
    #[inline(always)]
    fn default() -> State {
        State(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timer(pub u32);
impl Timer {
    #[doc = "Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
    #[inline(always)]
    pub const fn nonsec_write(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Control whether Non-secure software can write to the timer registers. All other registers are hardwired to be inaccessible to Non-secure."]
    #[inline(always)]
    pub fn set_nonsec_write(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
    #[inline(always)]
    pub const fn run(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Timer enable. Setting this bit causes the timer to begin counting up from its current value. Clearing this bit stops the timer from counting. Before enabling the timer, set the POWMAN_LPOSC_FREQ* and POWMAN_XOSC_FREQ* registers to configure the count rate, and initialise the current time by writing to SET_TIME_63TO48 through SET_TIME_15TO0. You must not write to the SET_TIME_x registers when the timer is running. Once configured, start the timer by setting POWMAN_TIMER_RUN=1. This will start the timer running from the LPOSC. When the XOSC is available switch the reference clock to XOSC then select it as the timer clock by setting POWMAN_TIMER_USE_XOSC=1"]
    #[inline(always)]
    pub fn set_run(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time."]
    #[inline(always)]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the timer, does not disable the timer and does not affect the alarm. This control can be written at any time."]
    #[inline(always)]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enables the alarm. The alarm must be disabled while writing the alarm time."]
    #[inline(always)]
    pub const fn alarm_enab(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enables the alarm. The alarm must be disabled while writing the alarm time."]
    #[inline(always)]
    pub fn set_alarm_enab(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Alarm wakes the chip from low power mode"]
    #[inline(always)]
    pub const fn pwrup_on_alarm(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm wakes the chip from low power mode"]
    #[inline(always)]
    pub fn set_pwrup_on_alarm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Alarm has fired. Write to 1 to clear the alarm."]
    #[inline(always)]
    pub const fn alarm(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Alarm has fired. Write to 1 to clear the alarm."]
    #[inline(always)]
    pub fn set_alarm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Switch to lposc as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub const fn use_lposc(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Switch to lposc as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub fn set_use_lposc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "switch to xosc as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub const fn use_xosc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "switch to xosc as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub fn set_use_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "switch to gpio as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub const fn use_gpio_1khz(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "switch to gpio as the source of the 1kHz timer tick"]
    #[inline(always)]
    pub fn set_use_gpio_1khz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
    #[inline(always)]
    pub const fn use_gpio_1hz(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Selects the gpio source as the reference for the sec counter. The msec counter will continue to use the lposc or xosc reference."]
    #[inline(always)]
    pub fn set_use_gpio_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Timer is running from xosc"]
    #[inline(always)]
    pub const fn using_xosc(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Timer is running from xosc"]
    #[inline(always)]
    pub fn set_using_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Timer is running from lposc"]
    #[inline(always)]
    pub const fn using_lposc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Timer is running from lposc"]
    #[inline(always)]
    pub fn set_using_lposc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Timer is running from a 1khz gpio source"]
    #[inline(always)]
    pub const fn using_gpio_1khz(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Timer is running from a 1khz gpio source"]
    #[inline(always)]
    pub fn set_using_gpio_1khz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Timer is synchronised to a 1hz gpio source"]
    #[inline(always)]
    pub const fn using_gpio_1hz(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Timer is synchronised to a 1hz gpio source"]
    #[inline(always)]
    pub fn set_using_gpio_1hz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for Timer {
    #[inline(always)]
    fn default() -> Timer {
        Timer(0)
    }
}
#[doc = "Voltage Regulator Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Vreg(pub u32);
impl Vreg {
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
    #[doc = "regulator state is being updated writes to the vreg register will be ignored when this field is set"]
    #[inline(always)]
    pub const fn update_in_progress(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "regulator state is being updated writes to the vreg register will be ignored when this field is set"]
    #[inline(always)]
    pub fn set_update_in_progress(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Vreg {
    #[inline(always)]
    fn default() -> Vreg {
        Vreg(0)
    }
}
#[doc = "Voltage Regulator Control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregCtrl(pub u32);
impl VregCtrl {
    #[doc = "high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
    #[inline(always)]
    pub const fn ht_th(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x07;
        val as u8
    }
    #[doc = "high temperature protection threshold regulator power transistors are disabled when junction temperature exceeds threshold 000 - 100C 001 - 105C 010 - 110C 011 - 115C 100 - 120C 101 - 125C 110 - 135C 111 - 150C"]
    #[inline(always)]
    pub fn set_ht_th(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 4usize)) | (((val as u32) & 0x07) << 4usize);
    }
    #[doc = "0=not disabled, 1=enabled"]
    #[inline(always)]
    pub const fn disable_voltage_limit(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "0=not disabled, 1=enabled"]
    #[inline(always)]
    pub fn set_disable_voltage_limit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub const fn isolate(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "isolates the VREG control interface 0 - not isolated (default) 1 - isolated"]
    #[inline(always)]
    pub fn set_isolate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
    #[inline(always)]
    pub const fn unlock(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "unlocks the VREG control interface after power up 0 - Locked (default) 1 - Unlocked It cannot be relocked when it is unlocked."]
    #[inline(always)]
    pub fn set_unlock(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
    #[inline(always)]
    pub const fn rst_n(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "returns the regulator to its startup settings 0 - reset 1 - not reset (default)"]
    #[inline(always)]
    pub fn set_rst_n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for VregCtrl {
    #[inline(always)]
    fn default() -> VregCtrl {
        VregCtrl(0)
    }
}
#[doc = "Voltage Regulator Low Power Entry Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregLpEntry(pub u32);
impl VregLpEntry {
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "selects either normal (switching) mode or low power (linear) mode low power mode can only be selected for output voltages up to 1.3V 0 = normal mode (switching) 1 = low power mode (linear)"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "selects either normal (switching) mode or low power (linear) mode low power mode can only be selected for output voltages up to 1.3V 0 = normal mode (switching) 1 = low power mode (linear)"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
}
impl Default for VregLpEntry {
    #[inline(always)]
    fn default() -> VregLpEntry {
        VregLpEntry(0)
    }
}
#[doc = "Voltage Regulator Low Power Exit Settings"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregLpExit(pub u32);
impl VregLpExit {
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub const fn hiz(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "high impedance mode select 0=not in high impedance mode, 1=in high impedance mode"]
    #[inline(always)]
    pub fn set_hiz(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "selects either normal (switching) mode or low power (linear) mode low power mode can only be selected for output voltages up to 1.3V 0 = normal mode (switching) 1 = low power mode (linear)"]
    #[inline(always)]
    pub const fn mode(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "selects either normal (switching) mode or low power (linear) mode low power mode can only be selected for output voltages up to 1.3V 0 = normal mode (switching) 1 = low power mode (linear)"]
    #[inline(always)]
    pub fn set_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub const fn vsel(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x1f;
        val as u8
    }
    #[doc = "output voltage select the regulator output voltage is limited to 1.3V unless the voltage limit is disabled using the disable_voltage_limit field in the vreg_ctrl register 00000 - 0.55V 00001 - 0.60V 00010 - 0.65V 00011 - 0.70V 00100 - 0.75V 00101 - 0.80V 00110 - 0.85V 00111 - 0.90V 01000 - 0.95V 01001 - 1.00V 01010 - 1.05V 01011 - 1.10V (default) 01100 - 1.15V 01101 - 1.20V 01110 - 1.25V 01111 - 1.30V 10000 - 1.35V 10001 - 1.40V 10010 - 1.50V 10011 - 1.60V 10100 - 1.65V 10101 - 1.70V 10110 - 1.80V 10111 - 1.90V 11000 - 2.00V 11001 - 2.35V 11010 - 2.50V 11011 - 2.65V 11100 - 2.80V 11101 - 3.00V 11110 - 3.15V 11111 - 3.30V"]
    #[inline(always)]
    pub fn set_vsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 4usize)) | (((val as u32) & 0x1f) << 4usize);
    }
}
impl Default for VregLpExit {
    #[inline(always)]
    fn default() -> VregLpExit {
        VregLpExit(0)
    }
}
#[doc = "Voltage Regulator Status"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct VregSts(pub u32);
impl VregSts {
    #[doc = "startup status 0=startup complete, 1=starting up"]
    #[inline(always)]
    pub const fn startup(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "startup status 0=startup complete, 1=starting up"]
    #[inline(always)]
    pub fn set_startup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "output regulation status 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub const fn vout_ok(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "output regulation status 0=not in regulation, 1=in regulation"]
    #[inline(always)]
    pub fn set_vout_ok(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for VregSts {
    #[inline(always)]
    fn default() -> VregSts {
        VregSts(0)
    }
}
#[doc = "Allows a watchdog reset to reset the internal state of powman in addition to the power-on state machine (PSM). Note that powman ignores watchdog resets that do not select at least the CLOCKS stage or earlier stages in the PSM. If using these bits, it's recommended to set PSM_WDSEL to all-ones in addition to the desired bits in this register. Failing to select CLOCKS or earlier will result in the POWMAN_WDSEL register having no effect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wdsel(pub u32);
impl Wdsel {
    #[doc = "If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
    #[inline(always)]
    pub const fn reset_powman_async(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core domain and run the full power-on state machine (PSM) sequence This does not rely on clk_ref running"]
    #[inline(always)]
    pub fn set_reset_powman_async(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
    #[inline(always)]
    pub const fn reset_powman(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, a watchdog reset will restore powman defaults, reset the timer, reset the switched core power domain and run the full power-on state machine (PSM) sequence This relies on clk_ref running. Use reset_powman_async if that may not be true"]
    #[inline(always)]
    pub fn set_reset_powman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
    #[inline(always)]
    pub const fn reset_swcore(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, a watchdog reset will reset the switched core power domain and run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a power-on reset for the switched core power domain"]
    #[inline(always)]
    pub fn set_reset_swcore(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
    #[inline(always)]
    pub const fn reset_rsm(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "If set to 1, a watchdog reset will run the full power-on state machine (PSM) sequence From a user perspective it is the same as setting RSM_WDSEL_PROC_COLD From a hardware debug perspective it has the same effect as a reset from a glitch detector"]
    #[inline(always)]
    pub fn set_reset_rsm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
}
impl Default for Wdsel {
    #[inline(always)]
    fn default() -> Wdsel {
        Wdsel(0)
    }
}
#[doc = "Informs the AON Timer of the fractional component of the clock frequency when running off the XOSC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XoscFreqKhzFrac(pub u32);
impl XoscFreqKhzFrac {
    #[doc = "Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub const fn xosc_freq_khz_frac(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Fractional component of the XOSC frequency in kHz. This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub fn set_xosc_freq_khz_frac(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for XoscFreqKhzFrac {
    #[inline(always)]
    fn default() -> XoscFreqKhzFrac {
        XoscFreqKhzFrac(0)
    }
}
#[doc = "Informs the AON Timer of the integer component of the clock frequency when running off the XOSC."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XoscFreqKhzInt(pub u32);
impl XoscFreqKhzInt {
    #[doc = "Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub const fn xosc_freq_khz_int(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Integer component of the XOSC frequency in kHz. Default = 12000 Must be >1 This field must only be written when POWMAN_TIMER_RUN=0 or POWMAN_TIMER_USING_XOSC=0"]
    #[inline(always)]
    pub fn set_xosc_freq_khz_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for XoscFreqKhzInt {
    #[inline(always)]
    fn default() -> XoscFreqKhzInt {
        XoscFreqKhzInt(0)
    }
}
