use crate::generic::*;
#[doc = "Programmable IO block"]
#[derive(Copy, Clone)]
pub struct Pio0(*mut u8);
unsafe impl Send for Pio0 {}
unsafe impl Sync for Pio0 {}
impl Pio0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "PIO control register"]
    pub fn ctrl(self) -> Reg<fields::Ctrl, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Ctrl::from_bits(0)) }
    }
    #[doc = "FIFO status register"]
    pub fn fstat(self) -> Reg<fields::Fstat, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Fstat::from_bits(251662080)) }
    }
    #[doc = "FIFO debug register"]
    pub fn fdebug(self) -> Reg<fields::Fdebug, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Fdebug::from_bits(0)) }
    }
    #[doc = "FIFO levels"]
    pub fn flevel(self) -> Reg<fields::Flevel, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Flevel::from_bits(0)) }
    }
    #[doc = "Interrupt request register. Write 1 to clear"]
    pub fn irq(self) -> Reg<fields::Irq, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Irq::from_bits(0)) }
    }
    #[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
    pub fn irq_force(self) -> Reg<fields::IrqForce, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::IrqForce::from_bits(0)) }
    }
    #[doc = "There is a 2-flipflop synchronizer on each GPIO input, which protects PIO logic from metastabilities. This increases input delay, and for fast synchronous IO (e.g. SPI) these synchronizers may need to be bypassed. Each bit in this register corresponds to one GPIO. 0 -> input is synchronized (default) 1 -> synchronizer is bypassed If in doubt, leave this register as all zeroes."]
    pub fn input_sync_bypass(self) -> Reg<u32, RW> {
        unsafe { Reg::new(self.0.add(56usize), 0) }
    }
    #[doc = "Read to sample the pad output values PIO is currently driving to the GPIOs."]
    pub fn dbg_padout(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(60usize), 0) }
    }
    #[doc = "Read to sample the pad output enables (direction) PIO is currently driving to the GPIOs."]
    pub fn dbg_padoe(self) -> Reg<u32, R> {
        unsafe { Reg::new(self.0.add(64usize), 0) }
    }
    #[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
    pub fn dbg_cfginfo(self) -> Reg<fields::DbgCfginfo, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::DbgCfginfo::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(296usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Write-only access to instruction memory location 0"]
    pub fn instr_mem(self, n: usize) -> Reg<fields::InstrMem, RW> {
        assert!(n < 32usize);
        unsafe {
            Reg::new(
                self.0.add(72usize + n * 4usize),
                fields::InstrMem::from_bits(0),
            )
        }
    }
    #[doc = "Direct write access to the TX FIFO for this state machine. Each write pushes one word to the FIFO."]
    pub fn txf(self, n: usize) -> Reg<u32, W> {
        assert!(n < 4usize);
        unsafe { Reg::new(self.0.add(16usize + n * 4usize), 0) }
    }
    #[doc = "Direct read access to the RX FIFO for this state machine. Each read pops one word from the FIFO."]
    pub fn rxf(self, n: usize) -> Reg<u32, R> {
        assert!(n < 4usize);
        unsafe { Reg::new(self.0.add(32usize + n * 4usize), 0) }
    }
    pub fn sm(self, n: usize) -> StateMachine {
        assert!(n < 4usize);
        unsafe { StateMachine::from_ptr(self.0.add(200usize + n * 24usize)) }
    }
    pub fn irqs(self, n: usize) -> Irq {
        assert!(n < 2usize);
        unsafe { Irq::from_ptr(self.0.add(300usize + n * 12usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct StateMachine(*mut u8);
unsafe impl Send for StateMachine {}
unsafe impl Sync for StateMachine {}
impl StateMachine {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Clock divider register for state machine 0 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
    pub fn clkdiv(self) -> Reg<fields::SmClkdiv, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::SmClkdiv::from_bits(65536)) }
    }
    #[doc = "Execution/behavioural settings for state machine 0"]
    pub fn execctrl(self) -> Reg<fields::SmExecctrl, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::SmExecctrl::from_bits(126976)) }
    }
    #[doc = "Control behaviour of the input/output shift registers for state machine 0"]
    pub fn shiftctrl(self) -> Reg<fields::SmShiftctrl, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::SmShiftctrl::from_bits(786432)) }
    }
    #[doc = "Current instruction address of state machine 0"]
    pub fn addr(self) -> Reg<fields::SmAddr, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::SmAddr::from_bits(0)) }
    }
    #[doc = "Instruction currently being executed by state machine 0 Write to execute an instruction immediately (including jumps) and then resume execution."]
    pub fn instr(self) -> Reg<fields::SmInstr, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::SmInstr::from_bits(0)) }
    }
    #[doc = "State machine pin control"]
    pub fn pinctrl(self) -> Reg<fields::SmPinctrl, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::SmPinctrl::from_bits(335544320)) }
    }
}
#[derive(Copy, Clone)]
pub struct Irq(*mut u8);
unsafe impl Send for Irq {}
unsafe impl Sync for Irq {}
impl Irq {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Interrupt Enable for irq1"]
    pub fn inte(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt Force for irq1"]
    pub fn intf(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Intr::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for irq1"]
    pub fn ints(self) -> Reg<fields::Intr, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Intr::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
