#[doc = "Read from the QMI direct-mode RX FIFO (fast bus access to QMI_DIRECT_RX)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QmiDirectRx(pub u32);
impl QmiDirectRx {
    #[doc = "With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data. When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received."]
    #[inline(always)]
    pub const fn qmi_direct_rx(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "With each byte clocked out on the serial interface, one byte will simultaneously be clocked in, and will appear in this FIFO. The serial interface will stall when this FIFO is full, to avoid dropping data. When 16-bit data is pushed into the TX FIFO, the corresponding RX FIFO push will also contain 16 bits of data. The least-significant byte is the first one received."]
    #[inline(always)]
    pub fn set_qmi_direct_rx(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for QmiDirectRx {
    #[inline(always)]
    fn default() -> QmiDirectRx {
        QmiDirectRx(0)
    }
}
#[doc = "Write to the QMI direct-mode TX FIFO (fast bus access to QMI_DIRECT_TX)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct QmiDirectTx(pub u32);
impl QmiDirectTx {
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
impl Default for QmiDirectTx {
    #[inline(always)]
    fn default() -> QmiDirectTx {
        QmiDirectTx(0)
    }
}
