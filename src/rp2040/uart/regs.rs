#[doc = "Control Register, UARTCR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartcr(pub u32);
impl Uartcr {
    #[doc = "UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
    #[inline(always)]
    pub const fn uarten(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "UART enable: 0 = UART is disabled. If the UART is disabled in the middle of transmission or reception, it completes the current character before stopping. 1 = the UART is enabled. Data transmission and reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit."]
    #[inline(always)]
    pub fn set_uarten(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART."]
    #[inline(always)]
    pub const fn siren(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "SIR enable: 0 = IrDA SIR ENDEC is disabled. nSIROUT remains LOW (no light pulse generated), and signal transitions on SIRIN have no effect. 1 = IrDA SIR ENDEC is enabled. Data is transmitted and received on nSIROUT and SIRIN. UARTTXD remains HIGH, in the marking state. Signal transitions on UARTRXD or modem status inputs have no effect. This bit has no effect if the UARTEN bit disables the UART."]
    #[inline(always)]
    pub fn set_siren(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub const fn sirlp(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "SIR low-power IrDA mode. This bit selects the IrDA encoding mode. If this bit is cleared to 0, low-level bits are transmitted as an active high pulse with a width of 3 / 16th of the bit period. If this bit is set to 1, low-level bits are transmitted with a pulse width that is 3 times the period of the IrLPBaud16 input signal, regardless of the selected bit rate. Setting this bit uses less power, but might reduce transmission distances."]
    #[inline(always)]
    pub fn set_sirlp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
    #[inline(always)]
    pub const fn lbe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Loopback enable. If this bit is set to 1 and the SIREN bit is set to 1 and the SIRTEST bit in the Test Control Register, UARTTCR is set to 1, then the nSIROUT path is inverted, and fed through to the SIRIN path. The SIRTEST bit in the test register must be set to 1 to override the normal half-duplex SIR operation. This must be the requirement for accessing the test registers during normal operation, and SIRTEST must be cleared to 0 when loopback testing is finished. This feature reduces the amount of external coupling required during system test. If this bit is set to 1, and the SIRTEST bit is set to 0, the UARTTXD path is fed through to the UARTRXD path. In either SIR mode or UART mode, when this bit is set, the modem outputs are also fed through to the modem inputs. This bit is cleared to 0 on reset, to disable loopback."]
    #[inline(always)]
    pub fn set_lbe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit enable. If this bit is set to 1, the transmit section of the UART is enabled. Data transmission occurs for either UART signals, or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of transmission, it completes the current character before stopping."]
    #[inline(always)]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub const fn rxe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Receive enable. If this bit is set to 1, the receive section of the UART is enabled. Data reception occurs for either UART signals or SIR signals depending on the setting of the SIREN bit. When the UART is disabled in the middle of reception, it completes the current character before stopping."]
    #[inline(always)]
    pub fn set_rxe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
    #[inline(always)]
    pub const fn dtr(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Data transmit ready. This bit is the complement of the UART data transmit ready, nUARTDTR, modem status output. That is, when the bit is programmed to a 1 then nUARTDTR is LOW."]
    #[inline(always)]
    pub fn set_dtr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
    #[inline(always)]
    pub const fn rts(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Request to send. This bit is the complement of the UART request to send, nUARTRTS, modem status output. That is, when the bit is programmed to a 1 then nUARTRTS is LOW."]
    #[inline(always)]
    pub fn set_rts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
    #[inline(always)]
    pub const fn out1(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the complement of the UART Out1 (nUARTOut1) modem status output. That is, when the bit is programmed to a 1 the output is 0. For DTE this can be used as Data Carrier Detect (DCD)."]
    #[inline(always)]
    pub fn set_out1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
    #[inline(always)]
    pub const fn out2(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "This bit is the complement of the UART Out2 (nUARTOut2) modem status output. That is, when the bit is programmed to a 1, the output is 0. For DTE this can be used as Ring Indicator (RI)."]
    #[inline(always)]
    pub fn set_out2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
    #[inline(always)]
    pub const fn rtsen(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "RTS hardware flow control enable. If this bit is set to 1, RTS hardware flow control is enabled. Data is only requested when there is space in the receive FIFO for it to be received."]
    #[inline(always)]
    pub fn set_rtsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
    #[inline(always)]
    pub const fn ctsen(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "CTS hardware flow control enable. If this bit is set to 1, CTS hardware flow control is enabled. Data is only transmitted when the nUARTCTS signal is asserted."]
    #[inline(always)]
    pub fn set_ctsen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
}
impl Default for Uartcr {
    #[inline(always)]
    fn default() -> Uartcr {
        Uartcr(0)
    }
}
impl core::fmt::Debug for Uartcr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartcr")
            .field("uarten", &self.uarten())
            .field("siren", &self.siren())
            .field("sirlp", &self.sirlp())
            .field("lbe", &self.lbe())
            .field("txe", &self.txe())
            .field("rxe", &self.rxe())
            .field("dtr", &self.dtr())
            .field("rts", &self.rts())
            .field("out1", &self.out1())
            .field("out2", &self.out2())
            .field("rtsen", &self.rtsen())
            .field("ctsen", &self.ctsen())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartcr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartcr {
            uarten: bool,
            siren: bool,
            sirlp: bool,
            lbe: bool,
            txe: bool,
            rxe: bool,
            dtr: bool,
            rts: bool,
            out1: bool,
            out2: bool,
            rtsen: bool,
            ctsen: bool,
        }
        let proxy = Uartcr {
            uarten: self.uarten(),
            siren: self.siren(),
            sirlp: self.sirlp(),
            lbe: self.lbe(),
            txe: self.txe(),
            rxe: self.rxe(),
            dtr: self.dtr(),
            rts: self.rts(),
            out1: self.out1(),
            out2: self.out2(),
            rtsen: self.rtsen(),
            ctsen: self.ctsen(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "DMA Control Register, UARTDMACR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartdmacr(pub u32);
impl Uartdmacr {
    #[doc = "Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub const fn rxdmae(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    #[inline(always)]
    pub fn set_rxdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub const fn txdmae(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    #[inline(always)]
    pub fn set_txdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    pub const fn dmaonerr(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "DMA on error. If this bit is set to 1, the DMA receive request outputs, UARTRXDMASREQ or UARTRXDMABREQ, are disabled when the UART error interrupt is asserted."]
    #[inline(always)]
    pub fn set_dmaonerr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Uartdmacr {
    #[inline(always)]
    fn default() -> Uartdmacr {
        Uartdmacr(0)
    }
}
impl core::fmt::Debug for Uartdmacr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartdmacr")
            .field("rxdmae", &self.rxdmae())
            .field("txdmae", &self.txdmae())
            .field("dmaonerr", &self.dmaonerr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartdmacr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartdmacr {
            rxdmae: bool,
            txdmae: bool,
            dmaonerr: bool,
        }
        let proxy = Uartdmacr {
            rxdmae: self.rxdmae(),
            txdmae: self.txdmae(),
            dmaonerr: self.dmaonerr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Data Register, UARTDR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartdr(pub u32);
impl Uartdr {
    #[doc = "Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    pub const fn data(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Receive (read) data character. Transmit (write) data character."]
    #[inline(always)]
    pub fn set_data(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub const fn be(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity and stop bits). In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state), and the next valid start bit is received."]
    #[inline(always)]
    pub fn set_be(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error. This bit is set to 1 if data is received and the receive FIFO is already full. This is cleared to 0 once there is an empty space in the FIFO and a new character can be written to it."]
    #[inline(always)]
    pub fn set_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
}
impl Default for Uartdr {
    #[inline(always)]
    fn default() -> Uartdr {
        Uartdr(0)
    }
}
impl core::fmt::Debug for Uartdr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartdr")
            .field("data", &self.data())
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .field("be", &self.be())
            .field("oe", &self.oe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartdr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartdr {
            data: u8,
            fe: bool,
            pe: bool,
            be: bool,
            oe: bool,
        }
        let proxy = Uartdr {
            data: self.data(),
            fe: self.fe(),
            pe: self.pe(),
            be: self.be(),
            oe: self.oe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Fractional Baud Rate Register, UARTFBRD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartfbrd(pub u32);
impl Uartfbrd {
    #[doc = "The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub const fn baud_divfrac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "The fractional baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn set_baud_divfrac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
}
impl Default for Uartfbrd {
    #[inline(always)]
    fn default() -> Uartfbrd {
        Uartfbrd(0)
    }
}
impl core::fmt::Debug for Uartfbrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartfbrd")
            .field("baud_divfrac", &self.baud_divfrac())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartfbrd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartfbrd {
            baud_divfrac: u8,
        }
        let proxy = Uartfbrd {
            baud_divfrac: self.baud_divfrac(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Flag Register, UARTFR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartfr(pub u32);
impl Uartfr {
    #[doc = "Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
    #[inline(always)]
    pub const fn cts(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clear to send. This bit is the complement of the UART clear to send, nUARTCTS, modem status input. That is, the bit is 1 when nUARTCTS is LOW."]
    #[inline(always)]
    pub fn set_cts(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
    #[inline(always)]
    pub const fn dsr(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Data set ready. This bit is the complement of the UART data set ready, nUARTDSR, modem status input. That is, the bit is 1 when nUARTDSR is LOW."]
    #[inline(always)]
    pub fn set_dsr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
    #[inline(always)]
    pub const fn dcd(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Data carrier detect. This bit is the complement of the UART data carrier detect, nUARTDCD, modem status input. That is, the bit is 1 when nUARTDCD is LOW."]
    #[inline(always)]
    pub fn set_dcd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "UART busy. If this bit is set to 1, the UART is busy transmitting data. This bit remains set until the complete byte, including all the stop bits, has been sent from the shift register. This bit is set as soon as the transmit FIFO becomes non-empty, regardless of whether the UART is enabled or not."]
    #[inline(always)]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
    #[inline(always)]
    pub const fn rxfe(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO empty. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is empty. If the FIFO is enabled, the RXFE bit is set when the receive FIFO is empty."]
    #[inline(always)]
    pub fn set_rxfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
    #[inline(always)]
    pub const fn txff(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the transmit holding register is full. If the FIFO is enabled, the TXFF bit is set when the transmit FIFO is full."]
    #[inline(always)]
    pub fn set_txff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
    #[inline(always)]
    pub const fn rxff(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full. The meaning of this bit depends on the state of the FEN bit in the UARTLCR_H Register. If the FIFO is disabled, this bit is set when the receive holding register is full. If the FIFO is enabled, the RXFF bit is set when the receive FIFO is full."]
    #[inline(always)]
    pub fn set_rxff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    pub const fn txfe(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty. The meaning of this bit depends on the state of the FEN bit in the Line Control Register, UARTLCR_H. If the FIFO is disabled, this bit is set when the transmit holding register is empty. If the FIFO is enabled, the TXFE bit is set when the transmit FIFO is empty. This bit does not indicate if there is data in the transmit shift register."]
    #[inline(always)]
    pub fn set_txfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW."]
    #[inline(always)]
    pub const fn ri(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Ring indicator. This bit is the complement of the UART ring indicator, nUARTRI, modem status input. That is, the bit is 1 when nUARTRI is LOW."]
    #[inline(always)]
    pub fn set_ri(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
}
impl Default for Uartfr {
    #[inline(always)]
    fn default() -> Uartfr {
        Uartfr(0)
    }
}
impl core::fmt::Debug for Uartfr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartfr")
            .field("cts", &self.cts())
            .field("dsr", &self.dsr())
            .field("dcd", &self.dcd())
            .field("busy", &self.busy())
            .field("rxfe", &self.rxfe())
            .field("txff", &self.txff())
            .field("rxff", &self.rxff())
            .field("txfe", &self.txfe())
            .field("ri", &self.ri())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartfr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartfr {
            cts: bool,
            dsr: bool,
            dcd: bool,
            busy: bool,
            rxfe: bool,
            txff: bool,
            rxff: bool,
            txfe: bool,
            ri: bool,
        }
        let proxy = Uartfr {
            cts: self.cts(),
            dsr: self.dsr(),
            dcd: self.dcd(),
            busy: self.busy(),
            rxfe: self.rxfe(),
            txff: self.txff(),
            rxff: self.rxff(),
            txfe: self.txfe(),
            ri: self.ri(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Integer Baud Rate Register, UARTIBRD"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartibrd(pub u32);
impl Uartibrd {
    #[doc = "The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub const fn baud_divint(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "The integer baud rate divisor. These bits are cleared to 0 on reset."]
    #[inline(always)]
    pub fn set_baud_divint(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Uartibrd {
    #[inline(always)]
    fn default() -> Uartibrd {
        Uartibrd(0)
    }
}
impl core::fmt::Debug for Uartibrd {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartibrd")
            .field("baud_divint", &self.baud_divint())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartibrd {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartibrd {
            baud_divint: u16,
        }
        let proxy = Uartibrd {
            baud_divint: self.baud_divint(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Clear Register, UARTICR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uarticr(pub u32);
impl Uarticr {
    #[doc = "nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    pub const fn rimic(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTRI modem interrupt clear. Clears the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn set_rimic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub const fn ctsmic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTCTS modem interrupt clear. Clears the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn set_ctsmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub const fn dcdmic(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDCD modem interrupt clear. Clears the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn set_dcdmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub const fn dsrmic(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDSR modem interrupt clear. Clears the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn set_dsrmic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub const fn rxic(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt clear. Clears the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn set_rxic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub const fn txic(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt clear. Clears the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn set_txic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt clear. Clears the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub const fn feic(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error interrupt clear. Clears the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn set_feic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub const fn peic(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt clear. Clears the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn set_peic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub const fn beic(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Break error interrupt clear. Clears the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn set_beic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub const fn oeic(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error interrupt clear. Clears the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn set_oeic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Uarticr {
    #[inline(always)]
    fn default() -> Uarticr {
        Uarticr(0)
    }
}
impl core::fmt::Debug for Uarticr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uarticr")
            .field("rimic", &self.rimic())
            .field("ctsmic", &self.ctsmic())
            .field("dcdmic", &self.dcdmic())
            .field("dsrmic", &self.dsrmic())
            .field("rxic", &self.rxic())
            .field("txic", &self.txic())
            .field("rtic", &self.rtic())
            .field("feic", &self.feic())
            .field("peic", &self.peic())
            .field("beic", &self.beic())
            .field("oeic", &self.oeic())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uarticr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uarticr {
            rimic: bool,
            ctsmic: bool,
            dcdmic: bool,
            dsrmic: bool,
            rxic: bool,
            txic: bool,
            rtic: bool,
            feic: bool,
            peic: bool,
            beic: bool,
            oeic: bool,
        }
        let proxy = Uarticr {
            rimic: self.rimic(),
            ctsmic: self.ctsmic(),
            dcdmic: self.dcdmic(),
            dsrmic: self.dsrmic(),
            rxic: self.rxic(),
            txic: self.txic(),
            rtic: self.rtic(),
            feic: self.feic(),
            peic: self.peic(),
            beic: self.beic(),
            oeic: self.oeic(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartifls(pub u32);
impl Uartifls {
    #[doc = "Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub const fn txiflsel(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Transmit interrupt FIFO level select. The trigger points for the transmit interrupt are as follows: b000 = Transmit FIFO becomes <= 1 / 8 full b001 = Transmit FIFO becomes <= 1 / 4 full b010 = Transmit FIFO becomes <= 1 / 2 full b011 = Transmit FIFO becomes <= 3 / 4 full b100 = Transmit FIFO becomes <= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub fn set_txiflsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub const fn rxiflsel(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Receive interrupt FIFO level select. The trigger points for the receive interrupt are as follows: b000 = Receive FIFO becomes >= 1 / 8 full b001 = Receive FIFO becomes >= 1 / 4 full b010 = Receive FIFO becomes >= 1 / 2 full b011 = Receive FIFO becomes >= 3 / 4 full b100 = Receive FIFO becomes >= 7 / 8 full b101-b111 = reserved."]
    #[inline(always)]
    pub fn set_rxiflsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
}
impl Default for Uartifls {
    #[inline(always)]
    fn default() -> Uartifls {
        Uartifls(0)
    }
}
impl core::fmt::Debug for Uartifls {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartifls")
            .field("txiflsel", &self.txiflsel())
            .field("rxiflsel", &self.rxiflsel())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartifls {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartifls {
            txiflsel: u8,
            rxiflsel: u8,
        }
        let proxy = Uartifls {
            txiflsel: self.txiflsel(),
            rxiflsel: self.rxiflsel(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "IrDA Low-Power Counter Register, UARTILPR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartilpr(pub u32);
impl Uartilpr {
    #[doc = "8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub const fn ilpdvsr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "8-bit low-power divisor value. These bits are cleared to 0 at reset."]
    #[inline(always)]
    pub fn set_ilpdvsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartilpr {
    #[inline(always)]
    fn default() -> Uartilpr {
        Uartilpr(0)
    }
}
impl core::fmt::Debug for Uartilpr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartilpr")
            .field("ilpdvsr", &self.ilpdvsr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartilpr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartilpr {
            ilpdvsr: u8,
        }
        let proxy = Uartilpr {
            ilpdvsr: self.ilpdvsr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartimsc(pub u32);
impl Uartimsc {
    #[doc = "nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn rimim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTRI modem interrupt mask. A read returns the current mask for the UARTRIINTR interrupt. On a write of 1, the mask of the UARTRIINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_rimim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn ctsmim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTCTS modem interrupt mask. A read returns the current mask for the UARTCTSINTR interrupt. On a write of 1, the mask of the UARTCTSINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_ctsmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn dcdmim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDCD modem interrupt mask. A read returns the current mask for the UARTDCDINTR interrupt. On a write of 1, the mask of the UARTDCDINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_dcdmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn dsrmim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDSR modem interrupt mask. A read returns the current mask for the UARTDSRINTR interrupt. On a write of 1, the mask of the UARTDSRINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_dsrmim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt mask. A read returns the current mask for the UARTRXINTR interrupt. On a write of 1, the mask of the UARTRXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt mask. A read returns the current mask for the UARTTXINTR interrupt. On a write of 1, the mask of the UARTTXINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt mask. A read returns the current mask for the UARTRTINTR interrupt. On a write of 1, the mask of the UARTRTINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn feim(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error interrupt mask. A read returns the current mask for the UARTFEINTR interrupt. On a write of 1, the mask of the UARTFEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_feim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn peim(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt mask. A read returns the current mask for the UARTPEINTR interrupt. On a write of 1, the mask of the UARTPEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_peim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn beim(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Break error interrupt mask. A read returns the current mask for the UARTBEINTR interrupt. On a write of 1, the mask of the UARTBEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_beim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub const fn oeim(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error interrupt mask. A read returns the current mask for the UARTOEINTR interrupt. On a write of 1, the mask of the UARTOEINTR interrupt is set. A write of 0 clears the mask."]
    #[inline(always)]
    pub fn set_oeim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Uartimsc {
    #[inline(always)]
    fn default() -> Uartimsc {
        Uartimsc(0)
    }
}
impl core::fmt::Debug for Uartimsc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartimsc")
            .field("rimim", &self.rimim())
            .field("ctsmim", &self.ctsmim())
            .field("dcdmim", &self.dcdmim())
            .field("dsrmim", &self.dsrmim())
            .field("rxim", &self.rxim())
            .field("txim", &self.txim())
            .field("rtim", &self.rtim())
            .field("feim", &self.feim())
            .field("peim", &self.peim())
            .field("beim", &self.beim())
            .field("oeim", &self.oeim())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartimsc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartimsc {
            rimim: bool,
            ctsmim: bool,
            dcdmim: bool,
            dsrmim: bool,
            rxim: bool,
            txim: bool,
            rtim: bool,
            feim: bool,
            peim: bool,
            beim: bool,
            oeim: bool,
        }
        let proxy = Uartimsc {
            rimim: self.rimim(),
            ctsmim: self.ctsmim(),
            dcdmim: self.dcdmim(),
            dsrmim: self.dsrmim(),
            rxim: self.rxim(),
            txim: self.txim(),
            rtim: self.rtim(),
            feim: self.feim(),
            peim: self.peim(),
            beim: self.beim(),
            oeim: self.oeim(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Line Control Register, UARTLCR_H"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UartlcrH(pub u32);
impl UartlcrH {
    #[doc = "Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub const fn brk(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Send break. If this bit is set to 1, a low-level is continually output on the UARTTXD output, after completing transmission of the current character. For the proper execution of the break command, the software must set this bit for at least two complete frames. For normal use, this bit must be cleared to 0."]
    #[inline(always)]
    pub fn set_brk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub const fn pen(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity enable: 0 = parity is disabled and no parity bit added to the data frame 1 = parity checking and generation is enabled."]
    #[inline(always)]
    pub fn set_pen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub const fn eps(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Even parity select. Controls the type of parity the UART uses during transmission and reception: 0 = odd parity. The UART generates or checks for an odd number of 1s in the data and parity bits. 1 = even parity. The UART generates or checks for an even number of 1s in the data and parity bits. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn set_eps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub const fn stp2(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Two stop bits select. If this bit is set to 1, two stop bits are transmitted at the end of the frame. The receive logic does not check for two stop bits being received."]
    #[inline(always)]
    pub fn set_stp2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub const fn fen(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable FIFOs: 0 = FIFOs are disabled (character mode) that is, the FIFOs become 1-byte-deep holding registers 1 = transmit and receive FIFO buffers are enabled (FIFO mode)."]
    #[inline(always)]
    pub fn set_fen(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    pub const fn wlen(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Word length. These bits indicate the number of data bits transmitted or received in a frame as follows: b11 = 8 bits b10 = 7 bits b01 = 6 bits b00 = 5 bits."]
    #[inline(always)]
    pub fn set_wlen(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub const fn sps(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Stick parity select. 0 = stick parity is disabled 1 = either: * if the EPS bit is 0 then the parity bit is transmitted and checked as a 1 * if the EPS bit is 1 then the parity bit is transmitted and checked as a 0. This bit has no effect when the PEN bit disables parity checking and generation."]
    #[inline(always)]
    pub fn set_sps(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for UartlcrH {
    #[inline(always)]
    fn default() -> UartlcrH {
        UartlcrH(0)
    }
}
impl core::fmt::Debug for UartlcrH {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UartlcrH")
            .field("brk", &self.brk())
            .field("pen", &self.pen())
            .field("eps", &self.eps())
            .field("stp2", &self.stp2())
            .field("fen", &self.fen())
            .field("wlen", &self.wlen())
            .field("sps", &self.sps())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UartlcrH {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UartlcrH {
            brk: bool,
            pen: bool,
            eps: bool,
            stp2: bool,
            fen: bool,
            wlen: u8,
            sps: bool,
        }
        let proxy = UartlcrH {
            brk: self.brk(),
            pen: self.pen(),
            eps: self.eps(),
            stp2: self.stp2(),
            fen: self.fen(),
            wlen: self.wlen(),
            sps: self.sps(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Masked Interrupt Status Register, UARTMIS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartmis(pub u32);
impl Uartmis {
    #[doc = "nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub const fn rimmis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTRI modem masked interrupt status. Returns the masked interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn set_rimmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub const fn ctsmmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTCTS modem masked interrupt status. Returns the masked interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn set_ctsmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub const fn dcdmmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDCD modem masked interrupt status. Returns the masked interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn set_dcdmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub const fn dsrmmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDSR modem masked interrupt status. Returns the masked interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn set_dsrmmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive masked interrupt status. Returns the masked interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit masked interrupt status. Returns the masked interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout masked interrupt status. Returns the masked interrupt state of the UARTRTINTR interrupt."]
    #[inline(always)]
    pub fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub const fn femis(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error masked interrupt status. Returns the masked interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn set_femis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub const fn pemis(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error masked interrupt status. Returns the masked interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn set_pemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub const fn bemis(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Break error masked interrupt status. Returns the masked interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn set_bemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub const fn oemis(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error masked interrupt status. Returns the masked interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn set_oemis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Uartmis {
    #[inline(always)]
    fn default() -> Uartmis {
        Uartmis(0)
    }
}
impl core::fmt::Debug for Uartmis {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartmis")
            .field("rimmis", &self.rimmis())
            .field("ctsmmis", &self.ctsmmis())
            .field("dcdmmis", &self.dcdmmis())
            .field("dsrmmis", &self.dsrmmis())
            .field("rxmis", &self.rxmis())
            .field("txmis", &self.txmis())
            .field("rtmis", &self.rtmis())
            .field("femis", &self.femis())
            .field("pemis", &self.pemis())
            .field("bemis", &self.bemis())
            .field("oemis", &self.oemis())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartmis {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartmis {
            rimmis: bool,
            ctsmmis: bool,
            dcdmmis: bool,
            dsrmmis: bool,
            rxmis: bool,
            txmis: bool,
            rtmis: bool,
            femis: bool,
            pemis: bool,
            bemis: bool,
            oemis: bool,
        }
        let proxy = Uartmis {
            rimmis: self.rimmis(),
            ctsmmis: self.ctsmmis(),
            dcdmmis: self.dcdmmis(),
            dsrmmis: self.dsrmmis(),
            rxmis: self.rxmis(),
            txmis: self.txmis(),
            rtmis: self.rtmis(),
            femis: self.femis(),
            pemis: self.pemis(),
            bemis: self.bemis(),
            oemis: self.oemis(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPCellID0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartpcellid0(pub u32);
impl Uartpcellid0 {
    #[doc = "These bits read back as 0x0D"]
    #[inline(always)]
    pub const fn uartpcellid0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x0D"]
    #[inline(always)]
    pub fn set_uartpcellid0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartpcellid0 {
    #[inline(always)]
    fn default() -> Uartpcellid0 {
        Uartpcellid0(0)
    }
}
impl core::fmt::Debug for Uartpcellid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartpcellid0")
            .field("uartpcellid0", &self.uartpcellid0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartpcellid0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartpcellid0 {
            uartpcellid0: u8,
        }
        let proxy = Uartpcellid0 {
            uartpcellid0: self.uartpcellid0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPCellID1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartpcellid1(pub u32);
impl Uartpcellid1 {
    #[doc = "These bits read back as 0xF0"]
    #[inline(always)]
    pub const fn uartpcellid1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xF0"]
    #[inline(always)]
    pub fn set_uartpcellid1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartpcellid1 {
    #[inline(always)]
    fn default() -> Uartpcellid1 {
        Uartpcellid1(0)
    }
}
impl core::fmt::Debug for Uartpcellid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartpcellid1")
            .field("uartpcellid1", &self.uartpcellid1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartpcellid1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartpcellid1 {
            uartpcellid1: u8,
        }
        let proxy = Uartpcellid1 {
            uartpcellid1: self.uartpcellid1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPCellID2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartpcellid2(pub u32);
impl Uartpcellid2 {
    #[doc = "These bits read back as 0x05"]
    #[inline(always)]
    pub const fn uartpcellid2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x05"]
    #[inline(always)]
    pub fn set_uartpcellid2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartpcellid2 {
    #[inline(always)]
    fn default() -> Uartpcellid2 {
        Uartpcellid2(0)
    }
}
impl core::fmt::Debug for Uartpcellid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartpcellid2")
            .field("uartpcellid2", &self.uartpcellid2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartpcellid2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartpcellid2 {
            uartpcellid2: u8,
        }
        let proxy = Uartpcellid2 {
            uartpcellid2: self.uartpcellid2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPCellID3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartpcellid3(pub u32);
impl Uartpcellid3 {
    #[doc = "These bits read back as 0xB1"]
    #[inline(always)]
    pub const fn uartpcellid3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xB1"]
    #[inline(always)]
    pub fn set_uartpcellid3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartpcellid3 {
    #[inline(always)]
    fn default() -> Uartpcellid3 {
        Uartpcellid3(0)
    }
}
impl core::fmt::Debug for Uartpcellid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartpcellid3")
            .field("uartpcellid3", &self.uartpcellid3())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartpcellid3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartpcellid3 {
            uartpcellid3: u8,
        }
        let proxy = Uartpcellid3 {
            uartpcellid3: self.uartpcellid3(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPeriphID0 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartperiphid0(pub u32);
impl Uartperiphid0 {
    #[doc = "These bits read back as 0x11"]
    #[inline(always)]
    pub const fn partnumber0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x11"]
    #[inline(always)]
    pub fn set_partnumber0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartperiphid0 {
    #[inline(always)]
    fn default() -> Uartperiphid0 {
        Uartperiphid0(0)
    }
}
impl core::fmt::Debug for Uartperiphid0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartperiphid0")
            .field("partnumber0", &self.partnumber0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartperiphid0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartperiphid0 {
            partnumber0: u8,
        }
        let proxy = Uartperiphid0 {
            partnumber0: self.partnumber0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPeriphID1 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartperiphid1(pub u32);
impl Uartperiphid1 {
    #[doc = "These bits read back as 0x0"]
    #[inline(always)]
    pub const fn partnumber1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x0"]
    #[inline(always)]
    pub fn set_partnumber1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits read back as 0x1"]
    #[inline(always)]
    pub const fn designer0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x1"]
    #[inline(always)]
    pub fn set_designer0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Uartperiphid1 {
    #[inline(always)]
    fn default() -> Uartperiphid1 {
        Uartperiphid1(0)
    }
}
impl core::fmt::Debug for Uartperiphid1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartperiphid1")
            .field("partnumber1", &self.partnumber1())
            .field("designer0", &self.designer0())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartperiphid1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartperiphid1 {
            partnumber1: u8,
            designer0: u8,
        }
        let proxy = Uartperiphid1 {
            partnumber1: self.partnumber1(),
            designer0: self.designer0(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPeriphID2 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartperiphid2(pub u32);
impl Uartperiphid2 {
    #[doc = "These bits read back as 0x4"]
    #[inline(always)]
    pub const fn designer1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x4"]
    #[inline(always)]
    pub fn set_designer1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
    #[inline(always)]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "This field depends on the revision of the UART: r1p0 0x0 r1p1 0x1 r1p3 0x2 r1p4 0x2 r1p5 0x3"]
    #[inline(always)]
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for Uartperiphid2 {
    #[inline(always)]
    fn default() -> Uartperiphid2 {
        Uartperiphid2(0)
    }
}
impl core::fmt::Debug for Uartperiphid2 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartperiphid2")
            .field("designer1", &self.designer1())
            .field("revision", &self.revision())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartperiphid2 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartperiphid2 {
            designer1: u8,
            revision: u8,
        }
        let proxy = Uartperiphid2 {
            designer1: self.designer1(),
            revision: self.revision(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "UARTPeriphID3 Register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartperiphid3(pub u32);
impl Uartperiphid3 {
    #[doc = "These bits read back as 0x00"]
    #[inline(always)]
    pub const fn configuration(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x00"]
    #[inline(always)]
    pub fn set_configuration(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for Uartperiphid3 {
    #[inline(always)]
    fn default() -> Uartperiphid3 {
        Uartperiphid3(0)
    }
}
impl core::fmt::Debug for Uartperiphid3 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartperiphid3")
            .field("configuration", &self.configuration())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartperiphid3 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartperiphid3 {
            configuration: u8,
        }
        let proxy = Uartperiphid3 {
            configuration: self.configuration(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Raw Interrupt Status Register, UARTRIS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartris(pub u32);
impl Uartris {
    #[doc = "nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub const fn rirmis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTRI modem interrupt status. Returns the raw interrupt state of the UARTRIINTR interrupt."]
    #[inline(always)]
    pub fn set_rirmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub const fn ctsrmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTCTS modem interrupt status. Returns the raw interrupt state of the UARTCTSINTR interrupt."]
    #[inline(always)]
    pub fn set_ctsrmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub const fn dcdrmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDCD modem interrupt status. Returns the raw interrupt state of the UARTDCDINTR interrupt."]
    #[inline(always)]
    pub fn set_dcdrmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub const fn dsrrmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "nUARTDSR modem interrupt status. Returns the raw interrupt state of the UARTDSRINTR interrupt."]
    #[inline(always)]
    pub fn set_dsrrmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Receive interrupt status. Returns the raw interrupt state of the UARTRXINTR interrupt."]
    #[inline(always)]
    pub fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit interrupt status. Returns the raw interrupt state of the UARTTXINTR interrupt."]
    #[inline(always)]
    pub fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
    #[inline(always)]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt status. Returns the raw interrupt state of the UARTRTINTR interrupt. a"]
    #[inline(always)]
    pub fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub const fn feris(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error interrupt status. Returns the raw interrupt state of the UARTFEINTR interrupt."]
    #[inline(always)]
    pub fn set_feris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub const fn peris(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error interrupt status. Returns the raw interrupt state of the UARTPEINTR interrupt."]
    #[inline(always)]
    pub fn set_peris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub const fn beris(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Break error interrupt status. Returns the raw interrupt state of the UARTBEINTR interrupt."]
    #[inline(always)]
    pub fn set_beris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub const fn oeris(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error interrupt status. Returns the raw interrupt state of the UARTOEINTR interrupt."]
    #[inline(always)]
    pub fn set_oeris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
}
impl Default for Uartris {
    #[inline(always)]
    fn default() -> Uartris {
        Uartris(0)
    }
}
impl core::fmt::Debug for Uartris {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartris")
            .field("rirmis", &self.rirmis())
            .field("ctsrmis", &self.ctsrmis())
            .field("dcdrmis", &self.dcdrmis())
            .field("dsrrmis", &self.dsrrmis())
            .field("rxris", &self.rxris())
            .field("txris", &self.txris())
            .field("rtris", &self.rtris())
            .field("feris", &self.feris())
            .field("peris", &self.peris())
            .field("beris", &self.beris())
            .field("oeris", &self.oeris())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartris {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartris {
            rirmis: bool,
            ctsrmis: bool,
            dcdrmis: bool,
            dsrrmis: bool,
            rxris: bool,
            txris: bool,
            rtris: bool,
            feris: bool,
            peris: bool,
            beris: bool,
            oeris: bool,
        }
        let proxy = Uartris {
            rirmis: self.rirmis(),
            ctsrmis: self.ctsrmis(),
            dcdrmis: self.dcdrmis(),
            dsrrmis: self.dsrrmis(),
            rxris: self.rxris(),
            txris: self.txris(),
            rtris: self.rtris(),
            feris: self.feris(),
            peris: self.peris(),
            beris: self.beris(),
            oeris: self.oeris(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uartrsr(pub u32);
impl Uartrsr {
    #[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub const fn fe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Framing error. When set to 1, it indicates that the received character did not have a valid stop bit (a valid stop bit is 1). This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn set_fe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub const fn pe(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Parity error. When set to 1, it indicates that the parity of the received data character does not match the parity that the EPS and SPS bits in the Line Control Register, UARTLCR_H. This bit is cleared to 0 by a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO."]
    #[inline(always)]
    pub fn set_pe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub const fn be(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Break error. This bit is set to 1 if a break condition was detected, indicating that the received data input was held LOW for longer than a full-word transmission time (defined as start, data, parity, and stop bits). This bit is cleared to 0 after a write to UARTECR. In FIFO mode, this error is associated with the character at the top of the FIFO. When a break occurs, only one 0 character is loaded into the FIFO. The next character is only enabled after the receive data input goes to a 1 (marking state) and the next valid start bit is received."]
    #[inline(always)]
    pub fn set_be(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    pub const fn oe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Overrun error. This bit is set to 1 if data is received and the FIFO is already full. This bit is cleared to 0 by a write to UARTECR. The FIFO contents remain valid because no more data is written when the FIFO is full, only the contents of the shift register are overwritten. The CPU must now read the data, to empty the FIFO."]
    #[inline(always)]
    pub fn set_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for Uartrsr {
    #[inline(always)]
    fn default() -> Uartrsr {
        Uartrsr(0)
    }
}
impl core::fmt::Debug for Uartrsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Uartrsr")
            .field("fe", &self.fe())
            .field("pe", &self.pe())
            .field("be", &self.be())
            .field("oe", &self.oe())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Uartrsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Uartrsr {
            fe: bool,
            pe: bool,
            be: bool,
            oe: bool,
        }
        let proxy = Uartrsr {
            fe: self.fe(),
            pe: self.pe(),
            be: self.be(),
            oe: self.oe(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
