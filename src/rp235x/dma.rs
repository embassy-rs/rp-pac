#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Channel {
    ptr: *mut u8,
}
unsafe impl Send for Channel {}
unsafe impl Sync for Channel {}
impl Channel {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "DMA Channel 12 Read Address pointer"]
    #[inline(always)]
    pub const fn read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "DMA Channel 12 Write Address pointer"]
    #[inline(always)]
    pub const fn write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "DMA Channel 12 Transfer Count"]
    #[inline(always)]
    pub const fn trans_count(self) -> crate::common::Reg<regs::ChTransCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "DMA Channel 12 Control and Status"]
    #[inline(always)]
    pub const fn ctrl_trig(self) -> crate::common::Reg<regs::CtrlTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Alias for channel 12 CTRL register"]
    #[inline(always)]
    pub const fn al1_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Alias for channel 12 READ_ADDR register"]
    #[inline(always)]
    pub const fn al1_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Alias for channel 12 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al1_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Alias for channel 12 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al1_trans_count_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Alias for channel 12 CTRL register"]
    #[inline(always)]
    pub const fn al2_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Alias for channel 12 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al2_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Alias for channel 12 READ_ADDR register"]
    #[inline(always)]
    pub const fn al2_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Alias for channel 12 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al2_write_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Alias for channel 12 CTRL register"]
    #[inline(always)]
    pub const fn al3_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Alias for channel 12 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al3_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Alias for channel 12 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al3_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Alias for channel 12 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al3_read_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn dbg_ctdreq(self) -> crate::common::Reg<regs::DbgCtdreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn dbg_tcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
}
#[doc = "DMA with separate read and write masters"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dma {
    ptr: *mut u8,
}
unsafe impl Send for Dma {}
unsafe impl Sync for Dma {}
impl Dma {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn ch(self, n: usize) -> Channel {
        assert!(n < 16usize);
        unsafe { Channel::from_ptr(self.ptr.add(0x0usize + n * 64usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0400usize + n * 16usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    #[inline(always)]
    pub const fn inte(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0404usize + n * 16usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0408usize + n * 16usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    #[inline(always)]
    pub const fn ints(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x040cusize + n * 16usize) as _) }
    }
    #[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer(self, n: usize) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0440usize + n * 4usize) as _) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    #[inline(always)]
    pub const fn multi_chan_trigger(
        self,
    ) -> crate::common::Reg<regs::MultiChanTrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0450usize) as _) }
    }
    #[doc = "Sniffer Control"]
    #[inline(always)]
    pub const fn sniff_ctrl(self) -> crate::common::Reg<regs::SniffCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0454usize) as _) }
    }
    #[doc = "Data accumulator for sniff hardware"]
    #[inline(always)]
    pub const fn sniff_data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0458usize) as _) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    #[inline(always)]
    pub const fn fifo_levels(self) -> crate::common::Reg<regs::FifoLevels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0460usize) as _) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    #[inline(always)]
    pub const fn chan_abort(self) -> crate::common::Reg<regs::ChanAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0464usize) as _) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    #[inline(always)]
    pub const fn n_channels(self) -> crate::common::Reg<regs::Nchannels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0468usize) as _) }
    }
    #[doc = "Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SeccfgCh, crate::common::RW> {
        assert!(n < 16usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0480usize + n * 4usize) as _) }
    }
    #[doc = "Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::SeccfgIrq, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04c0usize + n * 4usize) as _) }
    }
    #[doc = "Miscellaneous security configuration"]
    #[inline(always)]
    pub const fn seccfg_misc(self) -> crate::common::Reg<regs::SeccfgMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04d0usize) as _) }
    }
    #[doc = "Control register for DMA MPU. Accessible only from a Privileged context."]
    #[inline(always)]
    pub const fn mpu_ctrl(self) -> crate::common::Reg<regs::MpuCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0500usize) as _) }
    }
    #[doc = "Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0504usize + n * 8usize) as _) }
    }
    #[doc = "Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0508usize + n * 8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
