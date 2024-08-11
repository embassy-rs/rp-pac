#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Clocks {
    ptr: *mut u8,
}
unsafe impl Send for Clocks {}
unsafe impl Sync for Clocks {}
impl Clocks {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_gpout_ctrl(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ClkGpoutCtrl, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 12usize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_gpout_div(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::ClkGpoutDiv, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 12usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_gpout_selected(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize + n * 12usize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_ref_ctrl(self) -> crate::common::Reg<regs::ClkRefCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_ref_div(self) -> crate::common::Reg<regs::ClkRefDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_ref_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_sys_ctrl(self) -> crate::common::Reg<regs::ClkSysCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_sys_div(self) -> crate::common::Reg<regs::ClkSysDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_sys_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_peri_ctrl(self) -> crate::common::Reg<regs::ClkPeriCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_peri_div(self) -> crate::common::Reg<regs::ClkPeriDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_peri_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_usb_ctrl(self) -> crate::common::Reg<regs::ClkUsbCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_usb_div(self) -> crate::common::Reg<regs::ClkUsbDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_usb_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_adc_ctrl(self) -> crate::common::Reg<regs::ClkAdcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_adc_div(self) -> crate::common::Reg<regs::ClkAdcDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_adc_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Clock control, can be changed on-the-fly (except for auxsrc)"]
    #[inline(always)]
    pub const fn clk_rtc_ctrl(self) -> crate::common::Reg<regs::ClkRtcCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Clock divisor, can be changed on-the-fly"]
    #[inline(always)]
    pub const fn clk_rtc_div(self) -> crate::common::Reg<regs::ClkRtcDiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Indicates which SRC is currently selected by the glitchless mux (one-hot)."]
    #[inline(always)]
    pub const fn clk_rtc_selected(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[inline(always)]
    pub const fn clk_sys_resus_ctrl(
        self,
    ) -> crate::common::Reg<regs::ClkSysResusCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[inline(always)]
    pub const fn clk_sys_resus_status(
        self,
    ) -> crate::common::Reg<regs::ClkSysResusStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Reference clock frequency in kHz"]
    #[inline(always)]
    pub const fn fc0_ref_khz(self) -> crate::common::Reg<regs::Fc0refKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Minimum pass frequency in kHz. This is optional. Set to 0 if you are not using the pass/fail flags"]
    #[inline(always)]
    pub const fn fc0_min_khz(self) -> crate::common::Reg<regs::Fc0minKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Maximum pass frequency in kHz. This is optional. Set to 0x1ffffff if you are not using the pass/fail flags"]
    #[inline(always)]
    pub const fn fc0_max_khz(self) -> crate::common::Reg<regs::Fc0maxKhz, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Delays the start of frequency counting to allow the mux to settle Delay is measured in multiples of the reference clock period"]
    #[inline(always)]
    pub const fn fc0_delay(self) -> crate::common::Reg<regs::Fc0delay, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "The test interval is 0.98us * 2**interval, but let's call it 1us * 2**interval The default gives a test interval of 250us"]
    #[inline(always)]
    pub const fn fc0_interval(self) -> crate::common::Reg<regs::Fc0interval, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Clock sent to frequency counter, set to 0 when not required Writing to this register initiates the frequency count"]
    #[inline(always)]
    pub const fn fc0_src(self) -> crate::common::Reg<regs::Fc0src, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Frequency counter status"]
    #[inline(always)]
    pub const fn fc0_status(self) -> crate::common::Reg<regs::Fc0status, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Result of frequency measurement, only valid when status_done=1"]
    #[inline(always)]
    pub const fn fc0_result(self) -> crate::common::Reg<regs::Fc0result, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x9cusize) as _) }
    }
    #[doc = "enable clock in wake mode"]
    #[inline(always)]
    pub const fn wake_en0(self) -> crate::common::Reg<regs::WakeEn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa0usize) as _) }
    }
    #[doc = "enable clock in wake mode"]
    #[inline(always)]
    pub const fn wake_en1(self) -> crate::common::Reg<regs::WakeEn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa4usize) as _) }
    }
    #[doc = "enable clock in sleep mode"]
    #[inline(always)]
    pub const fn sleep_en0(self) -> crate::common::Reg<regs::SleepEn0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xa8usize) as _) }
    }
    #[doc = "enable clock in sleep mode"]
    #[inline(always)]
    pub const fn sleep_en1(self) -> crate::common::Reg<regs::SleepEn1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xacusize) as _) }
    }
    #[doc = "indicates the state of the clock enable"]
    #[inline(always)]
    pub const fn enabled0(self) -> crate::common::Reg<regs::Enabled0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb0usize) as _) }
    }
    #[doc = "indicates the state of the clock enable"]
    #[inline(always)]
    pub const fn enabled1(self) -> crate::common::Reg<regs::Enabled1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb4usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xb8usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xbcusize) as _) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc0usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xc4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
