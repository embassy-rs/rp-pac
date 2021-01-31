use crate::generic::*;
#[doc = "USB FS/LS controller device registers"]
#[derive(Copy, Clone)]
pub struct UsbctrlRegs(*mut u8);
unsafe impl Send for UsbctrlRegs {}
unsafe impl Sync for UsbctrlRegs {}
impl UsbctrlRegs {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Device address and endpoint control"]
    pub fn addr_endp(self) -> Reg<fields::AddrEndp, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::AddrEndp::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
    pub fn addr_endp1(self) -> Reg<fields::AddrEndp1, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::AddrEndp1::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 2. Only valid for HOST mode."]
    pub fn addr_endp2(self) -> Reg<fields::AddrEndp2, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::AddrEndp2::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 3. Only valid for HOST mode."]
    pub fn addr_endp3(self) -> Reg<fields::AddrEndp3, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::AddrEndp3::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 4. Only valid for HOST mode."]
    pub fn addr_endp4(self) -> Reg<fields::AddrEndp4, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::AddrEndp4::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 5. Only valid for HOST mode."]
    pub fn addr_endp5(self) -> Reg<fields::AddrEndp5, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::AddrEndp5::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 6. Only valid for HOST mode."]
    pub fn addr_endp6(self) -> Reg<fields::AddrEndp6, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::AddrEndp6::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 7. Only valid for HOST mode."]
    pub fn addr_endp7(self) -> Reg<fields::AddrEndp7, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::AddrEndp7::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 8. Only valid for HOST mode."]
    pub fn addr_endp8(self) -> Reg<fields::AddrEndp8, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::AddrEndp8::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 9. Only valid for HOST mode."]
    pub fn addr_endp9(self) -> Reg<fields::AddrEndp9, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::AddrEndp9::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 10. Only valid for HOST mode."]
    pub fn addr_endp10(self) -> Reg<fields::AddrEndp10, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::AddrEndp10::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 11. Only valid for HOST mode."]
    pub fn addr_endp11(self) -> Reg<fields::AddrEndp11, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::AddrEndp11::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 12. Only valid for HOST mode."]
    pub fn addr_endp12(self) -> Reg<fields::AddrEndp12, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::AddrEndp12::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 13. Only valid for HOST mode."]
    pub fn addr_endp13(self) -> Reg<fields::AddrEndp13, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::AddrEndp13::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 14. Only valid for HOST mode."]
    pub fn addr_endp14(self) -> Reg<fields::AddrEndp14, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::AddrEndp14::from_bits(0)) }
    }
    #[doc = "Interrupt endpoint 15. Only valid for HOST mode."]
    pub fn addr_endp15(self) -> Reg<fields::AddrEndp15, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::AddrEndp15::from_bits(0)) }
    }
    #[doc = "Main control register"]
    pub fn main_ctrl(self) -> Reg<fields::MainCtrl, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::MainCtrl::from_bits(0)) }
    }
    #[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    pub fn sof_wr(self) -> Reg<fields::SofWr, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::SofWr::from_bits(0)) }
    }
    #[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    pub fn sof_rd(self) -> Reg<fields::SofRd, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::SofRd::from_bits(0)) }
    }
    #[doc = "SIE control register"]
    pub fn sie_ctrl(self) -> Reg<fields::SieCtrl, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::SieCtrl::from_bits(0)) }
    }
    #[doc = "SIE status register"]
    pub fn sie_status(self) -> Reg<fields::SieStatus, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::SieStatus::from_bits(0)) }
    }
    #[doc = "interrupt endpoint control register"]
    pub fn int_ep_ctrl(self) -> Reg<fields::IntEpCtrl, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::IntEpCtrl::from_bits(0)) }
    }
    #[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    pub fn buff_status(self) -> Reg<fields::BuffStatus, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::BuffStatus::from_bits(0)) }
    }
    #[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    pub fn buff_cpu_should_handle(self) -> Reg<fields::BuffCpuShouldHandle, RW> {
        unsafe {
            Reg::new(
                self.0.add(92usize),
                fields::BuffCpuShouldHandle::from_bits(0),
            )
        }
    }
    #[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    pub fn ep_abort(self) -> Reg<fields::EpAbort, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::EpAbort::from_bits(0)) }
    }
    #[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    pub fn ep_abort_done(self) -> Reg<fields::EpAbortDone, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::EpAbortDone::from_bits(0)) }
    }
    #[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    pub fn ep_stall_arm(self) -> Reg<fields::EpStallArm, RW> {
        unsafe { Reg::new(self.0.add(104usize), fields::EpStallArm::from_bits(0)) }
    }
    #[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    pub fn nak_poll(self) -> Reg<fields::NakPoll, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::NakPoll::from_bits(1048592)) }
    }
    #[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    pub fn ep_status_stall_nak(self) -> Reg<fields::EpStatusStallNak, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::EpStatusStallNak::from_bits(0)) }
    }
    #[doc = "Where to connect the USB controller. Should be to_phy by default."]
    pub fn usb_muxing(self) -> Reg<fields::UsbMuxing, RW> {
        unsafe { Reg::new(self.0.add(116usize), fields::UsbMuxing::from_bits(0)) }
    }
    #[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    pub fn usb_pwr(self) -> Reg<fields::UsbPwr, RW> {
        unsafe { Reg::new(self.0.add(120usize), fields::UsbPwr::from_bits(0)) }
    }
    #[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    pub fn usbphy_direct(self) -> Reg<fields::UsbphyDirect, RW> {
        unsafe { Reg::new(self.0.add(124usize), fields::UsbphyDirect::from_bits(0)) }
    }
    #[doc = "Override enable for each control in usbphy_direct"]
    pub fn usbphy_direct_override(self) -> Reg<fields::UsbphyDirectOverride, RW> {
        unsafe {
            Reg::new(
                self.0.add(128usize),
                fields::UsbphyDirectOverride::from_bits(0),
            )
        }
    }
    #[doc = "Used to adjust trim values of USB phy pull down resistors."]
    pub fn usbphy_trim(self) -> Reg<fields::UsbphyTrim, RW> {
        unsafe { Reg::new(self.0.add(132usize), fields::UsbphyTrim::from_bits(7967)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Enable"]
    pub fn inte(self) -> Reg<fields::Inte, RW> {
        unsafe { Reg::new(self.0.add(144usize), fields::Inte::from_bits(0)) }
    }
    #[doc = "Interrupt Force"]
    pub fn intf(self) -> Reg<fields::Intf, RW> {
        unsafe { Reg::new(self.0.add(148usize), fields::Intf::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    pub fn ints(self) -> Reg<fields::Ints, RW> {
        unsafe { Reg::new(self.0.add(152usize), fields::Ints::from_bits(0)) }
    }
}
pub mod fields;
