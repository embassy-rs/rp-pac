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
    pub fn clk_gpout0_ctrl(self) -> Reg<fields::ClkGpout0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::ClkGpout0Ctrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout0_div(self) -> Reg<fields::ClkGpout0Div, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::ClkGpout0Div::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout0_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(8usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout1_ctrl(self) -> Reg<fields::ClkGpout1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::ClkGpout1Ctrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout1_div(self) -> Reg<fields::ClkGpout1Div, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::ClkGpout1Div::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout1_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(20usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout2_ctrl(self) -> Reg<fields::ClkGpout2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::ClkGpout2Ctrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout2_div(self) -> Reg<fields::ClkGpout2Div, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::ClkGpout2Div::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout2_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(32usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout3_ctrl(self) -> Reg<fields::ClkGpout3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::ClkGpout3Ctrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout3_div(self) -> Reg<fields::ClkGpout3Div, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::ClkGpout3Div::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout3_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(44usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_ref_ctrl(self) -> Reg<fields::ClkRefCtrl, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::ClkRefCtrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_ref_div(self) -> Reg<fields::ClkRefDiv, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::ClkRefDiv::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_ref_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(56usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_sys_ctrl(self) -> Reg<fields::ClkSysCtrl, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::ClkSysCtrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_sys_div(self) -> Reg<fields::ClkSysDiv, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::ClkSysDiv::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_sys_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(68usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_peri_ctrl(self) -> Reg<fields::ClkPeriCtrl, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::ClkPeriCtrl::from_bits(0)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_peri_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(80usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_usb_ctrl(self) -> Reg<fields::ClkUsbCtrl, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::ClkUsbCtrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_usb_div(self) -> Reg<fields::ClkUsbDiv, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::ClkUsbDiv::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_usb_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(92usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_adc_ctrl(self) -> Reg<fields::ClkAdcCtrl, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::ClkAdcCtrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_adc_div(self) -> Reg<fields::ClkAdcDiv, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::ClkAdcDiv::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_adc_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(104usize), 1) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_rtc_ctrl(self) -> Reg<fields::ClkRtcCtrl, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::ClkRtcCtrl::from_bits(0)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_rtc_div(self) -> Reg<fields::ClkRtcDiv, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::ClkRtcDiv::from_bits(256)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_rtc_selected(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(116usize), 1) }
    }
    pub fn clk_sys_resus_ctrl(self) -> Reg<fields::ClkSysResusCtrl, RW> {
        unsafe {
            Reg::new(
                self.0.add(120usize),
                fields::ClkSysResusCtrl::from_bits(255),
            )
        }
    }
    pub fn clk_sys_resus_status(self) -> Reg<fields::ClkSysResusStatus, RW> {
        unsafe {
            Reg::new(
                self.0.add(124usize),
                fields::ClkSysResusStatus::from_bits(0),
            )
        }
    }
    #[doc = "Reference clock frequency in kHz"]
    pub fn fc0_ref_khz(self) -> Reg<fields::Fc0RefKhz, RW> {
        unsafe { Reg::new(self.0.add(128usize), fields::Fc0RefKhz::from_bits(0)) }
    }
    #[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    pub fn fc0_min_khz(self) -> Reg<fields::Fc0MinKhz, RW> {
        unsafe { Reg::new(self.0.add(132usize), fields::Fc0MinKhz::from_bits(0)) }
    }
    #[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    pub fn fc0_max_khz(self) -> Reg<fields::Fc0MaxKhz, RW> {
        unsafe { Reg::new(self.0.add(136usize), fields::Fc0MaxKhz::from_bits(33554431)) }
    }
    #[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
    pub fn fc0_delay(self) -> Reg<fields::Fc0Delay, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::Fc0Delay::from_bits(1)) }
    }
    #[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
    pub fn fc0_interval(self) -> Reg<fields::Fc0Interval, RW> {
        unsafe { Reg::new(self.0.add(144usize), fields::Fc0Interval::from_bits(8)) }
    }
    #[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
    pub fn fc0_src(self) -> Reg<fields::Fc0Src, RW> {
        unsafe { Reg::new(self.0.add(148usize), fields::Fc0Src::from_bits(0)) }
    }
    #[doc = "Frequency counter status"]
    pub fn fc0_status(self) -> Reg<fields::Fc0Status, RW> {
        unsafe { Reg::new(self.0.add(152usize), fields::Fc0Status::from_bits(0)) }
    }
    #[doc = "Result of frequency measurement, only valid when status_done=1"]
    pub fn fc0_result(self) -> Reg<fields::Fc0Result, RW> {
        unsafe { Reg::new(self.0.add(156usize), fields::Fc0Result::from_bits(0)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en0(self) -> Reg<fields::WakeEn0, RW> {
        unsafe { Reg::new(self.0.add(160usize), fields::WakeEn0::from_bits(4294967295)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en1(self) -> Reg<fields::WakeEn1, RW> {
        unsafe { Reg::new(self.0.add(164usize), fields::WakeEn1::from_bits(32767)) }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en0(self) -> Reg<fields::SleepEn0, RW> {
        unsafe {
            Reg::new(
                self.0.add(168usize),
                fields::SleepEn0::from_bits(4294967295),
            )
        }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en1(self) -> Reg<fields::SleepEn1, RW> {
        unsafe { Reg::new(self.0.add(172usize), fields::SleepEn1::from_bits(32767)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled0(self) -> Reg<fields::Enabled0, RW> {
        unsafe { Reg::new(self.0.add(176usize), fields::Enabled0::from_bits(0)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled1(self) -> Reg<fields::Enabled1, RW> {
        unsafe { Reg::new(self.0.add(180usize), fields::Enabled1::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(184usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(188usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(192usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(196usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
