use crate::generic::*;
#[derive(Copy, Clone)]
pub struct StateMachine(pub *mut u8);
unsafe impl Send for StateMachine {}
unsafe impl Sync for StateMachine {}
impl StateMachine {
    #[doc = "Clock divider register for state machine 2 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub fn clkdiv(self) -> Reg<regs::SmClkdiv, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Execution/behavioural settings for state machine 2"]
    pub fn execctrl(self) -> Reg<regs::SmExecctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Control behaviour of the input/output shift registers for state machine 2"]
    pub fn shiftctrl(self) -> Reg<regs::SmShiftctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Current instruction address of state machine 2"]
    pub fn addr(self) -> Reg<regs::SmAddr, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Instruction currently being executed by state machine 2 Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub fn instr(self) -> Reg<regs::SmInstr, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "State machine pin control"]
    pub fn pinctrl(self) -> Reg<regs::SmPinctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Irq(pub *mut u8);
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    #[doc = "Interrupt Enable for irq0"]
    pub fn inte(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Interrupt Force for irq0"]
    pub fn intf(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for irq0"]
    pub fn ints(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
}
#[doc = "Programmable IO block"]
#[derive(Copy, Clone)]
pub struct Pio(pub *mut u8);
unsafe impl Send for Pio {}
unsafe impl Sync for Pio {}
impl Pio {
    #[doc = "PIO control register"]
    pub fn ctrl(self) -> Reg<regs::Ctrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "FIFO status register"]
    pub fn fstat(self) -> Reg<regs::Fstat, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "FIFO debug register"]
    pub fn fdebug(self) -> Reg<regs::Fdebug, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "FIFO levels"]
    pub fn flevel(self) -> Reg<regs::Flevel, RW> {
        unsafe { Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Interrupt request register. Write 1 to clear"]
    pub fn irq(self) -> Reg<regs::Irq, RW> {
        unsafe { Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    pub fn irq_force(self) -> Reg<regs::IrqForce, RW> {
        unsafe { Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
    pub fn input_sync_bypass(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs."]
    pub fn dbg_padout(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
    pub fn dbg_padoe(self) -> Reg<u32, R> {
        unsafe { Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
    pub fn dbg_cfginfo(self) -> Reg<regs::DbgCfginfo, RW> {
        unsafe { Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<regs::Intr, RW> {
        unsafe { Reg::from_ptr(self.0.add(296usize)) }
    }
    #[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub fn txf(self, n: usize) -> Reg<u32, W> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(16usize + n * 4usize)) }
    }
    #[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub fn rxf(self, n: usize) -> Reg<u32, R> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
    }
    #[doc = "Write-only access to instruction memory location 0"]
    pub fn instr_mem(self, n: usize) -> Reg<regs::InstrMem, RW> {
        assert!(n < 32usize);
        unsafe { Reg::from_ptr(self.0.add(72usize + n * 4usize)) }
    }
    pub fn sm(self, n: usize) -> StateMachine {
        assert!(n < 4usize);
        unsafe { StateMachine(self.0.add(200usize + n * 24usize)) }
    }
    pub fn irqs(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq(self.0.add(300usize + n * 12usize)) }
    }
}
pub mod regs;
pub mod vals;
