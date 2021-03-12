use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Clocks(*mut u8);
unsafe impl Send for Clocks {}
unsafe impl Sync for Clocks {}
impl Clocks {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout0_ctrl(self) -> Reg<regs::ClkGpout0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout0_div(self) -> Reg<regs::ClkGpout0Div, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout0_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout1_ctrl(self) -> Reg<regs::ClkGpout1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout1_div(self) -> Reg<regs::ClkGpout1Div, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout1_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout2_ctrl(self) -> Reg<regs::ClkGpout2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout2_div(self) -> Reg<regs::ClkGpout2Div, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout2_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout3_ctrl(self) -> Reg<regs::ClkGpout3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout3_div(self) -> Reg<regs::ClkGpout3Div, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout3_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_ref_ctrl(self) -> Reg<regs::ClkRefCtrl, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_ref_div(self) -> Reg<regs::ClkRefDiv, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_ref_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_sys_ctrl(self) -> Reg<regs::ClkSysCtrl, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_sys_div(self) -> Reg<regs::ClkSysDiv, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_sys_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_peri_ctrl(self) -> Reg<regs::ClkPeriCtrl, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_peri_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_usb_ctrl(self) -> Reg<regs::ClkUsbCtrl, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_usb_div(self) -> Reg<regs::ClkUsbDiv, RW> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_usb_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_adc_ctrl(self) -> Reg<regs::ClkAdcCtrl, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_adc_div(self) -> Reg<regs::ClkAdcDiv, RW> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_adc_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_rtc_ctrl(self) -> Reg<regs::ClkRtcCtrl, RW> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_rtc_div(self) -> Reg<regs::ClkRtcDiv, RW> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_rtc_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    pub fn clk_sys_resus_ctrl(self) -> Reg<regs::ClkSysResusCtrl, RW> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    pub fn clk_sys_resus_status(self) -> Reg<regs::ClkSysResusStatus, RW> {
        unsafe { Reg::new(self.0.add(124usize)) }
    }
    #[doc = "Reference clock frequency in kHz"]
    pub fn fc0_ref_khz(self) -> Reg<regs::Fc0RefKhz, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
    #[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    pub fn fc0_min_khz(self) -> Reg<regs::Fc0MinKhz, RW> {
        unsafe { Reg::new(self.0.add(132usize)) }
    }
    #[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    pub fn fc0_max_khz(self) -> Reg<regs::Fc0MaxKhz, RW> {
        unsafe { Reg::new(self.0.add(136usize)) }
    }
    #[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
    pub fn fc0_delay(self) -> Reg<regs::Fc0Delay, RW> {
        unsafe { Reg::new(self.0.add(140usize)) }
    }
    #[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
    pub fn fc0_interval(self) -> Reg<regs::Fc0Interval, RW> {
        unsafe { Reg::new(self.0.add(144usize)) }
    }
    #[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
    pub fn fc0_src(self) -> Reg<regs::Fc0Src, RW> {
        unsafe { Reg::new(self.0.add(148usize)) }
    }
    #[doc = "Frequency counter status"]
    pub fn fc0_status(self) -> Reg<regs::Fc0Status, RW> {
        unsafe { Reg::new(self.0.add(152usize)) }
    }
    #[doc = "Result of frequency measurement, only valid when status_done=1"]
    pub fn fc0_result(self) -> Reg<regs::Fc0Result, RW> {
        unsafe { Reg::new(self.0.add(156usize)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en0(self) -> Reg<regs::WakeEn0, RW> {
        unsafe { Reg::new(self.0.add(160usize)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en1(self) -> Reg<regs::WakeEn1, RW> {
        unsafe { Reg::new(self.0.add(164usize)) }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en0(self) -> Reg<regs::SleepEn0, RW> {
        unsafe { Reg::new(self.0.add(168usize)) }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en1(self) -> Reg<regs::SleepEn1, RW> {
        unsafe { Reg::new(self.0.add(172usize)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled0(self) -> Reg<regs::Enabled0, RW> {
        unsafe { Reg::new(self.0.add(176usize)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled1(self) -> Reg<regs::Enabled1, RW> {
        unsafe { Reg::new(self.0.add(180usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::new(self.0.add(184usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Inte, RW> {
        unsafe { Reg::new(self.0.add(188usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Intf, RW> {
        unsafe { Reg::new(self.0.add(192usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Ints, RW> {
        unsafe { Reg::new(self.0.add(196usize)) }
    }
}
pub mod regs;
pub mod vals;
