#[doc = "USB FS/LS controller device registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Usb {
    ptr: *mut u8,
}
unsafe impl Send for Usb {}
unsafe impl Sync for Usb {}
impl Usb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Device address and endpoint control"]
    #[inline(always)]
    pub const fn addr_endp(self) -> crate::common::Reg<regs::AddrEndp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
    #[inline(always)]
    pub const fn addr_endp_x(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::AddrEndpX, crate::common::RW> {
        assert!(n < 15usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[doc = "Main control register"]
    #[inline(always)]
    pub const fn main_ctrl(self) -> crate::common::Reg<regs::MainCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
    #[inline(always)]
    pub const fn sof_wr(self) -> crate::common::Reg<regs::SofWr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
    #[inline(always)]
    pub const fn sof_rd(self) -> crate::common::Reg<regs::SofRd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x48usize) as _) }
    }
    #[doc = "SIE control register"]
    #[inline(always)]
    pub const fn sie_ctrl(self) -> crate::common::Reg<regs::SieCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x4cusize) as _) }
    }
    #[doc = "SIE status register"]
    #[inline(always)]
    pub const fn sie_status(self) -> crate::common::Reg<regs::SieStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "interrupt endpoint control register"]
    #[inline(always)]
    pub const fn int_ep_ctrl(self) -> crate::common::Reg<regs::IntEpCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x54usize) as _) }
    }
    #[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
    #[inline(always)]
    pub const fn buff_status(self) -> crate::common::Reg<regs::BuffStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x58usize) as _) }
    }
    #[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
    #[inline(always)]
    pub const fn buff_cpu_should_handle(
        self,
    ) -> crate::common::Reg<regs::BuffCpuShouldHandle, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
    #[inline(always)]
    pub const fn ep_abort(self) -> crate::common::Reg<regs::EpAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
    #[inline(always)]
    pub const fn ep_abort_done(self) -> crate::common::Reg<regs::EpAbortDone, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x64usize) as _) }
    }
    #[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
    #[inline(always)]
    pub const fn ep_stall_arm(self) -> crate::common::Reg<regs::EpStallArm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x68usize) as _) }
    }
    #[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
    #[inline(always)]
    pub const fn nak_poll(self) -> crate::common::Reg<regs::NakPoll, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x6cusize) as _) }
    }
    #[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
    #[inline(always)]
    pub const fn ep_status_stall_nak(
        self,
    ) -> crate::common::Reg<regs::EpStatusStallNak, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x70usize) as _) }
    }
    #[doc = "Where to connect the USB controller. Should be to_phy by default."]
    #[inline(always)]
    pub const fn usb_muxing(self) -> crate::common::Reg<regs::UsbMuxing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x74usize) as _) }
    }
    #[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
    #[inline(always)]
    pub const fn usb_pwr(self) -> crate::common::Reg<regs::UsbPwr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x78usize) as _) }
    }
    #[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
    #[inline(always)]
    pub const fn usbphy_direct(self) -> crate::common::Reg<regs::UsbphyDirect, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x7cusize) as _) }
    }
    #[doc = "Override enable for each control in usbphy_direct"]
    #[inline(always)]
    pub const fn usbphy_direct_override(
        self,
    ) -> crate::common::Reg<regs::UsbphyDirectOverride, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x80usize) as _) }
    }
    #[doc = "Used to adjust trim values of USB phy pull down resistors."]
    #[inline(always)]
    pub const fn usbphy_trim(self) -> crate::common::Reg<regs::UsbphyTrim, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x84usize) as _) }
    }
    #[doc = "Used for debug only."]
    #[inline(always)]
    pub const fn linestate_tuning(
        self,
    ) -> crate::common::Reg<regs::LinestateTuning, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x88usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x8cusize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x90usize) as _) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x94usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Int, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x98usize) as _) }
    }
    #[doc = "Device only. Raw value of free-running PHY clock counter @48MHz. Used to calculate time between SOF events."]
    #[inline(always)]
    pub const fn sof_timestamp_raw(
        self,
    ) -> crate::common::Reg<regs::SofTimestampRaw, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Device only. Value of free-running PHY clock counter @48MHz when last SOF event occurred."]
    #[inline(always)]
    pub const fn sof_timestamp_last(
        self,
    ) -> crate::common::Reg<regs::SofTimestampLast, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[inline(always)]
    pub const fn sm_state(self) -> crate::common::Reg<regs::SmState, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "TX error count for each endpoint. Write to each field to reset the counter to 0."]
    #[inline(always)]
    pub const fn ep_tx_error(self) -> crate::common::Reg<regs::EpTxError, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "RX error count for each endpoint. Write to each field to reset the counter to 0."]
    #[inline(always)]
    pub const fn ep_rx_error(self) -> crate::common::Reg<regs::EpRxError, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Watchdog that forces the device state machine to idle and raises an interrupt if the device stays in a state that isn't idle for the configured limit. The counter is reset on every state transition. Set limit while enable is low and then set the enable."]
    #[inline(always)]
    pub const fn dev_sm_watchdog(
        self,
    ) -> crate::common::Reg<regs::DevSmWatchdog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
}
pub mod regs;
