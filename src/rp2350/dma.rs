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
    #[doc = "DMA Channel 1 Read Address pointer"]
    #[inline(always)]
    pub const fn read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "DMA Channel 1 Write Address pointer"]
    #[inline(always)]
    pub const fn write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "DMA Channel 1 Transfer Count"]
    #[inline(always)]
    pub const fn trans_count(self) -> crate::common::Reg<regs::Ch1transCount, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "DMA Channel 1 Control and Status"]
    #[inline(always)]
    pub const fn ctrl_trig(self) -> crate::common::Reg<regs::CtrlTrig, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    #[inline(always)]
    pub const fn al1_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    #[inline(always)]
    pub const fn al1_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al1_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al1_trans_count_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    #[inline(always)]
    pub const fn al2_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al2_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register"]
    #[inline(always)]
    pub const fn al2_read_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al2_write_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Alias for channel 1 CTRL register"]
    #[inline(always)]
    pub const fn al3_ctrl(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Alias for channel 1 WRITE_ADDR register"]
    #[inline(always)]
    pub const fn al3_write_addr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Alias for channel 1 TRANS_COUNT register"]
    #[inline(always)]
    pub const fn al3_trans_count(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Alias for channel 1 READ_ADDR register This is a trigger register (0xc). Writing a nonzero value will reload the channel counter and start the channel."]
    #[inline(always)]
    pub const fn al3_read_addr_trig(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Read: get channel DREQ counter (i.e. how many accesses the DMA expects it can perform on the peripheral without overflow/underflow. Write any value: clears the counter, and cause channel to re-initiate DREQ handshake."]
    #[inline(always)]
    pub const fn dbg_ctdreq(self) -> crate::common::Reg<regs::DbgCtdreq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize) as _) }
    }
    #[doc = "Read to get channel TRANS_COUNT reload value, i.e. the length of the next transfer"]
    #[inline(always)]
    pub const fn dbg_tcr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2052usize) as _) }
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
        unsafe { Channel::from_ptr(self.ptr.add(0usize + n * 64usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1024usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 0"]
    #[inline(always)]
    pub const fn inte0(self) -> crate::common::Reg<regs::Inte0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1028usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf0(self) -> crate::common::Reg<regs::Intf0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1032usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 0"]
    #[inline(always)]
    pub const fn ints0(self) -> crate::common::Reg<regs::Ints0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1036usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr1(self) -> crate::common::Reg<regs::Intr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1040usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 1"]
    #[inline(always)]
    pub const fn inte1(self) -> crate::common::Reg<regs::Inte1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1044usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf1(self) -> crate::common::Reg<regs::Intf1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1048usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 1"]
    #[inline(always)]
    pub const fn ints1(self) -> crate::common::Reg<regs::Ints1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1052usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr2(self) -> crate::common::Reg<regs::Intr2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1056usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 2"]
    #[inline(always)]
    pub const fn inte2(self) -> crate::common::Reg<regs::Inte2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1060usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf2(self) -> crate::common::Reg<regs::Intf2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1064usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 2"]
    #[inline(always)]
    pub const fn ints2(self) -> crate::common::Reg<regs::Ints2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1068usize) as _) }
    }
    #[doc = "Interrupt Status (raw)"]
    #[inline(always)]
    pub const fn intr3(self) -> crate::common::Reg<regs::Intr3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1072usize) as _) }
    }
    #[doc = "Interrupt Enables for IRQ 3"]
    #[inline(always)]
    pub const fn inte3(self) -> crate::common::Reg<regs::Inte3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1076usize) as _) }
    }
    #[doc = "Force Interrupts"]
    #[inline(always)]
    pub const fn intf3(self) -> crate::common::Reg<regs::Intf3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1080usize) as _) }
    }
    #[doc = "Interrupt Status for IRQ 3"]
    #[inline(always)]
    pub const fn ints3(self) -> crate::common::Reg<regs::Ints3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1084usize) as _) }
    }
    #[doc = "Pacing (X/Y) fractional timer The pacing timer produces TREQ assertions at a rate set by ((X/Y) * sys_clk). This equation is evaluated every sys_clk cycles and therefore can only generate TREQs at a rate of 1 per sys_clk (i.e. permanent TREQ) or less."]
    #[inline(always)]
    pub const fn timer(self, n: usize) -> crate::common::Reg<regs::Timer, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1088usize + n * 4usize) as _) }
    }
    #[doc = "Trigger one or more channels simultaneously"]
    #[inline(always)]
    pub const fn multi_chan_trigger(
        self,
    ) -> crate::common::Reg<regs::MultiChanTrigger, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1104usize) as _) }
    }
    #[doc = "Sniffer Control"]
    #[inline(always)]
    pub const fn sniff_ctrl(self) -> crate::common::Reg<regs::SniffCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1108usize) as _) }
    }
    #[doc = "Data accumulator for sniff hardware"]
    #[inline(always)]
    pub const fn sniff_data(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1112usize) as _) }
    }
    #[doc = "Debug RAF, WAF, TDF levels"]
    #[inline(always)]
    pub const fn fifo_levels(self) -> crate::common::Reg<regs::FifoLevels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1120usize) as _) }
    }
    #[doc = "Abort an in-progress transfer sequence on one or more channels"]
    #[inline(always)]
    pub const fn chan_abort(self) -> crate::common::Reg<regs::ChanAbort, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1124usize) as _) }
    }
    #[doc = "The number of channels this DMA instance is equipped with. This DMA supports up to 16 hardware channels, but can be configured with as few as one, to minimise silicon area."]
    #[inline(always)]
    pub const fn n_channels(self) -> crate::common::Reg<regs::Nchannels, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1128usize) as _) }
    }
    #[doc = "Security configuration for channel 0. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch0(self) -> crate::common::Reg<regs::SeccfgCh0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1152usize) as _) }
    }
    #[doc = "Security configuration for channel 1. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch1(self) -> crate::common::Reg<regs::SeccfgCh1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1156usize) as _) }
    }
    #[doc = "Security configuration for channel 2. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch2(self) -> crate::common::Reg<regs::SeccfgCh2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1160usize) as _) }
    }
    #[doc = "Security configuration for channel 3. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch3(self) -> crate::common::Reg<regs::SeccfgCh3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1164usize) as _) }
    }
    #[doc = "Security configuration for channel 4. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch4(self) -> crate::common::Reg<regs::SeccfgCh4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1168usize) as _) }
    }
    #[doc = "Security configuration for channel 5. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch5(self) -> crate::common::Reg<regs::SeccfgCh5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1172usize) as _) }
    }
    #[doc = "Security configuration for channel 6. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch6(self) -> crate::common::Reg<regs::SeccfgCh6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1176usize) as _) }
    }
    #[doc = "Security configuration for channel 7. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch7(self) -> crate::common::Reg<regs::SeccfgCh7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1180usize) as _) }
    }
    #[doc = "Security configuration for channel 8. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch8(self) -> crate::common::Reg<regs::SeccfgCh8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1184usize) as _) }
    }
    #[doc = "Security configuration for channel 9. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch9(self) -> crate::common::Reg<regs::SeccfgCh9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1188usize) as _) }
    }
    #[doc = "Security configuration for channel 10. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch10(self) -> crate::common::Reg<regs::SeccfgCh10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1192usize) as _) }
    }
    #[doc = "Security configuration for channel 11. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch11(self) -> crate::common::Reg<regs::SeccfgCh11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1196usize) as _) }
    }
    #[doc = "Security configuration for channel 12. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch12(self) -> crate::common::Reg<regs::SeccfgCh12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1200usize) as _) }
    }
    #[doc = "Security configuration for channel 13. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch13(self) -> crate::common::Reg<regs::SeccfgCh13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1204usize) as _) }
    }
    #[doc = "Security configuration for channel 14. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch14(self) -> crate::common::Reg<regs::SeccfgCh14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1208usize) as _) }
    }
    #[doc = "Security configuration for channel 15. Control whether this channel performs Secure/Non-secure and Privileged/Unprivileged bus accesses. If this channel generates bus accesses of some security level, an access of at least that level (in the order S+P > S+U > NS+P > NS+U) is required to program, trigger, abort, check the status of, interrupt on or acknowledge the interrupt of this channel. This register automatically locks down (becomes read-only) once software starts to configure the channel. This register is world-readable, but is writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn seccfg_ch15(self) -> crate::common::Reg<regs::SeccfgCh15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1212usize) as _) }
    }
    #[doc = "Security configuration for IRQ 0. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq0(self) -> crate::common::Reg<regs::SeccfgIrq0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1216usize) as _) }
    }
    #[doc = "Security configuration for IRQ 1. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq1(self) -> crate::common::Reg<regs::SeccfgIrq1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1220usize) as _) }
    }
    #[doc = "Security configuration for IRQ 2. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq2(self) -> crate::common::Reg<regs::SeccfgIrq2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1224usize) as _) }
    }
    #[doc = "Security configuration for IRQ 3. Control whether the IRQ permits configuration by Non-secure/Unprivileged contexts, and whether it can observe Secure/Privileged channel interrupt flags."]
    #[inline(always)]
    pub const fn seccfg_irq3(self) -> crate::common::Reg<regs::SeccfgIrq3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1228usize) as _) }
    }
    #[doc = "Miscellaneous security configuration"]
    #[inline(always)]
    pub const fn seccfg_misc(self) -> crate::common::Reg<regs::SeccfgMisc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1232usize) as _) }
    }
    #[doc = "Control register for DMA MPU. Accessible only from a Privileged context."]
    #[inline(always)]
    pub const fn mpu_ctrl(self) -> crate::common::Reg<regs::MpuCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1280usize) as _) }
    }
    #[doc = "Base address register for MPU region 0. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar0(self) -> crate::common::Reg<regs::MpuBar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1284usize) as _) }
    }
    #[doc = "Limit address register for MPU region 0. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar0(self) -> crate::common::Reg<regs::MpuLar0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1288usize) as _) }
    }
    #[doc = "Base address register for MPU region 1. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar1(self) -> crate::common::Reg<regs::MpuBar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1292usize) as _) }
    }
    #[doc = "Limit address register for MPU region 1. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar1(self) -> crate::common::Reg<regs::MpuLar1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1296usize) as _) }
    }
    #[doc = "Base address register for MPU region 2. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar2(self) -> crate::common::Reg<regs::MpuBar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1300usize) as _) }
    }
    #[doc = "Limit address register for MPU region 2. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar2(self) -> crate::common::Reg<regs::MpuLar2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1304usize) as _) }
    }
    #[doc = "Base address register for MPU region 3. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar3(self) -> crate::common::Reg<regs::MpuBar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1308usize) as _) }
    }
    #[doc = "Limit address register for MPU region 3. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar3(self) -> crate::common::Reg<regs::MpuLar3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1312usize) as _) }
    }
    #[doc = "Base address register for MPU region 4. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar4(self) -> crate::common::Reg<regs::MpuBar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1316usize) as _) }
    }
    #[doc = "Limit address register for MPU region 4. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar4(self) -> crate::common::Reg<regs::MpuLar4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1320usize) as _) }
    }
    #[doc = "Base address register for MPU region 5. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar5(self) -> crate::common::Reg<regs::MpuBar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1324usize) as _) }
    }
    #[doc = "Limit address register for MPU region 5. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar5(self) -> crate::common::Reg<regs::MpuLar5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1328usize) as _) }
    }
    #[doc = "Base address register for MPU region 6. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar6(self) -> crate::common::Reg<regs::MpuBar6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1332usize) as _) }
    }
    #[doc = "Limit address register for MPU region 6. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar6(self) -> crate::common::Reg<regs::MpuLar6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1336usize) as _) }
    }
    #[doc = "Base address register for MPU region 7. Writable only from a Secure, Privileged context."]
    #[inline(always)]
    pub const fn mpu_bar7(self) -> crate::common::Reg<regs::MpuBar7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1340usize) as _) }
    }
    #[doc = "Limit address register for MPU region 7. Writable only from a Secure, Privileged context, with the exception of the P bit."]
    #[inline(always)]
    pub const fn mpu_lar7(self) -> crate::common::Reg<regs::MpuLar7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(1344usize) as _) }
    }
}
pub mod regs;
pub mod vals;
