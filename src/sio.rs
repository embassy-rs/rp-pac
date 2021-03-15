use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Fifo(pub *mut u8);
unsafe impl Send for Fifo {}
unsafe impl Sync for Fifo {}
impl Fifo {
    #[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    pub fn st(self) -> Reg<regs::FifoSt, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Write access to this core's TX FIFO"]
    pub fn wr(self) -> Reg<u32, W> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read access to this core's RX FIFO"]
    pub fn rd(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Gpio(pub *mut u8);
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[doc = "QSPI output value"]
    pub fn value(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "QSPI output value set"]
    pub fn value_set(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "QSPI output value clear"]
    pub fn value_clr(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "QSPI output value XOR"]
    pub fn value_xor(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Interp(pub *mut u8);
unsafe impl Send for Interp {}
unsafe impl Sync for Interp {}
impl Interp {
    #[doc = "Read/write access to accumulator 0"]
    pub fn accum0(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Read/write access to accumulator 1"]
    pub fn accum1(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Read/write access to BASE0 register."]
    pub fn base0(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Read/write access to BASE1 register."]
    pub fn base1(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Read/write access to BASE2 register."]
    pub fn base2(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn pop_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn pop_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn pop_full(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    pub fn peek_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    pub fn peek_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    pub fn peek_full(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Control register for lane 0"]
    pub fn ctrl_lane0(self) -> Reg<regs::Interp0CtrlLane0, RW> {
        unsafe { Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Control register for lane 1"]
    pub fn ctrl_lane1(self) -> Reg<regs::Interp0CtrlLane1, RW> {
        unsafe { Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub fn accum0_add(self) -> Reg<regs::Interp0Accum0Add, RW> {
        unsafe { Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub fn accum1_add(self) -> Reg<regs::Interp0Accum1Add, RW> {
        unsafe { Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub fn base_1and0(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(60usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Div(pub *mut u8);
unsafe impl Send for Div {}
unsafe impl Sync for Div {}
impl Div {
    #[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    pub fn udividend(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    pub fn udivisor(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    pub fn sdividend(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    pub fn sdivisor(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    pub fn quotient(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    pub fn remainder(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Control and status register for divider."]
    pub fn csr(self) -> Reg<regs::DivCsr, RW> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
}
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
#[derive(Copy, Clone)]
pub struct Sio(pub *mut u8);
unsafe impl Send for Sio {}
unsafe impl Sync for Sio {}
impl Sio {
    #[doc = "Processor core identifier Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    pub fn cpuid(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    pub fn spinlock_st(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(92usize)) }
    }
    pub fn div(self) -> Div {
        unsafe { Div(self.0.add(96usize)) }
    }
    pub fn fifo(self) -> Fifo {
        unsafe { Fifo(self.0.add(80usize)) }
    }
    pub fn interp(self, n: usize) -> Interp {
        assert!(n < 2usize);
        unsafe { Interp(self.0.add(128usize + n * 64usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock(self, n: usize) -> Reg<u32, R> {
        assert!(n < 32usize);
        unsafe { Reg::from_ptr(self.0.add(256usize + n * 4usize)) }
    }
    pub fn gpio_oe(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio(self.0.add(32usize + n * 32usize)) }
    }
    #[doc = "Input value for GPIO pins"]
    pub fn gpio_in(self, n: usize) -> Reg<u32, RW> {
        assert!(n < 2usize);
        unsafe { Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
    pub fn gpio_out(self, n: usize) -> Gpio {
        assert!(n < 2usize);
        unsafe { Gpio(self.0.add(16usize + n * 32usize)) }
    }
}
pub mod regs;
