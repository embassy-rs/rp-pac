#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Div(pub *mut u8);
unsafe impl Send for Div {}
unsafe impl Sync for Div {}
impl Div {
    #[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub fn udividend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVISOR/SDIVISOR are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    #[inline(always)]
    pub fn udivisor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub fn sdividend(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    #[inline(always)]
    pub fn sdivisor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    #[inline(always)]
    pub fn quotient(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    #[inline(always)]
    pub fn remainder(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Control and status register for divider."]
    #[inline(always)]
    pub fn csr(self) -> crate::common::Reg<regs::DivCsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fifo(pub *mut u8);
unsafe impl Send for Fifo {}
unsafe impl Sync for Fifo {}
impl Fifo {
    #[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    #[inline(always)]
    pub fn st(self) -> crate::common::Reg<regs::FifoSt, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Write access to this core's TX FIFO"]
    #[inline(always)]
    pub fn wr(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read access to this core's RX FIFO"]
    #[inline(always)]
    pub fn rd(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio(pub *mut u8);
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[doc = "QSPI output enable"]
    #[inline(always)]
    pub fn value(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "QSPI output enable set"]
    #[inline(always)]
    pub fn value_set(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "QSPI output enable clear"]
    #[inline(always)]
    pub fn value_clr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "QSPI output enable XOR"]
    #[inline(always)]
    pub fn value_xor(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp(pub *mut u8);
unsafe impl Send for Interp {}
unsafe impl Sync for Interp {}
impl Interp {
    #[doc = "Read/write access to accumulator 0"]
    #[inline(always)]
    pub fn accum0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Read/write access to accumulator 1"]
    #[inline(always)]
    pub fn accum1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read/write access to BASE0 register."]
    #[inline(always)]
    pub fn base0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Read/write access to BASE1 register."]
    #[inline(always)]
    pub fn base1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Read/write access to BASE2 register."]
    #[inline(always)]
    pub fn base2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub fn pop_lane0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub fn pop_lane1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    #[inline(always)]
    pub fn pop_full(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub fn peek_lane0(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub fn peek_lane1(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    #[inline(always)]
    pub fn peek_full(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Control register for lane 0"]
    #[inline(always)]
    pub fn ctrl_lane0(self) -> crate::common::Reg<regs::Interp0ctrlLane0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Control register for lane 1"]
    #[inline(always)]
    pub fn ctrl_lane1(self) -> crate::common::Reg<regs::Interp0ctrlLane1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    #[inline(always)]
    pub fn accum0_add(self) -> crate::common::Reg<regs::Interp0accum0add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    #[inline(always)]
    pub fn accum1_add(self) -> crate::common::Reg<regs::Interp0accum1add, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    #[inline(always)]
    pub fn base_1and0(self) -> crate::common::Reg<u32, crate::common::W> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
}
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sio(pub *mut u8);
unsafe impl Send for Sio {}
unsafe impl Sync for Sio {}
impl Sio {
    #[doc = "Processor core identifier Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    #[inline(always)]
    pub fn cpuid(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Input value for GPIO pins"]
    #[inline(always)]
    pub fn gpio_in(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
    #[inline(always)]
    pub fn gpio_out(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio(self.0.add(16usize + n * 32usize)) }
    }
    #[inline(always)]
    pub fn gpio_oe(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio(self.0.add(32usize + n * 32usize)) }
    }
    #[inline(always)]
    pub fn fifo(self) -> Fifo {
        unsafe { Fifo(self.0.add(80usize)) }
    }
    #[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    #[inline(always)]
    pub fn spinlock_st(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(92usize)) }
    }
    #[inline(always)]
    pub fn div(self) -> Div {
        unsafe { Div(self.0.add(96usize)) }
    }
    #[inline(always)]
    pub fn interp(self, n: usize) -> Interp {
        assert!(n < 2usize);
        unsafe { Interp(self.0.add(128usize + n * 64usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    #[inline(always)]
    pub fn spinlock(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(256usize + n * 4usize)) }
    }
}
pub mod regs;
