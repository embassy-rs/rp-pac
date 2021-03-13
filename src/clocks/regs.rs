use crate::generic::*;
#[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Delay(pub u32);
impl Fc0Delay {
    pub const fn fc0_delay(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x07;
        val as u8
    }
    pub fn set_fc0_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0u32)) | (((val as u32) & 0x07) << 0u32);
    }
}
impl Default for Fc0Delay {
    fn default() -> Fc0Delay {
        Fc0Delay(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout2Ctrl(pub u32);
impl ClkGpout2Ctrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub const fn dc50(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub fn set_dc50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkGpout2CtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x0f;
        super::vals::ClkGpout2CtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkGpout2CtrlAuxsrc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.0 as u32) & 0x0f) << 5u32);
    }
}
impl Default for ClkGpout2Ctrl {
    fn default() -> ClkGpout2Ctrl {
        ClkGpout2Ctrl(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkAdcCtrl(pub u32);
impl ClkAdcCtrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkAdcCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x07;
        super::vals::ClkAdcCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkAdcCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5u32)) | (((val.0 as u32) & 0x07) << 5u32);
    }
}
impl Default for ClkAdcCtrl {
    fn default() -> ClkAdcCtrl {
        ClkAdcCtrl(0)
    }
}
#[doc = "Frequency counter status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Status(pub u32);
impl Fc0Status {
    #[doc = "Test clock stopped during test"]
    pub const fn died(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "Test clock stopped during test"]
    pub fn set_died(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "Test clock faster than expected, only valid when status_done=1"]
    pub const fn fast(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Test clock faster than expected, only valid when status_done=1"]
    pub fn set_fast(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Test clock slower than expected, only valid when status_done=1"]
    pub const fn slow(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "Test clock slower than expected, only valid when status_done=1"]
    pub fn set_slow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "Test failed"]
    pub const fn fail(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Test failed"]
    pub fn set_fail(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Waiting for test clock to start"]
    pub const fn waiting(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Waiting for test clock to start"]
    pub fn set_waiting(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Test running"]
    pub const fn running(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Test running"]
    pub fn set_running(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Test complete"]
    pub const fn done(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Test complete"]
    pub fn set_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Test passed"]
    pub const fn pass(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Test passed"]
    pub fn set_pass(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Fc0Status {
    fn default() -> Fc0Status {
        Fc0Status(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte(pub u32);
impl Inte {
    pub const fn clk_sys_resus(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Inte {
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRefDiv(pub u32);
impl ClkRefDiv {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x03;
        val as u8
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val as u32) & 0x03) << 8u32);
    }
}
impl Default for ClkRefDiv {
    fn default() -> ClkRefDiv {
        ClkRefDiv(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(pub u32);
impl Intr {
    pub const fn clk_sys_resus(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf(pub u32);
impl Intf {
    pub const fn clk_sys_resus(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intf {
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "enable clock in wake mode"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct WakeEn0(pub u32);
impl WakeEn0 {
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn clk_sys_rtc(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn clk_rtc_rtc(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_clk_rtc_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_adc_adc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_adc_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for WakeEn0 {
    fn default() -> WakeEn0 {
        WakeEn0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysResusCtrl(pub u32);
impl ClkSysResusCtrl {
    #[doc = "For clearing the resus after the fault that triggered it has been corrected"]
    pub const fn clear(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "For clearing the resus after the fault that triggered it has been corrected"]
    pub fn set_clear(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Force a resus, for test purposes only"]
    pub const fn frce(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Force a resus, for test purposes only"]
    pub fn set_frce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Enable resus"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Enable resus"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "This is expressed as a number of clk_ref cycles and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    pub const fn timeout(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "This is expressed as a number of clk_ref cycles and must be >= 2x clk_ref_freq/min_clk_tst_freq"]
    pub fn set_timeout(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkSysResusCtrl {
    fn default() -> ClkSysResusCtrl {
        ClkSysResusCtrl(0)
    }
}
#[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Src(pub u32);
impl Fc0Src {
    pub const fn fc0_src(&self) -> super::vals::Fc0Src {
        let val = (self.0 >> 0u32) & 0xff;
        super::vals::Fc0Src(val as u8)
    }
    pub fn set_fc0_src(&mut self, val: super::vals::Fc0Src) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val.0 as u32) & 0xff) << 0u32);
    }
}
impl Default for Fc0Src {
    fn default() -> Fc0Src {
        Fc0Src(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkUsbCtrl(pub u32);
impl ClkUsbCtrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkUsbCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x07;
        super::vals::ClkUsbCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkUsbCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5u32)) | (((val.0 as u32) & 0x07) << 5u32);
    }
}
impl Default for ClkUsbCtrl {
    fn default() -> ClkUsbCtrl {
        ClkUsbCtrl(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkAdcDiv(pub u32);
impl ClkAdcDiv {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x03;
        val as u8
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val as u32) & 0x03) << 8u32);
    }
}
impl Default for ClkAdcDiv {
    fn default() -> ClkAdcDiv {
        ClkAdcDiv(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysResusStatus(pub u32);
impl ClkSysResusStatus {
    #[doc = "Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    pub const fn resussed(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clock has been resuscitated, correct the error then send ctrl_clear=1"]
    pub fn set_resussed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for ClkSysResusStatus {
    fn default() -> ClkSysResusStatus {
        ClkSysResusStatus(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysCtrl(pub u32);
impl ClkSysCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkSysCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x07;
        super::vals::ClkSysCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkSysCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5u32)) | (((val.0 as u32) & 0x07) << 5u32);
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    pub const fn src(&self) -> super::vals::ClkSysCtrlSrc {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::ClkSysCtrlSrc(val as u8)
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    pub fn set_src(&mut self, val: super::vals::ClkSysCtrlSrc) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for ClkSysCtrl {
    fn default() -> ClkSysCtrl {
        ClkSysCtrl(0)
    }
}
#[doc = "Reference clock frequency in kHz"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0RefKhz(pub u32);
impl Fc0RefKhz {
    pub const fn fc0_ref_khz(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x000f_ffff;
        val as u32
    }
    pub fn set_fc0_ref_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x000f_ffff << 0u32)) | (((val as u32) & 0x000f_ffff) << 0u32);
    }
}
impl Default for Fc0RefKhz {
    fn default() -> Fc0RefKhz {
        Fc0RefKhz(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout3Ctrl(pub u32);
impl ClkGpout3Ctrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub const fn dc50(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub fn set_dc50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkGpout3CtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x0f;
        super::vals::ClkGpout3CtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkGpout3CtrlAuxsrc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.0 as u32) & 0x0f) << 5u32);
    }
}
impl Default for ClkGpout3Ctrl {
    fn default() -> ClkGpout3Ctrl {
        ClkGpout3Ctrl(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints(pub u32);
impl Ints {
    pub const fn clk_sys_resus(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ints {
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout0Div(pub u32);
impl ClkGpout0Div {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkGpout0Div {
    fn default() -> ClkGpout0Div {
        ClkGpout0Div(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkPeriCtrl(pub u32);
impl ClkPeriCtrl {
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkPeriCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x07;
        super::vals::ClkPeriCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkPeriCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5u32)) | (((val.0 as u32) & 0x07) << 5u32);
    }
}
impl Default for ClkPeriCtrl {
    fn default() -> ClkPeriCtrl {
        ClkPeriCtrl(0)
    }
}
#[doc = "indicates the state of the clock enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Enabled1(pub u32);
impl Enabled1 {
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_usb_usbctrl(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_usb_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_timer(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Enabled1 {
    fn default() -> Enabled1 {
        Enabled1(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkUsbDiv(pub u32);
impl ClkUsbDiv {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x03;
        val as u8
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val as u32) & 0x03) << 8u32);
    }
}
impl Default for ClkUsbDiv {
    fn default() -> ClkUsbDiv {
        ClkUsbDiv(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRtcCtrl(pub u32);
impl ClkRtcCtrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkRtcCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x07;
        super::vals::ClkRtcCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkRtcCtrlAuxsrc) {
        self.0 = (self.0 & !(0x07 << 5u32)) | (((val.0 as u32) & 0x07) << 5u32);
    }
}
impl Default for ClkRtcCtrl {
    fn default() -> ClkRtcCtrl {
        ClkRtcCtrl(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout1Div(pub u32);
impl ClkGpout1Div {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkGpout1Div {
    fn default() -> ClkGpout1Div {
        ClkGpout1Div(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout1Ctrl(pub u32);
impl ClkGpout1Ctrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub const fn dc50(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub fn set_dc50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkGpout1CtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x0f;
        super::vals::ClkGpout1CtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkGpout1CtrlAuxsrc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.0 as u32) & 0x0f) << 5u32);
    }
}
impl Default for ClkGpout1Ctrl {
    fn default() -> ClkGpout1Ctrl {
        ClkGpout1Ctrl(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout0Ctrl(pub u32);
impl ClkGpout0Ctrl {
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub const fn nudge(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "An edge on this signal shifts the phase of the output by 1 cycle of the input clock This can be done at any time"]
    pub fn set_nudge(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub const fn phase(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x03;
        val as u8
    }
    #[doc = "This delays the enable signal by up to 3 cycles of the input clock This must be set before the clock is enabled to have any effect"]
    pub fn set_phase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val as u32) & 0x03) << 16u32);
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub const fn dc50(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Enables duty cycle correction for odd divisors"]
    pub fn set_dc50(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Starts and stops the clock generator cleanly"]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub const fn kill(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Asynchronously kills the clock generator"]
    pub fn set_kill(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkGpout0CtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x0f;
        super::vals::ClkGpout0CtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkGpout0CtrlAuxsrc) {
        self.0 = (self.0 & !(0x0f << 5u32)) | (((val.0 as u32) & 0x0f) << 5u32);
    }
}
impl Default for ClkGpout0Ctrl {
    fn default() -> ClkGpout0Ctrl {
        ClkGpout0Ctrl(0)
    }
}
#[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0MinKhz(pub u32);
impl Fc0MinKhz {
    pub const fn fc0_min_khz(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x01ff_ffff;
        val as u32
    }
    pub fn set_fc0_min_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0u32)) | (((val as u32) & 0x01ff_ffff) << 0u32);
    }
}
impl Default for Fc0MinKhz {
    fn default() -> Fc0MinKhz {
        Fc0MinKhz(0)
    }
}
#[doc = "enable clock in sleep mode"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SleepEn0(pub u32);
impl SleepEn0 {
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn clk_sys_rtc(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn clk_rtc_rtc(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_clk_rtc_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_adc_adc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_adc_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SleepEn0 {
    fn default() -> SleepEn0 {
        SleepEn0(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRtcDiv(pub u32);
impl ClkRtcDiv {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkRtcDiv {
    fn default() -> ClkRtcDiv {
        ClkRtcDiv(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout3Div(pub u32);
impl ClkGpout3Div {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkGpout3Div {
    fn default() -> ClkGpout3Div {
        ClkGpout3Div(0)
    }
}
#[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Interval(pub u32);
impl Fc0Interval {
    pub const fn fc0_interval(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_fc0_interval(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Fc0Interval {
    fn default() -> Fc0Interval {
        Fc0Interval(0)
    }
}
#[doc = "indicates the state of the clock enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Enabled0(pub u32);
impl Enabled0 {
    pub const fn clk_sys_sram3(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn clk_sys_sram2(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn clk_sys_sram1(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn clk_sys_sram0(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn clk_sys_spi1(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn clk_peri_spi1(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn clk_sys_spi0(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn clk_peri_spi0(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_spi0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn clk_sys_sio(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn clk_sys_rtc(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn clk_rtc_rtc(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_clk_rtc_rtc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn clk_sys_rosc(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn clk_sys_rom(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_rom(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn clk_sys_resets(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_resets(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn clk_sys_pwm(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pwm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn clk_sys_psm(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_psm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn clk_sys_pll_usb(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_usb(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn clk_sys_pll_sys(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pll_sys(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_pio1(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_pio0(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pio0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_sys_pads(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_pads(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_vreg_and_chip_reset(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_vreg_and_chip_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_jtag(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_jtag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_sys_io(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_i2c1(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_sys_i2c0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_i2c0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_dma(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_busfabric(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busfabric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_busctrl(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_busctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_adc(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_adc_adc(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_adc_adc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_clocks(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_clocks(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Enabled0 {
    fn default() -> Enabled0 {
        Enabled0(0)
    }
}
#[doc = "enable clock in sleep mode"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SleepEn1(pub u32);
impl SleepEn1 {
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_usb_usbctrl(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_usb_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_timer(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SleepEn1 {
    fn default() -> SleepEn1 {
        SleepEn1(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkGpout2Div(pub u32);
impl ClkGpout2Div {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkGpout2Div {
    fn default() -> ClkGpout2Div {
        ClkGpout2Div(0)
    }
}
#[doc = "Clock divisor, can be changed on-the-fly"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkSysDiv(pub u32);
impl ClkSysDiv {
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub const fn int(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Integer component of the divisor, 0 -> divide by 2^16"]
    pub fn set_int(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "Fractional component of the divisor"]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional component of the divisor"]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for ClkSysDiv {
    fn default() -> ClkSysDiv {
        ClkSysDiv(0)
    }
}
#[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct ClkRefCtrl(pub u32);
impl ClkRefCtrl {
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub const fn auxsrc(&self) -> super::vals::ClkRefCtrlAuxsrc {
        let val = (self.0 >> 5u32) & 0x03;
        super::vals::ClkRefCtrlAuxsrc(val as u8)
    }
    #[doc = "Selects the auxiliary clock source, will glitch when switching"]
    pub fn set_auxsrc(&mut self, val: super::vals::ClkRefCtrlAuxsrc) {
        self.0 = (self.0 & !(0x03 << 5u32)) | (((val.0 as u32) & 0x03) << 5u32);
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    pub const fn src(&self) -> super::vals::ClkRefCtrlSrc {
        let val = (self.0 >> 0u32) & 0x03;
        super::vals::ClkRefCtrlSrc(val as u8)
    }
    #[doc = "Selects the clock source glitchlessly, can be changed on-the-fly"]
    pub fn set_src(&mut self, val: super::vals::ClkRefCtrlSrc) {
        self.0 = (self.0 & !(0x03 << 0u32)) | (((val.0 as u32) & 0x03) << 0u32);
    }
}
impl Default for ClkRefCtrl {
    fn default() -> ClkRefCtrl {
        ClkRefCtrl(0)
    }
}
#[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0MaxKhz(pub u32);
impl Fc0MaxKhz {
    pub const fn fc0_max_khz(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x01ff_ffff;
        val as u32
    }
    pub fn set_fc0_max_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 0u32)) | (((val as u32) & 0x01ff_ffff) << 0u32);
    }
}
impl Default for Fc0MaxKhz {
    fn default() -> Fc0MaxKhz {
        Fc0MaxKhz(0)
    }
}
#[doc = "enable clock in wake mode"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct WakeEn1(pub u32);
impl WakeEn1 {
    pub const fn clk_sys_xosc(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xosc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn clk_sys_xip(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_xip(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn clk_sys_watchdog(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_watchdog(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn clk_usb_usbctrl(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_clk_usb_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn clk_sys_usbctrl(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_usbctrl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn clk_sys_uart1(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn clk_peri_uart1(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn clk_sys_uart0(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn clk_peri_uart0(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_clk_peri_uart0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn clk_sys_timer(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_timer(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn clk_sys_tbman(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_tbman(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn clk_sys_sysinfo(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sysinfo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn clk_sys_syscfg(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_syscfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn clk_sys_sram5(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn clk_sys_sram4(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_clk_sys_sram4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for WakeEn1 {
    fn default() -> WakeEn1 {
        WakeEn1(0)
    }
}
#[doc = "Result of frequency measurement, only valid when status_done=1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fc0Result(pub u32);
impl Fc0Result {
    pub const fn khz(&self) -> u32 {
        let val = (self.0 >> 5u32) & 0x01ff_ffff;
        val as u32
    }
    pub fn set_khz(&mut self, val: u32) {
        self.0 = (self.0 & !(0x01ff_ffff << 5u32)) | (((val as u32) & 0x01ff_ffff) << 5u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Fc0Result {
    fn default() -> Fc0Result {
        Fc0Result(0)
    }
}
