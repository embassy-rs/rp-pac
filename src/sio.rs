use crate::generic::*;
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
#[derive(Copy, Clone)]
pub struct Sio(*mut u8);
unsafe impl Send for Sio {}
unsafe impl Sync for Sio {}
impl Sio {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Processor core identifier Value is 0 when read from processor core 0, and 1 when read from processor core 1."]
    pub fn cpuid(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Input value for GPIO pins"]
    pub fn gpio_in(self) -> Reg<regs::GpioIn, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Input value for QSPI pins"]
    pub fn gpio_hi_in(self) -> Reg<regs::GpioHiIn, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "GPIO output value"]
    pub fn gpio_out(self) -> Reg<regs::GpioOut, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "GPIO output value set"]
    pub fn gpio_out_set(self) -> Reg<regs::GpioOutSet, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "GPIO output value clear"]
    pub fn gpio_out_clr(self) -> Reg<regs::GpioOutClr, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "GPIO output value XOR"]
    pub fn gpio_out_xor(self) -> Reg<regs::GpioOutXor, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "GPIO output enable"]
    pub fn gpio_oe(self) -> Reg<regs::GpioOe, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "GPIO output enable set"]
    pub fn gpio_oe_set(self) -> Reg<regs::GpioOeSet, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "GPIO output enable clear"]
    pub fn gpio_oe_clr(self) -> Reg<regs::GpioOeClr, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "GPIO output enable XOR"]
    pub fn gpio_oe_xor(self) -> Reg<regs::GpioOeXor, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "QSPI output value"]
    pub fn gpio_hi_out(self) -> Reg<regs::GpioHiOut, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "QSPI output value set"]
    pub fn gpio_hi_out_set(self) -> Reg<regs::GpioHiOutSet, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "QSPI output value clear"]
    pub fn gpio_hi_out_clr(self) -> Reg<regs::GpioHiOutClr, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "QSPI output value XOR"]
    pub fn gpio_hi_out_xor(self) -> Reg<regs::GpioHiOutXor, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "QSPI output enable"]
    pub fn gpio_hi_oe(self) -> Reg<regs::GpioHiOe, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "QSPI output enable set"]
    pub fn gpio_hi_oe_set(self) -> Reg<regs::GpioHiOeSet, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "QSPI output enable clear"]
    pub fn gpio_hi_oe_clr(self) -> Reg<regs::GpioHiOeClr, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "QSPI output enable XOR"]
    pub fn gpio_hi_oe_xor(self) -> Reg<regs::GpioHiOeXor, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
    pub fn fifo_st(self) -> Reg<regs::FifoSt, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "Write access to this core's TX FIFO"]
    pub fn fifo_wr(self) -> Reg<u32, W> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Read access to this core's RX FIFO"]
    pub fn fifo_rd(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Spinlock state A bitmap containing the state of all 32 spinlocks (1=locked). Mainly intended for debugging."]
    pub fn spinlock_st(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Divider unsigned dividend Write to the DIVIDEND operand of the divider, i.e. the p in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    pub fn div_udividend(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "Divider unsigned divisor Write to the DIVISOR operand of the divider, i.e. the q in `p / q`. Any operand write starts a new calculation. The results appear in QUOTIENT, REMAINDER. UDIVIDEND/SDIVIDEND are aliases of the same internal register. The U alias starts an unsigned calculation, and the S alias starts a signed calculation."]
    pub fn div_udivisor(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "Divider signed dividend The same as UDIVIDEND, but starts a signed calculation, rather than unsigned."]
    pub fn div_sdividend(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "Divider signed divisor The same as UDIVISOR, but starts a signed calculation, rather than unsigned."]
    pub fn div_sdivisor(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "Divider result quotient The result of `DIVIDEND / DIVISOR` (division). Contents undefined while CSR_READY is low. For signed calculations, QUOTIENT is negative when the signs of DIVIDEND and DIVISOR differ. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags. Reading from QUOTIENT clears the CSR_DIRTY flag, so should read results in the order REMAINDER, QUOTIENT if CSR_DIRTY is used."]
    pub fn div_quotient(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "Divider result remainder The result of `DIVIDEND % DIVISOR` (modulo). Contents undefined while CSR_READY is low. For signed calculations, REMAINDER is negative only when DIVIDEND is negative. This register can be written to directly, for context save/restore purposes. This halts any in-progress calculation and sets the CSR_READY and CSR_DIRTY flags."]
    pub fn div_remainder(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    #[doc = "Control and status register for divider."]
    pub fn div_csr(self) -> Reg<regs::DivCsr, RW> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    #[doc = "Read/write access to accumulator 0"]
    pub fn interp0_accum0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
    #[doc = "Read/write access to accumulator 1"]
    pub fn interp0_accum1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(132usize)) }
    }
    #[doc = "Read/write access to BASE0 register."]
    pub fn interp0_base0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(136usize)) }
    }
    #[doc = "Read/write access to BASE1 register."]
    pub fn interp0_base1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(140usize)) }
    }
    #[doc = "Read/write access to BASE2 register."]
    pub fn interp0_base2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(144usize)) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp0_pop_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(148usize)) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp0_pop_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(152usize)) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp0_pop_full(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(156usize)) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    pub fn interp0_peek_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(160usize)) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    pub fn interp0_peek_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(164usize)) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    pub fn interp0_peek_full(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(168usize)) }
    }
    #[doc = "Control register for lane 0"]
    pub fn interp0_ctrl_lane0(self) -> Reg<regs::Interp0CtrlLane0, RW> {
        unsafe { Reg::new(self.0.add(172usize)) }
    }
    #[doc = "Control register for lane 1"]
    pub fn interp0_ctrl_lane1(self) -> Reg<regs::Interp0CtrlLane1, RW> {
        unsafe { Reg::new(self.0.add(176usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub fn interp0_accum0_add(self) -> Reg<regs::Interp0Accum0Add, RW> {
        unsafe { Reg::new(self.0.add(180usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub fn interp0_accum1_add(self) -> Reg<regs::Interp0Accum1Add, RW> {
        unsafe { Reg::new(self.0.add(184usize)) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub fn interp0_base_1and0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(188usize)) }
    }
    #[doc = "Read/write access to accumulator 0"]
    pub fn interp1_accum0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(192usize)) }
    }
    #[doc = "Read/write access to accumulator 1"]
    pub fn interp1_accum1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(196usize)) }
    }
    #[doc = "Read/write access to BASE0 register."]
    pub fn interp1_base0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(200usize)) }
    }
    #[doc = "Read/write access to BASE1 register."]
    pub fn interp1_base1(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(204usize)) }
    }
    #[doc = "Read/write access to BASE2 register."]
    pub fn interp1_base2(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(208usize)) }
    }
    #[doc = "Read LANE0 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp1_pop_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(212usize)) }
    }
    #[doc = "Read LANE1 result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp1_pop_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(216usize)) }
    }
    #[doc = "Read FULL result, and simultaneously write lane results to both accumulators (POP)."]
    pub fn interp1_pop_full(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(220usize)) }
    }
    #[doc = "Read LANE0 result, without altering any internal state (PEEK)."]
    pub fn interp1_peek_lane0(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(224usize)) }
    }
    #[doc = "Read LANE1 result, without altering any internal state (PEEK)."]
    pub fn interp1_peek_lane1(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(228usize)) }
    }
    #[doc = "Read FULL result, without altering any internal state (PEEK)."]
    pub fn interp1_peek_full(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(232usize)) }
    }
    #[doc = "Control register for lane 0"]
    pub fn interp1_ctrl_lane0(self) -> Reg<regs::Interp1CtrlLane0, RW> {
        unsafe { Reg::new(self.0.add(236usize)) }
    }
    #[doc = "Control register for lane 1"]
    pub fn interp1_ctrl_lane1(self) -> Reg<regs::Interp1CtrlLane1, RW> {
        unsafe { Reg::new(self.0.add(240usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
    pub fn interp1_accum0_add(self) -> Reg<regs::Interp1Accum0Add, RW> {
        unsafe { Reg::new(self.0.add(244usize)) }
    }
    #[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
    pub fn interp1_accum1_add(self) -> Reg<regs::Interp1Accum1Add, RW> {
        unsafe { Reg::new(self.0.add(248usize)) }
    }
    #[doc = "On write, the lower 16 bits go to BASE0, upper bits to BASE1 simultaneously. Each half is sign-extended to 32 bits if that lane's SIGNED flag is set."]
    pub fn interp1_base_1and0(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(252usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock0(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(256usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock1(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(260usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock2(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(264usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock3(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(268usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock4(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(272usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock5(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(276usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock6(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(280usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock7(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(284usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock8(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(288usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock9(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(292usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock10(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(296usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock11(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(300usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock12(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(304usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock13(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(308usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock14(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(312usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock15(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(316usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock16(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(320usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock17(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(324usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock18(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(328usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock19(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(332usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock20(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(336usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock21(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(340usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock22(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(344usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock23(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(348usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock24(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(352usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock25(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(356usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock26(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(360usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock27(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(364usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock28(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(368usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock29(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(372usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock30(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(376usize)) }
    }
    #[doc = "Reading from a spinlock address will: - Return 0 if lock is already locked - Otherwise return nonzero, and simultaneously claim the lock Writing (any value) releases the lock. If core 0 and core 1 attempt to claim the same lock simultaneously, core 0 wins. The value returned on success is 0x1 << lock number."]
    pub fn spinlock31(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(380usize)) }
    }
}
pub mod regs;
