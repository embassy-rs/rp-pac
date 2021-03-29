use crate::generic::*;
#[doc = "USB FS/LS controller device registers"]
#[derive(Copy, Clone)]
pub struct Usb(pub *mut u8);
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[doc = "Device address and endpoint control"]
    pub fn addr_endp(self) -> Reg<regs::AddrEndp, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Main control register"]
    pub fn main_ctrl(self) -> Reg<regs::MainCtrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    pub fn sof_wr(self) -> Reg<regs::SofWr, RW> {
        unsafe { Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    pub fn sof_rd(self) -> Reg<regs::SofRd, RW> {
        unsafe { Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "SIE control register"]
    pub fn sie_ctrl(self) -> Reg<regs::SieCtrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "SIE status register"]
    pub fn sie_status(self) -> Reg<regs::SieStatus, RW> {
        unsafe { Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "interrupt endpoint control register"]
    pub fn int_ep_ctrl(self) -> Reg<regs::IntEpCtrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    pub fn buff_status(self) -> Reg<regs::BuffStatus, RW> {
        unsafe { Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    pub fn buff_cpu_should_handle(self) -> Reg<regs::BuffCpuShouldHandle, RW> {
        unsafe { Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    pub fn ep_abort(self) -> Reg<regs::EpAbort, RW> {
        unsafe { Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    pub fn ep_abort_done(self) -> Reg<regs::EpAbortDone, RW> {
        unsafe { Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    pub fn ep_stall_arm(self) -> Reg<regs::EpStallArm, RW> {
        unsafe { Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    pub fn nak_poll(self) -> Reg<regs::NakPoll, RW> {
        unsafe { Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    pub fn ep_status_stall_nak(self) -> Reg<regs::EpStatusStallNak, RW> {
        unsafe { Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "Where to connect the USB controller. Should be to_phy by default."]
    pub fn usb_muxing(self) -> Reg<regs::UsbMuxing, RW> {
        unsafe { Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    pub fn usb_pwr(self) -> Reg<regs::UsbPwr, RW> {
        unsafe { Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    pub fn usbphy_direct(self) -> Reg<regs::UsbphyDirect, RW> {
        unsafe { Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "Override enable for each control in usbphy_direct"]
    pub fn usbphy_direct_override(self) -> Reg<regs::UsbphyDirectOverride, RW> {
        unsafe { Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Used to adjust trim values of USB phy pull down resistors."]
    pub fn usbphy_trim(self) -> Reg<regs::UsbphyTrim, RW> {
        unsafe { Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Int, RW> {
        unsafe { Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<regs::Int, RW> {
        unsafe { Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<regs::Int, RW> {
        unsafe { Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<regs::Int, RW> {
        unsafe { Reg::from_ptr(self.0.add(152usize)) }
    }
    #[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
    pub fn addr_endp_x(self, n: usize) -> Reg<regs::AddrEndpX, RW> {
        assert!(n < 15usize);
        unsafe { Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
}
pub mod regs;
