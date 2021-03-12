use crate::generic::*;
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxFull(u8);
impl IcIntrStatRRxFull {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRxFull {
        IcIntrStatRRxFull(val)
    }
    #[doc = "R_RX_FULL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_FULL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdCmd(u8);
impl IcDataCmdCmd {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDataCmdCmd {
        IcDataCmdCmd(val)
    }
    #[doc = "Master Write Command"]
    pub const WRITE: Self = Self(0);
    #[doc = "Master Read Command"]
    pub const READ: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMStopDet(u8);
impl IcIntrMaskMStopDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMStopDet {
        IcIntrMaskMStopDet(val)
    }
    #[doc = "STOP_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "STOP_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaCrRdmae(u8);
impl IcDmaCrRdmae {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDmaCrRdmae {
        IcDmaCrRdmae(val)
    }
    #[doc = "Receive FIFO DMA channel disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Receive FIFO DMA channel enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdRestart(u8);
impl IcDataCmdRestart {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDataCmdRestart {
        IcDataCmdRestart(val)
    }
    #[doc = "Don't Issue RESTART before this command"]
    pub const DISABLE: Self = Self(0);
    #[doc = "Issue RESTART before this command"]
    pub const ENABLE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSbyteNorstrt(u8);
impl IcTxAbrtSourceAbrtSbyteNorstrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtSbyteNorstrt {
        IcTxAbrtSourceAbrtSbyteNorstrt(val)
    }
    #[doc = "User trying to send START byte when RESTART disabled- scenario not present"]
    pub const ABRT_SBYTE_NORSTRT_VOID: Self = Self(0);
    #[doc = "User trying to send START byte when RESTART disabled"]
    pub const ABRT_SBYTE_NORSTRT_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxOver(u8);
impl IcIntrStatRRxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRxOver {
        IcIntrStatRRxOver(val)
    }
    #[doc = "R_RX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRdReq(u8);
impl IcIntrMaskMRdReq {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRdReq {
        IcIntrMaskMRdReq(val)
    }
    #[doc = "RD_REQ interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RD_REQ interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRActivity(u8);
impl IcIntrStatRActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRActivity {
        IcIntrStatRActivity(val)
    }
    #[doc = "R_ACTIVITY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_ACTIVITY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMStartDet(u8);
impl IcIntrMaskMStartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMStartDet {
        IcIntrMaskMStartDet(val)
    }
    #[doc = "START_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "START_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxOver(u8);
impl IcRawIntrStatTxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatTxOver {
        IcRawIntrStatTxOver(val)
    }
    #[doc = "TX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvrdIntx(u8);
impl IcTxAbrtSourceAbrtSlvrdIntx {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtSlvrdIntx {
        IcTxAbrtSourceAbrtSlvrdIntx(val)
    }
    #[doc = "Slave trying to transmit to remote master in read mode- scenario not present"]
    pub const ABRT_SLVRD_INTX_VOID: Self = Self(0);
    #[doc = "Slave trying to transmit to remote master in read mode"]
    pub const ABRT_SLVRD_INTX_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtTxdataNoack(u8);
impl IcTxAbrtSourceAbrtTxdataNoack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtTxdataNoack {
        IcTxAbrtSourceAbrtTxdataNoack(val)
    }
    #[doc = "Transmitted data non-ACKed by addressed slave-scenario not present"]
    pub const ABRT_TXDATA_NOACK_VOID: Self = Self(0);
    #[doc = "Transmitted data not ACKed by addressed slave"]
    pub const ABRT_TXDATA_NOACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10addr2Noack(u8);
impl IcTxAbrtSourceAbrt10addr2Noack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrt10addr2Noack {
        IcTxAbrtSourceAbrt10addr2Noack(val)
    }
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Byte 2 of 10Bit Address not ACKed by any slave"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "Generate Slave Data NACK Register The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect. A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0]
= 0) - Slave part is inactive (IC_STATUS[6]
= 0) Note: The IC_STATUS[6]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSlvDataNackOnlyNack(u8);
impl IcSlvDataNackOnlyNack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcSlvDataNackOnlyNack {
        IcSlvDataNackOnlyNack(val)
    }
    #[doc = "Slave receiver generates NACK normally"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Slave receiver generates NACK upon data reception only"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatStartDet(u8);
impl IcRawIntrStatStartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatStartDet {
        IcRawIntrStatStartDet(val)
    }
    #[doc = "START_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "START_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConTxEmptyCtrl(u8);
impl IcConTxEmptyCtrl {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConTxEmptyCtrl {
        IcConTxEmptyCtrl(val)
    }
    #[doc = "Default behaviour of TX_EMPTY interrupt"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Controlled generation of TX_EMPTY interrupt"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaCrTdmae(u8);
impl IcDmaCrTdmae {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDmaCrTdmae {
        IcDmaCrTdmae(val)
    }
    #[doc = "transmit FIFO DMA channel disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Transmit FIFO DMA channel enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusActivity(u8);
impl IcStatusActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusActivity {
        IcStatusActivity(val)
    }
    #[doc = "I2C is idle"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "I2C is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMGenCall(u8);
impl IcIntrMaskMGenCall {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMGenCall {
        IcIntrMaskMGenCall(val)
    }
    #[doc = "GEN_CALL interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "GEN_CALL interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdStop(u8);
impl IcDataCmdStop {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDataCmdStop {
        IcDataCmdStop(val)
    }
    #[doc = "Don't Issue STOP after this command"]
    pub const DISABLE: Self = Self(0);
    #[doc = "Issue STOP after this command"]
    pub const ENABLE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusRff(u8);
impl IcStatusRff {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusRff {
        IcStatusRff(val)
    }
    #[doc = "Rx FIFO not full"]
    pub const NOT_FULL: Self = Self(0);
    #[doc = "Rx FIFO is full"]
    pub const FULL: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtHsAckdet(u8);
impl IcTxAbrtSourceAbrtHsAckdet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtHsAckdet {
        IcTxAbrtSourceAbrtHsAckdet(val)
    }
    #[doc = "HS Master code ACKed in HS Mode- scenario not present"]
    pub const ABRT_HS_ACK_VOID: Self = Self(0);
    #[doc = "HS Master code ACKed in HS Mode"]
    pub const ABRT_HS_ACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatGenCall(u8);
impl IcRawIntrStatGenCall {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatGenCall {
        IcRawIntrStatGenCall(val)
    }
    #[doc = "GEN_CALL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "GEN_CALL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIcSlaveDisable(u8);
impl IcConIcSlaveDisable {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConIcSlaveDisable {
        IcConIcSlaveDisable(val)
    }
    #[doc = "Slave mode is enabled"]
    pub const SLAVE_ENABLED: Self = Self(0);
    #[doc = "Slave mode is disabled"]
    pub const SLAVE_DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRdReq(u8);
impl IcRawIntrStatRdReq {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRdReq {
        IcRawIntrStatRdReq(val)
    }
    #[doc = "RD_REQ interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RD_REQ interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusMstActivity(u8);
impl IcStatusMstActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusMstActivity {
        IcStatusMstActivity(val)
    }
    #[doc = "Master is idle"]
    pub const IDLE: Self = Self(0);
    #[doc = "Master not idle"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConStopDetIfaddressed(u8);
impl IcConStopDetIfaddressed {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConStopDetIfaddressed {
        IcConStopDetIfaddressed(val)
    }
    #[doc = "slave issues STOP_DET intr always"]
    pub const DISABLED: Self = Self(0);
    #[doc = "slave issues STOP_DET intr only if addressed"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxDone(u8);
impl IcIntrStatRRxDone {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRxDone {
        IcIntrStatRRxDone(val)
    }
    #[doc = "R_RX_DONE interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_DONE interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxOver(u8);
impl IcIntrMaskMTxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMTxOver {
        IcIntrMaskMTxOver(val)
    }
    #[doc = "TX_OVER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_OVER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRestartDet(u8);
impl IcIntrMaskMRestartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRestartDet {
        IcIntrMaskMRestartDet(val)
    }
    #[doc = "RESTART_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RESTART_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtMasterDis(u8);
impl IcTxAbrtSourceAbrtMasterDis {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtMasterDis {
        IcTxAbrtSourceAbrtMasterDis(val)
    }
    #[doc = "User initiating master operation when MASTER disabled- scenario not present"]
    pub const ABRT_MASTER_DIS_VOID: Self = Self(0);
    #[doc = "User initiating master operation when MASTER disabled"]
    pub const ABRT_MASTER_DIS_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt7bAddrNoack(u8);
impl IcTxAbrtSourceAbrt7bAddrNoack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrt7bAddrNoack {
        IcTxAbrtSourceAbrt7bAddrNoack(val)
    }
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "This abort is generated because of NOACK for 7-bit address"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTarGcOrStart(u8);
impl IcTarGcOrStart {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTarGcOrStart {
        IcTarGcOrStart(val)
    }
    #[doc = "GENERAL_CALL byte transmission"]
    pub const GENERAL_CALL: Self = Self(0);
    #[doc = "START byte transmission"]
    pub const START_BYTE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRGenCall(u8);
impl IcIntrStatRGenCall {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRGenCall {
        IcIntrStatRGenCall(val)
    }
    #[doc = "R_GEN_CALL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_GEN_CALL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxAbrt(u8);
impl IcIntrStatRTxAbrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRTxAbrt {
        IcIntrStatRTxAbrt(val)
    }
    #[doc = "R_TX_ABRT interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_ABRT interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxFull(u8);
impl IcRawIntrStatRxFull {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRxFull {
        IcRawIntrStatRxFull(val)
    }
    #[doc = "RX_FULL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_FULL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxEmpty(u8);
impl IcIntrMaskMTxEmpty {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMTxEmpty {
        IcIntrMaskMTxEmpty(val)
    }
    #[doc = "TX_EMPTY interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_EMPTY interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTarSpecial(u8);
impl IcTarSpecial {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTarSpecial {
        IcTarSpecial(val)
    }
    #[doc = "Disables programming of GENERAL_CALL or START_BYTE transmission"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enables programming of GENERAL_CALL or START_BYTE transmission"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRestartDet(u8);
impl IcIntrStatRRestartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRestartDet {
        IcIntrStatRRestartDet(val)
    }
    #[doc = "R_RESTART_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RESTART_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRStartDet(u8);
impl IcIntrStatRStartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRStartDet {
        IcIntrStatRStartDet(val)
    }
    #[doc = "R_START_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_START_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRStopDet(u8);
impl IcIntrStatRStopDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRStopDet {
        IcIntrStatRStopDet(val)
    }
    #[doc = "R_STOP_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_STOP_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxEmpty(u8);
impl IcIntrStatRTxEmpty {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRTxEmpty {
        IcIntrStatRTxEmpty(val)
    }
    #[doc = "R_TX_EMPTY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_EMPTY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxDone(u8);
impl IcIntrMaskMRxDone {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRxDone {
        IcIntrMaskMRxDone(val)
    }
    #[doc = "RX_DONE interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_DONE interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRestartDet(u8);
impl IcRawIntrStatRestartDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRestartDet {
        IcRawIntrStatRestartDet(val)
    }
    #[doc = "RESTART_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RESTART_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableTxCmdBlock(u8);
impl IcEnableTxCmdBlock {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableTxCmdBlock {
        IcEnableTxCmdBlock(val)
    }
    #[doc = "Tx Command execution not blocked"]
    pub const NOT_BLOCKED: Self = Self(0);
    #[doc = "Tx Command execution blocked"]
    pub const BLOCKED: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableAbort(u8);
impl IcEnableAbort {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableAbort {
        IcEnableAbort(val)
    }
    #[doc = "ABORT operation not in progress"]
    pub const DISABLE: Self = Self(0);
    #[doc = "ABORT operation in progress"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableEnable(u8);
impl IcEnableEnable {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableEnable {
        IcEnableEnable(val)
    }
    #[doc = "I2C is disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "I2C is enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceArbLost(u8);
impl IcTxAbrtSourceArbLost {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceArbLost {
        IcTxAbrtSourceArbLost(val)
    }
    #[doc = "Master or Slave-Transmitter lost arbitration- scenario not present"]
    pub const ABRT_LOST_VOID: Self = Self(0);
    #[doc = "Master or Slave-Transmitter lost arbitration"]
    pub const ABRT_LOST_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10addr1Noack(u8);
impl IcTxAbrtSourceAbrt10addr1Noack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrt10addr1Noack {
        IcTxAbrtSourceAbrt10addr1Noack(val)
    }
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Byte 1 of 10Bit Address not ACKed by any slave"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvflushTxfifo(u8);
impl IcTxAbrtSourceAbrtSlvflushTxfifo {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtSlvflushTxfifo {
        IcTxAbrtSourceAbrtSlvflushTxfifo(val)
    }
    #[doc = "Slave flushes existing data in TX-FIFO upon getting read command- scenario not present"]
    pub const ABRT_SLVFLUSH_TXFIFO_VOID: Self = Self(0);
    #[doc = "Slave flushes existing data in TX-FIFO upon getting read command"]
    pub const ABRT_SLVFLUSH_TXFIFO_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConSpeed(u8);
impl IcConSpeed {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConSpeed {
        IcConSpeed(val)
    }
    #[doc = "Standard Speed mode of operation"]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Fast or Fast Plus mode of operation"]
    pub const FAST: Self = Self(0x02);
    #[doc = "High Speed mode of operation"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxUnder(u8);
impl IcIntrMaskMRxUnder {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRxUnder {
        IcIntrMaskMRxUnder(val)
    }
    #[doc = "RX_UNDER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_UNDER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxUnder(u8);
impl IcRawIntrStatRxUnder {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRxUnder {
        IcRawIntrStatRxUnder(val)
    }
    #[doc = "RX_UNDER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_UNDER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusSlvRxDataLost(u8);
impl IcEnableStatusSlvRxDataLost {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableStatusSlvRxDataLost {
        IcEnableStatusSlvRxDataLost(val)
    }
    #[doc = "Slave RX Data is not lost"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Slave RX Data is lost"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConRxFifoFullHldCtrl(u8);
impl IcConRxFifoFullHldCtrl {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConRxFifoFullHldCtrl {
        IcConRxFifoFullHldCtrl(val)
    }
    #[doc = "Overflow when RX_FIFO is full"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Hold bus when RX_FIFO is full"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxUnder(u8);
impl IcIntrStatRRxUnder {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRxUnder {
        IcIntrStatRRxUnder(val)
    }
    #[doc = "RX_UNDER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_UNDER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10bRdNorstrt(u8);
impl IcTxAbrtSourceAbrt10bRdNorstrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrt10bRdNorstrt {
        IcTxAbrtSourceAbrt10bRdNorstrt(val)
    }
    #[doc = "Master not trying to read in 10Bit addressing mode when RESTART disabled"]
    pub const ABRT_10B_RD_VOID: Self = Self(0);
    #[doc = "Master trying to read in 10Bit addressing mode when RESTART disabled"]
    pub const ABRT_10B_RD_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvArblost(u8);
impl IcTxAbrtSourceAbrtSlvArblost {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtSlvArblost {
        IcTxAbrtSourceAbrtSlvArblost(val)
    }
    #[doc = "Slave lost arbitration to remote master- scenario not present"]
    pub const ABRT_SLV_ARBLOST_VOID: Self = Self(0);
    #[doc = "Slave lost arbitration to remote master"]
    pub const ABRT_SLV_ARBLOST_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxOver(u8);
impl IcIntrStatRTxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRTxOver {
        IcIntrStatRTxOver(val)
    }
    #[doc = "R_TX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIcRestartEn(u8);
impl IcConIcRestartEn {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConIcRestartEn {
        IcConIcRestartEn(val)
    }
    #[doc = "Master restart disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Master restart enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxOver(u8);
impl IcRawIntrStatRxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRxOver {
        IcRawIntrStatRxOver(val)
    }
    #[doc = "RX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtHsNorstrt(u8);
impl IcTxAbrtSourceAbrtHsNorstrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtHsNorstrt {
        IcTxAbrtSourceAbrtHsNorstrt(val)
    }
    #[doc = "User trying to switch Master to HS mode when RESTART disabled- scenario not present"]
    pub const ABRT_HS_NORSTRT_VOID: Self = Self(0);
    #[doc = "User trying to switch Master to HS mode when RESTART disabled"]
    pub const ABRT_HS_NORSTRT_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusTfe(u8);
impl IcStatusTfe {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusTfe {
        IcStatusTfe(val)
    }
    #[doc = "Tx FIFO not empty"]
    pub const NON_EMPTY: Self = Self(0);
    #[doc = "Tx FIFO is empty"]
    pub const EMPTY: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusTfnf(u8);
impl IcStatusTfnf {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusTfnf {
        IcStatusTfnf(val)
    }
    #[doc = "Tx FIFO is full"]
    pub const FULL: Self = Self(0);
    #[doc = "Tx FIFO not full"]
    pub const NOT_FULL: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusRfne(u8);
impl IcStatusRfne {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusRfne {
        IcStatusRfne(val)
    }
    #[doc = "Rx FIFO is empty"]
    pub const EMPTY: Self = Self(0);
    #[doc = "Rx FIFO not empty"]
    pub const NOT_EMPTY: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdFirstDataByte(u8);
impl IcDataCmdFirstDataByte {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcDataCmdFirstDataByte {
        IcDataCmdFirstDataByte(val)
    }
    #[doc = "Sequential data byte received"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Non sequential data byte received"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxFull(u8);
impl IcIntrMaskMRxFull {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRxFull {
        IcIntrMaskMRxFull(val)
    }
    #[doc = "RX_FULL interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_FULL interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C ACK General Call Register The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address. This register is applicable only when the DW_apb_i2c is in slave mode."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcAckGeneralCallAckGenCall(u8);
impl IcAckGeneralCallAckGenCall {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcAckGeneralCallAckGenCall {
        IcAckGeneralCallAckGenCall(val)
    }
    #[doc = "Generate NACK for a General Call"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Generate ACK for a General Call"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConMasterMode(u8);
impl IcConMasterMode {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConMasterMode {
        IcConMasterMode(val)
    }
    #[doc = "Master mode is disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Master mode is enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMActivity(u8);
impl IcIntrMaskMActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMActivity {
        IcIntrMaskMActivity(val)
    }
    #[doc = "ACTIVITY interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "ACTIVITY interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxAbrt(u8);
impl IcIntrMaskMTxAbrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMTxAbrt {
        IcIntrMaskMTxAbrt(val)
    }
    #[doc = "TX_ABORT interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_ABORT interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxOver(u8);
impl IcIntrMaskMRxOver {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMRxOver {
        IcIntrMaskMRxOver(val)
    }
    #[doc = "RX_OVER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_OVER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatStopDet(u8);
impl IcRawIntrStatStopDet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatStopDet {
        IcRawIntrStatStopDet(val)
    }
    #[doc = "STOP_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "STOP_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIc10bitaddrMaster(u8);
impl IcConIc10bitaddrMaster {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConIc10bitaddrMaster {
        IcConIc10bitaddrMaster(val)
    }
    #[doc = "Master 7Bit addressing mode"]
    pub const ADDR_7BITS: Self = Self(0);
    #[doc = "Master 10Bit addressing mode"]
    pub const ADDR_10BITS: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSbyteAckdet(u8);
impl IcTxAbrtSourceAbrtSbyteAckdet {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtSbyteAckdet {
        IcTxAbrtSourceAbrtSbyteAckdet(val)
    }
    #[doc = "ACK detected for START byte- scenario not present"]
    pub const ABRT_SBYTE_ACKDET_VOID: Self = Self(0);
    #[doc = "ACK detected for START byte"]
    pub const ABRT_SBYTE_ACKDET_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIc10bitaddrSlave(u8);
impl IcConIc10bitaddrSlave {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcConIc10bitaddrSlave {
        IcConIc10bitaddrSlave(val)
    }
    #[doc = "Slave 7Bit addressing"]
    pub const ADDR_7BITS: Self = Self(0);
    #[doc = "Slave 10Bit addressing"]
    pub const ADDR_10BITS: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxAbrt(u8);
impl IcRawIntrStatTxAbrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatTxAbrt {
        IcRawIntrStatTxAbrt(val)
    }
    #[doc = "TX_ABRT interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_ABRT interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusIcEn(u8);
impl IcEnableStatusIcEn {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableStatusIcEn {
        IcEnableStatusIcEn(val)
    }
    #[doc = "I2C disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "I2C enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtGcallNoack(u8);
impl IcTxAbrtSourceAbrtGcallNoack {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtGcallNoack {
        IcTxAbrtSourceAbrtGcallNoack(val)
    }
    #[doc = "GCALL not ACKed by any slave-scenario not present"]
    pub const ABRT_GCALL_NOACK_VOID: Self = Self(0);
    #[doc = "GCALL not ACKed by any slave"]
    pub const ABRT_GCALL_NOACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatMasterOnHold(u8);
impl IcRawIntrStatMasterOnHold {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatMasterOnHold {
        IcRawIntrStatMasterOnHold(val)
    }
    #[doc = "MASTER_ON_HOLD interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "MASTER_ON_HOLD interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxDone(u8);
impl IcRawIntrStatRxDone {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatRxDone {
        IcRawIntrStatRxDone(val)
    }
    #[doc = "RX_DONE interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_DONE interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtUserAbrt(u8);
impl IcTxAbrtSourceAbrtUserAbrt {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtUserAbrt {
        IcTxAbrtSourceAbrtUserAbrt(val)
    }
    #[doc = "Transfer abort detected by master- scenario not present"]
    pub const ABRT_USER_ABRT_VOID: Self = Self(0);
    #[doc = "Transfer abort detected by master"]
    pub const ABRT_USER_ABRT_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxEmpty(u8);
impl IcRawIntrStatTxEmpty {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatTxEmpty {
        IcRawIntrStatTxEmpty(val)
    }
    #[doc = "TX_EMPTY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_EMPTY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusSlvDisabledWhileBusy(u8);
impl IcEnableStatusSlvDisabledWhileBusy {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcEnableStatusSlvDisabledWhileBusy {
        IcEnableStatusSlvDisabledWhileBusy(val)
    }
    #[doc = "Slave is disabled when it is idle"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Slave is disabled when it is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtGcallRead(u8);
impl IcTxAbrtSourceAbrtGcallRead {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcTxAbrtSourceAbrtGcallRead {
        IcTxAbrtSourceAbrtGcallRead(val)
    }
    #[doc = "GCALL is followed by read from bus-scenario not present"]
    pub const ABRT_GCALL_READ_VOID: Self = Self(0);
    #[doc = "GCALL is followed by read from bus"]
    pub const ABRT_GCALL_READ_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRMasterOnHold(u8);
impl IcIntrStatRMasterOnHold {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRMasterOnHold {
        IcIntrStatRMasterOnHold(val)
    }
    #[doc = "R_MASTER_ON_HOLD interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_MASTER_ON_HOLD interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatActivity(u8);
impl IcRawIntrStatActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcRawIntrStatActivity {
        IcRawIntrStatActivity(val)
    }
    #[doc = "RAW_INTR_ACTIVITY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RAW_INTR_ACTIVITY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusSlvActivity(u8);
impl IcStatusSlvActivity {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcStatusSlvActivity {
        IcStatusSlvActivity(val)
    }
    #[doc = "Slave is idle"]
    pub const IDLE: Self = Self(0);
    #[doc = "Slave not idle"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRdReq(u8);
impl IcIntrStatRRdReq {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrStatRRdReq {
        IcIntrStatRRdReq(val)
    }
    #[doc = "R_RD_REQ interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RD_REQ interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMMasterOnHoldReadOnly(u8);
impl IcIntrMaskMMasterOnHoldReadOnly {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> IcIntrMaskMMasterOnHoldReadOnly {
        IcIntrMaskMMasterOnHoldReadOnly(val)
    }
    #[doc = "MASTER_ON_HOLD interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "MASTER_ON_HOLD interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
