#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo {
    ptr: *mut u8,
}
unsafe impl Send for Fifo {}
unsafe impl Sync for Fifo {}
impl Fifo {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    #[inline(always)]
    pub const fn st(self) -> crate::common::Reg<regs::FifoSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Write access to this core's TX FIFO"]
    #[inline(always)]
    pub const fn wr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Read access to this core's RX FIFO"]
    #[inline(always)]
    pub const fn rd(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO0...31 output enable"]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "GPIO0...31 output enable set"]
    #[inline(always)]
    pub const fn value_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "GPIO0...31 output enable clear"]
    #[inline(always)]
    pub const fn value_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "GPIO0...31 output enable XOR"]
    #[inline(always)]
    pub const fn value_xor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp {
    ptr: *mut u8,
}
unsafe impl Send for Interp {}
unsafe impl Sync for Interp {}
impl Interp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Read/write access to accumulator 0"]
    #[inline(always)]
    pub const fn accum0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn accum1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn base2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_lane0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_lane1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_full(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_lane0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_lane1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_full(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Control register for lane 0"]
    #[inline(always)]
    pub const fn ctrl_lane0(self) -> crate::common::Reg<regs::Interp1ctrlLane0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Control register for lane 1"]
    #[inline(always)]
    pub const fn ctrl_lane1(self) -> crate::common::Reg<regs::Interp1ctrlLane1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn accum0_add(self) -> crate::common::Reg<regs::Interp1accum0add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn accum1_add(self) -> crate::common::Reg<regs::Interp1accum1add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn base_1and0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sio {
    ptr: *mut u8,
}
unsafe impl Send for Sio {}
unsafe impl Sync for Sio {}
impl Sio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Processor core identifier"]
    #[inline(always)]
    pub const fn cpuid(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Input value for GPIO0...31. In the Non-secure SIO, Secure-only GPIOs (as per ACCESSCTRL) appear as zero."]
    #[inline(always)]
    pub const fn gpio_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio_out(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio::from_ptr(self.ptr.add(0x10usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio_oe(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn fifo(self) -> Fifo {
        unsafe { Fifo::from_ptr(self.ptr.add(0x50usize) as _) }
    }
    #[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    #[inline(always)]
    pub const fn spinlock_st(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x5cusize) as _) }
    }
    #[inline(always)]
    pub const fn interp(self, n: usize) -> Interp {
        assert!(n < 2usize);
        unsafe { Interp::from_ptr(self.ptr.add(0x80usize + n * 64usize) as _) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    #[inline(always)]
    pub const fn spinlock(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize + n * 4usize) as _) }
    }
    #[doc = "Trigger a doorbell interrupt on the opposite core. Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt. Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status."]
    #[inline(always)]
    pub const fn doorbell_out_set(
        self,
    ) -> crate::common::Reg<regs::DoorbellOutSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status."]
    #[inline(always)]
    pub const fn doorbell_out_clr(
        self,
    ) -> crate::common::Reg<regs::DoorbellOutClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[doc = "Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core."]
    #[inline(always)]
    pub const fn doorbell_in_set(
        self,
    ) -> crate::common::Reg<regs::DoorbellInSet, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0188usize) as _) }
    }
    #[doc = "Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core."]
    #[inline(always)]
    pub const fn doorbell_in_clr(
        self,
    ) -> crate::common::Reg<regs::DoorbellInClr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x018cusize) as _) }
    }
    #[doc = "Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register."]
    #[inline(always)]
    pub const fn peri_nonsec(self) -> crate::common::Reg<regs::PeriNonsec, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0190usize) as _) }
    }
    #[doc = "Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect."]
    #[inline(always)]
    pub const fn riscv_softirq(self) -> crate::common::Reg<regs::RiscvSoftirq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a0usize) as _) }
    }
    #[doc = "Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores."]
    #[inline(always)]
    pub const fn mtime_ctrl(self) -> crate::common::Reg<regs::MtimeCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01a4usize) as _) }
    }
    #[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
    #[inline(always)]
    pub const fn mtime(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b0usize) as _) }
    }
    #[doc = "Read/write access to the high half of RISC-V Machine-mode timer. This register is shared between both cores. If both cores write on the same cycle, core 1 takes precedence."]
    #[inline(always)]
    pub const fn mtimeh(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b4usize) as _) }
    }
    #[doc = "Low half of RISC-V Machine-mode timer comparator. This register is core-local, i.e., each core gets a copy of this register, with the comparison result routed to its own interrupt line. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
    #[inline(always)]
    pub const fn mtimecmp(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01b8usize) as _) }
    }
    #[doc = "High half of RISC-V Machine-mode timer comparator. This register is core-local. The timer interrupt is asserted whenever MTIME is greater than or equal to MTIMECMP. This comparison is unsigned, and performed on the full 64-bit values."]
    #[inline(always)]
    pub const fn mtimecmph(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01bcusize) as _) }
    }
    #[doc = "Control register for TMDS encoder."]
    #[inline(always)]
    pub const fn tmds_ctrl(self) -> crate::common::Reg<regs::TmdsCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c0usize) as _) }
    }
    #[doc = "Write-only access to the TMDS colour data register."]
    #[inline(always)]
    pub const fn tmds_wdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c4usize) as _) }
    }
    #[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value (3x10-bit symbols). The PEEK alias does not shift the colour register when read, but still advances the running DC balance state of each encoder. This is useful for pixel doubling."]
    #[inline(always)]
    pub const fn tmds_peek_single(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01c8usize) as _) }
    }
    #[doc = "Get the encoding of one pixel's worth of colour data, packed into a 32-bit value. The packing is 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane. This format is intended for shifting out with the HSTX peripheral on RP2350. The POP alias shifts the colour register when read, as well as advancing the running DC balance state of each encoder."]
    #[inline(always)]
    pub const fn tmds_pop_single(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01ccusize) as _) }
    }
    #[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 0 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d0usize) as _) }
    }
    #[doc = "Get lane 0 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d4usize) as _) }
    }
    #[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 1 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01d8usize) as _) }
    }
    #[doc = "Get lane 1 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01dcusize) as _) }
    }
    #[doc = "Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The PEEK alias does not shift the colour register when read, but still advances the lane 2 DC balance state. This is useful if all 3 lanes' worth of encode are to be read at once, rather than processing the entire scanline for one lane before moving to the next lane."]
    #[inline(always)]
    pub const fn tmds_peek_double_l2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e0usize) as _) }
    }
    #[doc = "Get lane 2 of the encoding of two pixels' worth of colour data. Two 10-bit TMDS symbols are packed at the bottom of a 32-bit word. The POP alias shifts the colour register when read, according to the values of PIX_SHIFT and PIX2_NOSHIFT."]
    #[inline(always)]
    pub const fn tmds_pop_double_l2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x01e4usize) as _) }
    }
}
pub mod regs;
pub mod vals;
