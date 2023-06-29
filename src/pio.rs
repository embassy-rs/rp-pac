#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq {
    ptr: *mut u8,
}
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Enable for irq0"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Interrupt Force for irq0"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for irq0"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
#[doc = "Programmable IO block"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pio {
    ptr: *mut u8,
}
unsafe impl Send for Pio {}
unsafe impl Sync for Pio {}
impl Pio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "PIO control register"]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::Ctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "FIFO status register"]
    #[inline(always)]
    pub const fn fstat(self) -> crate::common::Reg<regs::Fstat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "FIFO debug register"]
    #[inline(always)]
    pub const fn fdebug(self) -> crate::common::Reg<regs::Fdebug, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "FIFO levels"]
    #[inline(always)]
    pub const fn flevel(self) -> crate::common::Reg<regs::Flevel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO. Attempting to write to a full FIFO has no effect on the FIFO state or contents, and sets the sticky FDEBUG_TXOVER error flag for this FIFO."]
    #[inline(always)]
    pub const fn txf(self, n: usize) -> crate::common::Reg<u32, crate::common::W> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize + n * 4usize) as _) }
    }
    #[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO. Attempting to read from an empty FIFO has no effect on the FIFO state, and sets the sticky FDEBUG_RXUNDER error flag for this FIFO. The data returned to the system on a read from an empty FIFO is undefined."]
    #[inline(always)]
    pub const fn rxf(self, n: usize) -> crate::common::Reg<u32, crate::common::R> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 4usize) as _) }
    }
    #[doc = "State machine IRQ flags register. Write 1 to clear. There are 8 state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the 8 flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. The lower four of these flags are also routed out to system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
    #[inline(always)]
    pub const fn irq(self) -> crate::common::Reg<regs::Irq, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    #[inline(always)]
    pub const fn irq_force(self) -> crate::common::Reg<regs::IrqForce, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
    #[inline(always)]
    pub const fn input_sync_bypass(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub const fn dbg_padout(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs. On RP2040 there are 30 GPIOs, so the two most significant bits are hardwired to 0."]
    #[inline(always)]
    pub const fn dbg_padoe(self) -> crate::common::Reg<u32, crate::common::R> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
    #[inline(always)]
    pub const fn dbg_cfginfo(self) -> crate::common::Reg<regs::DbgCfginfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Write-only access to instruction memory location 0"]
    #[inline(always)]
    pub const fn instr_mem(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::InstrMem, crate::common::RW> {
        assert!(n < 32usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn sm(self, n: usize) -> StateMachine {
        assert!(n < 4usize);
        unsafe { StateMachine::from_ptr(self.ptr.add(200usize + n * 24usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[inline(always)]
    pub const fn irqs(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq::from_ptr(self.ptr.add(300usize + n * 12usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct StateMachine {
    ptr: *mut u8,
}
unsafe impl Send for StateMachine {}
unsafe impl Sync for StateMachine {}
impl StateMachine {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Clock divisor register for state machine 0 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    #[inline(always)]
    pub const fn clkdiv(self) -> crate::common::Reg<regs::SmClkdiv, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Execution/behavioural settings for state machine 0"]
    #[inline(always)]
    pub const fn execctrl(self) -> crate::common::Reg<regs::SmExecctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Control behaviour of the input/output shift registers for state machine 0"]
    #[inline(always)]
    pub const fn shiftctrl(self) -> crate::common::Reg<regs::SmShiftctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Current instruction address of state machine 0"]
    #[inline(always)]
    pub const fn addr(self) -> crate::common::Reg<regs::SmAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Read to see the instruction currently addressed by state machine 0's program counter Write to execute an instruction immediately (including jumps) and then resume execution."]
    #[inline(always)]
    pub const fn instr(self) -> crate::common::Reg<regs::SmInstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "State machine pin control"]
    #[inline(always)]
    pub const fn pinctrl(self) -> crate::common::Reg<regs::SmPinctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
}
pub mod regs;
pub mod vals;
