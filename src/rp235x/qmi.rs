#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Mem {
    ptr: *mut u8,
}
unsafe impl Send for Mem {}
unsafe impl Sync for Mem {}
impl Mem {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Timing configuration register for memory address window 1."]
    #[inline(always)]
    pub const fn timing(self) -> crate::common::Reg<regs::Timing, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Read transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn rfmt(self) -> crate::common::Reg<regs::Rfmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Command constants used for reads from memory address window 1. The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
    #[inline(always)]
    pub const fn rcmd(self) -> crate::common::Reg<regs::Rcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Write transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default."]
    #[inline(always)]
    pub const fn wfmt(self) -> crate::common::Reg<regs::Wfmt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
    #[inline(always)]
    pub const fn wcmd(self) -> crate::common::Reg<regs::Wcmd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
}
#[doc = "QSPI Memory Interface. Provides a memory-mapped interface to up to two SPI/DSPI/QSPI flash or PSRAM devices. Also provides a serial interface for programming and configuration of the external device."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Qmi {
    ptr: *mut u8,
}
unsafe impl Send for Qmi {}
unsafe impl Sync for Qmi {}
impl Qmi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported."]
    #[inline(always)]
    pub const fn direct_csr(self) -> crate::common::Reg<regs::DirectCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Transmit FIFO for direct mode"]
    #[inline(always)]
    pub const fn direct_tx(self) -> crate::common::Reg<regs::DirectTx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Receive FIFO for direct mode"]
    #[inline(always)]
    pub const fn direct_rx(self) -> crate::common::Reg<regs::DirectRx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[inline(always)]
    pub const fn mem(self, n: usize) -> Mem {
        assert!(n < 2usize);
        unsafe { Mem::from_ptr(self.ptr.add(0x0cusize + n * 20usize) as _) }
    }
    #[doc = "Configure address translation for XIP virtual addresses 0x000000 through 0x3fffff (a 4 MiB window starting at +0 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
    #[inline(always)]
    pub const fn atrans(self, n: usize) -> crate::common::Reg<regs::Atrans, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize + n * 4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
