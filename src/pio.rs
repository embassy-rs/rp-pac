#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub *mut u8);
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    #[doc = "Interrupt Enable for irq1"]
    #[inline(always)]
    pub fn inte(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Interrupt Force for irq1"]
    #[inline(always)]
    pub fn intf(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for irq1"]
    #[inline(always)]
    pub fn ints(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Programmable IO block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio(pub *mut u8);
unsafe impl Send for Pio {}
unsafe impl Sync for Pio {}
impl Pio {
    #[doc = "PIO control register"]
    #[inline(always)]
    pub fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "FIFO status register"]
    #[inline(always)]
    pub fn fstat(self) -> crate::common::Reg<regs::Fstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "FIFO debug register"]
    #[inline(always)]
    pub fn fdebug(self) -> crate::common::Reg<regs::Fdebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "FIFO levels"]
    #[inline(always)]
    pub fn flevel(self) -> crate::common::Reg<regs::Flevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    #[inline(always)]
    pub fn txf(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize + n * 4usize)) }
    }
    #[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    #[inline(always)]
    pub fn rxf(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
    }
    #[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
    #[inline(always)]
    pub fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    #[inline(always)]
    pub fn irq_force(self) -> crate::common::Reg<regs::IrqForce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
    #[inline(always)]
    pub fn input_sync_bypass(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub fn dbg_padout(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub fn dbg_padoe(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
    #[inline(always)]
    pub fn dbg_cfginfo(self) -> crate::common::Reg<regs::DbgCfginfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Write-only access to instruction memory location 0"]
    #[inline(always)]
    pub fn instr_mem(self, n: usize) -> crate::common::Reg<regs::InstrMem, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize + n * 4usize)) }
    }
    #[inline(always)]
    pub fn sm(self, n: usize) -> StateMachine {
        assert!(n < 4usize);
        unsafe { StateMachine(self.0.add(200usize + n * 24usize)) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(296usize)) }
    }
    #[inline(always)]
    pub fn irqs(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq(self.0.add(300usize + n * 12usize)) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StateMachine(pub *mut u8);
unsafe impl Send for StateMachine {}
unsafe impl Sync for StateMachine {}
impl StateMachine {
    #[doc = "Clock divisor register for state machine 2 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    #[inline(always)]
    pub fn clkdiv(self) -> crate::common::Reg<regs::SmClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Execution/behavioural settings for state machine 2"]
    #[inline(always)]
    pub fn execctrl(self) -> crate::common::Reg<regs::SmExecctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Control behaviour of the input/output shift registers for state machine 2"]
    #[inline(always)]
    pub fn shiftctrl(self) -> crate::common::Reg<regs::SmShiftctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Current instruction address of state machine 2"]
    #[inline(always)]
    pub fn addr(self) -> crate::common::Reg<regs::SmAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Read to see the instruction currently addressed by state machine 2's program counter Write to execute an instruction immediately (including jumps) and then resume execution."]
    #[inline(always)]
    pub fn instr(self) -> crate::common::Reg<regs::SmInstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "State machine pin control"]
    #[inline(always)]
    pub fn pinctrl(self) -> crate::common::Reg<regs::SmPinctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
}
pub mod regs;
pub mod vals;
