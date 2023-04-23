#[doc = "USB FS/LS controller device registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb(pub *mut u8);
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[doc = "Device address and endpoint control"]
    #[inline(always)]
    pub fn addr_endp(self) -> crate::common::Reg<regs::AddrEndp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
    #[inline(always)]
    pub fn addr_endp_x(self, n: usize) -> crate::common::Reg<regs::AddrEndpX, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
    #[doc = "Main control register"]
    #[inline(always)]
    pub fn main_ctrl(self) -> crate::common::Reg<regs::MainCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    #[inline(always)]
    pub fn sof_wr(self) -> crate::common::Reg<regs::SofWr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    #[inline(always)]
    pub fn sof_rd(self) -> crate::common::Reg<regs::SofRd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "SIE control register"]
    #[inline(always)]
    pub fn sie_ctrl(self) -> crate::common::Reg<regs::SieCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "SIE status register"]
    #[inline(always)]
    pub fn sie_status(self) -> crate::common::Reg<regs::SieStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "interrupt endpoint control register"]
    #[inline(always)]
    pub fn int_ep_ctrl(self) -> crate::common::Reg<regs::IntEpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    #[inline(always)]
    pub fn buff_status(self) -> crate::common::Reg<regs::BuffStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    #[inline(always)]
    pub fn buff_cpu_should_handle(
        self,
    ) -> crate::common::Reg<regs::BuffCpuShouldHandle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    #[inline(always)]
    pub fn ep_abort(self) -> crate::common::Reg<regs::EpAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    #[inline(always)]
    pub fn ep_abort_done(self) -> crate::common::Reg<regs::EpAbortDone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    #[inline(always)]
    pub fn ep_stall_arm(self) -> crate::common::Reg<regs::EpStallArm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    #[inline(always)]
    pub fn nak_poll(self) -> crate::common::Reg<regs::NakPoll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    #[inline(always)]
    pub fn ep_status_stall_nak(
        self,
    ) -> crate::common::Reg<regs::EpStatusStallNak, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Where to connect the USB controller. Should be to_phy by default."]
    #[inline(always)]
    pub fn usb_muxing(self) -> crate::common::Reg<regs::UsbMuxing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    #[inline(always)]
    pub fn usb_pwr(self) -> crate::common::Reg<regs::UsbPwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    #[inline(always)]
    pub fn usbphy_direct(self) -> crate::common::Reg<regs::UsbphyDirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Override enable for each control in usbphy_direct"]
    #[inline(always)]
    pub fn usbphy_direct_override(
        self,
    ) -> crate::common::Reg<regs::UsbphyDirectOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Used to adjust trim values of USB phy pull down resistors."]
    #[inline(always)]
    pub fn usbphy_trim(self) -> crate::common::Reg<regs::UsbphyTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(152usize)) }
    }
}
pub mod regs;
