#[doc = "Configure address translation for XIP virtual addresses 0x1000000 through 0x13fffff (a 4 MiB window starting at +16 MiB). Address translation allows a program image to be executed in place at multiple physical flash addresses (for example, a double-buffered flash image for over-the-air updates), without the overhead of position-independent code. At reset, the address translation registers are initialised to an identity mapping, so that they can be ignored if address translation is not required. Note that the XIP cache is fully virtually addressed, so a cache flush is required after changing the address translation."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Atrans(pub u32);
impl Atrans {
    #[doc = "Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
    #[inline(always)]
    pub const fn base(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x0fff;
        val as u16
    }
    #[doc = "Physical address base for this virtual address range, in units of 4 kiB (one flash sector). Taking a 24-bit virtual address, firstly bits 23:22 (the two MSBs) are masked to zero, and then BASE is added to bits 23:12 (the upper 12 bits) to form the physical address. Translation wraps on a 16 MiB boundary."]
    #[inline(always)]
    pub fn set_base(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0usize)) | (((val as u32) & 0x0fff) << 0usize);
    }
    #[doc = "Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
    #[inline(always)]
    pub const fn size(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0x07ff;
        val as u16
    }
    #[doc = "Translation aperture size for this virtual address range, in units of 4 kiB (one flash sector). Bits 21:12 of the virtual address are compared to SIZE. Offsets greater than SIZE return a bus error, and do not cause a QSPI access."]
    #[inline(always)]
    pub fn set_size(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 16usize)) | (((val as u32) & 0x07ff) << 16usize);
    }
}
impl Default for Atrans {
    #[inline(always)]
    fn default() -> Atrans {
        Atrans(0)
    }
}
#[doc = "Control and status for direct serial mode Direct serial mode allows the processor to send and receive raw serial frames, for programming, configuration and control of the external memory devices. Only SPI mode 0 (CPOL=0 CPHA=0) is supported."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DirectCsr(pub u32);
impl DirectCsr {
    #[doc = "Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable direct mode. In direct mode, software controls the chip select lines, and can perform direct SPI transfers by pushing data to the DIRECT_TX FIFO, and popping the same amount of data from the DIRECT_RX FIFO. Memory-mapped accesses will generate bus errors when direct serial mode is enabled."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted. The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select. (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)"]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Direct mode busy flag. If 1, data is currently being shifted in/out (or would be if the interface were not stalled on the RX FIFO), and the chip select must not yet be deasserted. The busy flag will also be set to 1 if a memory-mapped transfer is still in progress when direct mode is enabled. Direct mode blocks new memory-mapped transfers, but can't halt a transfer that is already in progress. If there is a chance that memory-mapped transfers may be in progress, the busy flag should be polled for 0 before asserting the chip select. (In practice you will usually discover this timing condition through other means, because any subsequent memory-mapped transfers when direct mode is enabled will return bus errors, which are difficult to ignore.)"]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub const fn assert_cs0n(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, assert (i.e. drive low) the CS0n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub fn set_assert_cs0n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub const fn assert_cs1n(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, assert (i.e. drive low) the CS1n chip select line. Note that this applies even when DIRECT_CSR_EN is 0."]
    #[inline(always)]
    pub fn set_assert_cs1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub const fn auto_cs0n(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, automatically assert the CS0n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub fn set_auto_cs0n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub const fn auto_cs1n(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, automatically assert the CS1n chip select line whenever the BUSY flag is set."]
    #[inline(always)]
    pub fn set_auto_cs1n(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored."]
    #[inline(always)]
    pub const fn txfull(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, the DIRECT_TX FIFO is currently full. If the processor tries to write more data, that data will be ignored."]
    #[inline(always)]
    pub fn set_txfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes."]
    #[inline(always)]
    pub const fn txempty(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, the DIRECT_TX FIFO is currently empty. Unless the processor pushes more data, transmission will stop and BUSY will go low once the current 8-bit serial frame completes."]
    #[inline(always)]
    pub fn set_txempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Current level of DIRECT_TX FIFO"]
    #[inline(always)]
    pub const fn txlevel(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Current level of DIRECT_TX FIFO"]
    #[inline(always)]
    pub fn set_txlevel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined."]
    #[inline(always)]
    pub const fn rxempty(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, the DIRECT_RX FIFO is currently empty. If the processor attempts to read more data, the FIFO state is not affected, but the value returned to the processor is undefined."]
    #[inline(always)]
    pub fn set_rxempty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full."]
    #[inline(always)]
    pub const fn rxfull(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "When 1, the DIRECT_RX FIFO is currently full. The serial interface will be stalled until data is popped; the interface will not begin a new serial frame when the DIRECT_TX FIFO is empty or the DIRECT_RX FIFO is full."]
    #[inline(always)]
    pub fn set_rxfull(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "Current level of DIRECT_RX FIFO"]
    #[inline(always)]
    pub const fn rxlevel(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Current level of DIRECT_RX FIFO"]
    #[inline(always)]
    pub fn set_rxlevel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> u8 {
        let val = (self.0 >> 22usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divisor for direct serial mode. Divisors of 1..255 are encoded directly, and the maximum divisor of 256 is encoded by a value of CLKDIV=0. The clock divisor can be changed on-the-fly by software, without halting or otherwise coordinating with the serial interface. The serial interface will sample the latest clock divisor each time it begins the transmission of a new byte."]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 22usize)) | (((val as u32) & 0xff) << 22usize);
    }
    #[doc = "Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
    #[inline(always)]
    pub const fn rxdelay(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.)"]
    #[inline(always)]
    pub fn set_rxdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for DirectCsr {
    #[inline(always)]
    fn default() -> DirectCsr {
        DirectCsr(0)
    }
}
#[doc = "Receive FIFO for direct mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DirectRx(pub u32);
impl DirectRx {
    #[doc = "With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data. When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received."]
    #[inline(always)]
    pub const fn direct_rx(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data. When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received."]
    #[inline(always)]
    pub fn set_direct_rx(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for DirectRx {
    #[inline(always)]
    fn default() -> DirectRx {
        DirectRx(0)
    }
}
#[doc = "Transmit FIFO for direct mode"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DirectTx(pub u32);
impl DirectTx {
    #[doc = "Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO. For 16-bit data, the least-significant byte is transmitted first."]
    #[inline(always)]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Data pushed here will be clocked out falling edges of SCK (or before the very first rising edge of SCK, if this is the first pulse). For each byte clocked out, the interface will simultaneously sample one byte, on rising edges of SCK, and push this to the DIRECT_RX FIFO. For 16-bit data, the least-significant byte is transmitted first."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely."]
    #[inline(always)]
    pub const fn iwidth(&self) -> super::vals::Iwidth {
        let val = (self.0 >> 16usize) & 0x03;
        super::vals::Iwidth::from_bits(val as u8)
    }
    #[doc = "Configure whether this FIFO record is transferred with single/dual/quad interface width (0/1/2). Different widths can be mixed freely."]
    #[inline(always)]
    pub fn set_iwidth(&mut self, val: super::vals::Iwidth) {
        self.0 = (self.0 & !(0x03 << 16usize)) | (((val.to_bits() as u32) & 0x03) << 16usize);
    }
    #[doc = "Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely."]
    #[inline(always)]
    pub const fn dwidth(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "Data width. If 0, hardware will transmit the 8 LSBs of the DIRECT_TX DATA field, and return an 8-bit value in the 8 LSBs of DIRECT_RX. If 1, the full 16-bit width is used. 8-bit and 16-bit transfers can be mixed freely."]
    #[inline(always)]
    pub fn set_dwidth(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input. For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer."]
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Output enable (active-high). For single width (SPI), this field is ignored, and SD0 is always set to output, with SD1 always set to input. For dual and quad width (DSPI/QSPI), this sets whether the relevant SDx pads are set to output whilst transferring this FIFO record. In this case the command/address should have OE set, and the data transfer should have OE set or clear depending on the direction of the transfer."]
    #[inline(always)]
    pub fn set_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Inhibit the RX FIFO push that would correspond to this TX FIFO entry. Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer."]
    #[inline(always)]
    pub const fn nopush(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Inhibit the RX FIFO push that would correspond to this TX FIFO entry. Useful to avoid garbage appearing in the RX FIFO when pushing the command at the beginning of a SPI transfer."]
    #[inline(always)]
    pub fn set_nopush(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
}
impl Default for DirectTx {
    #[inline(always)]
    fn default() -> DirectTx {
        DirectTx(0)
    }
}
#[doc = "Command constants used for reads from memory address window 1. The reset value of the M1_RCMD register is configured to support a basic 03h serial read transfer with no additional configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rcmd(pub u32);
impl Rcmd {
    #[doc = "The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    pub const fn prefix(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The command prefix bits to prepend on each new transfer, if Mx_RFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn set_prefix(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    pub const fn suffix(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The command suffix bits following the address, if Mx_RFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn set_suffix(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Rcmd {
    #[inline(always)]
    fn default() -> Rcmd {
        Rcmd(0)
    }
}
#[doc = "Read transfer format configuration for memory address window 0. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M0_RFMT register is configured to support a basic 03h serial read transfer with no additional configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Rfmt(pub u32);
impl Rfmt {
    #[doc = "The transfer width used for the command prefix, if any"]
    #[inline(always)]
    pub const fn prefix_width(&self) -> super::vals::PrefixWidth {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PrefixWidth::from_bits(val as u8)
    }
    #[doc = "The transfer width used for the command prefix, if any"]
    #[inline(always)]
    pub fn set_prefix_width(&mut self, val: super::vals::PrefixWidth) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    pub const fn addr_width(&self) -> super::vals::AddrWidth {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AddrWidth::from_bits(val as u8)
    }
    #[doc = "The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    pub fn set_addr_width(&mut self, val: super::vals::AddrWidth) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "The width used for the post-address command suffix, if any"]
    #[inline(always)]
    pub const fn suffix_width(&self) -> super::vals::SuffixWidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SuffixWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the post-address command suffix, if any"]
    #[inline(always)]
    pub fn set_suffix_width(&mut self, val: super::vals::SuffixWidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    pub const fn dummy_width(&self) -> super::vals::DummyWidth {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DummyWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    pub fn set_dummy_width(&mut self, val: super::vals::DummyWidth) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "The width used for the data transfer"]
    #[inline(always)]
    pub const fn data_width(&self) -> super::vals::DataWidth {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DataWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the data transfer"]
    #[inline(always)]
    pub fn set_data_width(&mut self, val: super::vals::DataWidth) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    pub const fn prefix_len(&self) -> super::vals::PrefixLen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PrefixLen::from_bits(val as u8)
    }
    #[doc = "Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    pub fn set_prefix_len(&mut self, val: super::vals::PrefixLen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    pub const fn suffix_len(&self) -> super::vals::SuffixLen {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SuffixLen::from_bits(val as u8)
    }
    #[doc = "Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    pub fn set_suffix_len(&mut self, val: super::vals::SuffixLen) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    pub const fn dummy_len(&self) -> super::vals::DummyLen {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::DummyLen::from_bits(val as u8)
    }
    #[doc = "Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    pub fn set_dummy_len(&mut self, val: super::vals::DummyLen) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable double transfer rate (DTR) for read commands: address, suffix and read data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    pub fn set_dtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Rfmt {
    #[inline(always)]
    fn default() -> Rfmt {
        Rfmt(0)
    }
}
#[doc = "Timing configuration register for memory address window 1."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Timing(pub u32);
impl Timing {
    #[doc = "Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock divisor. Odd and even divisors are supported. Defines the SCK clock period in units of 1 system clock cycle. Divisors 1..255 are encoded directly, and a divisor of 256 is encoded with a value of CLKDIV=0. The clock divisor can be changed on-the-fly, even when the QMI is currently accessing memory in this address window. All other parameters must only be changed when the QMI is idle. If software is increasing CLKDIV in anticipation of an increase in the system clock frequency, a dummy access to either memory window (and appropriate processor barriers/fences) must be inserted after the Mx_TIMING write to ensure the SCK divisor change is in effect _before_ the system clock is changed."]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
    #[inline(always)]
    pub const fn rxdelay(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x07;
        val as u8
    }
    #[doc = "Delay the read data sample timing, in units of one half of a system clock cycle. (Not necessarily half of an SCK cycle.) An RXDELAY of 0 means the sample is captured at the SDI input registers simultaneously with the rising edge of SCK launched from the SCK output register. At higher SCK frequencies, RXDELAY may need to be increased to account for the round trip delay of the pads, and the clock-to-Q delay of the QSPI memory device."]
    #[inline(always)]
    pub fn set_rxdelay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 8usize)) | (((val as u32) & 0x07) << 8usize);
    }
    #[doc = "After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
    #[inline(always)]
    pub const fn min_deselect(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x1f;
        val as u8
    }
    #[doc = "After this window's chip select is deasserted, it remains deasserted for half an SCK cycle (rounded up to an integer number of system clock cycles), plus MIN_DESELECT additional system clock cycles, before the QMI reasserts either chip select pin. Nonzero values may be required for PSRAM devices which enforce a longer minimum CS deselect time, so that they can perform internal DRAM refresh cycles whilst deselected."]
    #[inline(always)]
    pub fn set_min_deselect(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 12usize)) | (((val as u32) & 0x1f) << 12usize);
    }
    #[doc = "Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
    #[inline(always)]
    pub const fn max_select(&self) -> u8 {
        let val = (self.0 >> 17usize) & 0x3f;
        val as u8
    }
    #[doc = "Enforce a maximum assertion duration for this window's chip select, in units of 64 system clock cycles. If 0, the QMI is permitted to keep the chip select asserted indefinitely when servicing sequential memory accesses (see COOLDOWN). This feature is required to meet timing constraints of PSRAM devices, which specify a maximum chip select assertion so they can perform DRAM refresh cycles. See also MIN_DESELECT, which can enforce a minimum deselect time. If a memory access is in progress at the time MAX_SELECT is reached, the QMI will wait for the access to complete before deasserting the chip select. This additional time must be accounted for to calculate a safe MAX_SELECT value. In the worst case, this may be a fully-formed serial transfer, including command prefix and address, with a data payload as large as one cache line."]
    #[inline(always)]
    pub fn set_max_select(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 17usize)) | (((val as u32) & 0x3f) << 17usize);
    }
    #[doc = "Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
    #[inline(always)]
    pub const fn select_hold(&self) -> u8 {
        let val = (self.0 >> 23usize) & 0x03;
        val as u8
    }
    #[doc = "Add up to three additional system clock cycles of active hold between the last falling edge of SCK and the deassertion of this window's chip select. The default hold time is one system clock cycle. Note that flash datasheets usually give chip select active hold time from the last *rising* edge of SCK, and so even zero hold from the last falling edge would be safe. Note that this is a minimum hold time guaranteed by the QMI: the actual chip select active hold may be slightly longer for read transfers with low clock divisors and/or high sample delays. Specifically, if the point two cycles after the last RX data sample is later than the last SCK falling edge, then the hold time is measured from *this* point. Note also that, in case the final SCK pulse is masked to save energy (true for non-DTR reads when COOLDOWN is disabled or PAGE_BREAK is reached), all of QMI's timing logic behaves as though the clock pulse were still present. The SELECT_HOLD time is applied from the point where the last SCK falling edge would be if the clock pulse were not masked."]
    #[inline(always)]
    pub fn set_select_hold(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 23usize)) | (((val as u32) & 0x03) << 23usize);
    }
    #[doc = "Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
    #[inline(always)]
    pub const fn select_setup(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Add up to one additional system clock cycle of setup between chip select assertion and the first rising edge of SCK. The default setup time is one half SCK period, which is usually sufficient except for very high SCK frequencies with some flash devices."]
    #[inline(always)]
    pub fn set_select_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
    #[doc = "When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
    #[inline(always)]
    pub const fn pagebreak(&self) -> super::vals::Pagebreak {
        let val = (self.0 >> 28usize) & 0x03;
        super::vals::Pagebreak::from_bits(val as u8)
    }
    #[doc = "When page break is enabled, chip select will automatically deassert when crossing certain power-of-2-aligned address boundaries. The next access will always begin a new read/write SPI burst, even if the address of the next access follows in sequence with the last access before the page boundary. Some flash and PSRAM devices forbid crossing page boundaries with a single read/write transfer, or restrict the operating frequency for transfers that do cross page a boundary. This option allows the QMI to safely support those devices. This field has no effect when COOLDOWN is disabled."]
    #[inline(always)]
    pub fn set_pagebreak(&mut self, val: super::vals::Pagebreak) {
        self.0 = (self.0 & !(0x03 << 28usize)) | (((val.to_bits() as u32) & 0x03) << 28usize);
    }
    #[doc = "Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
    #[inline(always)]
    pub const fn cooldown(&self) -> u8 {
        let val = (self.0 >> 30usize) & 0x03;
        val as u8
    }
    #[doc = "Chip select cooldown period. When a memory transfer finishes, the chip select remains asserted for 64 x COOLDOWN system clock cycles, plus half an SCK clock period (rounded up for odd SCK divisors). After this cooldown expires, the chip select is always deasserted to save power. If the next memory access arrives within the cooldown period, the QMI may be able to append more SCK cycles to the currently ongoing SPI transfer, rather than starting a new transfer. This reduces access latency and increases bus throughput. Specifically, the next access must be in the same direction (read/write), access the same memory window (chip select 0/1), and follow sequentially the address of the last transfer. If any of these are false, the new access will first deassert the chip select, then begin a new transfer. If COOLDOWN is 0, the address alignment configured by PAGEBREAK has been reached, or the total chip select assertion limit MAX_SELECT has been reached, the cooldown period is skipped, and the chip select will always be deasserted one half SCK period after the transfer finishes."]
    #[inline(always)]
    pub fn set_cooldown(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30usize)) | (((val as u32) & 0x03) << 30usize);
    }
}
impl Default for Timing {
    #[inline(always)]
    fn default() -> Timing {
        Timing(0)
    }
}
#[doc = "Command constants used for writes to memory address window 1. The reset value of the M1_WCMD register is configured to support a basic 02h serial write transfer with no additional configuration."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wcmd(pub u32);
impl Wcmd {
    #[doc = "The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    pub const fn prefix(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "The command prefix bits to prepend on each new transfer, if Mx_WFMT_PREFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn set_prefix(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    pub const fn suffix(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "The command suffix bits following the address, if Mx_WFMT_SUFFIX_LEN is nonzero."]
    #[inline(always)]
    pub fn set_suffix(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for Wcmd {
    #[inline(always)]
    fn default() -> Wcmd {
        Wcmd(0)
    }
}
#[doc = "Write transfer format configuration for memory address window 1. Configure the bus width of each transfer phase individually, and configure the length or presence of the command prefix, command suffix and dummy/turnaround transfer phases. Only 24-bit addresses are supported. The reset value of the M1_WFMT register is configured to support a basic 02h serial write transfer. However, writes to this window must first be enabled via the XIP_CTRL_WRITABLE_M1 bit, as XIP memory is read-only by default."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Wfmt(pub u32);
impl Wfmt {
    #[doc = "The transfer width used for the command prefix, if any"]
    #[inline(always)]
    pub const fn prefix_width(&self) -> super::vals::PrefixWidth {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PrefixWidth::from_bits(val as u8)
    }
    #[doc = "The transfer width used for the command prefix, if any"]
    #[inline(always)]
    pub fn set_prefix_width(&mut self, val: super::vals::PrefixWidth) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    pub const fn addr_width(&self) -> super::vals::AddrWidth {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::AddrWidth::from_bits(val as u8)
    }
    #[doc = "The transfer width used for the address. The address phase always transfers 24 bits in total."]
    #[inline(always)]
    pub fn set_addr_width(&mut self, val: super::vals::AddrWidth) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "The width used for the post-address command suffix, if any"]
    #[inline(always)]
    pub const fn suffix_width(&self) -> super::vals::SuffixWidth {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::SuffixWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the post-address command suffix, if any"]
    #[inline(always)]
    pub fn set_suffix_width(&mut self, val: super::vals::SuffixWidth) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    pub const fn dummy_width(&self) -> super::vals::DummyWidth {
        let val = (self.0 >> 6usize) & 0x03;
        super::vals::DummyWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the dummy phase, if any. If width is single, SD0/MOSI is held asserted low during the dummy phase, and SD1...SD3 are tristated. If width is dual/quad, all IOs are tristated during the dummy phase."]
    #[inline(always)]
    pub fn set_dummy_width(&mut self, val: super::vals::DummyWidth) {
        self.0 = (self.0 & !(0x03 << 6usize)) | (((val.to_bits() as u32) & 0x03) << 6usize);
    }
    #[doc = "The width used for the data transfer"]
    #[inline(always)]
    pub const fn data_width(&self) -> super::vals::DataWidth {
        let val = (self.0 >> 8usize) & 0x03;
        super::vals::DataWidth::from_bits(val as u8)
    }
    #[doc = "The width used for the data transfer"]
    #[inline(always)]
    pub fn set_data_width(&mut self, val: super::vals::DataWidth) {
        self.0 = (self.0 & !(0x03 << 8usize)) | (((val.to_bits() as u32) & 0x03) << 8usize);
    }
    #[doc = "Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    pub const fn prefix_len(&self) -> super::vals::PrefixLen {
        let val = (self.0 >> 12usize) & 0x01;
        super::vals::PrefixLen::from_bits(val as u8)
    }
    #[doc = "Length of command prefix, in units of 8 bits. (i.e. 2 cycles for quad width, 4 for dual, 8 for single)"]
    #[inline(always)]
    pub fn set_prefix_len(&mut self, val: super::vals::PrefixLen) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val.to_bits() as u32) & 0x01) << 12usize);
    }
    #[doc = "Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    pub const fn suffix_len(&self) -> super::vals::SuffixLen {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::SuffixLen::from_bits(val as u8)
    }
    #[doc = "Length of post-address command suffix, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single) Only values of 0 and 8 bits are supported."]
    #[inline(always)]
    pub fn set_suffix_len(&mut self, val: super::vals::SuffixLen) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u32) & 0x03) << 14usize);
    }
    #[doc = "Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    pub const fn dummy_len(&self) -> super::vals::DummyLen {
        let val = (self.0 >> 16usize) & 0x07;
        super::vals::DummyLen::from_bits(val as u8)
    }
    #[doc = "Length of dummy phase between command suffix and data phase, in units of 4 bits. (i.e. 1 cycle for quad width, 2 for dual, 4 for single)"]
    #[inline(always)]
    pub fn set_dummy_len(&mut self, val: super::vals::DummyLen) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val.to_bits() as u32) & 0x07) << 16usize);
    }
    #[doc = "Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Enable double transfer rate (DTR) for write commands: address, suffix and write data phases are active on both edges of SCK. SDO data is launched centre-aligned on each SCK edge, and SDI data is captured on the SCK edge that follows its launch. DTR is implemented by halving the clock rate; SCK has a period of 2 x CLK_DIV throughout the transfer. The prefix and dummy phases are still single transfer rate. If the suffix is quad-width, it must be 0 or 8 bits in length, to ensure an even number of SCK edges."]
    #[inline(always)]
    pub fn set_dtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for Wfmt {
    #[inline(always)]
    fn default() -> Wfmt {
        Wfmt(0)
    }
}
