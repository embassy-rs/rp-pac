#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clocks(pub *mut u8);
unsafe impl Send for Clocks {}
unsafe impl Sync for Clocks {}
impl Clocks {
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout0_ctrl(self) -> crate::common::Reg<regs::ClkGpout0Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout0_div(self) -> crate::common::Reg<regs::ClkGpout0Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout0_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout1_ctrl(self) -> crate::common::Reg<regs::ClkGpout1Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout1_div(self) -> crate::common::Reg<regs::ClkGpout1Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout1_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout2_ctrl(self) -> crate::common::Reg<regs::ClkGpout2Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout2_div(self) -> crate::common::Reg<regs::ClkGpout2Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout2_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_gpout3_ctrl(self) -> crate::common::Reg<regs::ClkGpout3Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_gpout3_div(self) -> crate::common::Reg<regs::ClkGpout3Div, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_gpout3_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_ref_ctrl(self) -> crate::common::Reg<regs::ClkRefCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_ref_div(self) -> crate::common::Reg<regs::ClkRefDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_ref_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_sys_ctrl(self) -> crate::common::Reg<regs::ClkSysCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_sys_div(self) -> crate::common::Reg<regs::ClkSysDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_sys_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_peri_ctrl(self) -> crate::common::Reg<regs::ClkPeriCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_peri_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_usb_ctrl(self) -> crate::common::Reg<regs::ClkUsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_usb_div(self) -> crate::common::Reg<regs::ClkUsbDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_usb_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_adc_ctrl(self) -> crate::common::Reg<regs::ClkAdcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_adc_div(self) -> crate::common::Reg<regs::ClkAdcDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_adc_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    pub fn clk_rtc_ctrl(self) -> crate::common::Reg<regs::ClkRtcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    pub fn clk_rtc_div(self) -> crate::common::Reg<regs::ClkRtcDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Indicates which src is currently selected (one-hot)"]
    pub fn clk_rtc_selected(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }
    pub fn clk_sys_resus_ctrl(
        self,
    ) -> crate::common::Reg<regs::ClkSysResusCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    pub fn clk_sys_resus_status(
        self,
    ) -> crate::common::Reg<regs::ClkSysResusStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Reference clock frequency in kHz"]
    pub fn fc0_ref_khz(self) -> crate::common::Reg<regs::Fc0RefKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    pub fn fc0_min_khz(self) -> crate::common::Reg<regs::Fc0MinKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    pub fn fc0_max_khz(self) -> crate::common::Reg<regs::Fc0MaxKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
    pub fn fc0_delay(self) -> crate::common::Reg<regs::Fc0Delay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
    pub fn fc0_interval(self) -> crate::common::Reg<regs::Fc0Interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
    pub fn fc0_src(self) -> crate::common::Reg<regs::Fc0Src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "Frequency counter status"]
    pub fn fc0_status(self) -> crate::common::Reg<regs::Fc0Status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }
    #[doc = "Result of frequency measurement, only valid when status_done=1"]
    pub fn fc0_result(self) -> crate::common::Reg<regs::Fc0Result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(156usize)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en0(self) -> crate::common::Reg<regs::WakeEn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "enable clock in wake mode"]
    pub fn wake_en1(self) -> crate::common::Reg<regs::WakeEn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(164usize)) }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en0(self) -> crate::common::Reg<regs::SleepEn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "enable clock in sleep mode"]
    pub fn sleep_en1(self) -> crate::common::Reg<regs::SleepEn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(172usize)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled0(self) -> crate::common::Reg<regs::Enabled0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(176usize)) }
    }
    #[doc = "indicates the state of the clock enable"]
    pub fn enabled1(self) -> crate::common::Reg<regs::Enabled1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(180usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(184usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(188usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(192usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(196usize)) }
    }
}
pub mod regs;
pub mod vals;
