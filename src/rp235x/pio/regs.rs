#[doc = "PIO control register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ctrl(pub u32);
impl Ctrl {
    #[doc = "Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    pub const fn sm_enable(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Enable/disable each of the four state machines by writing 1/0 to each of these four bits. When disabled, a state machine will cease executing instructions, except those written directly to SMx_INSTR by the system. Multiple bits can be set/cleared at once to run/halt multiple state machines simultaneously."]
    #[inline(always)]
    pub fn set_sm_enable(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution. Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY. The contents of the output shift register and the X/Y scratch registers are not affected."]
    #[inline(always)]
    pub const fn sm_restart(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Write 1 to instantly clear internal SM state which may be otherwise difficult to access and will affect future execution. Specifically, the following are cleared: input and output shift counters; the contents of the input shift register; the delay counter; the waiting-on-IRQ state; any stalled instruction written to SMx_INSTR or run by OUT/MOV EXEC; any pin write left asserted due to OUT_STICKY. The contents of the output shift register and the X/Y scratch registers are not affected."]
    #[inline(always)]
    pub fn set_sm_restart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep. Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync. Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
    #[inline(always)]
    pub const fn clkdiv_restart(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Restart a state machine's clock divider from an initial phase of 0. Clock dividers are free-running, so once started, their output (including fractional jitter) is completely determined by the integer/fractional divisor configured in SMx_CLKDIV. This means that, if multiple clock dividers with the same divisor are restarted simultaneously, by writing multiple 1 bits to this field, the execution clocks of those state machines will run in precise lockstep. Note that setting/clearing SM_ENABLE does not stop the clock divider from running, so once multiple state machines' clocks are synchronised, it is safe to disable/reenable a state machine, whilst keeping the clock dividers in sync. Note also that CLKDIV_RESTART can be written to whilst the state machine is running, and this is useful to resynchronise clock dividers after the divisors (SMx_CLKDIV) have been changed on-the-fly."]
    #[inline(always)]
    pub fn set_clkdiv_restart(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    pub const fn prev_pio_mask(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "A mask of state machines in the neighbouring lower-numbered PIO block in the system (or the highest-numbered PIO block if this is PIO block 0) to which to apply the operations specified by OP_CLKDIV_RESTART, OP_ENABLE, OP_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    pub fn set_prev_pio_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together). Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    pub const fn next_pio_mask(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[doc = "A mask of state machines in the neighbouring higher-numbered PIO block in the system (or PIO block 0 if this is the highest-numbered PIO block) to which to apply the operations specified by NEXTPREV_CLKDIV_RESTART, NEXTPREV_SM_ENABLE, and NEXTPREV_SM_DISABLE in the same write. This allows state machines in a neighbouring PIO block to be started/stopped/clock-synced exactly simultaneously with a write to this PIO block's CTRL register. Note that in a system with two PIOs, NEXT_PIO_MASK and PREV_PIO_MASK actually indicate the same PIO block. In this case the effects are applied cumulatively (as though the masks were OR'd together). Neighbouring PIO blocks are disconnected (status signals tied to 0 and control signals ignored) if one block is accessible to NonSecure code, and one is not."]
    #[inline(always)]
    pub fn set_next_pio_mask(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[doc = "Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers. If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence."]
    #[inline(always)]
    pub const fn nextprev_sm_enable(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to enable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to setting the corresponding SM_ENABLE bits in those PIOs' CTRL registers. If both OTHERS_SM_ENABLE and OTHERS_SM_DISABLE are set, the disable takes precedence."]
    #[inline(always)]
    pub fn set_nextprev_sm_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers."]
    #[inline(always)]
    pub const fn nextprev_sm_disable(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to disable state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to clearing the corresponding SM_ENABLE bits in those PIOs' CTRL registers."]
    #[inline(always)]
    pub fn set_nextprev_sm_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers."]
    #[inline(always)]
    pub const fn nextprev_clkdiv_restart(&self) -> bool {
        let val = (self.0 >> 26usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to restart the clock dividers of state machines in neighbouring PIO blocks, as specified by NEXT_PIO_MASK and PREV_PIO_MASK in the same write. This is equivalent to writing 1 to the corresponding CLKDIV_RESTART bits in those PIOs' CTRL registers."]
    #[inline(always)]
    pub fn set_nextprev_clkdiv_restart(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26usize)) | (((val as u32) & 0x01) << 26usize);
    }
}
impl Default for Ctrl {
    #[inline(always)]
    fn default() -> Ctrl {
        Ctrl(0)
    }
}
impl core::fmt::Debug for Ctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ctrl")
            .field("sm_enable", &self.sm_enable())
            .field("sm_restart", &self.sm_restart())
            .field("clkdiv_restart", &self.clkdiv_restart())
            .field("prev_pio_mask", &self.prev_pio_mask())
            .field("next_pio_mask", &self.next_pio_mask())
            .field("nextprev_sm_enable", &self.nextprev_sm_enable())
            .field("nextprev_sm_disable", &self.nextprev_sm_disable())
            .field("nextprev_clkdiv_restart", &self.nextprev_clkdiv_restart())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ctrl {
            sm_enable: u8,
            sm_restart: u8,
            clkdiv_restart: u8,
            prev_pio_mask: u8,
            next_pio_mask: u8,
            nextprev_sm_enable: bool,
            nextprev_sm_disable: bool,
            nextprev_clkdiv_restart: bool,
        }
        let proxy = Ctrl {
            sm_enable: self.sm_enable(),
            sm_restart: self.sm_restart(),
            clkdiv_restart: self.clkdiv_restart(),
            prev_pio_mask: self.prev_pio_mask(),
            next_pio_mask: self.next_pio_mask(),
            nextprev_sm_enable: self.nextprev_sm_enable(),
            nextprev_sm_disable: self.nextprev_sm_disable(),
            nextprev_clkdiv_restart: self.nextprev_clkdiv_restart(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "The PIO hardware has some free parameters that may vary between chip products. These should be provided in the chip datasheet, but are also exposed here."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DbgCfginfo(pub u32);
impl DbgCfginfo {
    #[doc = "The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
    #[inline(always)]
    pub const fn fifo_depth(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "The depth of the state machine TX/RX FIFOs, measured in words. Joining fifos via SHIFTCTRL_FJOIN gives one FIFO with double this depth."]
    #[inline(always)]
    pub fn set_fifo_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "The number of state machines this PIO instance is equipped with."]
    #[inline(always)]
    pub const fn sm_count(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "The number of state machines this PIO instance is equipped with."]
    #[inline(always)]
    pub fn set_sm_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "The size of the instruction memory, measured in units of one instruction"]
    #[inline(always)]
    pub const fn imem_size(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x3f;
        val as u8
    }
    #[doc = "The size of the instruction memory, measured in units of one instruction"]
    #[inline(always)]
    pub fn set_imem_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 16usize)) | (((val as u32) & 0x3f) << 16usize);
    }
    #[doc = "Version of the core PIO hardware."]
    #[inline(always)]
    pub const fn version(&self) -> super::vals::Version {
        let val = (self.0 >> 28usize) & 0x0f;
        super::vals::Version::from_bits(val as u8)
    }
    #[doc = "Version of the core PIO hardware."]
    #[inline(always)]
    pub fn set_version(&mut self, val: super::vals::Version) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val.to_bits() as u32) & 0x0f) << 28usize);
    }
}
impl Default for DbgCfginfo {
    #[inline(always)]
    fn default() -> DbgCfginfo {
        DbgCfginfo(0)
    }
}
impl core::fmt::Debug for DbgCfginfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DbgCfginfo")
            .field("fifo_depth", &self.fifo_depth())
            .field("sm_count", &self.sm_count())
            .field("imem_size", &self.imem_size())
            .field("version", &self.version())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for DbgCfginfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct DbgCfginfo {
            fifo_depth: u8,
            sm_count: u8,
            imem_size: u8,
            version: super::vals::Version,
        }
        let proxy = DbgCfginfo {
            fifo_depth: self.fifo_depth(),
            sm_count: self.sm_count(),
            imem_size: self.imem_size(),
            version: self.version(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO debug register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fdebug(pub u32);
impl Fdebug {
    #[doc = "State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    pub const fn rxstall(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine has stalled on full RX FIFO during a blocking PUSH, or an IN with autopush enabled. This flag is also set when a nonblocking PUSH to a full FIFO took place, in which case the state machine has dropped data. Write 1 to clear."]
    #[inline(always)]
    pub fn set_rxstall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    pub const fn rxunder(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "RX FIFO underflow (i.e. read-on-empty by the system) has occurred. Write 1 to clear. Note that read-on-empty does not perturb the state of the FIFO in any way, but the data returned by reading from an empty FIFO is undefined, so this flag generally only becomes set due to some kind of software error."]
    #[inline(always)]
    pub fn set_rxunder(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    pub const fn txover(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "TX FIFO overflow (i.e. write-on-full by the system) has occurred. Write 1 to clear. Note that write-on-full does not alter the state or contents of the FIFO in any way, but the data that the system attempted to write is dropped, so if this flag is set, your software has quite likely dropped some data on the floor."]
    #[inline(always)]
    pub fn set_txover(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    pub const fn txstall(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine has stalled on empty TX FIFO during a blocking PULL, or an OUT with autopull enabled. Write 1 to clear."]
    #[inline(always)]
    pub fn set_txstall(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fdebug {
    #[inline(always)]
    fn default() -> Fdebug {
        Fdebug(0)
    }
}
impl core::fmt::Debug for Fdebug {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fdebug")
            .field("rxstall", &self.rxstall())
            .field("rxunder", &self.rxunder())
            .field("txover", &self.txover())
            .field("txstall", &self.txstall())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fdebug {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fdebug {
            rxstall: u8,
            rxunder: u8,
            txover: u8,
            txstall: u8,
        }
        let proxy = Fdebug {
            rxstall: self.rxstall(),
            rxunder: self.rxunder(),
            txover: self.txover(),
            txstall: self.txstall(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO levels"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Flevel(pub u32);
impl Flevel {
    #[inline(always)]
    pub const fn tx0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_tx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn rx0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rx0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[inline(always)]
    pub const fn tx1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_tx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[inline(always)]
    pub const fn rx1(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rx1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val as u32) & 0x0f) << 12usize);
    }
    #[inline(always)]
    pub const fn tx2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_tx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[inline(always)]
    pub const fn rx2(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rx2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20usize)) | (((val as u32) & 0x0f) << 20usize);
    }
    #[inline(always)]
    pub const fn tx3(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_tx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[inline(always)]
    pub const fn rx3(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_rx3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Flevel {
    #[inline(always)]
    fn default() -> Flevel {
        Flevel(0)
    }
}
impl core::fmt::Debug for Flevel {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Flevel")
            .field("tx0", &self.tx0())
            .field("rx0", &self.rx0())
            .field("tx1", &self.tx1())
            .field("rx1", &self.rx1())
            .field("tx2", &self.tx2())
            .field("rx2", &self.rx2())
            .field("tx3", &self.tx3())
            .field("rx3", &self.rx3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Flevel {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Flevel {
            tx0: u8,
            rx0: u8,
            tx1: u8,
            rx1: u8,
            tx2: u8,
            rx2: u8,
            tx3: u8,
            rx3: u8,
        }
        let proxy = Flevel {
            tx0: self.tx0(),
            rx0: self.rx0(),
            tx1: self.tx1(),
            rx1: self.rx1(),
            tx2: self.tx2(),
            rx2: self.rx2(),
            tx3: self.tx3(),
            rx3: self.rx3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "FIFO status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Fstat(pub u32);
impl Fstat {
    #[doc = "State machine RX FIFO is full"]
    #[inline(always)]
    pub const fn rxfull(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine RX FIFO is full"]
    #[inline(always)]
    pub fn set_rxfull(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "State machine RX FIFO is empty"]
    #[inline(always)]
    pub const fn rxempty(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine RX FIFO is empty"]
    #[inline(always)]
    pub fn set_rxempty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "State machine TX FIFO is full"]
    #[inline(always)]
    pub const fn txfull(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine TX FIFO is full"]
    #[inline(always)]
    pub fn set_txfull(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16usize)) | (((val as u32) & 0x0f) << 16usize);
    }
    #[doc = "State machine TX FIFO is empty"]
    #[inline(always)]
    pub const fn txempty(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "State machine TX FIFO is empty"]
    #[inline(always)]
    pub fn set_txempty(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
}
impl Default for Fstat {
    #[inline(always)]
    fn default() -> Fstat {
        Fstat(0)
    }
}
impl core::fmt::Debug for Fstat {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Fstat")
            .field("rxfull", &self.rxfull())
            .field("rxempty", &self.rxempty())
            .field("txfull", &self.txfull())
            .field("txempty", &self.txempty())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Fstat {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Fstat {
            rxfull: u8,
            rxempty: u8,
            txfull: u8,
            txempty: u8,
        }
        let proxy = Fstat {
            rxfull: self.rxfull(),
            rxempty: self.rxempty(),
            txfull: self.txfull(),
            txempty: self.txempty(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Relocate GPIO 0 (from PIO's point of view) in the system GPIO numbering, to access more than 32 GPIOs from PIO. Only the values 0 and 16 are supported (only bit 4 is writable)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpiobase(pub u32);
impl Gpiobase {
    #[inline(always)]
    pub const fn gpiobase(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_gpiobase(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for Gpiobase {
    #[inline(always)]
    fn default() -> Gpiobase {
        Gpiobase(0)
    }
}
impl core::fmt::Debug for Gpiobase {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Gpiobase")
            .field("gpiobase", &self.gpiobase())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Gpiobase {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Gpiobase {
            gpiobase: bool,
        }
        let proxy = Gpiobase {
            gpiobase: self.gpiobase(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Write-only access to instruction memory location 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InstrMem(pub u32);
impl InstrMem {
    #[inline(always)]
    pub const fn instr_mem(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_instr_mem(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for InstrMem {
    #[inline(always)]
    fn default() -> InstrMem {
        InstrMem(0)
    }
}
impl core::fmt::Debug for InstrMem {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("InstrMem")
            .field("instr_mem", &self.instr_mem())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for InstrMem {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct InstrMem {
            instr_mem: u16,
        }
        let proxy = InstrMem {
            instr_mem: self.instr_mem(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[inline(always)]
    pub const fn sm0_rxnempty(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm0_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn sm1_rxnempty(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm1_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn sm2_rxnempty(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm2_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn sm3_rxnempty(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm3_rxnempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn sm0_txnfull(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm0_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn sm1_txnfull(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm1_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn sm2_txnfull(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm2_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn sm3_txnfull(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm3_txnfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[inline(always)]
    pub const fn sm0(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[inline(always)]
    pub const fn sm1(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[inline(always)]
    pub const fn sm2(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[inline(always)]
    pub const fn sm3(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn sm4(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[inline(always)]
    pub const fn sm5(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[inline(always)]
    pub const fn sm6(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn sm7(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_sm7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("sm0_rxnempty", &self.sm0_rxnempty())
            .field("sm1_rxnempty", &self.sm1_rxnempty())
            .field("sm2_rxnempty", &self.sm2_rxnempty())
            .field("sm3_rxnempty", &self.sm3_rxnempty())
            .field("sm0_txnfull", &self.sm0_txnfull())
            .field("sm1_txnfull", &self.sm1_txnfull())
            .field("sm2_txnfull", &self.sm2_txnfull())
            .field("sm3_txnfull", &self.sm3_txnfull())
            .field("sm0", &self.sm0())
            .field("sm1", &self.sm1())
            .field("sm2", &self.sm2())
            .field("sm3", &self.sm3())
            .field("sm4", &self.sm4())
            .field("sm5", &self.sm5())
            .field("sm6", &self.sm6())
            .field("sm7", &self.sm7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intr {
            sm0_rxnempty: bool,
            sm1_rxnempty: bool,
            sm2_rxnempty: bool,
            sm3_rxnempty: bool,
            sm0_txnfull: bool,
            sm1_txnfull: bool,
            sm2_txnfull: bool,
            sm3_txnfull: bool,
            sm0: bool,
            sm1: bool,
            sm2: bool,
            sm3: bool,
            sm4: bool,
            sm5: bool,
            sm6: bool,
            sm7: bool,
        }
        let proxy = Intr {
            sm0_rxnempty: self.sm0_rxnempty(),
            sm1_rxnempty: self.sm1_rxnempty(),
            sm2_rxnempty: self.sm2_rxnempty(),
            sm3_rxnempty: self.sm3_rxnempty(),
            sm0_txnfull: self.sm0_txnfull(),
            sm1_txnfull: self.sm1_txnfull(),
            sm2_txnfull: self.sm2_txnfull(),
            sm3_txnfull: self.sm3_txnfull(),
            sm0: self.sm0(),
            sm1: self.sm1(),
            sm2: self.sm2(),
            sm3: self.sm3(),
            sm4: self.sm4(),
            sm5: self.sm5(),
            sm6: self.sm6(),
            sm7: self.sm7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "State machine IRQ flags register. Write 1 to clear. There are eight state machine IRQ flags, which can be set, cleared, and waited on by the state machines. There's no fixed association between flags and state machines -- any state machine can use any flag. Any of the eight flags can be used for timing synchronisation between state machines, using IRQ and WAIT instructions. Any combination of the eight flags can also routed out to either of the two system-level interrupt requests, alongside FIFO status interrupts -- see e.g. IRQ0_INTE."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Irq(pub u32);
impl Irq {
    #[inline(always)]
    pub const fn irq(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_irq(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Irq {
    #[inline(always)]
    fn default() -> Irq {
        Irq(0)
    }
}
impl core::fmt::Debug for Irq {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Irq").field("irq", &self.irq()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Irq {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Irq {
            irq: u8,
        }
        let proxy = Irq { irq: self.irq() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Writing a 1 to each of these bits will forcibly assert the corresponding IRQ. Note this is different to the INTF register: writing here affects PIO internal state. INTF just asserts the processor-facing IRQ signal for testing ISRs, and is not visible to the state machines."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct IrqForce(pub u32);
impl IrqForce {
    #[inline(always)]
    pub const fn irq_force(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_irq_force(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for IrqForce {
    #[inline(always)]
    fn default() -> IrqForce {
        IrqForce(0)
    }
}
impl core::fmt::Debug for IrqForce {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IrqForce")
            .field("irq_force", &self.irq_force())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for IrqForce {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct IrqForce {
            irq_force: u8,
        }
        let proxy = IrqForce {
            irq_force: self.irq_force(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Current instruction address of state machine 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmAddr(pub u32);
impl SmAddr {
    #[inline(always)]
    pub const fn addr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[inline(always)]
    pub fn set_addr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
}
impl Default for SmAddr {
    #[inline(always)]
    fn default() -> SmAddr {
        SmAddr(0)
    }
}
impl core::fmt::Debug for SmAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmAddr")
            .field("addr", &self.addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmAddr {
            addr: u8,
        }
        let proxy = SmAddr { addr: self.addr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Clock divisor register for state machine 0 Frequency = clock freq / (CLKDIV_INT + CLKDIV_FRAC / 256)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmClkdiv(pub u32);
impl SmClkdiv {
    #[doc = "Fractional part of clock divisor"]
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Fractional part of clock divisor"]
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Effective frequency is sysclk/(int + frac/256). Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[doc = "Effective frequency is sysclk/(int + frac/256). Value of 0 is interpreted as 65536. If INT is 0, FRAC must also be 0."]
    #[inline(always)]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SmClkdiv {
    #[inline(always)]
    fn default() -> SmClkdiv {
        SmClkdiv(0)
    }
}
impl core::fmt::Debug for SmClkdiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmClkdiv")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmClkdiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmClkdiv {
            frac: u8,
            int: u16,
        }
        let proxy = SmClkdiv {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Execution/behavioural settings for state machine 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmExecctrl(pub u32);
impl SmExecctrl {
    #[doc = "Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
    #[inline(always)]
    pub const fn status_n(&self) -> super::vals::ExecctrlStatusN {
        let val = (self.0 >> 0usize) & 0x1f;
        super::vals::ExecctrlStatusN::from_bits(val as u8)
    }
    #[doc = "Comparison level or IRQ index for the MOV x, STATUS instruction. If STATUS_SEL is TXLEVEL or RXLEVEL, then values of STATUS_N greater than the current FIFO depth are reserved, and have undefined behaviour."]
    #[inline(always)]
    pub fn set_status_n(&mut self, val: super::vals::ExecctrlStatusN) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val.to_bits() as u32) & 0x1f) << 0usize);
    }
    #[doc = "Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    pub const fn status_sel(&self) -> super::vals::ExecctrlStatusSel {
        let val = (self.0 >> 5usize) & 0x03;
        super::vals::ExecctrlStatusSel::from_bits(val as u8)
    }
    #[doc = "Comparison used for the MOV x, STATUS instruction."]
    #[inline(always)]
    pub fn set_status_sel(&mut self, val: super::vals::ExecctrlStatusSel) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val.to_bits() as u32) & 0x03) << 5usize);
    }
    #[doc = "After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    pub const fn wrap_bottom(&self) -> u8 {
        let val = (self.0 >> 7usize) & 0x1f;
        val as u8
    }
    #[doc = "After reaching wrap_top, execution is wrapped to this address."]
    #[inline(always)]
    pub fn set_wrap_bottom(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 7usize)) | (((val as u32) & 0x1f) << 7usize);
    }
    #[doc = "After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    pub const fn wrap_top(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "After reaching this address, execution is wrapped to wrap_bottom. If the instruction is a jump, and the jump condition is true, the jump takes priority."]
    #[inline(always)]
    pub fn set_wrap_top(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub const fn out_sticky(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Continuously assert the most recent OUT/SET to the pins"]
    #[inline(always)]
    pub fn set_out_sticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub const fn inline_out_en(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, use a bit of OUT data as an auxiliary write enable When used in conjunction with OUT_STICKY, writes with an enable of 0 will deassert the latest pin write. This can create useful masking/override behaviour due to the priority ordering of state machine pin writes (SM0 < SM1 < ...)"]
    #[inline(always)]
    pub fn set_inline_out_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Which data bit to use for inline OUT enable"]
    #[inline(always)]
    pub const fn out_en_sel(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x1f;
        val as u8
    }
    #[doc = "Which data bit to use for inline OUT enable"]
    #[inline(always)]
    pub fn set_out_en_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 19usize)) | (((val as u32) & 0x1f) << 19usize);
    }
    #[doc = "The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    pub const fn jmp_pin(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "The GPIO number to use as condition for JMP PIN. Unaffected by input mapping."]
    #[inline(always)]
    pub fn set_jmp_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
    #[doc = "If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    pub const fn side_pindir(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, side-set data is asserted to pin directions, instead of pin values"]
    #[inline(always)]
    pub fn set_side_pindir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    pub const fn side_en(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the MSB of the Delay/Side-set instruction field is used as side-set enable, rather than a side-set data bit. This allows instructions to perform side-set optionally, rather than on every instruction, but the maximum possible side-set width is reduced from 5 to 4. Note that the value of PINCTRL_SIDESET_COUNT is inclusive of this enable bit."]
    #[inline(always)]
    pub fn set_side_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
    #[inline(always)]
    pub const fn exec_stalled(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, an instruction written to SMx_INSTR is stalled, and latched by the state machine. Will clear to 0 once this instruction completes."]
    #[inline(always)]
    pub fn set_exec_stalled(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SmExecctrl {
    #[inline(always)]
    fn default() -> SmExecctrl {
        SmExecctrl(0)
    }
}
impl core::fmt::Debug for SmExecctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmExecctrl")
            .field("status_n", &self.status_n())
            .field("status_sel", &self.status_sel())
            .field("wrap_bottom", &self.wrap_bottom())
            .field("wrap_top", &self.wrap_top())
            .field("out_sticky", &self.out_sticky())
            .field("inline_out_en", &self.inline_out_en())
            .field("out_en_sel", &self.out_en_sel())
            .field("jmp_pin", &self.jmp_pin())
            .field("side_pindir", &self.side_pindir())
            .field("side_en", &self.side_en())
            .field("exec_stalled", &self.exec_stalled())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmExecctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmExecctrl {
            status_n: super::vals::ExecctrlStatusN,
            status_sel: super::vals::ExecctrlStatusSel,
            wrap_bottom: u8,
            wrap_top: u8,
            out_sticky: bool,
            inline_out_en: bool,
            out_en_sel: u8,
            jmp_pin: u8,
            side_pindir: bool,
            side_en: bool,
            exec_stalled: bool,
        }
        let proxy = SmExecctrl {
            status_n: self.status_n(),
            status_sel: self.status_sel(),
            wrap_bottom: self.wrap_bottom(),
            wrap_top: self.wrap_top(),
            out_sticky: self.out_sticky(),
            inline_out_en: self.inline_out_en(),
            out_en_sel: self.out_en_sel(),
            jmp_pin: self.jmp_pin(),
            side_pindir: self.side_pindir(),
            side_en: self.side_en(),
            exec_stalled: self.exec_stalled(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Read to see the instruction currently addressed by state machine 0's program counter Write to execute an instruction immediately (including jumps) and then resume execution."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmInstr(pub u32);
impl SmInstr {
    #[inline(always)]
    pub const fn instr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_instr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for SmInstr {
    #[inline(always)]
    fn default() -> SmInstr {
        SmInstr(0)
    }
}
impl core::fmt::Debug for SmInstr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmInstr")
            .field("instr", &self.instr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmInstr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmInstr {
            instr: u16,
        }
        let proxy = SmInstr {
            instr: self.instr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "State machine pin control"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmPinctrl(pub u32);
impl SmPinctrl {
    #[doc = "The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    pub const fn out_base(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "The lowest-numbered pin that will be affected by an OUT PINS, OUT PINDIRS or MOV PINS instruction. The data written to this pin will always be the least-significant bit of the OUT or MOV data."]
    #[inline(always)]
    pub fn set_out_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    pub const fn set_base(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The lowest-numbered pin that will be affected by a SET PINS or SET PINDIRS instruction. The data written to this pin is the least-significant bit of the SET data."]
    #[inline(always)]
    pub fn set_set_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    pub const fn sideset_base(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The lowest-numbered pin that will be affected by a side-set operation. The MSBs of an instruction's side-set/delay field (up to 5, determined by SIDESET_COUNT) are used for side-set data, with the remaining LSBs used for delay. The least-significant bit of the side-set portion is the bit written to this pin, with more-significant bits written to higher-numbered pins."]
    #[inline(always)]
    pub fn set_sideset_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    pub const fn in_base(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x1f;
        val as u8
    }
    #[doc = "The pin which is mapped to the least-significant bit of a state machine's IN data bus. Higher-numbered pins are mapped to consecutively more-significant data bits, with a modulo of 32 applied to pin number."]
    #[inline(always)]
    pub fn set_in_base(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 15usize)) | (((val as u32) & 0x1f) << 15usize);
    }
    #[doc = "The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    pub const fn out_count(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x3f;
        val as u8
    }
    #[doc = "The number of pins asserted by an OUT PINS, OUT PINDIRS or MOV PINS instruction. In the range 0 to 32 inclusive."]
    #[inline(always)]
    pub fn set_out_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 20usize)) | (((val as u32) & 0x3f) << 20usize);
    }
    #[doc = "The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    pub const fn set_count(&self) -> u8 {
        let val = (self.0 >> 26usize) & 0x07;
        val as u8
    }
    #[doc = "The number of pins asserted by a SET. In the range 0 to 5 inclusive."]
    #[inline(always)]
    pub fn set_set_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 26usize)) | (((val as u32) & 0x07) << 26usize);
    }
    #[doc = "The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    pub const fn sideset_count(&self) -> u8 {
        let val = (self.0 >> 29usize) & 0x07;
        val as u8
    }
    #[doc = "The number of MSBs of the Delay/Side-set instruction field which are used for side-set. Inclusive of the enable bit, if present. Minimum of 0 (all delay bits, no side-set) and maximum of 5 (all side-set, no delay)."]
    #[inline(always)]
    pub fn set_sideset_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 29usize)) | (((val as u32) & 0x07) << 29usize);
    }
}
impl Default for SmPinctrl {
    #[inline(always)]
    fn default() -> SmPinctrl {
        SmPinctrl(0)
    }
}
impl core::fmt::Debug for SmPinctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmPinctrl")
            .field("out_base", &self.out_base())
            .field("set_base", &self.set_base())
            .field("sideset_base", &self.sideset_base())
            .field("in_base", &self.in_base())
            .field("out_count", &self.out_count())
            .field("set_count", &self.set_count())
            .field("sideset_count", &self.sideset_count())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmPinctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmPinctrl {
            out_base: u8,
            set_base: u8,
            sideset_base: u8,
            in_base: u8,
            out_count: u8,
            set_count: u8,
            sideset_count: u8,
        }
        let proxy = SmPinctrl {
            out_base: self.out_base(),
            set_base: self.set_base(),
            sideset_base: self.sideset_base(),
            in_base: self.in_base(),
            out_count: self.out_count(),
            set_count: self.set_count(),
            sideset_count: self.sideset_count(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control behaviour of the input/output shift registers for state machine 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SmShiftctrl(pub u32);
impl SmShiftctrl {
    #[doc = "Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
    #[inline(always)]
    pub const fn in_count(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Set the number of pins which are not masked to 0 when read by an IN PINS, WAIT PIN or MOV x, PINS instruction. For example, an IN_COUNT of 5 means that the 5 LSBs of the IN pin group are visible (bits 4:0), but the remaining 27 MSBs are masked to 0. A count of 32 is encoded with a field value of 0, so the default behaviour is to not perform any masking. Note this masking is applied in addition to the masking usually performed by the IN instruction. This is mainly useful for the MOV x, PINS instruction, which otherwise has no way of masking pins."]
    #[inline(always)]
    pub fn set_in_count(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub const fn fjoin_rx_get(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, disable this state machine's RX FIFO, make its storage available for random read access by the state machine (using the `get` instruction) and, unless FJOIN_RX_PUT is also set, random write access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub fn set_fjoin_rx_get(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub const fn fjoin_rx_put(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, disable this state machine's RX FIFO, make its storage available for random write access by the state machine (using the `put` instruction) and, unless FJOIN_RX_GET is also set, random read access by the processor (through the RXFx_PUTGETy registers). If FJOIN_RX_PUT and FJOIN_RX_GET are both set, then the RX FIFO's registers can be randomly read/written by the state machine, but are completely inaccessible to the processor. Setting this bit will clear the FJOIN_TX and FJOIN_RX bits."]
    #[inline(always)]
    pub fn set_fjoin_rx_put(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    pub const fn autopush(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Push automatically when the input shift register is filled, i.e. on an IN instruction which causes the input shift counter to reach or exceed PUSH_THRESH."]
    #[inline(always)]
    pub fn set_autopush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    pub const fn autopull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Pull automatically when the output shift register is emptied, i.e. on or following an OUT instruction which causes the output shift counter to reach or exceed PULL_THRESH."]
    #[inline(always)]
    pub fn set_autopull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub const fn in_shiftdir(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "1 = shift input shift register to right (data enters from left). 0 = to left."]
    #[inline(always)]
    pub fn set_in_shiftdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub const fn out_shiftdir(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "1 = shift out of output shift register to right. 0 = to left."]
    #[inline(always)]
    pub fn set_out_shiftdir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub const fn push_thresh(&self) -> u8 {
        let val = (self.0 >> 20usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of bits shifted into ISR before autopush, or conditional push (PUSH IFFULL), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub fn set_push_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 20usize)) | (((val as u32) & 0x1f) << 20usize);
    }
    #[doc = "Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub const fn pull_thresh(&self) -> u8 {
        let val = (self.0 >> 25usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of bits shifted out of OSR before autopull, or conditional pull (PULL IFEMPTY), will take place. Write 0 for value of 32."]
    #[inline(always)]
    pub fn set_pull_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 25usize)) | (((val as u32) & 0x1f) << 25usize);
    }
    #[doc = "When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub const fn fjoin_tx(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, TX FIFO steals the RX FIFO's storage, and becomes twice as deep. RX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn set_fjoin_tx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub const fn fjoin_rx(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, RX FIFO steals the TX FIFO's storage, and becomes twice as deep. TX FIFO is disabled as a result (always reads as both full and empty). FIFOs are flushed when this bit is changed."]
    #[inline(always)]
    pub fn set_fjoin_rx(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for SmShiftctrl {
    #[inline(always)]
    fn default() -> SmShiftctrl {
        SmShiftctrl(0)
    }
}
impl core::fmt::Debug for SmShiftctrl {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SmShiftctrl")
            .field("in_count", &self.in_count())
            .field("fjoin_rx_get", &self.fjoin_rx_get())
            .field("fjoin_rx_put", &self.fjoin_rx_put())
            .field("autopush", &self.autopush())
            .field("autopull", &self.autopull())
            .field("in_shiftdir", &self.in_shiftdir())
            .field("out_shiftdir", &self.out_shiftdir())
            .field("push_thresh", &self.push_thresh())
            .field("pull_thresh", &self.pull_thresh())
            .field("fjoin_tx", &self.fjoin_tx())
            .field("fjoin_rx", &self.fjoin_rx())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SmShiftctrl {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct SmShiftctrl {
            in_count: u8,
            fjoin_rx_get: bool,
            fjoin_rx_put: bool,
            autopush: bool,
            autopull: bool,
            in_shiftdir: bool,
            out_shiftdir: bool,
            push_thresh: u8,
            pull_thresh: u8,
            fjoin_tx: bool,
            fjoin_rx: bool,
        }
        let proxy = SmShiftctrl {
            in_count: self.in_count(),
            fjoin_rx_get: self.fjoin_rx_get(),
            fjoin_rx_put: self.fjoin_rx_put(),
            autopush: self.autopush(),
            autopull: self.autopull(),
            in_shiftdir: self.in_shiftdir(),
            out_shiftdir: self.out_shiftdir(),
            push_thresh: self.push_thresh(),
            pull_thresh: self.pull_thresh(),
            fjoin_tx: self.fjoin_tx(),
            fjoin_rx: self.fjoin_rx(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
