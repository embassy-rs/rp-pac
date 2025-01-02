#[doc = "Device address and endpoint control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AddrEndp(pub u32);
impl AddrEndp {
    #[doc = "In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    #[inline(always)]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    pub const fn endpoint(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Device endpoint to send data to. Only valid for HOST mode."]
    #[inline(always)]
    pub fn set_endpoint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
}
impl Default for AddrEndp {
    #[inline(always)]
    fn default() -> AddrEndp {
        AddrEndp(0)
    }
}
impl core::fmt::Debug for AddrEndp {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AddrEndp")
            .field("address", &self.address())
            .field("endpoint", &self.endpoint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AddrEndp {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AddrEndp {
            address: u8,
            endpoint: u8,
        }
        let proxy = AddrEndp {
            address: self.address(),
            endpoint: self.endpoint(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt endpoint 1. Only valid for HOST mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct AddrEndpX(pub u32);
impl AddrEndpX {
    #[doc = "Device address"]
    #[inline(always)]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x7f;
        val as u8
    }
    #[doc = "Device address"]
    #[inline(always)]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0usize)) | (((val as u32) & 0x7f) << 0usize);
    }
    #[doc = "Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    pub const fn endpoint(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint number of the interrupt endpoint"]
    #[inline(always)]
    pub fn set_endpoint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    pub const fn intep_dir(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Direction of the interrupt endpoint. In=0, Out=1"]
    #[inline(always)]
    pub fn set_intep_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    pub const fn intep_preamble(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    #[inline(always)]
    pub fn set_intep_preamble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for AddrEndpX {
    #[inline(always)]
    fn default() -> AddrEndpX {
        AddrEndpX(0)
    }
}
impl core::fmt::Debug for AddrEndpX {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AddrEndpX")
            .field("address", &self.address())
            .field("endpoint", &self.endpoint())
            .field("intep_dir", &self.intep_dir())
            .field("intep_preamble", &self.intep_preamble())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for AddrEndpX {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct AddrEndpX {
            address: u8,
            endpoint: u8,
            intep_dir: bool,
            intep_preamble: bool,
        }
        let proxy = AddrEndpX {
            address: self.address(),
            endpoint: self.endpoint(),
            intep_dir: self.intep_dir(),
            intep_preamble: self.intep_preamble(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuffCpuShouldHandle(pub u32);
impl BuffCpuShouldHandle {
    #[inline(always)]
    pub const fn ep_in(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_in(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn ep_out(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_out(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for BuffCpuShouldHandle {
    #[inline(always)]
    fn default() -> BuffCpuShouldHandle {
        BuffCpuShouldHandle(0)
    }
}
impl core::fmt::Debug for BuffCpuShouldHandle {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BuffCpuShouldHandle")
            .field(
                "ep_in",
                &[
                    self.ep_in(0usize),
                    self.ep_in(1usize),
                    self.ep_in(2usize),
                    self.ep_in(3usize),
                    self.ep_in(4usize),
                    self.ep_in(5usize),
                    self.ep_in(6usize),
                    self.ep_in(7usize),
                    self.ep_in(8usize),
                    self.ep_in(9usize),
                    self.ep_in(10usize),
                    self.ep_in(11usize),
                    self.ep_in(12usize),
                    self.ep_in(13usize),
                    self.ep_in(14usize),
                    self.ep_in(15usize),
                ],
            )
            .field(
                "ep_out",
                &[
                    self.ep_out(0usize),
                    self.ep_out(1usize),
                    self.ep_out(2usize),
                    self.ep_out(3usize),
                    self.ep_out(4usize),
                    self.ep_out(5usize),
                    self.ep_out(6usize),
                    self.ep_out(7usize),
                    self.ep_out(8usize),
                    self.ep_out(9usize),
                    self.ep_out(10usize),
                    self.ep_out(11usize),
                    self.ep_out(12usize),
                    self.ep_out(13usize),
                    self.ep_out(14usize),
                    self.ep_out(15usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BuffCpuShouldHandle {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BuffCpuShouldHandle {
            ep_in: [bool; 16usize],
            ep_out: [bool; 16usize],
        }
        let proxy = BuffCpuShouldHandle {
            ep_in: [
                self.ep_in(0usize),
                self.ep_in(1usize),
                self.ep_in(2usize),
                self.ep_in(3usize),
                self.ep_in(4usize),
                self.ep_in(5usize),
                self.ep_in(6usize),
                self.ep_in(7usize),
                self.ep_in(8usize),
                self.ep_in(9usize),
                self.ep_in(10usize),
                self.ep_in(11usize),
                self.ep_in(12usize),
                self.ep_in(13usize),
                self.ep_in(14usize),
                self.ep_in(15usize),
            ],
            ep_out: [
                self.ep_out(0usize),
                self.ep_out(1usize),
                self.ep_out(2usize),
                self.ep_out(3usize),
                self.ep_out(4usize),
                self.ep_out(5usize),
                self.ep_out(6usize),
                self.ep_out(7usize),
                self.ep_out(8usize),
                self.ep_out(9usize),
                self.ep_out(10usize),
                self.ep_out(11usize),
                self.ep_out(12usize),
                self.ep_out(13usize),
                self.ep_out(14usize),
                self.ep_out(15usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BuffStatus(pub u32);
impl BuffStatus {
    #[inline(always)]
    pub const fn ep_in(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_in(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn ep_out(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_out(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for BuffStatus {
    #[inline(always)]
    fn default() -> BuffStatus {
        BuffStatus(0)
    }
}
impl core::fmt::Debug for BuffStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BuffStatus")
            .field(
                "ep_in",
                &[
                    self.ep_in(0usize),
                    self.ep_in(1usize),
                    self.ep_in(2usize),
                    self.ep_in(3usize),
                    self.ep_in(4usize),
                    self.ep_in(5usize),
                    self.ep_in(6usize),
                    self.ep_in(7usize),
                    self.ep_in(8usize),
                    self.ep_in(9usize),
                    self.ep_in(10usize),
                    self.ep_in(11usize),
                    self.ep_in(12usize),
                    self.ep_in(13usize),
                    self.ep_in(14usize),
                    self.ep_in(15usize),
                ],
            )
            .field(
                "ep_out",
                &[
                    self.ep_out(0usize),
                    self.ep_out(1usize),
                    self.ep_out(2usize),
                    self.ep_out(3usize),
                    self.ep_out(4usize),
                    self.ep_out(5usize),
                    self.ep_out(6usize),
                    self.ep_out(7usize),
                    self.ep_out(8usize),
                    self.ep_out(9usize),
                    self.ep_out(10usize),
                    self.ep_out(11usize),
                    self.ep_out(12usize),
                    self.ep_out(13usize),
                    self.ep_out(14usize),
                    self.ep_out(15usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BuffStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BuffStatus {
            ep_in: [bool; 16usize],
            ep_out: [bool; 16usize],
        }
        let proxy = BuffStatus {
            ep_in: [
                self.ep_in(0usize),
                self.ep_in(1usize),
                self.ep_in(2usize),
                self.ep_in(3usize),
                self.ep_in(4usize),
                self.ep_in(5usize),
                self.ep_in(6usize),
                self.ep_in(7usize),
                self.ep_in(8usize),
                self.ep_in(9usize),
                self.ep_in(10usize),
                self.ep_in(11usize),
                self.ep_in(12usize),
                self.ep_in(13usize),
                self.ep_in(14usize),
                self.ep_in(15usize),
            ],
            ep_out: [
                self.ep_out(0usize),
                self.ep_out(1usize),
                self.ep_out(2usize),
                self.ep_out(3usize),
                self.ep_out(4usize),
                self.ep_out(5usize),
                self.ep_out(6usize),
                self.ep_out(7usize),
                self.ep_out(8usize),
                self.ep_out(9usize),
                self.ep_out(10usize),
                self.ep_out(11usize),
                self.ep_out(12usize),
                self.ep_out(13usize),
                self.ep_out(14usize),
                self.ep_out(15usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Watchdog that forces the device state machine to idle and raises an interrupt if the device stays in a state that isn't idle for the configured limit. The counter is reset on every state transition. Set limit while enable is low and then set the enable."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DevSmWatchdog(pub u32);
impl DevSmWatchdog {
    #[inline(always)]
    pub const fn limit(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x0003_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_limit(&mut self, val: u32) {
        self.0 = (self.0 & !(0x0003_ffff << 0usize)) | (((val as u32) & 0x0003_ffff) << 0usize);
    }
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Set to 1 to forcibly reset the device state machine on watchdog expiry"]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Set to 1 to forcibly reset the device state machine on watchdog expiry"]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn fired(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_fired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for DevSmWatchdog {
    #[inline(always)]
    fn default() -> DevSmWatchdog {
        DevSmWatchdog(0)
    }
}
impl core::fmt::Debug for DevSmWatchdog {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DevSmWatchdog")
            .field("limit", &self.limit())
            .field("enable", &self.enable())
            .field("reset", &self.reset())
            .field("fired", &self.fired())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DevSmWatchdog {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DevSmWatchdog {
            limit: u32,
            enable: bool,
            reset: bool,
            fired: bool,
        }
        let proxy = DevSmWatchdog {
            limit: self.limit(),
            enable: self.enable(),
            reset: self.reset(),
            fired: self.fired(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpAbort(pub u32);
impl EpAbort {
    #[inline(always)]
    pub const fn ep_in(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_in(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn ep_out(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_out(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for EpAbort {
    #[inline(always)]
    fn default() -> EpAbort {
        EpAbort(0)
    }
}
impl core::fmt::Debug for EpAbort {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpAbort")
            .field(
                "ep_in",
                &[
                    self.ep_in(0usize),
                    self.ep_in(1usize),
                    self.ep_in(2usize),
                    self.ep_in(3usize),
                    self.ep_in(4usize),
                    self.ep_in(5usize),
                    self.ep_in(6usize),
                    self.ep_in(7usize),
                    self.ep_in(8usize),
                    self.ep_in(9usize),
                    self.ep_in(10usize),
                    self.ep_in(11usize),
                    self.ep_in(12usize),
                    self.ep_in(13usize),
                    self.ep_in(14usize),
                    self.ep_in(15usize),
                ],
            )
            .field(
                "ep_out",
                &[
                    self.ep_out(0usize),
                    self.ep_out(1usize),
                    self.ep_out(2usize),
                    self.ep_out(3usize),
                    self.ep_out(4usize),
                    self.ep_out(5usize),
                    self.ep_out(6usize),
                    self.ep_out(7usize),
                    self.ep_out(8usize),
                    self.ep_out(9usize),
                    self.ep_out(10usize),
                    self.ep_out(11usize),
                    self.ep_out(12usize),
                    self.ep_out(13usize),
                    self.ep_out(14usize),
                    self.ep_out(15usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpAbort {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpAbort {
            ep_in: [bool; 16usize],
            ep_out: [bool; 16usize],
        }
        let proxy = EpAbort {
            ep_in: [
                self.ep_in(0usize),
                self.ep_in(1usize),
                self.ep_in(2usize),
                self.ep_in(3usize),
                self.ep_in(4usize),
                self.ep_in(5usize),
                self.ep_in(6usize),
                self.ep_in(7usize),
                self.ep_in(8usize),
                self.ep_in(9usize),
                self.ep_in(10usize),
                self.ep_in(11usize),
                self.ep_in(12usize),
                self.ep_in(13usize),
                self.ep_in(14usize),
                self.ep_in(15usize),
            ],
            ep_out: [
                self.ep_out(0usize),
                self.ep_out(1usize),
                self.ep_out(2usize),
                self.ep_out(3usize),
                self.ep_out(4usize),
                self.ep_out(5usize),
                self.ep_out(6usize),
                self.ep_out(7usize),
                self.ep_out(8usize),
                self.ep_out(9usize),
                self.ep_out(10usize),
                self.ep_out(11usize),
                self.ep_out(12usize),
                self.ep_out(13usize),
                self.ep_out(14usize),
                self.ep_out(15usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpAbortDone(pub u32);
impl EpAbortDone {
    #[inline(always)]
    pub const fn ep_in(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_in(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn ep_out(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_out(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for EpAbortDone {
    #[inline(always)]
    fn default() -> EpAbortDone {
        EpAbortDone(0)
    }
}
impl core::fmt::Debug for EpAbortDone {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpAbortDone")
            .field(
                "ep_in",
                &[
                    self.ep_in(0usize),
                    self.ep_in(1usize),
                    self.ep_in(2usize),
                    self.ep_in(3usize),
                    self.ep_in(4usize),
                    self.ep_in(5usize),
                    self.ep_in(6usize),
                    self.ep_in(7usize),
                    self.ep_in(8usize),
                    self.ep_in(9usize),
                    self.ep_in(10usize),
                    self.ep_in(11usize),
                    self.ep_in(12usize),
                    self.ep_in(13usize),
                    self.ep_in(14usize),
                    self.ep_in(15usize),
                ],
            )
            .field(
                "ep_out",
                &[
                    self.ep_out(0usize),
                    self.ep_out(1usize),
                    self.ep_out(2usize),
                    self.ep_out(3usize),
                    self.ep_out(4usize),
                    self.ep_out(5usize),
                    self.ep_out(6usize),
                    self.ep_out(7usize),
                    self.ep_out(8usize),
                    self.ep_out(9usize),
                    self.ep_out(10usize),
                    self.ep_out(11usize),
                    self.ep_out(12usize),
                    self.ep_out(13usize),
                    self.ep_out(14usize),
                    self.ep_out(15usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpAbortDone {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpAbortDone {
            ep_in: [bool; 16usize],
            ep_out: [bool; 16usize],
        }
        let proxy = EpAbortDone {
            ep_in: [
                self.ep_in(0usize),
                self.ep_in(1usize),
                self.ep_in(2usize),
                self.ep_in(3usize),
                self.ep_in(4usize),
                self.ep_in(5usize),
                self.ep_in(6usize),
                self.ep_in(7usize),
                self.ep_in(8usize),
                self.ep_in(9usize),
                self.ep_in(10usize),
                self.ep_in(11usize),
                self.ep_in(12usize),
                self.ep_in(13usize),
                self.ep_in(14usize),
                self.ep_in(15usize),
            ],
            ep_out: [
                self.ep_out(0usize),
                self.ep_out(1usize),
                self.ep_out(2usize),
                self.ep_out(3usize),
                self.ep_out(4usize),
                self.ep_out(5usize),
                self.ep_out(6usize),
                self.ep_out(7usize),
                self.ep_out(8usize),
                self.ep_out(9usize),
                self.ep_out(10usize),
                self.ep_out(11usize),
                self.ep_out(12usize),
                self.ep_out(13usize),
                self.ep_out(14usize),
                self.ep_out(15usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "RX error count for each endpoint. Write to each field to reset the counter to 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpRxError(pub u32);
impl EpRxError {
    #[inline(always)]
    pub const fn ep0_transaction(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep0_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ep0_seq(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep0_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ep1_transaction(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep1_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ep1_seq(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep1_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ep2_transaction(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep2_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ep2_seq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep2_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ep3_transaction(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep3_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ep3_seq(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep3_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn ep4_transaction(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep4_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn ep4_seq(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep4_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn ep5_transaction(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep5_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn ep5_seq(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep5_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn ep6_transaction(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep6_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn ep6_seq(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep6_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn ep7_transaction(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep7_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn ep7_seq(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep7_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn ep8_transaction(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep8_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn ep8_seq(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep8_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn ep9_transaction(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep9_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn ep9_seq(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep9_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[inline(always)]
    pub const fn ep10_transaction(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep10_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn ep10_seq(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep10_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[inline(always)]
    pub const fn ep11_transaction(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep11_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[inline(always)]
    pub const fn ep11_seq(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep11_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[inline(always)]
    pub const fn ep12_transaction(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep12_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[inline(always)]
    pub const fn ep12_seq(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep12_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[inline(always)]
    pub const fn ep13_transaction(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep13_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[inline(always)]
    pub const fn ep13_seq(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep13_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[inline(always)]
    pub const fn ep14_transaction(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep14_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[inline(always)]
    pub const fn ep14_seq(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep14_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[inline(always)]
    pub const fn ep15_transaction(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep15_transaction(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[inline(always)]
    pub const fn ep15_seq(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep15_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for EpRxError {
    #[inline(always)]
    fn default() -> EpRxError {
        EpRxError(0)
    }
}
impl core::fmt::Debug for EpRxError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpRxError")
            .field("ep0_transaction", &self.ep0_transaction())
            .field("ep0_seq", &self.ep0_seq())
            .field("ep1_transaction", &self.ep1_transaction())
            .field("ep1_seq", &self.ep1_seq())
            .field("ep2_transaction", &self.ep2_transaction())
            .field("ep2_seq", &self.ep2_seq())
            .field("ep3_transaction", &self.ep3_transaction())
            .field("ep3_seq", &self.ep3_seq())
            .field("ep4_transaction", &self.ep4_transaction())
            .field("ep4_seq", &self.ep4_seq())
            .field("ep5_transaction", &self.ep5_transaction())
            .field("ep5_seq", &self.ep5_seq())
            .field("ep6_transaction", &self.ep6_transaction())
            .field("ep6_seq", &self.ep6_seq())
            .field("ep7_transaction", &self.ep7_transaction())
            .field("ep7_seq", &self.ep7_seq())
            .field("ep8_transaction", &self.ep8_transaction())
            .field("ep8_seq", &self.ep8_seq())
            .field("ep9_transaction", &self.ep9_transaction())
            .field("ep9_seq", &self.ep9_seq())
            .field("ep10_transaction", &self.ep10_transaction())
            .field("ep10_seq", &self.ep10_seq())
            .field("ep11_transaction", &self.ep11_transaction())
            .field("ep11_seq", &self.ep11_seq())
            .field("ep12_transaction", &self.ep12_transaction())
            .field("ep12_seq", &self.ep12_seq())
            .field("ep13_transaction", &self.ep13_transaction())
            .field("ep13_seq", &self.ep13_seq())
            .field("ep14_transaction", &self.ep14_transaction())
            .field("ep14_seq", &self.ep14_seq())
            .field("ep15_transaction", &self.ep15_transaction())
            .field("ep15_seq", &self.ep15_seq())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpRxError {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpRxError {
            ep0_transaction: bool,
            ep0_seq: bool,
            ep1_transaction: bool,
            ep1_seq: bool,
            ep2_transaction: bool,
            ep2_seq: bool,
            ep3_transaction: bool,
            ep3_seq: bool,
            ep4_transaction: bool,
            ep4_seq: bool,
            ep5_transaction: bool,
            ep5_seq: bool,
            ep6_transaction: bool,
            ep6_seq: bool,
            ep7_transaction: bool,
            ep7_seq: bool,
            ep8_transaction: bool,
            ep8_seq: bool,
            ep9_transaction: bool,
            ep9_seq: bool,
            ep10_transaction: bool,
            ep10_seq: bool,
            ep11_transaction: bool,
            ep11_seq: bool,
            ep12_transaction: bool,
            ep12_seq: bool,
            ep13_transaction: bool,
            ep13_seq: bool,
            ep14_transaction: bool,
            ep14_seq: bool,
            ep15_transaction: bool,
            ep15_seq: bool,
        }
        let proxy = EpRxError {
            ep0_transaction: self.ep0_transaction(),
            ep0_seq: self.ep0_seq(),
            ep1_transaction: self.ep1_transaction(),
            ep1_seq: self.ep1_seq(),
            ep2_transaction: self.ep2_transaction(),
            ep2_seq: self.ep2_seq(),
            ep3_transaction: self.ep3_transaction(),
            ep3_seq: self.ep3_seq(),
            ep4_transaction: self.ep4_transaction(),
            ep4_seq: self.ep4_seq(),
            ep5_transaction: self.ep5_transaction(),
            ep5_seq: self.ep5_seq(),
            ep6_transaction: self.ep6_transaction(),
            ep6_seq: self.ep6_seq(),
            ep7_transaction: self.ep7_transaction(),
            ep7_seq: self.ep7_seq(),
            ep8_transaction: self.ep8_transaction(),
            ep8_seq: self.ep8_seq(),
            ep9_transaction: self.ep9_transaction(),
            ep9_seq: self.ep9_seq(),
            ep10_transaction: self.ep10_transaction(),
            ep10_seq: self.ep10_seq(),
            ep11_transaction: self.ep11_transaction(),
            ep11_seq: self.ep11_seq(),
            ep12_transaction: self.ep12_transaction(),
            ep12_seq: self.ep12_seq(),
            ep13_transaction: self.ep13_transaction(),
            ep13_seq: self.ep13_seq(),
            ep14_transaction: self.ep14_transaction(),
            ep14_seq: self.ep14_seq(),
            ep15_transaction: self.ep15_transaction(),
            ep15_seq: self.ep15_seq(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpStallArm(pub u32);
impl EpStallArm {
    #[inline(always)]
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for EpStallArm {
    #[inline(always)]
    fn default() -> EpStallArm {
        EpStallArm(0)
    }
}
impl core::fmt::Debug for EpStallArm {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpStallArm")
            .field("ep0_in", &self.ep0_in())
            .field("ep0_out", &self.ep0_out())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpStallArm {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpStallArm {
            ep0_in: bool,
            ep0_out: bool,
        }
        let proxy = EpStallArm {
            ep0_in: self.ep0_in(),
            ep0_out: self.ep0_out(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpStatusStallNak(pub u32);
impl EpStatusStallNak {
    #[inline(always)]
    pub const fn ep_in(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_in(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 0usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[inline(always)]
    pub const fn ep_out(&self, n: usize) -> bool {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ep_out(&mut self, n: usize, val: bool) {
        assert!(n < 16usize);
        let offs = 1usize + n * 2usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for EpStatusStallNak {
    #[inline(always)]
    fn default() -> EpStatusStallNak {
        EpStatusStallNak(0)
    }
}
impl core::fmt::Debug for EpStatusStallNak {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpStatusStallNak")
            .field(
                "ep_in",
                &[
                    self.ep_in(0usize),
                    self.ep_in(1usize),
                    self.ep_in(2usize),
                    self.ep_in(3usize),
                    self.ep_in(4usize),
                    self.ep_in(5usize),
                    self.ep_in(6usize),
                    self.ep_in(7usize),
                    self.ep_in(8usize),
                    self.ep_in(9usize),
                    self.ep_in(10usize),
                    self.ep_in(11usize),
                    self.ep_in(12usize),
                    self.ep_in(13usize),
                    self.ep_in(14usize),
                    self.ep_in(15usize),
                ],
            )
            .field(
                "ep_out",
                &[
                    self.ep_out(0usize),
                    self.ep_out(1usize),
                    self.ep_out(2usize),
                    self.ep_out(3usize),
                    self.ep_out(4usize),
                    self.ep_out(5usize),
                    self.ep_out(6usize),
                    self.ep_out(7usize),
                    self.ep_out(8usize),
                    self.ep_out(9usize),
                    self.ep_out(10usize),
                    self.ep_out(11usize),
                    self.ep_out(12usize),
                    self.ep_out(13usize),
                    self.ep_out(14usize),
                    self.ep_out(15usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpStatusStallNak {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpStatusStallNak {
            ep_in: [bool; 16usize],
            ep_out: [bool; 16usize],
        }
        let proxy = EpStatusStallNak {
            ep_in: [
                self.ep_in(0usize),
                self.ep_in(1usize),
                self.ep_in(2usize),
                self.ep_in(3usize),
                self.ep_in(4usize),
                self.ep_in(5usize),
                self.ep_in(6usize),
                self.ep_in(7usize),
                self.ep_in(8usize),
                self.ep_in(9usize),
                self.ep_in(10usize),
                self.ep_in(11usize),
                self.ep_in(12usize),
                self.ep_in(13usize),
                self.ep_in(14usize),
                self.ep_in(15usize),
            ],
            ep_out: [
                self.ep_out(0usize),
                self.ep_out(1usize),
                self.ep_out(2usize),
                self.ep_out(3usize),
                self.ep_out(4usize),
                self.ep_out(5usize),
                self.ep_out(6usize),
                self.ep_out(7usize),
                self.ep_out(8usize),
                self.ep_out(9usize),
                self.ep_out(10usize),
                self.ep_out(11usize),
                self.ep_out(12usize),
                self.ep_out(13usize),
                self.ep_out(14usize),
                self.ep_out(15usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "TX error count for each endpoint. Write to each field to reset the counter to 0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpTxError(pub u32);
impl EpTxError {
    #[inline(always)]
    pub const fn ep0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val as u32) & 0x03) << 0usize);
    }
    #[inline(always)]
    pub const fn ep1(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[inline(always)]
    pub const fn ep2(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[inline(always)]
    pub const fn ep3(&self) -> u8 {
        let val = (self.0 >> 6usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val as u32) & 0x03) << 6usize);
    }
    #[inline(always)]
    pub const fn ep4(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[inline(always)]
    pub const fn ep5(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 10usize)) | (((val as u32) & 0x03) << 10usize);
    }
    #[inline(always)]
    pub const fn ep6(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 12usize)) | (((val as u32) & 0x03) << 12usize);
    }
    #[inline(always)]
    pub const fn ep7(&self) -> u8 {
        let val = (self.0 >> 14usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val as u32) & 0x03) << 14usize);
    }
    #[inline(always)]
    pub const fn ep8(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val as u32) & 0x03) << 16usize);
    }
    #[inline(always)]
    pub const fn ep9(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 18usize)) | (((val as u32) & 0x03) << 18usize);
    }
    #[inline(always)]
    pub const fn ep10(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 20usize)) | (((val as u32) & 0x03) << 20usize);
    }
    #[inline(always)]
    pub const fn ep11(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22usize)) | (((val as u32) & 0x03) << 22usize);
    }
    #[inline(always)]
    pub const fn ep12(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 24usize)) | (((val as u32) & 0x03) << 24usize);
    }
    #[inline(always)]
    pub const fn ep13(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val as u32) & 0x03) << 26usize);
    }
    #[inline(always)]
    pub const fn ep14(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val as u32) & 0x03) << 28usize);
    }
    #[inline(always)]
    pub const fn ep15(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[inline(always)]
    pub fn set_ep15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for EpTxError {
    #[inline(always)]
    fn default() -> EpTxError {
        EpTxError(0)
    }
}
impl core::fmt::Debug for EpTxError {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EpTxError")
            .field("ep0", &self.ep0())
            .field("ep1", &self.ep1())
            .field("ep2", &self.ep2())
            .field("ep3", &self.ep3())
            .field("ep4", &self.ep4())
            .field("ep5", &self.ep5())
            .field("ep6", &self.ep6())
            .field("ep7", &self.ep7())
            .field("ep8", &self.ep8())
            .field("ep9", &self.ep9())
            .field("ep10", &self.ep10())
            .field("ep11", &self.ep11())
            .field("ep12", &self.ep12())
            .field("ep13", &self.ep13())
            .field("ep14", &self.ep14())
            .field("ep15", &self.ep15())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for EpTxError {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct EpTxError {
            ep0: u8,
            ep1: u8,
            ep2: u8,
            ep3: u8,
            ep4: u8,
            ep5: u8,
            ep6: u8,
            ep7: u8,
            ep8: u8,
            ep9: u8,
            ep10: u8,
            ep11: u8,
            ep12: u8,
            ep13: u8,
            ep14: u8,
            ep15: u8,
        }
        let proxy = EpTxError {
            ep0: self.ep0(),
            ep1: self.ep1(),
            ep2: self.ep2(),
            ep3: self.ep3(),
            ep4: self.ep4(),
            ep5: self.ep5(),
            ep6: self.ep6(),
            ep7: self.ep7(),
            ep8: self.ep8(),
            ep9: self.ep9(),
            ep10: self.ep10(),
            ep11: self.ep11(),
            ep12: self.ep12(),
            ep13: self.ep13(),
            ep14: self.ep14(),
            ep15: self.ep15(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub const fn host_conn_dis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    #[inline(always)]
    pub fn set_host_conn_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub const fn host_resume(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn set_host_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub const fn host_sof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn set_host_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    #[inline(always)]
    pub fn set_trans_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub const fn buff_status(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    #[inline(always)]
    pub fn set_buff_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub const fn error_data_seq(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.DATA_SEQ_ERROR"]
    #[inline(always)]
    pub fn set_error_data_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub const fn error_rx_timeout(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.RX_TIMEOUT"]
    #[inline(always)]
    pub fn set_error_rx_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub const fn error_rx_overflow(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.RX_OVERFLOW"]
    #[inline(always)]
    pub fn set_error_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub const fn error_bit_stuff(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.BIT_STUFF_ERROR"]
    #[inline(always)]
    pub fn set_error_bit_stuff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub const fn error_crc(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.CRC_ERROR"]
    #[inline(always)]
    pub fn set_error_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.STALL_REC"]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.VBUS_DETECTED"]
    #[inline(always)]
    pub fn set_vbus_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.BUS_RESET"]
    #[inline(always)]
    pub fn set_bus_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub const fn dev_conn_dis(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    #[inline(always)]
    pub fn set_dev_conn_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub const fn dev_suspend(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    #[inline(always)]
    pub fn set_dev_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub const fn dev_resume_from_host(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    #[inline(always)]
    pub fn set_dev_resume_from_host(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub const fn setup_req(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device. Source: SIE_STATUS.SETUP_REC"]
    #[inline(always)]
    pub fn set_setup_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub const fn dev_sof(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    #[inline(always)]
    pub fn set_dev_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub const fn abort_done(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    #[inline(always)]
    pub fn set_abort_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub const fn ep_stall_nak(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    #[inline(always)]
    pub fn set_ep_stall_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Source: SIE_STATUS.RX_SHORT_PACKET"]
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.RX_SHORT_PACKET"]
    #[inline(always)]
    pub fn set_rx_short_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "Source: SIE_STATUS.ENDPOINT_ERROR"]
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.ENDPOINT_ERROR"]
    #[inline(always)]
    pub fn set_endpoint_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Source: DEV_SM_WATCHDOG.FIRED"]
    #[inline(always)]
    pub const fn dev_sm_watchdog_fired(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Source: DEV_SM_WATCHDOG.FIRED"]
    #[inline(always)]
    pub fn set_dev_sm_watchdog_fired(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Source: NAK_POLL.EPX_STOPPED_ON_NAK"]
    #[inline(always)]
    pub fn set_epx_stopped_on_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field("host_conn_dis", &self.host_conn_dis())
            .field("host_resume", &self.host_resume())
            .field("host_sof", &self.host_sof())
            .field("trans_complete", &self.trans_complete())
            .field("buff_status", &self.buff_status())
            .field("error_data_seq", &self.error_data_seq())
            .field("error_rx_timeout", &self.error_rx_timeout())
            .field("error_rx_overflow", &self.error_rx_overflow())
            .field("error_bit_stuff", &self.error_bit_stuff())
            .field("error_crc", &self.error_crc())
            .field("stall", &self.stall())
            .field("vbus_detect", &self.vbus_detect())
            .field("bus_reset", &self.bus_reset())
            .field("dev_conn_dis", &self.dev_conn_dis())
            .field("dev_suspend", &self.dev_suspend())
            .field("dev_resume_from_host", &self.dev_resume_from_host())
            .field("setup_req", &self.setup_req())
            .field("dev_sof", &self.dev_sof())
            .field("abort_done", &self.abort_done())
            .field("ep_stall_nak", &self.ep_stall_nak())
            .field("rx_short_packet", &self.rx_short_packet())
            .field("endpoint_error", &self.endpoint_error())
            .field("dev_sm_watchdog_fired", &self.dev_sm_watchdog_fired())
            .field("epx_stopped_on_nak", &self.epx_stopped_on_nak())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            host_conn_dis: bool,
            host_resume: bool,
            host_sof: bool,
            trans_complete: bool,
            buff_status: bool,
            error_data_seq: bool,
            error_rx_timeout: bool,
            error_rx_overflow: bool,
            error_bit_stuff: bool,
            error_crc: bool,
            stall: bool,
            vbus_detect: bool,
            bus_reset: bool,
            dev_conn_dis: bool,
            dev_suspend: bool,
            dev_resume_from_host: bool,
            setup_req: bool,
            dev_sof: bool,
            abort_done: bool,
            ep_stall_nak: bool,
            rx_short_packet: bool,
            endpoint_error: bool,
            dev_sm_watchdog_fired: bool,
            epx_stopped_on_nak: bool,
        }
        let proxy = Int {
            host_conn_dis: self.host_conn_dis(),
            host_resume: self.host_resume(),
            host_sof: self.host_sof(),
            trans_complete: self.trans_complete(),
            buff_status: self.buff_status(),
            error_data_seq: self.error_data_seq(),
            error_rx_timeout: self.error_rx_timeout(),
            error_rx_overflow: self.error_rx_overflow(),
            error_bit_stuff: self.error_bit_stuff(),
            error_crc: self.error_crc(),
            stall: self.stall(),
            vbus_detect: self.vbus_detect(),
            bus_reset: self.bus_reset(),
            dev_conn_dis: self.dev_conn_dis(),
            dev_suspend: self.dev_suspend(),
            dev_resume_from_host: self.dev_resume_from_host(),
            setup_req: self.setup_req(),
            dev_sof: self.dev_sof(),
            abort_done: self.abort_done(),
            ep_stall_nak: self.ep_stall_nak(),
            rx_short_packet: self.rx_short_packet(),
            endpoint_error: self.endpoint_error(),
            dev_sm_watchdog_fired: self.dev_sm_watchdog_fired(),
            epx_stopped_on_nak: self.epx_stopped_on_nak(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "interrupt endpoint control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IntEpCtrl(pub u32);
impl IntEpCtrl {
    #[doc = "Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub const fn int_ep_active(&self) -> u16 {
        let val = (self.0 >> 1usize) & 0x7fff;
        val as u16
    }
    #[doc = "Host: Enable interrupt endpoint 1 -> 15"]
    #[inline(always)]
    pub fn set_int_ep_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 1usize)) | (((val as u32) & 0x7fff) << 1usize);
    }
}
impl Default for IntEpCtrl {
    #[inline(always)]
    fn default() -> IntEpCtrl {
        IntEpCtrl(0)
    }
}
impl core::fmt::Debug for IntEpCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IntEpCtrl")
            .field("int_ep_active", &self.int_ep_active())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IntEpCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IntEpCtrl {
            int_ep_active: u16,
        }
        let proxy = IntEpCtrl {
            int_ep_active: self.int_ep_active(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Used for debug only."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LinestateTuning(pub u32);
impl LinestateTuning {
    #[doc = "Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
    #[inline(always)]
    pub const fn rcv_delay(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Device - register the received data to account for hub bit dribble before EOP. Only affects certain hubs."]
    #[inline(always)]
    pub fn set_rcv_delay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Device/Host - add an extra 1-bit debounce of linestate sampling."]
    #[inline(always)]
    pub const fn linestate_delay(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Device/Host - add an extra 1-bit debounce of linestate sampling."]
    #[inline(always)]
    pub fn set_linestate_delay(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
    #[inline(always)]
    pub const fn multi_hub_fix(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host - increase inter-packet and turnaround timeouts to accommodate worst-case hub delays."]
    #[inline(always)]
    pub fn set_multi_hub_fix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
    #[inline(always)]
    pub const fn dev_buff_control_double_read_fix(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Device - the controller FSM performs two reads of the buffer status memory address to avoid sampling metastable data. An enabled buffer is only used if both reads match."]
    #[inline(always)]
    pub fn set_dev_buff_control_double_read_fix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
    #[inline(always)]
    pub const fn sie_rx_bitstuff_fix(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "RX - when a bitstuff error is signalled by rx_dasm, unconditionally terminate RX decode to avoid a hang during certain packet phases."]
    #[inline(always)]
    pub fn set_sie_rx_bitstuff_fix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
    #[inline(always)]
    pub const fn sie_rx_chatter_se0_fix(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "RX - when recovering from line chatter or bitstuff errors, treat SE0 as the end of chatter as well as 8 consecutive idle bits."]
    #[inline(always)]
    pub fn set_sie_rx_chatter_se0_fix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
    #[inline(always)]
    pub const fn dev_rx_err_quiesce(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Device - suppress repeated errors until the device FSM is next in the process of decoding an inbound packet."]
    #[inline(always)]
    pub fn set_dev_rx_err_quiesce(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
    #[inline(always)]
    pub const fn dev_ls_wake_fix(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Device - exit suspend on any non-idle signalling, not qualified with a 1ms timer"]
    #[inline(always)]
    pub fn set_dev_ls_wake_fix(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn spare_fix(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_spare_fix(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for LinestateTuning {
    #[inline(always)]
    fn default() -> LinestateTuning {
        LinestateTuning(0)
    }
}
impl core::fmt::Debug for LinestateTuning {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LinestateTuning")
            .field("rcv_delay", &self.rcv_delay())
            .field("linestate_delay", &self.linestate_delay())
            .field("multi_hub_fix", &self.multi_hub_fix())
            .field(
                "dev_buff_control_double_read_fix",
                &self.dev_buff_control_double_read_fix(),
            )
            .field("sie_rx_bitstuff_fix", &self.sie_rx_bitstuff_fix())
            .field("sie_rx_chatter_se0_fix", &self.sie_rx_chatter_se0_fix())
            .field("dev_rx_err_quiesce", &self.dev_rx_err_quiesce())
            .field("dev_ls_wake_fix", &self.dev_ls_wake_fix())
            .field("spare_fix", &self.spare_fix())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for LinestateTuning {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct LinestateTuning {
            rcv_delay: bool,
            linestate_delay: bool,
            multi_hub_fix: bool,
            dev_buff_control_double_read_fix: bool,
            sie_rx_bitstuff_fix: bool,
            sie_rx_chatter_se0_fix: bool,
            dev_rx_err_quiesce: bool,
            dev_ls_wake_fix: bool,
            spare_fix: u8,
        }
        let proxy = LinestateTuning {
            rcv_delay: self.rcv_delay(),
            linestate_delay: self.linestate_delay(),
            multi_hub_fix: self.multi_hub_fix(),
            dev_buff_control_double_read_fix: self.dev_buff_control_double_read_fix(),
            sie_rx_bitstuff_fix: self.sie_rx_bitstuff_fix(),
            sie_rx_chatter_se0_fix: self.sie_rx_chatter_se0_fix(),
            dev_rx_err_quiesce: self.dev_rx_err_quiesce(),
            dev_ls_wake_fix: self.dev_ls_wake_fix(),
            spare_fix: self.spare_fix(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Main control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MainCtrl(pub u32);
impl MainCtrl {
    #[doc = "Enable controller"]
    #[inline(always)]
    pub const fn controller_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable controller"]
    #[inline(always)]
    pub fn set_controller_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Device mode = 0, Host mode = 1"]
    #[inline(always)]
    pub const fn host_ndevice(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Device mode = 0, Host mode = 1"]
    #[inline(always)]
    pub fn set_host_ndevice(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Isolates USB phy after controller power-up Remove isolation once software has configured the controller Not isolated = 0, Isolated = 1"]
    #[inline(always)]
    pub const fn phy_iso(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Isolates USB phy after controller power-up Remove isolation once software has configured the controller Not isolated = 0, Isolated = 1"]
    #[inline(always)]
    pub fn set_phy_iso(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Reduced timings for simulation"]
    #[inline(always)]
    pub const fn sim_timing(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Reduced timings for simulation"]
    #[inline(always)]
    pub fn set_sim_timing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for MainCtrl {
    #[inline(always)]
    fn default() -> MainCtrl {
        MainCtrl(0)
    }
}
impl core::fmt::Debug for MainCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MainCtrl")
            .field("controller_en", &self.controller_en())
            .field("host_ndevice", &self.host_ndevice())
            .field("phy_iso", &self.phy_iso())
            .field("sim_timing", &self.sim_timing())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for MainCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct MainCtrl {
            controller_en: bool,
            host_ndevice: bool,
            phy_iso: bool,
            sim_timing: bool,
        }
        let proxy = MainCtrl {
            controller_en: self.controller_en(),
            host_ndevice: self.host_ndevice(),
            phy_iso: self.phy_iso(),
            sim_timing: self.sim_timing(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NakPoll(pub u32);
impl NakPoll {
    #[doc = "NAK polling interval for a low speed device"]
    #[inline(always)]
    pub const fn delay_ls(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x03ff;
        val as u16
    }
    #[doc = "NAK polling interval for a low speed device"]
    #[inline(always)]
    pub fn set_delay_ls(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0usize)) | (((val as u32) & 0x03ff) << 0usize);
    }
    #[doc = "Bits 5:0 of nak_retry_count"]
    #[inline(always)]
    pub const fn retry_count_lo(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x3f;
        val as u8
    }
    #[doc = "Bits 5:0 of nak_retry_count"]
    #[inline(always)]
    pub fn set_retry_count_lo(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 10usize)) | (((val as u32) & 0x3f) << 10usize);
    }
    #[doc = "NAK polling interval for a full speed device"]
    #[inline(always)]
    pub const fn delay_fs(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x03ff;
        val as u16
    }
    #[doc = "NAK polling interval for a full speed device"]
    #[inline(always)]
    pub fn set_delay_fs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16usize)) | (((val as u32) & 0x03ff) << 16usize);
    }
    #[doc = "Stop polling epx when a nak is received"]
    #[inline(always)]
    pub const fn stop_epx_on_nak(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Stop polling epx when a nak is received"]
    #[inline(always)]
    pub fn set_stop_epx_on_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "EPX polling has stopped because a nak was received"]
    #[inline(always)]
    pub const fn epx_stopped_on_nak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "EPX polling has stopped because a nak was received"]
    #[inline(always)]
    pub fn set_epx_stopped_on_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Bits 9:6 of nak_retry count"]
    #[inline(always)]
    pub const fn retry_count_hi(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Bits 9:6 of nak_retry count"]
    #[inline(always)]
    pub fn set_retry_count_hi(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for NakPoll {
    #[inline(always)]
    fn default() -> NakPoll {
        NakPoll(0)
    }
}
impl core::fmt::Debug for NakPoll {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NakPoll")
            .field("delay_ls", &self.delay_ls())
            .field("retry_count_lo", &self.retry_count_lo())
            .field("delay_fs", &self.delay_fs())
            .field("stop_epx_on_nak", &self.stop_epx_on_nak())
            .field("epx_stopped_on_nak", &self.epx_stopped_on_nak())
            .field("retry_count_hi", &self.retry_count_hi())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NakPoll {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NakPoll {
            delay_ls: u16,
            retry_count_lo: u8,
            delay_fs: u16,
            stop_epx_on_nak: bool,
            epx_stopped_on_nak: bool,
            retry_count_hi: u8,
        }
        let proxy = NakPoll {
            delay_ls: self.delay_ls(),
            retry_count_lo: self.retry_count_lo(),
            delay_fs: self.delay_fs(),
            stop_epx_on_nak: self.stop_epx_on_nak(),
            epx_stopped_on_nak: self.epx_stopped_on_nak(),
            retry_count_hi: self.retry_count_hi(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SIE control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieCtrl(pub u32);
impl SieCtrl {
    #[doc = "Host: Start transaction"]
    #[inline(always)]
    pub const fn start_trans(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Start transaction"]
    #[inline(always)]
    pub fn set_start_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Host: Send Setup packet"]
    #[inline(always)]
    pub const fn send_setup(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Send Setup packet"]
    #[inline(always)]
    pub fn set_send_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub const fn send_data(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Send transaction (OUT from host)"]
    #[inline(always)]
    pub fn set_send_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub const fn receive_data(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Receive transaction (IN to host)"]
    #[inline(always)]
    pub fn set_receive_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Host: Stop transaction"]
    #[inline(always)]
    pub const fn stop_trans(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Stop transaction"]
    #[inline(always)]
    pub fn set_stop_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub const fn preamble_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Preable enable for LS device on FS hub"]
    #[inline(always)]
    pub fn set_preamble_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub const fn sof_sync(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Delay packet(s) until after SOF"]
    #[inline(always)]
    pub fn set_sof_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub const fn sof_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable SOF generation (for full speed bus)"]
    #[inline(always)]
    pub fn set_sof_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub const fn keep_alive_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable keep alive packet (for low speed bus)"]
    #[inline(always)]
    pub fn set_keep_alive_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Host: Enable VBUS"]
    #[inline(always)]
    pub const fn vbus_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable VBUS"]
    #[inline(always)]
    pub fn set_vbus_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Remote wakeup. Device can initiate its own resume after suspend."]
    #[inline(always)]
    pub fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Host: Reset bus"]
    #[inline(always)]
    pub const fn reset_bus(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Reset bus"]
    #[inline(always)]
    pub fn set_reset_bus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Host: Enable pull down resistors"]
    #[inline(always)]
    pub const fn pulldown_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable pull down resistors"]
    #[inline(always)]
    pub fn set_pulldown_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Device: Enable pull up resistor"]
    #[inline(always)]
    pub const fn pullup_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Enable pull up resistor"]
    #[inline(always)]
    pub fn set_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub const fn rpu_opt(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Pull-up strength (0=1K2, 1=2k3)"]
    #[inline(always)]
    pub fn set_rpu_opt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Power down bus transceiver"]
    #[inline(always)]
    pub const fn transceiver_pd(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Power down bus transceiver"]
    #[inline(always)]
    pub fn set_transceiver_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Device: Stop EP0 on a short packet."]
    #[inline(always)]
    pub const fn ep0_stop_on_short_packet(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Stop EP0 on a short packet."]
    #[inline(always)]
    pub fn set_ep0_stop_on_short_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Direct control of DM"]
    #[inline(always)]
    pub const fn direct_dm(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Direct control of DM"]
    #[inline(always)]
    pub fn set_direct_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Direct control of DP"]
    #[inline(always)]
    pub const fn direct_dp(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Direct control of DP"]
    #[inline(always)]
    pub fn set_direct_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Direct bus drive enable"]
    #[inline(always)]
    pub const fn direct_en(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Direct bus drive enable"]
    #[inline(always)]
    pub fn set_direct_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub const fn ep0_int_nak(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    #[inline(always)]
    pub fn set_ep0_int_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub const fn ep0_int_2buf(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    #[inline(always)]
    pub fn set_ep0_int_2buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub const fn ep0_int_1buf(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    #[inline(always)]
    pub fn set_ep0_int_1buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub const fn ep0_double_buf(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "Device: EP0 single buffered = 0, double buffered = 1"]
    #[inline(always)]
    pub fn set_ep0_double_buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub const fn ep0_int_stall(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    #[inline(always)]
    pub fn set_ep0_int_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SieCtrl {
    #[inline(always)]
    fn default() -> SieCtrl {
        SieCtrl(0)
    }
}
impl core::fmt::Debug for SieCtrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SieCtrl")
            .field("start_trans", &self.start_trans())
            .field("send_setup", &self.send_setup())
            .field("send_data", &self.send_data())
            .field("receive_data", &self.receive_data())
            .field("stop_trans", &self.stop_trans())
            .field("preamble_en", &self.preamble_en())
            .field("sof_sync", &self.sof_sync())
            .field("sof_en", &self.sof_en())
            .field("keep_alive_en", &self.keep_alive_en())
            .field("vbus_en", &self.vbus_en())
            .field("resume", &self.resume())
            .field("reset_bus", &self.reset_bus())
            .field("pulldown_en", &self.pulldown_en())
            .field("pullup_en", &self.pullup_en())
            .field("rpu_opt", &self.rpu_opt())
            .field("transceiver_pd", &self.transceiver_pd())
            .field("ep0_stop_on_short_packet", &self.ep0_stop_on_short_packet())
            .field("direct_dm", &self.direct_dm())
            .field("direct_dp", &self.direct_dp())
            .field("direct_en", &self.direct_en())
            .field("ep0_int_nak", &self.ep0_int_nak())
            .field("ep0_int_2buf", &self.ep0_int_2buf())
            .field("ep0_int_1buf", &self.ep0_int_1buf())
            .field("ep0_double_buf", &self.ep0_double_buf())
            .field("ep0_int_stall", &self.ep0_int_stall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SieCtrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SieCtrl {
            start_trans: bool,
            send_setup: bool,
            send_data: bool,
            receive_data: bool,
            stop_trans: bool,
            preamble_en: bool,
            sof_sync: bool,
            sof_en: bool,
            keep_alive_en: bool,
            vbus_en: bool,
            resume: bool,
            reset_bus: bool,
            pulldown_en: bool,
            pullup_en: bool,
            rpu_opt: bool,
            transceiver_pd: bool,
            ep0_stop_on_short_packet: bool,
            direct_dm: bool,
            direct_dp: bool,
            direct_en: bool,
            ep0_int_nak: bool,
            ep0_int_2buf: bool,
            ep0_int_1buf: bool,
            ep0_double_buf: bool,
            ep0_int_stall: bool,
        }
        let proxy = SieCtrl {
            start_trans: self.start_trans(),
            send_setup: self.send_setup(),
            send_data: self.send_data(),
            receive_data: self.receive_data(),
            stop_trans: self.stop_trans(),
            preamble_en: self.preamble_en(),
            sof_sync: self.sof_sync(),
            sof_en: self.sof_en(),
            keep_alive_en: self.keep_alive_en(),
            vbus_en: self.vbus_en(),
            resume: self.resume(),
            reset_bus: self.reset_bus(),
            pulldown_en: self.pulldown_en(),
            pullup_en: self.pullup_en(),
            rpu_opt: self.rpu_opt(),
            transceiver_pd: self.transceiver_pd(),
            ep0_stop_on_short_packet: self.ep0_stop_on_short_packet(),
            direct_dm: self.direct_dm(),
            direct_dp: self.direct_dp(),
            direct_en: self.direct_en(),
            ep0_int_nak: self.ep0_int_nak(),
            ep0_int_2buf: self.ep0_int_2buf(),
            ep0_int_1buf: self.ep0_int_1buf(),
            ep0_double_buf: self.ep0_double_buf(),
            ep0_int_stall: self.ep0_int_stall(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "SIE status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SieStatus(pub u32);
impl SieStatus {
    #[doc = "Device: VBUS Detected"]
    #[inline(always)]
    pub const fn vbus_detected(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Device: VBUS Detected"]
    #[inline(always)]
    pub fn set_vbus_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "USB bus line state"]
    #[inline(always)]
    pub const fn line_state(&self) -> u8 {
        let val = (self.0 >> 2usize) & 0x03;
        val as u8
    }
    #[doc = "USB bus line state"]
    #[inline(always)]
    pub fn set_line_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val as u32) & 0x03) << 2usize);
    }
    #[doc = "Bus in suspended state. Valid for device. Device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    pub const fn suspended(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Bus in suspended state. Valid for device. Device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    #[inline(always)]
    pub fn set_suspended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x03;
        val as u8
    }
    #[doc = "Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    #[inline(always)]
    pub fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val as u32) & 0x03) << 8usize);
    }
    #[doc = "VBUS over current detected"]
    #[inline(always)]
    pub const fn vbus_over_curr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "VBUS over current detected"]
    #[inline(always)]
    pub fn set_vbus_over_curr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    #[inline(always)]
    pub fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early."]
    #[inline(always)]
    pub const fn rx_short_packet(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Device or Host has received a short packet. This is when the data received is less than configured in the buffer control register. Device: If using double buffered mode on device the buffer select will not be toggled after writing status back to the buffer control register. This is to prevent any further transactions on that endpoint until the user has reset the buffer control registers. Host: the current transfer will be stopped early."]
    #[inline(always)]
    pub fn set_rx_short_packet(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Device: connected"]
    #[inline(always)]
    pub const fn connected(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Device: connected"]
    #[inline(always)]
    pub fn set_connected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Device: Setup packet received"]
    #[inline(always)]
    pub const fn setup_rec(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Device: Setup packet received"]
    #[inline(always)]
    pub fn set_setup_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Transaction complete. Raised by device if: * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register Raised by host if: * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub const fn trans_complete(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Transaction complete. Raised by device if: * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register Raised by host if: * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    #[inline(always)]
    pub fn set_trans_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Device: bus reset received"]
    #[inline(always)]
    pub const fn bus_reset(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Device: bus reset received"]
    #[inline(always)]
    pub fn set_bus_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error."]
    #[inline(always)]
    pub const fn endpoint_error(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "An endpoint has encountered an error. Read the ep_rx_error and ep_tx_error registers to find out which endpoint had an error."]
    #[inline(always)]
    pub fn set_endpoint_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub const fn crc_error(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "CRC Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn set_crc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub const fn bit_stuff_error(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Bit Stuff Error. Raised by the Serial RX engine."]
    #[inline(always)]
    pub fn set_bit_stuff_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    #[inline(always)]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
    #[doc = "RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    pub const fn rx_timeout(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    #[inline(always)]
    pub fn set_rx_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Host: NAK received"]
    #[inline(always)]
    pub const fn nak_rec(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Host: NAK received"]
    #[inline(always)]
    pub fn set_nak_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Host: STALL received"]
    #[inline(always)]
    pub const fn stall_rec(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Host: STALL received"]
    #[inline(always)]
    pub fn set_stall_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "ACK received. Raised by both host and device."]
    #[inline(always)]
    pub const fn ack_rec(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "ACK received. Raised by both host and device."]
    #[inline(always)]
    pub fn set_ack_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Data Sequence Error. The device can raise a sequence error in the following conditions: * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM The host can raise a data sequence error in the following conditions: * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    pub const fn data_seq_error(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Data Sequence Error. The device can raise a sequence error in the following conditions: * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM The host can raise a data sequence error in the following conditions: * An IN packet from the device has the wrong data PID"]
    #[inline(always)]
    pub fn set_data_seq_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SieStatus {
    #[inline(always)]
    fn default() -> SieStatus {
        SieStatus(0)
    }
}
impl core::fmt::Debug for SieStatus {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SieStatus")
            .field("vbus_detected", &self.vbus_detected())
            .field("line_state", &self.line_state())
            .field("suspended", &self.suspended())
            .field("speed", &self.speed())
            .field("vbus_over_curr", &self.vbus_over_curr())
            .field("resume", &self.resume())
            .field("rx_short_packet", &self.rx_short_packet())
            .field("connected", &self.connected())
            .field("setup_rec", &self.setup_rec())
            .field("trans_complete", &self.trans_complete())
            .field("bus_reset", &self.bus_reset())
            .field("endpoint_error", &self.endpoint_error())
            .field("crc_error", &self.crc_error())
            .field("bit_stuff_error", &self.bit_stuff_error())
            .field("rx_overflow", &self.rx_overflow())
            .field("rx_timeout", &self.rx_timeout())
            .field("nak_rec", &self.nak_rec())
            .field("stall_rec", &self.stall_rec())
            .field("ack_rec", &self.ack_rec())
            .field("data_seq_error", &self.data_seq_error())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SieStatus {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SieStatus {
            vbus_detected: bool,
            line_state: u8,
            suspended: bool,
            speed: u8,
            vbus_over_curr: bool,
            resume: bool,
            rx_short_packet: bool,
            connected: bool,
            setup_rec: bool,
            trans_complete: bool,
            bus_reset: bool,
            endpoint_error: bool,
            crc_error: bool,
            bit_stuff_error: bool,
            rx_overflow: bool,
            rx_timeout: bool,
            nak_rec: bool,
            stall_rec: bool,
            ack_rec: bool,
            data_seq_error: bool,
        }
        let proxy = SieStatus {
            vbus_detected: self.vbus_detected(),
            line_state: self.line_state(),
            suspended: self.suspended(),
            speed: self.speed(),
            vbus_over_curr: self.vbus_over_curr(),
            resume: self.resume(),
            rx_short_packet: self.rx_short_packet(),
            connected: self.connected(),
            setup_rec: self.setup_rec(),
            trans_complete: self.trans_complete(),
            bus_reset: self.bus_reset(),
            endpoint_error: self.endpoint_error(),
            crc_error: self.crc_error(),
            bit_stuff_error: self.bit_stuff_error(),
            rx_overflow: self.rx_overflow(),
            rx_timeout: self.rx_timeout(),
            nak_rec: self.nak_rec(),
            stall_rec: self.stall_rec(),
            ack_rec: self.ack_rec(),
            data_seq_error: self.data_seq_error(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmState(pub u32);
impl SmState {
    #[inline(always)]
    pub const fn state(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub fn set_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[inline(always)]
    pub const fn bc_state(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[inline(always)]
    pub fn set_bc_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[inline(always)]
    pub const fn rx_dasm(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rx_dasm(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
}
impl Default for SmState {
    #[inline(always)]
    fn default() -> SmState {
        SmState(0)
    }
}
impl core::fmt::Debug for SmState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmState")
            .field("state", &self.state())
            .field("bc_state", &self.bc_state())
            .field("rx_dasm", &self.rx_dasm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmState {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmState {
            state: u8,
            bc_state: u8,
            rx_dasm: u8,
        }
        let proxy = SmState {
            state: self.state(),
            bc_state: self.bc_state(),
            rx_dasm: self.rx_dasm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SofRd(pub u32);
impl SofRd {
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for SofRd {
    #[inline(always)]
    fn default() -> SofRd {
        SofRd(0)
    }
}
impl core::fmt::Debug for SofRd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SofRd")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SofRd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SofRd {
            count: u16,
        }
        let proxy = SofRd {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device only. Value of free-running PHY clock counter @48MHz when last SOF event occurred."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SofTimestampLast(pub u32);
impl SofTimestampLast {
    #[inline(always)]
    pub const fn sof_timestamp_last(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_sof_timestamp_last(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for SofTimestampLast {
    #[inline(always)]
    fn default() -> SofTimestampLast {
        SofTimestampLast(0)
    }
}
impl core::fmt::Debug for SofTimestampLast {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SofTimestampLast")
            .field("sof_timestamp_last", &self.sof_timestamp_last())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SofTimestampLast {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SofTimestampLast {
            sof_timestamp_last: u32,
        }
        let proxy = SofTimestampLast {
            sof_timestamp_last: self.sof_timestamp_last(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Device only. Raw value of free-running PHY clock counter @48MHz. Used to calculate time between SOF events."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SofTimestampRaw(pub u32);
impl SofTimestampRaw {
    #[inline(always)]
    pub const fn sof_timestamp_raw(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x001f_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_sof_timestamp_raw(&mut self, val: u32) {
        self.0 = (self.0 & !(0x001f_ffff << 0usize)) | (((val as u32) & 0x001f_ffff) << 0usize);
    }
}
impl Default for SofTimestampRaw {
    #[inline(always)]
    fn default() -> SofTimestampRaw {
        SofTimestampRaw(0)
    }
}
impl core::fmt::Debug for SofTimestampRaw {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SofTimestampRaw")
            .field("sof_timestamp_raw", &self.sof_timestamp_raw())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SofTimestampRaw {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SofTimestampRaw {
            sof_timestamp_raw: u32,
        }
        let proxy = SofTimestampRaw {
            sof_timestamp_raw: self.sof_timestamp_raw(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SofWr(pub u32);
impl SofWr {
    #[inline(always)]
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x07ff;
        val as u16
    }
    #[inline(always)]
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0usize)) | (((val as u32) & 0x07ff) << 0usize);
    }
}
impl Default for SofWr {
    #[inline(always)]
    fn default() -> SofWr {
        SofWr(0)
    }
}
impl core::fmt::Debug for SofWr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SofWr")
            .field("count", &self.count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SofWr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SofWr {
            count: u16,
        }
        let proxy = SofWr {
            count: self.count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Where to connect the USB controller. Should be to_phy by default."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbMuxing(pub u32);
impl UsbMuxing {
    #[inline(always)]
    pub const fn to_phy(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_to_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn to_extphy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_to_extphy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn to_digital_pad(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_to_digital_pad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn softcon(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_softcon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller."]
    #[inline(always)]
    pub const fn usbphy_as_gpio(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Use the usb DP and DM pins as GPIO pins instead of connecting them to the USB controller."]
    #[inline(always)]
    pub fn set_usbphy_as_gpio(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB. This is done at a low level so overrides all other controls."]
    #[inline(always)]
    pub const fn swap_dpdm(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Swap the USB PHY DP and DM pins and all related controls and flip receive differential data. Can be used to switch USB DP/DP on the PCB. This is done at a low level so overrides all other controls."]
    #[inline(always)]
    pub fn set_swap_dpdm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for UsbMuxing {
    #[inline(always)]
    fn default() -> UsbMuxing {
        UsbMuxing(0)
    }
}
impl core::fmt::Debug for UsbMuxing {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbMuxing")
            .field("to_phy", &self.to_phy())
            .field("to_extphy", &self.to_extphy())
            .field("to_digital_pad", &self.to_digital_pad())
            .field("softcon", &self.softcon())
            .field("usbphy_as_gpio", &self.usbphy_as_gpio())
            .field("swap_dpdm", &self.swap_dpdm())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbMuxing {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbMuxing {
            to_phy: bool,
            to_extphy: bool,
            to_digital_pad: bool,
            softcon: bool,
            usbphy_as_gpio: bool,
            swap_dpdm: bool,
        }
        let proxy = UsbMuxing {
            to_phy: self.to_phy(),
            to_extphy: self.to_extphy(),
            to_digital_pad: self.to_digital_pad(),
            softcon: self.softcon(),
            usbphy_as_gpio: self.usbphy_as_gpio(),
            swap_dpdm: self.swap_dpdm(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbPwr(pub u32);
impl UsbPwr {
    #[inline(always)]
    pub const fn vbus_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vbus_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn vbus_en_override_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vbus_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn vbus_detect(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vbus_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn vbus_detect_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_vbus_detect_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn overcurr_detect(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_overcurr_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn overcurr_detect_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_overcurr_detect_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for UsbPwr {
    #[inline(always)]
    fn default() -> UsbPwr {
        UsbPwr(0)
    }
}
impl core::fmt::Debug for UsbPwr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbPwr")
            .field("vbus_en", &self.vbus_en())
            .field("vbus_en_override_en", &self.vbus_en_override_en())
            .field("vbus_detect", &self.vbus_detect())
            .field("vbus_detect_override_en", &self.vbus_detect_override_en())
            .field("overcurr_detect", &self.overcurr_detect())
            .field("overcurr_detect_en", &self.overcurr_detect_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbPwr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbPwr {
            vbus_en: bool,
            vbus_en_override_en: bool,
            vbus_detect: bool,
            vbus_detect_override_en: bool,
            overcurr_detect: bool,
            overcurr_detect_en: bool,
        }
        let proxy = UsbPwr {
            vbus_en: self.vbus_en(),
            vbus_en_override_en: self.vbus_en_override_en(),
            vbus_detect: self.vbus_detect(),
            vbus_detect_override_en: self.vbus_detect_override_en(),
            overcurr_detect: self.overcurr_detect(),
            overcurr_detect_en: self.overcurr_detect_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbphyDirect(pub u32);
impl UsbphyDirect {
    #[doc = "Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub const fn dp_pullup_hisel(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn set_dp_pullup_hisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "DP pull up enable"]
    #[inline(always)]
    pub const fn dp_pullup_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "DP pull up enable"]
    #[inline(always)]
    pub fn set_dp_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DP pull down enable"]
    #[inline(always)]
    pub const fn dp_pulldn_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DP pull down enable"]
    #[inline(always)]
    pub fn set_dp_pulldn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub const fn dm_pullup_hisel(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    #[inline(always)]
    pub fn set_dm_pullup_hisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "DM pull up enable"]
    #[inline(always)]
    pub const fn dm_pullup_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "DM pull up enable"]
    #[inline(always)]
    pub fn set_dm_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "DM pull down enable"]
    #[inline(always)]
    pub const fn dm_pulldn_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "DM pull down enable"]
    #[inline(always)]
    pub fn set_dm_pulldn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub const fn tx_dp_oe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    #[inline(always)]
    pub fn set_tx_dp_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, Ignored. If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub const fn tx_dm_oe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, Ignored. If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    #[inline(always)]
    pub fn set_tx_dm_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub const fn tx_dp(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    #[inline(always)]
    pub fn set_tx_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Output data. TX_DIFFMODE=1, Ignored TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub const fn tx_dm(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Output data. TX_DIFFMODE=1, Ignored TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    #[inline(always)]
    pub fn set_tx_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub const fn rx_pd(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "RX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn set_rx_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub const fn tx_pd(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "TX power down override (if override enable is set). 1 = powered down."]
    #[inline(always)]
    pub fn set_tx_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "TX_FSSLEW=0: Low speed slew rate TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    pub const fn tx_fsslew(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "TX_FSSLEW=0: Low speed slew rate TX_FSSLEW=1: Full speed slew rate"]
    #[inline(always)]
    pub fn set_tx_fsslew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "TX_DIFFMODE=0: Single ended mode TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    pub const fn tx_diffmode(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "TX_DIFFMODE=0: Single ended mode TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    #[inline(always)]
    pub fn set_tx_diffmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Differential RX"]
    #[inline(always)]
    pub const fn rx_dd(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Differential RX"]
    #[inline(always)]
    pub fn set_rx_dd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "DPP pin state"]
    #[inline(always)]
    pub const fn rx_dp(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "DPP pin state"]
    #[inline(always)]
    pub fn set_rx_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "DPM pin state"]
    #[inline(always)]
    pub const fn rx_dm(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "DPM pin state"]
    #[inline(always)]
    pub fn set_rx_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "DP overcurrent"]
    #[inline(always)]
    pub const fn dp_ovcn(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "DP overcurrent"]
    #[inline(always)]
    pub fn set_dp_ovcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "DM overcurrent"]
    #[inline(always)]
    pub const fn dm_ovcn(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "DM overcurrent"]
    #[inline(always)]
    pub fn set_dm_ovcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[doc = "DP over voltage"]
    #[inline(always)]
    pub const fn dp_ovv(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "DP over voltage"]
    #[inline(always)]
    pub fn set_dp_ovv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "DM over voltage"]
    #[inline(always)]
    pub const fn dm_ovv(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "DM over voltage"]
    #[inline(always)]
    pub fn set_dm_ovv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Override rx_dd value into controller"]
    #[inline(always)]
    pub const fn rx_dd_override(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Override rx_dd value into controller"]
    #[inline(always)]
    pub fn set_rx_dd_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Override rx_dp value into controller"]
    #[inline(always)]
    pub const fn rx_dp_override(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Override rx_dp value into controller"]
    #[inline(always)]
    pub fn set_rx_dp_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Override rx_dm value into controller"]
    #[inline(always)]
    pub const fn rx_dm_override(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Override rx_dm value into controller"]
    #[inline(always)]
    pub fn set_rx_dm_override(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for UsbphyDirect {
    #[inline(always)]
    fn default() -> UsbphyDirect {
        UsbphyDirect(0)
    }
}
impl core::fmt::Debug for UsbphyDirect {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbphyDirect")
            .field("dp_pullup_hisel", &self.dp_pullup_hisel())
            .field("dp_pullup_en", &self.dp_pullup_en())
            .field("dp_pulldn_en", &self.dp_pulldn_en())
            .field("dm_pullup_hisel", &self.dm_pullup_hisel())
            .field("dm_pullup_en", &self.dm_pullup_en())
            .field("dm_pulldn_en", &self.dm_pulldn_en())
            .field("tx_dp_oe", &self.tx_dp_oe())
            .field("tx_dm_oe", &self.tx_dm_oe())
            .field("tx_dp", &self.tx_dp())
            .field("tx_dm", &self.tx_dm())
            .field("rx_pd", &self.rx_pd())
            .field("tx_pd", &self.tx_pd())
            .field("tx_fsslew", &self.tx_fsslew())
            .field("tx_diffmode", &self.tx_diffmode())
            .field("rx_dd", &self.rx_dd())
            .field("rx_dp", &self.rx_dp())
            .field("rx_dm", &self.rx_dm())
            .field("dp_ovcn", &self.dp_ovcn())
            .field("dm_ovcn", &self.dm_ovcn())
            .field("dp_ovv", &self.dp_ovv())
            .field("dm_ovv", &self.dm_ovv())
            .field("rx_dd_override", &self.rx_dd_override())
            .field("rx_dp_override", &self.rx_dp_override())
            .field("rx_dm_override", &self.rx_dm_override())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbphyDirect {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbphyDirect {
            dp_pullup_hisel: bool,
            dp_pullup_en: bool,
            dp_pulldn_en: bool,
            dm_pullup_hisel: bool,
            dm_pullup_en: bool,
            dm_pulldn_en: bool,
            tx_dp_oe: bool,
            tx_dm_oe: bool,
            tx_dp: bool,
            tx_dm: bool,
            rx_pd: bool,
            tx_pd: bool,
            tx_fsslew: bool,
            tx_diffmode: bool,
            rx_dd: bool,
            rx_dp: bool,
            rx_dm: bool,
            dp_ovcn: bool,
            dm_ovcn: bool,
            dp_ovv: bool,
            dm_ovv: bool,
            rx_dd_override: bool,
            rx_dp_override: bool,
            rx_dm_override: bool,
        }
        let proxy = UsbphyDirect {
            dp_pullup_hisel: self.dp_pullup_hisel(),
            dp_pullup_en: self.dp_pullup_en(),
            dp_pulldn_en: self.dp_pulldn_en(),
            dm_pullup_hisel: self.dm_pullup_hisel(),
            dm_pullup_en: self.dm_pullup_en(),
            dm_pulldn_en: self.dm_pulldn_en(),
            tx_dp_oe: self.tx_dp_oe(),
            tx_dm_oe: self.tx_dm_oe(),
            tx_dp: self.tx_dp(),
            tx_dm: self.tx_dm(),
            rx_pd: self.rx_pd(),
            tx_pd: self.tx_pd(),
            tx_fsslew: self.tx_fsslew(),
            tx_diffmode: self.tx_diffmode(),
            rx_dd: self.rx_dd(),
            rx_dp: self.rx_dp(),
            rx_dm: self.rx_dm(),
            dp_ovcn: self.dp_ovcn(),
            dm_ovcn: self.dm_ovcn(),
            dp_ovv: self.dp_ovv(),
            dm_ovv: self.dm_ovv(),
            rx_dd_override: self.rx_dd_override(),
            rx_dp_override: self.rx_dp_override(),
            rx_dm_override: self.rx_dm_override(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Override enable for each control in usbphy_direct"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbphyDirectOverride(pub u32);
impl UsbphyDirectOverride {
    #[inline(always)]
    pub const fn dp_pullup_hisel_override_en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dp_pullup_hisel_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn dm_pullup_hisel_override_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dm_pullup_hisel_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn dp_pullup_en_override_en(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dp_pullup_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn dp_pulldn_en_override_en(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dp_pulldn_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn dm_pulldn_en_override_en(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dm_pulldn_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn tx_dp_oe_override_en(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_dp_oe_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn tx_dm_oe_override_en(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_dm_oe_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn tx_dp_override_en(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_dp_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn tx_dm_override_en(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_dm_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn rx_pd_override_en(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rx_pd_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn tx_pd_override_en(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_pd_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn tx_fsslew_override_en(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_fsslew_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn dm_pullup_override_en(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_dm_pullup_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn tx_diffmode_override_en(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_tx_diffmode_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn rx_dd_override_en(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rx_dd_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn rx_dp_override_en(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rx_dp_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn rx_dm_override_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_rx_dm_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
}
impl Default for UsbphyDirectOverride {
    #[inline(always)]
    fn default() -> UsbphyDirectOverride {
        UsbphyDirectOverride(0)
    }
}
impl core::fmt::Debug for UsbphyDirectOverride {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbphyDirectOverride")
            .field(
                "dp_pullup_hisel_override_en",
                &self.dp_pullup_hisel_override_en(),
            )
            .field(
                "dm_pullup_hisel_override_en",
                &self.dm_pullup_hisel_override_en(),
            )
            .field("dp_pullup_en_override_en", &self.dp_pullup_en_override_en())
            .field("dp_pulldn_en_override_en", &self.dp_pulldn_en_override_en())
            .field("dm_pulldn_en_override_en", &self.dm_pulldn_en_override_en())
            .field("tx_dp_oe_override_en", &self.tx_dp_oe_override_en())
            .field("tx_dm_oe_override_en", &self.tx_dm_oe_override_en())
            .field("tx_dp_override_en", &self.tx_dp_override_en())
            .field("tx_dm_override_en", &self.tx_dm_override_en())
            .field("rx_pd_override_en", &self.rx_pd_override_en())
            .field("tx_pd_override_en", &self.tx_pd_override_en())
            .field("tx_fsslew_override_en", &self.tx_fsslew_override_en())
            .field("dm_pullup_override_en", &self.dm_pullup_override_en())
            .field("tx_diffmode_override_en", &self.tx_diffmode_override_en())
            .field("rx_dd_override_en", &self.rx_dd_override_en())
            .field("rx_dp_override_en", &self.rx_dp_override_en())
            .field("rx_dm_override_en", &self.rx_dm_override_en())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbphyDirectOverride {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbphyDirectOverride {
            dp_pullup_hisel_override_en: bool,
            dm_pullup_hisel_override_en: bool,
            dp_pullup_en_override_en: bool,
            dp_pulldn_en_override_en: bool,
            dm_pulldn_en_override_en: bool,
            tx_dp_oe_override_en: bool,
            tx_dm_oe_override_en: bool,
            tx_dp_override_en: bool,
            tx_dm_override_en: bool,
            rx_pd_override_en: bool,
            tx_pd_override_en: bool,
            tx_fsslew_override_en: bool,
            dm_pullup_override_en: bool,
            tx_diffmode_override_en: bool,
            rx_dd_override_en: bool,
            rx_dp_override_en: bool,
            rx_dm_override_en: bool,
        }
        let proxy = UsbphyDirectOverride {
            dp_pullup_hisel_override_en: self.dp_pullup_hisel_override_en(),
            dm_pullup_hisel_override_en: self.dm_pullup_hisel_override_en(),
            dp_pullup_en_override_en: self.dp_pullup_en_override_en(),
            dp_pulldn_en_override_en: self.dp_pulldn_en_override_en(),
            dm_pulldn_en_override_en: self.dm_pulldn_en_override_en(),
            tx_dp_oe_override_en: self.tx_dp_oe_override_en(),
            tx_dm_oe_override_en: self.tx_dm_oe_override_en(),
            tx_dp_override_en: self.tx_dp_override_en(),
            tx_dm_override_en: self.tx_dm_override_en(),
            rx_pd_override_en: self.rx_pd_override_en(),
            tx_pd_override_en: self.tx_pd_override_en(),
            tx_fsslew_override_en: self.tx_fsslew_override_en(),
            dm_pullup_override_en: self.dm_pullup_override_en(),
            tx_diffmode_override_en: self.tx_diffmode_override_en(),
            rx_dd_override_en: self.rx_dd_override_en(),
            rx_dp_override_en: self.rx_dp_override_en(),
            rx_dm_override_en: self.rx_dm_override_en(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Used to adjust trim values of USB phy pull down resistors."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbphyTrim(pub u32);
impl UsbphyTrim {
    #[doc = "Value to drive to USB PHY DP pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub const fn dp_pulldn_trim(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Value to drive to USB PHY DP pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn set_dp_pulldn_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Value to drive to USB PHY DM pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub const fn dm_pulldn_trim(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Value to drive to USB PHY DM pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    #[inline(always)]
    pub fn set_dm_pulldn_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
}
impl Default for UsbphyTrim {
    #[inline(always)]
    fn default() -> UsbphyTrim {
        UsbphyTrim(0)
    }
}
impl core::fmt::Debug for UsbphyTrim {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbphyTrim")
            .field("dp_pulldn_trim", &self.dp_pulldn_trim())
            .field("dm_pulldn_trim", &self.dm_pulldn_trim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbphyTrim {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbphyTrim {
            dp_pulldn_trim: u8,
            dm_pulldn_trim: u8,
        }
        let proxy = UsbphyTrim {
            dp_pulldn_trim: self.dp_pulldn_trim(),
            dm_pulldn_trim: self.dm_pulldn_trim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
