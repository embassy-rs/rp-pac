#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div {
    ptr: *mut u8,
}
unsafe impl Send for Div {}
unsafe impl Sync for Div {}
impl Div {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub const fn udividend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub const fn udivisor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub const fn sdividend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub const fn sdivisor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    #[inline(always)]
    pub const fn quotient(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    #[inline(always)]
    pub const fn remainder(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Control and status register for divider."]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::DivCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
}
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Write access to this core's TX FIFO"]
    #[inline(always)]
    pub const fn wr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Read access to this core's RX FIFO"]
    #[inline(always)]
    pub const fn rd(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
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
    #[doc = "QSPI output value"]
    #[inline(always)]
    pub const fn value(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "QSPI output value set"]
    #[inline(always)]
    pub const fn value_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "QSPI output value clear"]
    #[inline(always)]
    pub const fn value_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "QSPI output value XOR"]
    #[inline(always)]
    pub const fn value_xor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Read/write access to accumulator 1"]
    #[inline(always)]
    pub const fn accum1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Read/write access to BASE0 register."]
    #[inline(always)]
    pub const fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Read/write access to BASE1 register."]
    #[inline(always)]
    pub const fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Read/write access to BASE2 register."]
    #[inline(always)]
    pub const fn base2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_lane0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_lane1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub const fn pop_full(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_lane0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_lane1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub const fn peek_full(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Control register for lane 0"]
    #[inline(always)]
    pub const fn ctrl_lane0(self) -> crate::common::Reg<regs::Interp1ctrlLane0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Control register for lane 1"]
    #[inline(always)]
    pub const fn ctrl_lane1(self) -> crate::common::Reg<regs::Interp1ctrlLane1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub const fn accum0_add(self) -> crate::common::Reg<regs::Interp1accum0add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub const fn accum1_add(self) -> crate::common::Reg<regs::Interp1accum1add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub const fn base_1and0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
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
    #[doc = "Processor core identifier Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    #[inline(always)]
    pub const fn cpuid(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Input value for GPIO pins"]
    #[inline(always)]
    pub const fn gpio_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio_out(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio::from_ptr(self.ptr.add(16usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn gpio_oe(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio::from_ptr(self.ptr.add(32usize + n * 32usize) as _) }
    }
    #[inline(always)]
    pub const fn fifo(self) -> Fifo {
        unsafe { Fifo::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    #[inline(always)]
    pub const fn spinlock_st(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[inline(always)]
    pub const fn div(self) -> Div {
        unsafe { Div::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[inline(always)]
    pub const fn interp(self, n: usize) -> Interp {
        assert!(n < 2usize);
        unsafe { Interp::from_ptr(self.ptr.add(128usize + n * 64usize) as _) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    #[inline(always)]
    pub const fn spinlock(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize + n * 4usize) as _) }
    }
}
pub mod regs;
