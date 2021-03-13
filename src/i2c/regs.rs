use crate::generic::*;
#[doc = "I2C Slave Address Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSar(pub u32);
impl IcSar {
    #[doc = "The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    pub const fn ic_sar(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x03ff;
        val as u16
    }
    #[doc = "The IC_SAR holds the slave address when the I2C is operating as a slave. For 7-bit addressing, only IC_SAR[6:0]
is used. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Note: The default values cannot be any of the reserved address locations: that is, 0x00 to 0x07, or 0x78 to 0x7f. The correct operation of the device is not guaranteed if you program the IC_SAR or IC_TAR to a reserved value. Refer to <<table_I2C_firstbyte_bit_defs>> for a complete list of these reserved values."]
    pub fn set_ic_sar(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0u32)) | (((val as u32) & 0x03ff) << 0u32);
    }
}
impl Default for IcSar {
    fn default() -> IcSar {
        IcSar(0)
    }
}
#[doc = "Clear RX_DONE Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrRxDone(pub u32);
impl IcClrRxDone {
    #[doc = "Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_rx_done(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_DONE interrupt (bit 7) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_rx_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrRxDone {
    fn default() -> IcClrRxDone {
        IcClrRxDone(0)
    }
}
#[doc = "DMA Transmit Data Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaTdlr(pub u32);
impl IcDmaTdlr {
    #[doc = "Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. Reset value: 0x0"]
    pub const fn dmatdl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Transmit Data Level. This bit field controls the level at which a DMA request is made by the transmit logic. It is equal to the watermark level; that is, the dma_tx_req signal is generated when the number of valid data entries in the transmit FIFO is equal to or below this field value, and TDMAE = 1. Reset value: 0x0"]
    pub fn set_dmatdl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for IcDmaTdlr {
    fn default() -> IcDmaTdlr {
        IcDmaTdlr(0)
    }
}
#[doc = "I2C Receive Data Level Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaRdlr(pub u32);
impl IcDmaRdlr {
    #[doc = "Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO. Reset value: 0x0"]
    pub const fn dmardl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Receive Data Level. This bit field controls the level at which a DMA request is made by the receive logic. The watermark level = DMARDL+1; that is, dma_rx_req is generated when the number of valid data entries in the receive FIFO is equal to or more than this field value + 1, and RDMAE =1. For instance, when DMARDL is 0, then dma_rx_req is asserted when 1 or more data entries are present in the receive FIFO. Reset value: 0x0"]
    pub fn set_dmardl(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for IcDmaRdlr {
    fn default() -> IcDmaRdlr {
        IcDmaRdlr(0)
    }
}
#[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaCr(pub u32);
impl IcDmaCr {
    #[doc = "Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    pub const fn tdmae(&self) -> super::vals::IcDmaCrTdmae {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcDmaCrTdmae(val as u8)
    }
    #[doc = "Transmit DMA Enable. This bit enables/disables the transmit FIFO DMA channel. Reset value: 0x0"]
    pub fn set_tdmae(&mut self, val: super::vals::IcDmaCrTdmae) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    pub const fn rdmae(&self) -> super::vals::IcDmaCrRdmae {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcDmaCrRdmae(val as u8)
    }
    #[doc = "Receive DMA Enable. This bit enables/disables the receive FIFO DMA channel. Reset value: 0x0"]
    pub fn set_rdmae(&mut self, val: super::vals::IcDmaCrRdmae) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcDmaCr {
    fn default() -> IcDmaCr {
        IcDmaCr(0)
    }
}
#[doc = "Standard Speed I2C Clock SCL High Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSsSclHcnt(pub u32);
impl IcSsSclHcnt {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    pub const fn ic_ss_scl_hcnt(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration'. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. NOTE: This register must not be programmed to a value higher than 65525, because DW_apb_i2c uses a 16-bit counter to flag an I2C bus idle condition when this counter reaches a value of IC_SS_SCL_HCNT + 10."]
    pub fn set_ic_ss_scl_hcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for IcSsSclHcnt {
    fn default() -> IcSsSclHcnt {
        IcSsSclHcnt(0)
    }
}
#[doc = "Clear GEN_CALL Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrGenCall(pub u32);
impl IcClrGenCall {
    #[doc = "Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_gen_call(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the GEN_CALL interrupt (bit 11) of IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_gen_call(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrGenCall {
    fn default() -> IcClrGenCall {
        IcClrGenCall(0)
    }
}
#[doc = "I2C ACK General Call Register The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address. This register is applicable only when the DW_apb_i2c is in slave mode."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcAckGeneralCall(pub u32);
impl IcAckGeneralCall {
    #[doc = "ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    pub const fn ack_gen_call(&self) -> super::vals::IcAckGeneralCallAckGenCall {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcAckGeneralCallAckGenCall(val as u8)
    }
    #[doc = "ACK General Call. When set to 1, DW_apb_i2c responds with a ACK (by asserting ic_data_oe) when it receives a General Call. Otherwise, DW_apb_i2c responds with a NACK (by negating ic_data_oe)."]
    pub fn set_ack_gen_call(&mut self, val: super::vals::IcAckGeneralCallAckGenCall) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcAckGeneralCall {
    fn default() -> IcAckGeneralCall {
        IcAckGeneralCall(0)
    }
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSource(pub u32);
impl IcTxAbrtSource {
    #[doc = "This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    pub const fn tx_flush_cnt(&self) -> u16 {
        let val = (self.0 >> 23u32) & 0x01ff;
        val as u16
    }
    #[doc = "This field indicates the number of Tx FIFO Data Commands which are flushed due to TX_ABRT interrupt. It is cleared whenever I2C is disabled. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    pub fn set_tx_flush_cnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 23u32)) | (((val as u32) & 0x01ff) << 23u32);
    }
    #[doc = "This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1]) Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub const fn abrt_user_abrt(&self) -> super::vals::IcTxAbrtSourceAbrtUserAbrt {
        let val = (self.0 >> 16u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtUserAbrt(val as u8)
    }
    #[doc = "This is a master-mode-only bit. Master has detected the transfer abort (IC_ENABLE[1]) Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub fn set_abrt_user_abrt(&mut self, val: super::vals::IcTxAbrtSourceAbrtUserAbrt) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val.0 as u32) & 0x01) << 16u32);
    }
    #[doc = "1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub const fn abrt_slvrd_intx(&self) -> super::vals::IcTxAbrtSourceAbrtSlvrdIntx {
        let val = (self.0 >> 15u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtSlvrdIntx(val as u8)
    }
    #[doc = "1: When the processor side responds to a slave mode request for data to be transmitted to a remote master and user writes a 1 in CMD (bit 8) of IC_DATA_CMD register. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub fn set_abrt_slvrd_intx(&mut self, val: super::vals::IcTxAbrtSourceAbrtSlvrdIntx) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val.0 as u32) & 0x01) << 15u32);
    }
    #[doc = "This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12]
is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub const fn abrt_slv_arblost(&self) -> super::vals::IcTxAbrtSourceAbrtSlvArblost {
        let val = (self.0 >> 14u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtSlvArblost(val as u8)
    }
    #[doc = "This field indicates that a Slave has lost the bus while transmitting data to a remote master. IC_TX_ABRT_SOURCE[12]
is set at the same time. Note: Even though the slave never 'owns' the bus, something could go wrong on the bus. This is a fail safe check. For instance, during a data transmission at the low-to-high transition of SCL, if what is on the data bus is not what is supposed to be transmitted, then DW_apb_i2c no longer own the bus. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub fn set_abrt_slv_arblost(&mut self, val: super::vals::IcTxAbrtSourceAbrtSlvArblost) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val.0 as u32) & 0x01) << 14u32);
    }
    #[doc = "This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub const fn abrt_slvflush_txfifo(&self) -> super::vals::IcTxAbrtSourceAbrtSlvflushTxfifo {
        let val = (self.0 >> 13u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtSlvflushTxfifo(val as u8)
    }
    #[doc = "This field specifies that the Slave has received a read command and some data exists in the TX FIFO, so the slave issues a TX_ABRT interrupt to flush old data in TX FIFO. Reset value: 0x0 Role of DW_apb_i2c: Slave-Transmitter"]
    pub fn set_abrt_slvflush_txfifo(&mut self, val: super::vals::IcTxAbrtSourceAbrtSlvflushTxfifo) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
    }
    #[doc = "This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14]
is also set, then the slave transmitter has lost arbitration. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    pub const fn arb_lost(&self) -> super::vals::IcTxAbrtSourceArbLost {
        let val = (self.0 >> 12u32) & 0x01;
        super::vals::IcTxAbrtSourceArbLost(val as u8)
    }
    #[doc = "This field specifies that the Master has lost arbitration, or if IC_TX_ABRT_SOURCE[14]
is also set, then the slave transmitter has lost arbitration. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Slave-Transmitter"]
    pub fn set_arb_lost(&mut self, val: super::vals::IcTxAbrtSourceArbLost) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val.0 as u32) & 0x01) << 12u32);
    }
    #[doc = "This field indicates that the User tries to initiate a Master operation with the Master mode disabled. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub const fn abrt_master_dis(&self) -> super::vals::IcTxAbrtSourceAbrtMasterDis {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtMasterDis(val as u8)
    }
    #[doc = "This field indicates that the User tries to initiate a Master operation with the Master mode disabled. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub fn set_abrt_master_dis(&mut self, val: super::vals::IcTxAbrtSourceAbrtMasterDis) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode. Reset value: 0x0 Role of DW_apb_i2c: Master-Receiver"]
    pub const fn abrt_10b_rd_norstrt(&self) -> super::vals::IcTxAbrtSourceAbrt10bRdNorstrt {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrt10bRdNorstrt(val as u8)
    }
    #[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the master sends a read command in 10-bit addressing mode. Reset value: 0x0 Role of DW_apb_i2c: Master-Receiver"]
    pub fn set_abrt_10b_rd_norstrt(&mut self, val: super::vals::IcTxAbrtSourceAbrt10bRdNorstrt) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte. Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub const fn abrt_sbyte_norstrt(&self) -> super::vals::IcTxAbrtSourceAbrtSbyteNorstrt {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtSbyteNorstrt(val as u8)
    }
    #[doc = "To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; restart must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, bit 9 clears for one cycle and then gets reasserted. When this field is set to 1, the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to send a START Byte. Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub fn set_abrt_sbyte_norstrt(&mut self, val: super::vals::IcTxAbrtSourceAbrtSbyteNorstrt) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub const fn abrt_hs_norstrt(&self) -> super::vals::IcTxAbrtSourceAbrtHsNorstrt {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtHsNorstrt(val as u8)
    }
    #[doc = "This field indicates that the restart is disabled (IC_RESTART_EN bit (IC_CON[5]) =0) and the user is trying to use the master to transfer data in High Speed mode. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub fn set_abrt_hs_norstrt(&mut self, val: super::vals::IcTxAbrtSourceAbrtHsNorstrt) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior). Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub const fn abrt_sbyte_ackdet(&self) -> super::vals::IcTxAbrtSourceAbrtSbyteAckdet {
        let val = (self.0 >> 7u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtSbyteAckdet(val as u8)
    }
    #[doc = "This field indicates that the Master has sent a START Byte and the START Byte was acknowledged (wrong behavior). Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub fn set_abrt_sbyte_ackdet(&mut self, val: super::vals::IcTxAbrtSourceAbrtSbyteAckdet) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
    }
    #[doc = "This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior). Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub const fn abrt_hs_ackdet(&self) -> super::vals::IcTxAbrtSourceAbrtHsAckdet {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtHsAckdet(val as u8)
    }
    #[doc = "This field indicates that the Master is in High Speed mode and the High Speed Master code was acknowledged (wrong behavior). Reset value: 0x0 Role of DW_apb_i2c: Master"]
    pub fn set_abrt_hs_ackdet(&mut self, val: super::vals::IcTxAbrtSourceAbrtHsAckdet) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9]
is set to 1). Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub const fn abrt_gcall_read(&self) -> super::vals::IcTxAbrtSourceAbrtGcallRead {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtGcallRead(val as u8)
    }
    #[doc = "This field indicates that DW_apb_i2c in the master mode has sent a General Call but the user programmed the byte following the General Call to be a read from the bus (IC_DATA_CMD[9]
is set to 1). Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub fn set_abrt_gcall_read(&mut self, val: super::vals::IcTxAbrtSourceAbrtGcallRead) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub const fn abrt_gcall_noack(&self) -> super::vals::IcTxAbrtSourceAbrtGcallNoack {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtGcallNoack(val as u8)
    }
    #[doc = "This field indicates that DW_apb_i2c in master mode has sent a General Call and no slave on the bus acknowledged the General Call. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub fn set_abrt_gcall_noack(&mut self, val: super::vals::IcTxAbrtSourceAbrtGcallNoack) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s). Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub const fn abrt_txdata_noack(&self) -> super::vals::IcTxAbrtSourceAbrtTxdataNoack {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrtTxdataNoack(val as u8)
    }
    #[doc = "This field indicates the master-mode only bit. When the master receives an acknowledgement for the address, but when it sends data byte(s) following the address, it did not receive an acknowledge from the remote slave(s). Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter"]
    pub fn set_abrt_txdata_noack(&mut self, val: super::vals::IcTxAbrtSourceAbrtTxdataNoack) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub const fn abrt_10addr2_noack(&self) -> super::vals::IcTxAbrtSourceAbrt10addr2Noack {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrt10addr2Noack(val as u8)
    }
    #[doc = "This field indicates that the Master is in 10-bit address mode and that the second address byte of the 10-bit address was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub fn set_abrt_10addr2_noack(&mut self, val: super::vals::IcTxAbrtSourceAbrt10addr2Noack) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub const fn abrt_10addr1_noack(&self) -> super::vals::IcTxAbrtSourceAbrt10addr1Noack {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrt10addr1Noack(val as u8)
    }
    #[doc = "This field indicates that the Master is in 10-bit address mode and the first 10-bit address byte was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub fn set_abrt_10addr1_noack(&mut self, val: super::vals::IcTxAbrtSourceAbrt10addr1Noack) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub const fn abrt_7b_addr_noack(&self) -> super::vals::IcTxAbrtSourceAbrt7bAddrNoack {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcTxAbrtSourceAbrt7bAddrNoack(val as u8)
    }
    #[doc = "This field indicates that the Master is in 7-bit addressing mode and the address sent was not acknowledged by any slave. Reset value: 0x0 Role of DW_apb_i2c: Master-Transmitter or Master-Receiver"]
    pub fn set_abrt_7b_addr_noack(&mut self, val: super::vals::IcTxAbrtSourceAbrt7bAddrNoack) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcTxAbrtSource {
    fn default() -> IcTxAbrtSource {
        IcTxAbrtSource(0)
    }
}
#[doc = "I2C Receive FIFO Threshold Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRxTl(pub u32);
impl IcRxTl {
    #[doc = "Receive FIFO Threshold Level. Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    pub const fn rx_tl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO Threshold Level. Controls the level of entries (or above) that triggers the RX_FULL interrupt (bit 2 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that hardware does not allow this value to be set to a value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 1 entry, and a value of 255 sets the threshold for 256 entries."]
    pub fn set_rx_tl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for IcRxTl {
    fn default() -> IcRxTl {
        IcRxTl(0)
    }
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatus(pub u32);
impl IcEnableStatus {
    #[doc = "Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK. Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0]
has been set to 0, then this bit is also set to 1. When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer. Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0. Reset value: 0x0"]
    pub const fn slv_rx_data_lost(&self) -> super::vals::IcEnableStatusSlvRxDataLost {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcEnableStatusSlvRxDataLost(val as u8)
    }
    #[doc = "Slave Received Data Lost. This bit indicates if a Slave-Receiver operation has been aborted with at least one data byte received from an I2C transfer due to the setting bit 0 of IC_ENABLE from 1 to 0. When read as 1, DW_apb_i2c is deemed to have been actively engaged in an aborted I2C transfer (with matching address) and the data phase of the I2C transfer has been entered, even though a data byte has been responded with a NACK. Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0]
has been set to 0, then this bit is also set to 1. When read as 0, DW_apb_i2c is deemed to have been disabled without being actively involved in the data phase of a Slave-Receiver transfer. Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0. Reset value: 0x0"]
    pub fn set_slv_rx_data_lost(&mut self, val: super::vals::IcEnableStatusSlvRxDataLost) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while: (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master; OR, (b) address and data bytes of the Slave-Receiver operation from a remote master. When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect. Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0]
has been set to 0, then this bit will also be set to 1. When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle. Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0. Reset value: 0x0"]
    pub const fn slv_disabled_while_busy(&self) -> super::vals::IcEnableStatusSlvDisabledWhileBusy {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcEnableStatusSlvDisabledWhileBusy(val as u8)
    }
    #[doc = "Slave Disabled While Busy (Transmit, Receive). This bit indicates if a potential or active Slave operation has been aborted due to the setting bit 0 of the IC_ENABLE register from 1 to 0. This bit is set when the CPU writes a 0 to the IC_ENABLE register while: (a) DW_apb_i2c is receiving the address byte of the Slave-Transmitter operation from a remote master; OR, (b) address and data bytes of the Slave-Receiver operation from a remote master. When read as 1, DW_apb_i2c is deemed to have forced a NACK during any part of an I2C transfer, irrespective of whether the I2C address matches the slave address set in DW_apb_i2c (IC_SAR register) OR if the transfer is completed before IC_ENABLE is set to 0 but has not taken effect. Note: If the remote I2C master terminates the transfer with a STOP condition before the DW_apb_i2c has a chance to NACK a transfer, and IC_ENABLE[0]
has been set to 0, then this bit will also be set to 1. When read as 0, DW_apb_i2c is deemed to have been disabled when there is master activity, or when the I2C bus is idle. Note: The CPU can safely read this bit when IC_EN (bit 0) is read as 0. Reset value: 0x0"]
    pub fn set_slv_disabled_while_busy(
        &mut self,
        val: super::vals::IcEnableStatusSlvDisabledWhileBusy,
    ) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1). Reset value: 0x0"]
    pub const fn ic_en(&self) -> super::vals::IcEnableStatusIcEn {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcEnableStatusIcEn(val as u8)
    }
    #[doc = "ic_en Status. This bit always reflects the value driven on the output port ic_en. - When read as 1, DW_apb_i2c is deemed to be in an enabled state. - When read as 0, DW_apb_i2c is deemed completely inactive. Note: The CPU can safely read this bit anytime. When this bit is read as 0, the CPU can safely read SLV_RX_DATA_LOST (bit 2) and SLV_DISABLED_WHILE_BUSY (bit 1). Reset value: 0x0"]
    pub fn set_ic_en(&mut self, val: super::vals::IcEnableStatusIcEn) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcEnableStatus {
    fn default() -> IcEnableStatus {
        IcEnableStatus(0)
    }
}
#[doc = "I2C SS, FS or FM+ spike suppression limit This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcFsSpklen(pub u32);
impl IcFsSpklen {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
    pub const fn ic_fs_spklen(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure stable operation. This register sets the duration, measured in ic_clk cycles, of the longest spike in the SCL or SDA lines that will be filtered out by the spike suppression logic. This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 1; hardware prevents values less than this being written, and if attempted results in 1 being set. or more information, refer to 'Spike Suppression'."]
    pub fn set_ic_fs_spklen(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for IcFsSpklen {
    fn default() -> IcFsSpklen {
        IcFsSpklen(0)
    }
}
#[doc = "Standard Speed I2C Clock SCL Low Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSsSclLcnt(pub u32);
impl IcSsSclLcnt {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration' This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    pub const fn ic_ss_scl_lcnt(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for standard speed. For more information, refer to 'IC_CLK Frequency Configuration' This register can be written only when the I2C interface is disabled which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 8; hardware prevents values less than this being written, and if attempted, results in 8 being set. For designs with APB_DATA_WIDTH = 8, the order of programming is important to ensure the correct operation of DW_apb_i2c. The lower byte must be programmed first, and then the upper byte is programmed."]
    pub fn set_ic_ss_scl_lcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for IcSsSclLcnt {
    fn default() -> IcSsSclLcnt {
        IcSsSclLcnt(0)
    }
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStat(pub u32);
impl IcRawIntrStat {
    #[doc = "Indicates whether master is holding the bus and TX FIFO is empty. Enabled only when I2C_DYNAMIC_TAR_UPDATE=1 and IC_EMPTYFIFO_HOLD_MASTER_EN=1. Reset value: 0x0"]
    pub const fn master_on_hold(&self) -> super::vals::IcRawIntrStatMasterOnHold {
        let val = (self.0 >> 13u32) & 0x01;
        super::vals::IcRawIntrStatMasterOnHold(val as u8)
    }
    #[doc = "Indicates whether master is holding the bus and TX FIFO is empty. Enabled only when I2C_DYNAMIC_TAR_UPDATE=1 and IC_EMPTYFIFO_HOLD_MASTER_EN=1. Reset value: 0x0"]
    pub fn set_master_on_hold(&mut self, val: super::vals::IcRawIntrStatMasterOnHold) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
    }
    #[doc = "Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1. Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt. Reset value: 0x0"]
    pub const fn restart_det(&self) -> super::vals::IcRawIntrStatRestartDet {
        let val = (self.0 >> 12u32) & 0x01;
        super::vals::IcRawIntrStatRestartDet(val as u8)
    }
    #[doc = "Indicates whether a RESTART condition has occurred on the I2C interface when DW_apb_i2c is operating in Slave mode and the slave is being addressed. Enabled only when IC_SLV_RESTART_DET_EN=1. Note: However, in high-speed mode or during a START BYTE transfer, the RESTART comes before the address field as per the I2C protocol. In this case, the slave is not the addressed slave when the RESTART is issued, therefore DW_apb_i2c does not generate the RESTART_DET interrupt. Reset value: 0x0"]
    pub fn set_restart_det(&mut self, val: super::vals::IcRawIntrStatRestartDet) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val.0 as u32) & 0x01) << 12u32);
    }
    #[doc = "Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer. Reset value: 0x0"]
    pub const fn gen_call(&self) -> super::vals::IcRawIntrStatGenCall {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcRawIntrStatGenCall(val as u8)
    }
    #[doc = "Set only when a General Call address is received and it is acknowledged. It stays set until it is cleared either by disabling DW_apb_i2c or when the CPU reads bit 0 of the IC_CLR_GEN_CALL register. DW_apb_i2c stores the received data in the Rx buffer. Reset value: 0x0"]
    pub fn set_gen_call(&mut self, val: super::vals::IcRawIntrStatGenCall) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode. Reset value: 0x0"]
    pub const fn start_det(&self) -> super::vals::IcRawIntrStatStartDet {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcRawIntrStatStartDet(val as u8)
    }
    #[doc = "Indicates whether a START or RESTART condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode. Reset value: 0x0"]
    pub fn set_start_det(&mut self, val: super::vals::IcRawIntrStatStartDet) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode. In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0"]
    pub const fn stop_det(&self) -> super::vals::IcRawIntrStatStopDet {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcRawIntrStatStopDet(val as u8)
    }
    #[doc = "Indicates whether a STOP condition has occurred on the I2C interface regardless of whether DW_apb_i2c is operating in slave or master mode. In Slave Mode: - If IC_CON[7]=1'b1 (STOP_DET_IFADDRESSED), the STOP_DET interrupt will be issued only if slave is addressed. Note: During a general call address, this slave does not issue a STOP_DET interrupt if STOP_DET_IF_ADDRESSED=1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR). - If IC_CON[7]=1'b0 (STOP_DET_IFADDRESSED), the STOP_DET interrupt is issued irrespective of whether it is being addressed. In Master Mode: - If IC_CON[10]=1'b1 (STOP_DET_IF_MASTER_ACTIVE),the STOP_DET interrupt will be issued only if Master is active. - If IC_CON[10]=1'b0 (STOP_DET_IFADDRESSED),the STOP_DET interrupt will be issued irrespective of whether master is active or not. Reset value: 0x0"]
    pub fn set_stop_det(&mut self, val: super::vals::IcRawIntrStatStopDet) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus. Reset value: 0x0"]
    pub const fn activity(&self) -> super::vals::IcRawIntrStatActivity {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcRawIntrStatActivity(val as u8)
    }
    #[doc = "This bit captures DW_apb_i2c activity and stays set until it is cleared. There are four ways to clear it: - Disabling the DW_apb_i2c - Reading the IC_CLR_ACTIVITY register - Reading the IC_CLR_INTR register - System reset Once this bit is set, it stays set unless one of the four methods is used to clear it. Even if the DW_apb_i2c module is idle, this bit remains set until cleared, indicating that there was activity on the bus. Reset value: 0x0"]
    pub fn set_activity(&mut self, val: super::vals::IcRawIntrStatActivity) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done. Reset value: 0x0"]
    pub const fn rx_done(&self) -> super::vals::IcRawIntrStatRxDone {
        let val = (self.0 >> 7u32) & 0x01;
        super::vals::IcRawIntrStatRxDone(val as u8)
    }
    #[doc = "When the DW_apb_i2c is acting as a slave-transmitter, this bit is set to 1 if the master does not acknowledge a transmitted byte. This occurs on the last byte of the transmission, indicating that the transmission is done. Reset value: 0x0"]
    pub fn set_rx_done(&mut self, val: super::vals::IcRawIntrStatRxDone) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
    }
    #[doc = "This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places. Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface. Reset value: 0x0"]
    pub const fn tx_abrt(&self) -> super::vals::IcRawIntrStatTxAbrt {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcRawIntrStatTxAbrt(val as u8)
    }
    #[doc = "This bit indicates if DW_apb_i2c, as an I2C transmitter, is unable to complete the intended actions on the contents of the transmit FIFO. This situation can occur both as an I2C master or an I2C slave, and is referred to as a 'transmit abort'. When this bit is set to 1, the IC_TX_ABRT_SOURCE register indicates the reason why the transmit abort takes places. Note: The DW_apb_i2c flushes/resets/empties the TX_FIFO and RX_FIFO whenever there is a transmit abort caused by any of the events tracked by the IC_TX_ABRT_SOURCE register. The FIFOs remains in this flushed state until the register IC_CLR_TX_ABRT is read. Once this read is performed, the Tx FIFO is then ready to accept more data bytes from the APB interface. Reset value: 0x0"]
    pub fn set_tx_abrt(&mut self, val: super::vals::IcRawIntrStatTxAbrt) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register. Reset value: 0x0"]
    pub const fn rd_req(&self) -> super::vals::IcRawIntrStatRdReq {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcRawIntrStatRdReq(val as u8)
    }
    #[doc = "This bit is set to 1 when DW_apb_i2c is acting as a slave and another I2C master is attempting to read data from DW_apb_i2c. The DW_apb_i2c holds the I2C bus in a wait state (SCL=0) until this interrupt is serviced, which means that the slave has been addressed by a remote master that is asking for data to be transferred. The processor must respond to this interrupt and then write the requested data to the IC_DATA_CMD register. This bit is set to 0 just after the processor reads the IC_CLR_RD_REQ register. Reset value: 0x0"]
    pub fn set_rd_req(&mut self, val: super::vals::IcRawIntrStatRdReq) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0]
is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0. Reset value: 0x0."]
    pub const fn tx_empty(&self) -> super::vals::IcRawIntrStatTxEmpty {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcRawIntrStatTxEmpty(val as u8)
    }
    #[doc = "The behavior of the TX_EMPTY interrupt status differs based on the TX_EMPTY_CTRL selection in the IC_CON register. - When TX_EMPTY_CTRL = 0: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register. - When TX_EMPTY_CTRL = 1: This bit is set to 1 when the transmit buffer is at or below the threshold value set in the IC_TX_TL register and the transmission of the address/data from the internal shift register for the most recently popped command is completed. It is automatically cleared by hardware when the buffer level goes above the threshold. When IC_ENABLE[0]
is set to 0, the TX FIFO is flushed and held in reset. There the TX FIFO looks like it has no data within it, so this bit is set to 1, provided there is activity in the master or slave state machines. When there is no longer any activity, then with ic_en=0, this bit is set to 0. Reset value: 0x0."]
    pub fn set_tx_empty(&mut self, val: super::vals::IcRawIntrStatTxEmpty) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Reset value: 0x0"]
    pub const fn tx_over(&self) -> super::vals::IcRawIntrStatTxOver {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcRawIntrStatTxOver(val as u8)
    }
    #[doc = "Set during transmit if the transmit buffer is filled to IC_TX_BUFFER_DEPTH and the processor attempts to issue another I2C command by writing to the IC_DATA_CMD register. When the module is disabled, this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Reset value: 0x0"]
    pub fn set_tx_over(&mut self, val: super::vals::IcRawIntrStatTxOver) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues. Reset value: 0x0"]
    pub const fn rx_full(&self) -> super::vals::IcRawIntrStatRxFull {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcRawIntrStatRxFull(val as u8)
    }
    #[doc = "Set when the receive buffer reaches or goes above the RX_TL threshold in the IC_RX_TL register. It is automatically cleared by hardware when buffer level goes below the threshold. If the module is disabled (IC_ENABLE[0]=0), the RX FIFO is flushed and held in reset; therefore the RX FIFO is not full. So this bit is cleared once the IC_ENABLE bit 0 is programmed with a 0, regardless of the activity that continues. Reset value: 0x0"]
    pub fn set_rx_full(&mut self, val: super::vals::IcRawIntrStatRxFull) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows. Reset value: 0x0"]
    pub const fn rx_over(&self) -> super::vals::IcRawIntrStatRxOver {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcRawIntrStatRxOver(val as u8)
    }
    #[doc = "Set if the receive buffer is completely filled to IC_RX_BUFFER_DEPTH and an additional byte is received from an external I2C device. The DW_apb_i2c acknowledges this, but any data bytes received after the FIFO is full are lost. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Note: If bit 9 of the IC_CON register (RX_FIFO_FULL_HLD_CTRL) is programmed to HIGH, then the RX_OVER interrupt never occurs, because the Rx FIFO never overflows. Reset value: 0x0"]
    pub fn set_rx_over(&mut self, val: super::vals::IcRawIntrStatRxOver) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Reset value: 0x0"]
    pub const fn rx_under(&self) -> super::vals::IcRawIntrStatRxUnder {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcRawIntrStatRxUnder(val as u8)
    }
    #[doc = "Set if the processor attempts to read the receive buffer when it is empty by reading from the IC_DATA_CMD register. If the module is disabled (IC_ENABLE[0]=0), this bit keeps its level until the master or slave state machines go into idle, and when ic_en goes to 0, this interrupt is cleared. Reset value: 0x0"]
    pub fn set_rx_under(&mut self, val: super::vals::IcRawIntrStatRxUnder) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcRawIntrStat {
    fn default() -> IcRawIntrStat {
        IcRawIntrStat(0)
    }
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnable(pub u32);
impl IcEnable {
    #[doc = "In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
    pub const fn tx_cmd_block(&self) -> super::vals::IcEnableTxCmdBlock {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcEnableTxCmdBlock(val as u8)
    }
    #[doc = "In Master mode: - 1'b1: Blocks the transmission of data on I2C bus even if Tx FIFO has data to transmit. - 1'b0: The transmission of data starts on I2C bus automatically, as soon as the first data is available in the Tx FIFO. Note: To block the execution of Master commands, set the TX_CMD_BLOCK bit only when Tx FIFO is empty (IC_STATUS[2]==1) and Master is in Idle state (IC_STATUS[5]
== 0). Any further commands put in the Tx FIFO are not executed until TX_CMD_BLOCK bit is unset. Reset value: IC_TX_CMD_BLOCK_DEFAULT"]
    pub fn set_tx_cmd_block(&mut self, val: super::vals::IcEnableTxCmdBlock) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation. For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'. Reset value: 0x0"]
    pub const fn abort(&self) -> super::vals::IcEnableAbort {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcEnableAbort(val as u8)
    }
    #[doc = "When set, the controller initiates the transfer abort. - 0: ABORT not initiated or ABORT done - 1: ABORT operation in progress The software can abort the I2C transfer in master mode by setting this bit. The software can set this bit only when ENABLE is already set; otherwise, the controller ignores any write to ABORT bit. The software cannot clear the ABORT bit once set. In response to an ABORT, the controller issues a STOP and flushes the Tx FIFO after completing the current transfer, then sets the TX_ABORT interrupt after the abort operation. The ABORT bit is cleared automatically after the abort operation. For a detailed description on how to abort I2C transfers, refer to 'Aborting I2C Transfers'. Reset value: 0x0"]
    pub fn set_abort(&mut self, val: super::vals::IcEnableAbort) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'. When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer. In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c' Reset value: 0x0"]
    pub const fn enable(&self) -> super::vals::IcEnableEnable {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcEnableEnable(val as u8)
    }
    #[doc = "Controls whether the DW_apb_i2c is enabled. - 0: Disables DW_apb_i2c (TX and RX FIFOs are held in an erased state) - 1: Enables DW_apb_i2c Software can disable DW_apb_i2c while it is active. However, it is important that care be taken to ensure that DW_apb_i2c is disabled properly. A recommended procedure is described in 'Disabling DW_apb_i2c'. When DW_apb_i2c is disabled, the following occurs: - The TX FIFO and RX FIFO get flushed. - Status bits in the IC_INTR_STAT register are still active until DW_apb_i2c goes into IDLE state. If the module is transmitting, it stops as well as deletes the contents of the transmit buffer after the current transfer is complete. If the module is receiving, the DW_apb_i2c stops the current transfer at the end of the current byte and does not acknowledge the transfer. In systems with asynchronous pclk and ic_clk when IC_CLK_TYPE parameter set to asynchronous (1), there is a two ic_clk delay when enabling or disabling the DW_apb_i2c. For a detailed description on how to disable DW_apb_i2c, refer to 'Disabling DW_apb_i2c' Reset value: 0x0"]
    pub fn set_enable(&mut self, val: super::vals::IcEnableEnable) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcEnable {
    fn default() -> IcEnable {
        IcEnable(0)
    }
}
#[doc = "Clear ACTIVITY Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrActivity(pub u32);
impl IcClrActivity {
    #[doc = "Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_activity(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Reading this register clears the ACTIVITY interrupt if the I2C is not active anymore. If the I2C module is still active on the bus, the ACTIVITY interrupt bit continues to be set. It is automatically cleared by hardware if the module is disabled and if there is no further activity on the bus. The value read from this register to get status of the ACTIVITY interrupt (bit 8) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_activity(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrActivity {
    fn default() -> IcClrActivity {
        IcClrActivity(0)
    }
}
#[doc = "Clear TX_ABRT Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrTxAbrt(pub u32);
impl IcClrTxAbrt {
    #[doc = "Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE. Reset value: 0x0"]
    pub const fn clr_tx_abrt(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the TX_ABRT interrupt (bit 6) of the IC_RAW_INTR_STAT register, and the IC_TX_ABRT_SOURCE register. This also releases the TX FIFO from the flushed/reset state, allowing more writes to the TX FIFO. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE. Reset value: 0x0"]
    pub fn set_clr_tx_abrt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrTxAbrt {
    fn default() -> IcClrTxAbrt {
        IcClrTxAbrt(0)
    }
}
#[doc = "Clear RX_UNDER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrRxUnder(pub u32);
impl IcClrRxUnder {
    #[doc = "Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_rx_under(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_UNDER interrupt (bit 0) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_rx_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrRxUnder {
    fn default() -> IcClrRxUnder {
        IcClrRxUnder(0)
    }
}
#[doc = "Generate Slave Data NACK Register The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect. A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0]
= 0) - Slave part is inactive (IC_STATUS[6]
= 0) Note: The IC_STATUS[6]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSlvDataNackOnly(pub u32);
impl IcSlvDataNackOnly {
    #[doc = "Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
    pub const fn nack(&self) -> super::vals::IcSlvDataNackOnlyNack {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcSlvDataNackOnlyNack(val as u8)
    }
    #[doc = "Generate NACK. This NACK generation only occurs when DW_apb_i2c is a slave-receiver. If this register is set to a value of 1, it can only generate a NACK after a data byte is received; hence, the data transfer is aborted and the data received is not pushed to the receive buffer. When the register is set to a value of 0, it generates NACK/ACK, depending on normal criteria. - 1: generate NACK after data byte received - 0: generate NACK/ACK normally Reset value: 0x0"]
    pub fn set_nack(&mut self, val: super::vals::IcSlvDataNackOnlyNack) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcSlvDataNackOnly {
    fn default() -> IcSlvDataNackOnly {
        IcSlvDataNackOnly(0)
    }
}
#[doc = "Clear RESTART_DET Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrRestartDet(pub u32);
impl IcClrRestartDet {
    #[doc = "Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_restart_det(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RESTART_DET interrupt (bit 12) of IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_restart_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrRestartDet {
    fn default() -> IcClrRestartDet {
        IcClrRestartDet(0)
    }
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMask(pub u32);
impl IcIntrMask {
    #[doc = "This M_MASTER_ON_HOLD_read_only bit masks the R_MASTER_ON_HOLD interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub const fn m_master_on_hold_read_only(&self) -> super::vals::IcIntrMaskMMasterOnHoldReadOnly {
        let val = (self.0 >> 13u32) & 0x01;
        super::vals::IcIntrMaskMMasterOnHoldReadOnly(val as u8)
    }
    #[doc = "This M_MASTER_ON_HOLD_read_only bit masks the R_MASTER_ON_HOLD interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub fn set_m_master_on_hold_read_only(
        &mut self,
        val: super::vals::IcIntrMaskMMasterOnHoldReadOnly,
    ) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
    }
    #[doc = "This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub const fn m_restart_det(&self) -> super::vals::IcIntrMaskMRestartDet {
        let val = (self.0 >> 12u32) & 0x01;
        super::vals::IcIntrMaskMRestartDet(val as u8)
    }
    #[doc = "This bit masks the R_RESTART_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub fn set_m_restart_det(&mut self, val: super::vals::IcIntrMaskMRestartDet) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val.0 as u32) & 0x01) << 12u32);
    }
    #[doc = "This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_gen_call(&self) -> super::vals::IcIntrMaskMGenCall {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcIntrMaskMGenCall(val as u8)
    }
    #[doc = "This bit masks the R_GEN_CALL interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_gen_call(&mut self, val: super::vals::IcIntrMaskMGenCall) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub const fn m_start_det(&self) -> super::vals::IcIntrMaskMStartDet {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcIntrMaskMStartDet(val as u8)
    }
    #[doc = "This bit masks the R_START_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub fn set_m_start_det(&mut self, val: super::vals::IcIntrMaskMStartDet) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub const fn m_stop_det(&self) -> super::vals::IcIntrMaskMStopDet {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcIntrMaskMStopDet(val as u8)
    }
    #[doc = "This bit masks the R_STOP_DET interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub fn set_m_stop_det(&mut self, val: super::vals::IcIntrMaskMStopDet) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub const fn m_activity(&self) -> super::vals::IcIntrMaskMActivity {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcIntrMaskMActivity(val as u8)
    }
    #[doc = "This bit masks the R_ACTIVITY interrupt in IC_INTR_STAT register. Reset value: 0x0"]
    pub fn set_m_activity(&mut self, val: super::vals::IcIntrMaskMActivity) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_rx_done(&self) -> super::vals::IcIntrMaskMRxDone {
        let val = (self.0 >> 7u32) & 0x01;
        super::vals::IcIntrMaskMRxDone(val as u8)
    }
    #[doc = "This bit masks the R_RX_DONE interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_rx_done(&mut self, val: super::vals::IcIntrMaskMRxDone) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
    }
    #[doc = "This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_tx_abrt(&self) -> super::vals::IcIntrMaskMTxAbrt {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcIntrMaskMTxAbrt(val as u8)
    }
    #[doc = "This bit masks the R_TX_ABRT interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_tx_abrt(&mut self, val: super::vals::IcIntrMaskMTxAbrt) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_rd_req(&self) -> super::vals::IcIntrMaskMRdReq {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcIntrMaskMRdReq(val as u8)
    }
    #[doc = "This bit masks the R_RD_REQ interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_rd_req(&mut self, val: super::vals::IcIntrMaskMRdReq) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_tx_empty(&self) -> super::vals::IcIntrMaskMTxEmpty {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcIntrMaskMTxEmpty(val as u8)
    }
    #[doc = "This bit masks the R_TX_EMPTY interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_tx_empty(&mut self, val: super::vals::IcIntrMaskMTxEmpty) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_tx_over(&self) -> super::vals::IcIntrMaskMTxOver {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcIntrMaskMTxOver(val as u8)
    }
    #[doc = "This bit masks the R_TX_OVER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_tx_over(&mut self, val: super::vals::IcIntrMaskMTxOver) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_rx_full(&self) -> super::vals::IcIntrMaskMRxFull {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcIntrMaskMRxFull(val as u8)
    }
    #[doc = "This bit masks the R_RX_FULL interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_rx_full(&mut self, val: super::vals::IcIntrMaskMRxFull) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_rx_over(&self) -> super::vals::IcIntrMaskMRxOver {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcIntrMaskMRxOver(val as u8)
    }
    #[doc = "This bit masks the R_RX_OVER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_rx_over(&mut self, val: super::vals::IcIntrMaskMRxOver) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub const fn m_rx_under(&self) -> super::vals::IcIntrMaskMRxUnder {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcIntrMaskMRxUnder(val as u8)
    }
    #[doc = "This bit masks the R_RX_UNDER interrupt in IC_INTR_STAT register. Reset value: 0x1"]
    pub fn set_m_rx_under(&mut self, val: super::vals::IcIntrMaskMRxUnder) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcIntrMask {
    fn default() -> IcIntrMask {
        IcIntrMask(0)
    }
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatus(pub u32);
impl IcStatus {
    #[doc = "Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0"]
    pub const fn slv_activity(&self) -> super::vals::IcStatusSlvActivity {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcStatusSlvActivity(val as u8)
    }
    #[doc = "Slave FSM Activity Status. When the Slave Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Slave FSM is in IDLE state so the Slave part of DW_apb_i2c is not Active - 1: Slave FSM is not in IDLE state so the Slave part of DW_apb_i2c is Active Reset value: 0x0"]
    pub fn set_slv_activity(&mut self, val: super::vals::IcStatusSlvActivity) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits. Reset value: 0x0"]
    pub const fn mst_activity(&self) -> super::vals::IcStatusMstActivity {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcStatusMstActivity(val as u8)
    }
    #[doc = "Master FSM Activity Status. When the Master Finite State Machine (FSM) is not in the IDLE state, this bit is set. - 0: Master FSM is in IDLE state so the Master part of DW_apb_i2c is not Active - 1: Master FSM is not in IDLE state so the Master part of DW_apb_i2c is Active Note: IC_STATUS[0]-that is, ACTIVITY bit-is the OR of SLV_ACTIVITY and MST_ACTIVITY bits. Reset value: 0x0"]
    pub fn set_mst_activity(&mut self, val: super::vals::IcStatusMstActivity) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0"]
    pub const fn rff(&self) -> super::vals::IcStatusRff {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcStatusRff(val as u8)
    }
    #[doc = "Receive FIFO Completely Full. When the receive FIFO is completely full, this bit is set. When the receive FIFO contains one or more empty location, this bit is cleared. - 0: Receive FIFO is not full - 1: Receive FIFO is full Reset value: 0x0"]
    pub fn set_rff(&mut self, val: super::vals::IcStatusRff) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0"]
    pub const fn rfne(&self) -> super::vals::IcStatusRfne {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcStatusRfne(val as u8)
    }
    #[doc = "Receive FIFO Not Empty. This bit is set when the receive FIFO contains one or more entries; it is cleared when the receive FIFO is empty. - 0: Receive FIFO is empty - 1: Receive FIFO is not empty Reset value: 0x0"]
    pub fn set_rfne(&mut self, val: super::vals::IcStatusRfne) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1"]
    pub const fn tfe(&self) -> super::vals::IcStatusTfe {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcStatusTfe(val as u8)
    }
    #[doc = "Transmit FIFO Completely Empty. When the transmit FIFO is completely empty, this bit is set. When it contains one or more valid entries, this bit is cleared. This bit field does not request an interrupt. - 0: Transmit FIFO is not empty - 1: Transmit FIFO is empty Reset value: 0x1"]
    pub fn set_tfe(&mut self, val: super::vals::IcStatusTfe) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1"]
    pub const fn tfnf(&self) -> super::vals::IcStatusTfnf {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcStatusTfnf(val as u8)
    }
    #[doc = "Transmit FIFO Not Full. Set when the transmit FIFO contains one or more empty locations, and is cleared when the FIFO is full. - 0: Transmit FIFO is full - 1: Transmit FIFO is not full Reset value: 0x1"]
    pub fn set_tfnf(&mut self, val: super::vals::IcStatusTfnf) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "I2C Activity Status. Reset value: 0x0"]
    pub const fn activity(&self) -> super::vals::IcStatusActivity {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcStatusActivity(val as u8)
    }
    #[doc = "I2C Activity Status. Reset value: 0x0"]
    pub fn set_activity(&mut self, val: super::vals::IcStatusActivity) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcStatus {
    fn default() -> IcStatus {
        IcStatus(0)
    }
}
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcFsSclHcnt(pub u32);
impl IcFsSclHcnt {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
    pub const fn ic_fs_scl_hcnt(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock high-period count for fast mode or fast mode plus. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 6; hardware prevents values less than this being written, and if attempted results in 6 being set. For designs with APB_DATA_WIDTH == 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed."]
    pub fn set_ic_fs_scl_hcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for IcFsSclHcnt {
    fn default() -> IcFsSclHcnt {
        IcFsSclHcnt(0)
    }
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmd(pub u32);
impl IcDataCmd {
    #[doc = "Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode. Reset value : 0x0 NOTE: In case of APB_DATA_WIDTH=8, 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit. 2. In order to read the 11 bit, the user has to perform the first data byte read [7:0]
(offset 0x10) and then perform the second read [15:8]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not). 3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
    pub const fn first_data_byte(&self) -> super::vals::IcDataCmdFirstDataByte {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcDataCmdFirstDataByte(val as u8)
    }
    #[doc = "Indicates the first data byte received after the address phase for receive transfer in Master receiver or Slave receiver mode. Reset value : 0x0 NOTE: In case of APB_DATA_WIDTH=8, 1. The user has to perform two APB Reads to IC_DATA_CMD in order to get status on 11 bit. 2. In order to read the 11 bit, the user has to perform the first data byte read [7:0]
(offset 0x10) and then perform the second read [15:8]
(offset 0x11) in order to know the status of 11 bit (whether the data received in previous read is a first data byte or not). 3. The 11th bit is an optional read field, user can ignore 2nd byte read [15:8]
(offset 0x11) if not interested in FIRST_DATA_BYTE status."]
    pub fn set_first_data_byte(&mut self, val: super::vals::IcDataCmdFirstDataByte) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "This bit controls whether a RESTART is issued before the byte is sent or received. 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead. 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead. Reset value: 0x0"]
    pub const fn restart(&self) -> super::vals::IcDataCmdRestart {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcDataCmdRestart(val as u8)
    }
    #[doc = "This bit controls whether a RESTART is issued before the byte is sent or received. 1 - If IC_RESTART_EN is 1, a RESTART is issued before the data is sent/received (according to the value of CMD), regardless of whether or not the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead. 0 - If IC_RESTART_EN is 1, a RESTART is issued only if the transfer direction is changing from the previous command; if IC_RESTART_EN is 0, a STOP followed by a START is issued instead. Reset value: 0x0"]
    pub fn set_restart(&mut self, val: super::vals::IcDataCmdRestart) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "This bit controls whether a STOP is issued after the byte is sent or received. - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    pub const fn stop(&self) -> super::vals::IcDataCmdStop {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcDataCmdStop(val as u8)
    }
    #[doc = "This bit controls whether a STOP is issued after the byte is sent or received. - 1 - STOP is issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master immediately tries to start a new transfer by issuing a START and arbitrating for the bus. - 0 - STOP is not issued after this byte, regardless of whether or not the Tx FIFO is empty. If the Tx FIFO is not empty, the master continues the current transfer by sending/receiving data bytes according to the value of the CMD bit. If the Tx FIFO is empty, the master holds the SCL line low and stalls the bus until a new command is available in the Tx FIFO. Reset value: 0x0"]
    pub fn set_stop(&mut self, val: super::vals::IcDataCmdStop) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master. When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted. When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs. Reset value: 0x0"]
    pub const fn cmd(&self) -> super::vals::IcDataCmdCmd {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcDataCmdCmd(val as u8)
    }
    #[doc = "This bit controls whether a read or a write is performed. This bit does not control the direction when the DW_apb_i2con acts as a slave. It controls only the direction when it acts as a master. When a command is entered in the TX FIFO, this bit distinguishes the write and read commands. In slave-receiver mode, this bit is a 'don't care' because writes to this register are not required. In slave-transmitter mode, a '0' indicates that the data in IC_DATA_CMD is to be transmitted. When programming this bit, you should remember the following: attempting to perform a read operation after a General Call command has been sent results in a TX_ABRT interrupt (bit 6 of the IC_RAW_INTR_STAT register), unless bit 11 (SPECIAL) in the IC_TAR register has been cleared. If a '1' is written to this bit after receiving a RD_REQ interrupt, then a TX_ABRT interrupt occurs. Reset value: 0x0"]
    pub fn set_cmd(&mut self, val: super::vals::IcDataCmdCmd) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface. Reset value: 0x0"]
    pub const fn dat(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "This register contains the data to be transmitted or received on the I2C bus. If you are writing to this register and want to perform a read, bits 7:0 (DAT) are ignored by the DW_apb_i2c. However, when you read this register, these bits return the value of data received on the DW_apb_i2c interface. Reset value: 0x0"]
    pub fn set_dat(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for IcDataCmd {
    fn default() -> IcDataCmd {
        IcDataCmd(0)
    }
}
#[doc = "Clear RD_REQ Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrRdReq(pub u32);
impl IcClrRdReq {
    #[doc = "Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_rd_req(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RD_REQ interrupt (bit 5) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_rd_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrRdReq {
    fn default() -> IcClrRdReq {
        IcClrRdReq(0)
    }
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStat(pub u32);
impl IcIntrStat {
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_MASTER_ON_HOLD bit. Reset value: 0x0"]
    pub const fn r_master_on_hold(&self) -> super::vals::IcIntrStatRMasterOnHold {
        let val = (self.0 >> 13u32) & 0x01;
        super::vals::IcIntrStatRMasterOnHold(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_MASTER_ON_HOLD bit. Reset value: 0x0"]
    pub fn set_r_master_on_hold(&mut self, val: super::vals::IcIntrStatRMasterOnHold) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val.0 as u32) & 0x01) << 13u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset value: 0x0"]
    pub const fn r_restart_det(&self) -> super::vals::IcIntrStatRRestartDet {
        let val = (self.0 >> 12u32) & 0x01;
        super::vals::IcIntrStatRRestartDet(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RESTART_DET bit. Reset value: 0x0"]
    pub fn set_r_restart_det(&mut self, val: super::vals::IcIntrStatRRestartDet) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val.0 as u32) & 0x01) << 12u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset value: 0x0"]
    pub const fn r_gen_call(&self) -> super::vals::IcIntrStatRGenCall {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcIntrStatRGenCall(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_GEN_CALL bit. Reset value: 0x0"]
    pub fn set_r_gen_call(&mut self, val: super::vals::IcIntrStatRGenCall) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset value: 0x0"]
    pub const fn r_start_det(&self) -> super::vals::IcIntrStatRStartDet {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcIntrStatRStartDet(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_START_DET bit. Reset value: 0x0"]
    pub fn set_r_start_det(&mut self, val: super::vals::IcIntrStatRStartDet) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset value: 0x0"]
    pub const fn r_stop_det(&self) -> super::vals::IcIntrStatRStopDet {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcIntrStatRStopDet(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_STOP_DET bit. Reset value: 0x0"]
    pub fn set_r_stop_det(&mut self, val: super::vals::IcIntrStatRStopDet) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset value: 0x0"]
    pub const fn r_activity(&self) -> super::vals::IcIntrStatRActivity {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcIntrStatRActivity(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_ACTIVITY bit. Reset value: 0x0"]
    pub fn set_r_activity(&mut self, val: super::vals::IcIntrStatRActivity) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset value: 0x0"]
    pub const fn r_rx_done(&self) -> super::vals::IcIntrStatRRxDone {
        let val = (self.0 >> 7u32) & 0x01;
        super::vals::IcIntrStatRRxDone(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_DONE bit. Reset value: 0x0"]
    pub fn set_r_rx_done(&mut self, val: super::vals::IcIntrStatRRxDone) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset value: 0x0"]
    pub const fn r_tx_abrt(&self) -> super::vals::IcIntrStatRTxAbrt {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcIntrStatRTxAbrt(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_ABRT bit. Reset value: 0x0"]
    pub fn set_r_tx_abrt(&mut self, val: super::vals::IcIntrStatRTxAbrt) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset value: 0x0"]
    pub const fn r_rd_req(&self) -> super::vals::IcIntrStatRRdReq {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcIntrStatRRdReq(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RD_REQ bit. Reset value: 0x0"]
    pub fn set_r_rd_req(&mut self, val: super::vals::IcIntrStatRRdReq) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset value: 0x0"]
    pub const fn r_tx_empty(&self) -> super::vals::IcIntrStatRTxEmpty {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcIntrStatRTxEmpty(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_EMPTY bit. Reset value: 0x0"]
    pub fn set_r_tx_empty(&mut self, val: super::vals::IcIntrStatRTxEmpty) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset value: 0x0"]
    pub const fn r_tx_over(&self) -> super::vals::IcIntrStatRTxOver {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcIntrStatRTxOver(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_TX_OVER bit. Reset value: 0x0"]
    pub fn set_r_tx_over(&mut self, val: super::vals::IcIntrStatRTxOver) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset value: 0x0"]
    pub const fn r_rx_full(&self) -> super::vals::IcIntrStatRRxFull {
        let val = (self.0 >> 2u32) & 0x01;
        super::vals::IcIntrStatRRxFull(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_FULL bit. Reset value: 0x0"]
    pub fn set_r_rx_full(&mut self, val: super::vals::IcIntrStatRRxFull) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val.0 as u32) & 0x01) << 2u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset value: 0x0"]
    pub const fn r_rx_over(&self) -> super::vals::IcIntrStatRRxOver {
        let val = (self.0 >> 1u32) & 0x01;
        super::vals::IcIntrStatRRxOver(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_OVER bit. Reset value: 0x0"]
    pub fn set_r_rx_over(&mut self, val: super::vals::IcIntrStatRRxOver) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val.0 as u32) & 0x01) << 1u32);
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset value: 0x0"]
    pub const fn r_rx_under(&self) -> super::vals::IcIntrStatRRxUnder {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcIntrStatRRxUnder(val as u8)
    }
    #[doc = "See IC_RAW_INTR_STAT for a detailed description of R_RX_UNDER bit. Reset value: 0x0"]
    pub fn set_r_rx_under(&mut self, val: super::vals::IcIntrStatRRxUnder) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcIntrStat {
    fn default() -> IcIntrStat {
        IcIntrStat(0)
    }
}
#[doc = "Clear RX_OVER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrRxOver(pub u32);
impl IcClrRxOver {
    #[doc = "Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_rx_over(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the RX_OVER interrupt (bit 1) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_rx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrRxOver {
    fn default() -> IcClrRxOver {
        IcClrRxOver(0)
    }
}
#[doc = "I2C SDA Hold Time Length Register The bits [15:0]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW). The bits [23:16]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode. Writes to this register succeed only when IC_ENABLE[0]=0. The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented. The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSdaHold(pub u32);
impl IcSdaHold {
    #[doc = "Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver. Reset value: IC_DEFAULT_SDA_HOLD[23:16]."]
    pub const fn ic_sda_rx_hold(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0xff;
        val as u8
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a receiver. Reset value: IC_DEFAULT_SDA_HOLD[23:16]."]
    pub fn set_ic_sda_rx_hold(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16u32)) | (((val as u32) & 0xff) << 16u32);
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter. Reset value: IC_DEFAULT_SDA_HOLD[15:0]."]
    pub const fn ic_sda_tx_hold(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Sets the required SDA hold time in units of ic_clk period, when DW_apb_i2c acts as a transmitter. Reset value: IC_DEFAULT_SDA_HOLD[15:0]."]
    pub fn set_ic_sda_tx_hold(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for IcSdaHold {
    fn default() -> IcSdaHold {
        IcSdaHold(0)
    }
}
#[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxflr(pub u32);
impl IcTxflr {
    #[doc = "Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Reset value: 0x0"]
    pub const fn txflr(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Transmit FIFO Level. Contains the number of valid data entries in the transmit FIFO. Reset value: 0x0"]
    pub fn set_txflr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for IcTxflr {
    fn default() -> IcTxflr {
        IcTxflr(0)
    }
}
#[doc = "I2C Transmit FIFO Threshold Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxTl(pub u32);
impl IcTxTl {
    #[doc = "Transmit FIFO Threshold Level. Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    pub const fn tx_tl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO Threshold Level. Controls the level of entries (or below) that trigger the TX_EMPTY interrupt (bit 4 in IC_RAW_INTR_STAT register). The valid range is 0-255, with the additional restriction that it may not be set to value larger than the depth of the buffer. If an attempt is made to do that, the actual value set will be the maximum depth of the buffer. A value of 0 sets the threshold for 0 entries, and a value of 255 sets the threshold for 255 entries."]
    pub fn set_tx_tl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for IcTxTl {
    fn default() -> IcTxTl {
        IcTxTl(0)
    }
}
#[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcFsSclLcnt(pub u32);
impl IcFsSclLcnt {
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8."]
    pub const fn ic_fs_scl_lcnt(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "This register must be set before any I2C bus transaction can take place to ensure proper I/O timing. This register sets the SCL clock low period count for fast speed. It is used in high-speed mode to send the Master Code and START BYTE or General CALL. For more information, refer to 'IC_CLK Frequency Configuration'. This register goes away and becomes read-only returning 0s if IC_MAX_SPEED_MODE = standard. This register can be written only when the I2C interface is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. The minimum valid value is 8; hardware prevents values less than this being written, and if attempted results in 8 being set. For designs with APB_DATA_WIDTH = 8 the order of programming is important to ensure the correct operation of the DW_apb_i2c. The lower byte must be programmed first. Then the upper byte is programmed. If the value is less than 8 then the count value gets changed to 8."]
    pub fn set_ic_fs_scl_lcnt(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for IcFsSclLcnt {
    fn default() -> IcFsSclLcnt {
        IcFsSclLcnt(0)
    }
}
#[doc = "Clear START_DET Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrStartDet(pub u32);
impl IcClrStartDet {
    #[doc = "Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_start_det(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the START_DET interrupt (bit 10) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_start_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrStartDet {
    fn default() -> IcClrStartDet {
        IcClrStartDet(0)
    }
}
#[doc = "Clear STOP_DET Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrStopDet(pub u32);
impl IcClrStopDet {
    #[doc = "Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_stop_det(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the STOP_DET interrupt (bit 9) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_stop_det(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrStopDet {
    fn default() -> IcClrStopDet {
        IcClrStopDet(0)
    }
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcCon(pub u32);
impl IcCon {
    #[doc = "Master issues the STOP_DET interrupt irrespective of whether master is active or not"]
    pub const fn stop_det_if_master_active(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Master issues the STOP_DET interrupt irrespective of whether master is active or not"]
    pub fn set_stop_det_if_master_active(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter. Reset value: 0x0."]
    pub const fn rx_fifo_full_hld_ctrl(&self) -> super::vals::IcConRxFifoFullHldCtrl {
        let val = (self.0 >> 9u32) & 0x01;
        super::vals::IcConRxFifoFullHldCtrl(val as u8)
    }
    #[doc = "This bit controls whether DW_apb_i2c should hold the bus when the Rx FIFO is physically full to its RX_BUFFER_DEPTH, as described in the IC_RX_FULL_HLD_BUS_EN parameter. Reset value: 0x0."]
    pub fn set_rx_fifo_full_hld_ctrl(&mut self, val: super::vals::IcConRxFifoFullHldCtrl) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val.0 as u32) & 0x01) << 9u32);
    }
    #[doc = "This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register. Reset value: 0x0."]
    pub const fn tx_empty_ctrl(&self) -> super::vals::IcConTxEmptyCtrl {
        let val = (self.0 >> 8u32) & 0x01;
        super::vals::IcConTxEmptyCtrl(val as u8)
    }
    #[doc = "This bit controls the generation of the TX_EMPTY interrupt, as described in the IC_RAW_INTR_STAT register. Reset value: 0x0."]
    pub fn set_tx_empty_ctrl(&mut self, val: super::vals::IcConTxEmptyCtrl) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val.0 as u32) & 0x01) << 8u32);
    }
    #[doc = "In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    pub const fn stop_det_ifaddressed(&self) -> super::vals::IcConStopDetIfaddressed {
        let val = (self.0 >> 7u32) & 0x01;
        super::vals::IcConStopDetIfaddressed(val as u8)
    }
    #[doc = "In slave mode: - 1'b1: issues the STOP_DET interrupt only when it is addressed. - 1'b0: issues the STOP_DET irrespective of whether it's addressed or not. Reset value: 0x0 NOTE: During a general call address, this slave does not issue the STOP_DET interrupt if STOP_DET_IF_ADDRESSED = 1'b1, even if the slave responds to the general call address by generating ACK. The STOP_DET interrupt is generated only when the transmitted address matches the slave address (SAR)."]
    pub fn set_stop_det_ifaddressed(&mut self, val: super::vals::IcConStopDetIfaddressed) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val.0 as u32) & 0x01) << 7u32);
    }
    #[doc = "This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled. If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave. NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    pub const fn ic_slave_disable(&self) -> super::vals::IcConIcSlaveDisable {
        let val = (self.0 >> 6u32) & 0x01;
        super::vals::IcConIcSlaveDisable(val as u8)
    }
    #[doc = "This bit controls whether I2C has its slave disabled, which means once the presetn signal is applied, then this bit is set and the slave is disabled. If this bit is set (slave is disabled), DW_apb_i2c functions only as a master and does not perform any action that requires a slave. NOTE: Software should ensure that if this bit is written with 0, then bit 0 should also be written with a 0."]
    pub fn set_ic_slave_disable(&mut self, val: super::vals::IcConIcSlaveDisable) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val.0 as u32) & 0x01) << 6u32);
    }
    #[doc = "Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. Reset value: ENABLED"]
    pub const fn ic_restart_en(&self) -> super::vals::IcConIcRestartEn {
        let val = (self.0 >> 5u32) & 0x01;
        super::vals::IcConIcRestartEn(val as u8)
    }
    #[doc = "Determines whether RESTART conditions may be sent when acting as a master. Some older slaves do not support handling RESTART conditions; however, RESTART conditions are used in several DW_apb_i2c operations. When RESTART is disabled, the master is prohibited from performing the following functions: - Sending a START BYTE - Performing any high-speed mode operation - High-speed mode operation - Performing direction changes in combined format mode - Performing a read operation with a 10-bit address By replacing RESTART condition followed by a STOP and a subsequent START condition, split operations are broken down into multiple DW_apb_i2c transfers. If the above operations are performed, it will result in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. Reset value: ENABLED"]
    pub fn set_ic_restart_en(&mut self, val: super::vals::IcConIcRestartEn) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val.0 as u32) & 0x01) << 5u32);
    }
    #[doc = "Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    pub const fn ic_10bitaddr_master(&self) -> super::vals::IcConIc10bitaddrMaster {
        let val = (self.0 >> 4u32) & 0x01;
        super::vals::IcConIc10bitaddrMaster(val as u8)
    }
    #[doc = "Controls whether the DW_apb_i2c starts its transfers in 7- or 10-bit addressing mode when acting as a master. - 0: 7-bit addressing - 1: 10-bit addressing"]
    pub fn set_ic_10bitaddr_master(&mut self, val: super::vals::IcConIc10bitaddrMaster) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val.0 as u32) & 0x01) << 4u32);
    }
    #[doc = "When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    pub const fn ic_10bitaddr_slave(&self) -> super::vals::IcConIc10bitaddrSlave {
        let val = (self.0 >> 3u32) & 0x01;
        super::vals::IcConIc10bitaddrSlave(val as u8)
    }
    #[doc = "When acting as a slave, this bit controls whether the DW_apb_i2c responds to 7- or 10-bit addresses. - 0: 7-bit addressing. The DW_apb_i2c ignores transactions that involve 10-bit addressing; for 7-bit addressing, only the lower 7 bits of the IC_SAR register are compared. - 1: 10-bit addressing. The DW_apb_i2c responds to only 10-bit addressing transfers that match the full 10 bits of the IC_SAR register."]
    pub fn set_ic_10bitaddr_slave(&mut self, val: super::vals::IcConIc10bitaddrSlave) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val.0 as u32) & 0x01) << 3u32);
    }
    #[doc = "These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode. This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE. 1: standard mode (100 kbit/s) 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s) 3: high speed mode (3.4 Mbit/s) Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    pub const fn speed(&self) -> super::vals::IcConSpeed {
        let val = (self.0 >> 1u32) & 0x03;
        super::vals::IcConSpeed(val as u8)
    }
    #[doc = "These bits control at which speed the DW_apb_i2c operates; its setting is relevant only if one is operating the DW_apb_i2c in master mode. Hardware protects against illegal values being programmed by software. These bits must be programmed appropriately for slave mode also, as it is used to capture correct value of spike filter as per the speed mode. This register should be programmed only with a value in the range of 1 to IC_MAX_SPEED_MODE; otherwise, hardware updates this register with the value of IC_MAX_SPEED_MODE. 1: standard mode (100 kbit/s) 2: fast mode (<=400 kbit/s) or fast mode plus (<=1000Kbit/s) 3: high speed mode (3.4 Mbit/s) Note: This field is not applicable when IC_ULTRA_FAST_MODE=1"]
    pub fn set_speed(&mut self, val: super::vals::IcConSpeed) {
        self.0 = (self.0 & !(0x03 << 1u32)) | (((val.0 as u32) & 0x03) << 1u32);
    }
    #[doc = "This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    pub const fn master_mode(&self) -> super::vals::IcConMasterMode {
        let val = (self.0 >> 0u32) & 0x01;
        super::vals::IcConMasterMode(val as u8)
    }
    #[doc = "This bit controls whether the DW_apb_i2c master is enabled. NOTE: Software should ensure that if this bit is written with '1' then bit 6 should also be written with a '1'."]
    pub fn set_master_mode(&mut self, val: super::vals::IcConMasterMode) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val.0 as u32) & 0x01) << 0u32);
    }
}
impl Default for IcCon {
    fn default() -> IcCon {
        IcCon(0)
    }
}
#[doc = "I2C SDA Setup Register This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. Writes to this register succeed only when IC_ENABLE[0]
= 0. Note: The length of setup time is calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSdaSetup(pub u32);
impl IcSdaSetup {
    #[doc = "SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    pub const fn sda_setup(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "SDA Setup. It is recommended that if the required delay is 1000ns, then for an ic_clk frequency of 10 MHz, IC_SDA_SETUP should be programmed to a value of 11. IC_SDA_SETUP must be programmed with a minimum value of 2."]
    pub fn set_sda_setup(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for IcSdaSetup {
    fn default() -> IcSdaSetup {
        IcSdaSetup(0)
    }
}
#[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTar(pub u32);
impl IcTar {
    #[doc = "This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    pub const fn special(&self) -> super::vals::IcTarSpecial {
        let val = (self.0 >> 11u32) & 0x01;
        super::vals::IcTarSpecial(val as u8)
    }
    #[doc = "This bit indicates whether software performs a Device-ID or General Call or START BYTE command. - 0: ignore bit 10 GC_OR_START and use IC_TAR normally - 1: perform special I2C command as specified in Device_ID or GC_OR_START bit Reset value: 0x0"]
    pub fn set_special(&mut self, val: super::vals::IcTarSpecial) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val.0 as u32) & 0x01) << 11u32);
    }
    #[doc = "If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    pub const fn gc_or_start(&self) -> super::vals::IcTarGcOrStart {
        let val = (self.0 >> 10u32) & 0x01;
        super::vals::IcTarGcOrStart(val as u8)
    }
    #[doc = "If bit 11 (SPECIAL) is set to 1 and bit 13(Device-ID) is set to 0, then this bit indicates whether a General Call or START byte command is to be performed by the DW_apb_i2c. - 0: General Call Address - after issuing a General Call, only writes may be performed. Attempting to issue a read command results in setting bit 6 (TX_ABRT) of the IC_RAW_INTR_STAT register. The DW_apb_i2c remains in General Call mode until the SPECIAL bit value (bit 11) is cleared. - 1: START BYTE Reset value: 0x0"]
    pub fn set_gc_or_start(&mut self, val: super::vals::IcTarGcOrStart) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val.0 as u32) & 0x01) << 10u32);
    }
    #[doc = "This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    pub const fn ic_tar(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x03ff;
        val as u16
    }
    #[doc = "This is the target address for any master transaction. When transmitting a General Call, these bits are ignored. To generate a START BYTE, the CPU needs to write only once into these bits. If the IC_TAR and IC_SAR are the same, loopback exists but the FIFOs are shared between master and slave, so full loopback is not feasible. Only one direction loopback mode is supported (simplex), not duplex. A master cannot transmit to itself; it can transmit to only a slave."]
    pub fn set_ic_tar(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0u32)) | (((val as u32) & 0x03ff) << 0u32);
    }
}
impl Default for IcTar {
    fn default() -> IcTar {
        IcTar(0)
    }
}
#[doc = "Clear TX_OVER Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrTxOver(pub u32);
impl IcClrTxOver {
    #[doc = "Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub const fn clr_tx_over(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the TX_OVER interrupt (bit 3) of the IC_RAW_INTR_STAT register. Reset value: 0x0"]
    pub fn set_clr_tx_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrTxOver {
    fn default() -> IcClrTxOver {
        IcClrTxOver(0)
    }
}
#[doc = "Component Parameter Register 1 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcCompParam1(pub u32);
impl IcCompParam1 {
    #[doc = "TX Buffer Depth = 16"]
    pub const fn tx_buffer_depth(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0xff;
        val as u8
    }
    #[doc = "TX Buffer Depth = 16"]
    pub fn set_tx_buffer_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16u32)) | (((val as u32) & 0xff) << 16u32);
    }
    #[doc = "RX Buffer Depth = 16"]
    pub const fn rx_buffer_depth(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0xff;
        val as u8
    }
    #[doc = "RX Buffer Depth = 16"]
    pub fn set_rx_buffer_depth(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
    }
    #[doc = "Encoded parameters not visible"]
    pub const fn add_encoded_params(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Encoded parameters not visible"]
    pub fn set_add_encoded_params(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "DMA handshaking signals are enabled"]
    pub const fn has_dma(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "DMA handshaking signals are enabled"]
    pub fn set_has_dma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "COMBINED Interrupt outputs"]
    pub const fn intr_io(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "COMBINED Interrupt outputs"]
    pub fn set_intr_io(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Programmable count values for each mode."]
    pub const fn hc_count_values(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Programmable count values for each mode."]
    pub fn set_hc_count_values(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "MAX SPEED MODE = FAST MODE"]
    pub const fn max_speed_mode(&self) -> u8 {
        let val = (self.0 >> 2u32) & 0x03;
        val as u8
    }
    #[doc = "MAX SPEED MODE = FAST MODE"]
    pub fn set_max_speed_mode(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val as u32) & 0x03) << 2u32);
    }
    #[doc = "APB data bus width is 32 bits"]
    pub const fn apb_data_width(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x03;
        val as u8
    }
    #[doc = "APB data bus width is 32 bits"]
    pub fn set_apb_data_width(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 0u32)) | (((val as u32) & 0x03) << 0u32);
    }
}
impl Default for IcCompParam1 {
    fn default() -> IcCompParam1 {
        IcCompParam1(0)
    }
}
#[doc = "Clear Combined and Individual Interrupt Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcClrIntr(pub u32);
impl IcClrIntr {
    #[doc = "Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE. Reset value: 0x0"]
    pub const fn clr_intr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Read this register to clear the combined interrupt, all individual interrupts, and the IC_TX_ABRT_SOURCE register. This bit does not clear hardware clearable interrupts but software clearable interrupts. Refer to Bit 9 of the IC_TX_ABRT_SOURCE register for an exception to clearing IC_TX_ABRT_SOURCE. Reset value: 0x0"]
    pub fn set_clr_intr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for IcClrIntr {
    fn default() -> IcClrIntr {
        IcClrIntr(0)
    }
}
#[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRxflr(pub u32);
impl IcRxflr {
    #[doc = "Receive FIFO Level. Contains the number of valid data entries in the receive FIFO. Reset value: 0x0"]
    pub const fn rxflr(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Receive FIFO Level. Contains the number of valid data entries in the receive FIFO. Reset value: 0x0"]
    pub fn set_rxflr(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for IcRxflr {
    fn default() -> IcRxflr {
        IcRxflr(0)
    }
}
