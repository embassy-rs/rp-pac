use crate::generic::*;
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIcRestartEn(pub u8);
impl IcConIcRestartEn {
    #[doc = "Master restart disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Master restart enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdStop(pub u8);
impl IcDataCmdStop {
    #[doc = "Don't Issue STOP after this command"]
    pub const DISABLE: Self = Self(0);
    #[doc = "Issue STOP after this command"]
    pub const ENABLE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxFull(pub u8);
impl IcIntrStatRRxFull {
    #[doc = "R_RX_FULL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_FULL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxOver(pub u8);
impl IcIntrMaskMTxOver {
    #[doc = "TX_OVER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_OVER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxUnder(pub u8);
impl IcIntrMaskMRxUnder {
    #[doc = "RX_UNDER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_UNDER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvArblost(pub u8);
impl IcTxAbrtSourceAbrtSlvArblost {
    #[doc = "Slave lost arbitration to remote master- scenario not present"]
    pub const ABRT_SLV_ARBLOST_VOID: Self = Self(0);
    #[doc = "Slave lost arbitration to remote master"]
    pub const ABRT_SLV_ARBLOST_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtGcallRead(pub u8);
impl IcTxAbrtSourceAbrtGcallRead {
    #[doc = "GCALL is followed by read from bus-scenario not present"]
    pub const ABRT_GCALL_READ_VOID: Self = Self(0);
    #[doc = "GCALL is followed by read from bus"]
    pub const ABRT_GCALL_READ_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdFirstDataByte(pub u8);
impl IcDataCmdFirstDataByte {
    #[doc = "Sequential data byte received"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Non sequential data byte received"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRStartDet(pub u8);
impl IcIntrStatRStartDet {
    #[doc = "R_START_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_START_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxOver(pub u8);
impl IcIntrStatRTxOver {
    #[doc = "R_TX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxOver(pub u8);
impl IcRawIntrStatRxOver {
    #[doc = "RX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtUserAbrt(pub u8);
impl IcTxAbrtSourceAbrtUserAbrt {
    #[doc = "Transfer abort detected by master- scenario not present"]
    pub const ABRT_USER_ABRT_VOID: Self = Self(0);
    #[doc = "Transfer abort detected by master"]
    pub const ABRT_USER_ABRT_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConMasterMode(pub u8);
impl IcConMasterMode {
    #[doc = "Master mode is disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Master mode is enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxAbrt(pub u8);
impl IcIntrMaskMTxAbrt {
    #[doc = "TX_ABORT interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_ABORT interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMTxEmpty(pub u8);
impl IcIntrMaskMTxEmpty {
    #[doc = "TX_EMPTY interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "TX_EMPTY interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSbyteAckdet(pub u8);
impl IcTxAbrtSourceAbrtSbyteAckdet {
    #[doc = "ACK detected for START byte- scenario not present"]
    pub const ABRT_SBYTE_ACKDET_VOID: Self = Self(0);
    #[doc = "ACK detected for START byte"]
    pub const ABRT_SBYTE_ACKDET_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMMasterOnHoldReadOnly(pub u8);
impl IcIntrMaskMMasterOnHoldReadOnly {
    #[doc = "MASTER_ON_HOLD interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "MASTER_ON_HOLD interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtHsNorstrt(pub u8);
impl IcTxAbrtSourceAbrtHsNorstrt {
    #[doc = "User trying to switch Master to HS mode when RESTART disabled- scenario not present"]
    pub const ABRT_HS_NORSTRT_VOID: Self = Self(0);
    #[doc = "User trying to switch Master to HS mode when RESTART disabled"]
    pub const ABRT_HS_NORSTRT_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxUnder(pub u8);
impl IcIntrStatRRxUnder {
    #[doc = "RX_UNDER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_UNDER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConSpeed(pub u8);
impl IcConSpeed {
    #[doc = "Standard Speed mode of operation"]
    pub const STANDARD: Self = Self(0x01);
    #[doc = "Fast or Fast Plus mode of operation"]
    pub const FAST: Self = Self(0x02);
    #[doc = "High Speed mode of operation"]
    pub const HIGH: Self = Self(0x03);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxAbrt(pub u8);
impl IcRawIntrStatTxAbrt {
    #[doc = "TX_ABRT interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_ABRT interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatStopDet(pub u8);
impl IcRawIntrStatStopDet {
    #[doc = "STOP_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "STOP_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRestartDet(pub u8);
impl IcIntrMaskMRestartDet {
    #[doc = "RESTART_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RESTART_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdRestart(pub u8);
impl IcDataCmdRestart {
    #[doc = "Don't Issue RESTART before this command"]
    pub const DISABLE: Self = Self(0);
    #[doc = "Issue RESTART before this command"]
    pub const ENABLE: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusIcEn(pub u8);
impl IcEnableStatusIcEn {
    #[doc = "I2C disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "I2C enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusRff(pub u8);
impl IcStatusRff {
    #[doc = "Rx FIFO not full"]
    pub const NOT_FULL: Self = Self(0);
    #[doc = "Rx FIFO is full"]
    pub const FULL: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10addr2Noack(pub u8);
impl IcTxAbrtSourceAbrt10addr2Noack {
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Byte 2 of 10Bit Address not ACKed by any slave"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt7bAddrNoack(pub u8);
impl IcTxAbrtSourceAbrt7bAddrNoack {
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "This abort is generated because of NOACK for 7-bit address"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatStartDet(pub u8);
impl IcRawIntrStatStartDet {
    #[doc = "START_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "START_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatActivity(pub u8);
impl IcRawIntrStatActivity {
    #[doc = "RAW_INTR_ACTIVITY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RAW_INTR_ACTIVITY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIcSlaveDisable(pub u8);
impl IcConIcSlaveDisable {
    #[doc = "Slave mode is enabled"]
    pub const SLAVE_ENABLED: Self = Self(0);
    #[doc = "Slave mode is disabled"]
    pub const SLAVE_DISABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRdReq(pub u8);
impl IcRawIntrStatRdReq {
    #[doc = "RD_REQ interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RD_REQ interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceArbLost(pub u8);
impl IcTxAbrtSourceArbLost {
    #[doc = "Master or Slave-Transmitter lost arbitration- scenario not present"]
    pub const ABRT_LOST_VOID: Self = Self(0);
    #[doc = "Master or Slave-Transmitter lost arbitration"]
    pub const ABRT_LOST_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtTxdataNoack(pub u8);
impl IcTxAbrtSourceAbrtTxdataNoack {
    #[doc = "Transmitted data non-ACKed by addressed slave-scenario not present"]
    pub const ABRT_TXDATA_NOACK_VOID: Self = Self(0);
    #[doc = "Transmitted data not ACKed by addressed slave"]
    pub const ABRT_TXDATA_NOACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusSlvRxDataLost(pub u8);
impl IcEnableStatusSlvRxDataLost {
    #[doc = "Slave RX Data is not lost"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Slave RX Data is lost"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableTxCmdBlock(pub u8);
impl IcEnableTxCmdBlock {
    #[doc = "Tx Command execution not blocked"]
    pub const NOT_BLOCKED: Self = Self(0);
    #[doc = "Tx Command execution blocked"]
    pub const BLOCKED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRdReq(pub u8);
impl IcIntrStatRRdReq {
    #[doc = "R_RD_REQ interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RD_REQ interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRestartDet(pub u8);
impl IcIntrStatRRestartDet {
    #[doc = "R_RESTART_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RESTART_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C ACK General Call Register The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address. This register is applicable only when the DW_apb_i2c is in slave mode."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcAckGeneralCallAckGenCall(pub u8);
impl IcAckGeneralCallAckGenCall {
    #[doc = "Generate NACK for a General Call"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Generate ACK for a General Call"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMStopDet(pub u8);
impl IcIntrMaskMStopDet {
    #[doc = "STOP_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "STOP_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConRxFifoFullHldCtrl(pub u8);
impl IcConRxFifoFullHldCtrl {
    #[doc = "Overflow when RX_FIFO is full"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Hold bus when RX_FIFO is full"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMStartDet(pub u8);
impl IcIntrMaskMStartDet {
    #[doc = "START_DET interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "START_DET interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableAbort(pub u8);
impl IcEnableAbort {
    #[doc = "ABORT operation not in progress"]
    pub const DISABLE: Self = Self(0);
    #[doc = "ABORT operation in progress"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxFull(pub u8);
impl IcIntrMaskMRxFull {
    #[doc = "RX_FULL interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_FULL interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxOver(pub u8);
impl IcIntrStatRRxOver {
    #[doc = "R_RX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxOver(pub u8);
impl IcRawIntrStatTxOver {
    #[doc = "TX_OVER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_OVER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMGenCall(pub u8);
impl IcIntrMaskMGenCall {
    #[doc = "GEN_CALL interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "GEN_CALL interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtMasterDis(pub u8);
impl IcTxAbrtSourceAbrtMasterDis {
    #[doc = "User initiating master operation when MASTER disabled- scenario not present"]
    pub const ABRT_MASTER_DIS_VOID: Self = Self(0);
    #[doc = "User initiating master operation when MASTER disabled"]
    pub const ABRT_MASTER_DIS_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusSlvActivity(pub u8);
impl IcStatusSlvActivity {
    #[doc = "Slave is idle"]
    pub const IDLE: Self = Self(0);
    #[doc = "Slave not idle"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10bRdNorstrt(pub u8);
impl IcTxAbrtSourceAbrt10bRdNorstrt {
    #[doc = "Master not trying to read in 10Bit addressing mode when RESTART disabled"]
    pub const ABRT_10B_RD_VOID: Self = Self(0);
    #[doc = "Master trying to read in 10Bit addressing mode when RESTART disabled"]
    pub const ABRT_10B_RD_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxDone(pub u8);
impl IcIntrMaskMRxDone {
    #[doc = "RX_DONE interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_DONE interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtGcallNoack(pub u8);
impl IcTxAbrtSourceAbrtGcallNoack {
    #[doc = "GCALL not ACKed by any slave-scenario not present"]
    pub const ABRT_GCALL_NOACK_VOID: Self = Self(0);
    #[doc = "GCALL not ACKed by any slave"]
    pub const ABRT_GCALL_NOACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatMasterOnHold(pub u8);
impl IcRawIntrStatMasterOnHold {
    #[doc = "MASTER_ON_HOLD interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "MASTER_ON_HOLD interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDataCmdCmd(pub u8);
impl IcDataCmdCmd {
    #[doc = "Master Write Command"]
    pub const WRITE: Self = Self(0);
    #[doc = "Master Read Command"]
    pub const READ: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRMasterOnHold(pub u8);
impl IcIntrStatRMasterOnHold {
    #[doc = "R_MASTER_ON_HOLD interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_MASTER_ON_HOLD interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvrdIntx(pub u8);
impl IcTxAbrtSourceAbrtSlvrdIntx {
    #[doc = "Slave trying to transmit to remote master in read mode- scenario not present"]
    pub const ABRT_SLVRD_INTX_VOID: Self = Self(0);
    #[doc = "Slave trying to transmit to remote master in read mode"]
    pub const ABRT_SLVRD_INTX_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRxOver(pub u8);
impl IcIntrMaskMRxOver {
    #[doc = "RX_OVER interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RX_OVER interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRRxDone(pub u8);
impl IcIntrStatRRxDone {
    #[doc = "R_RX_DONE interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_RX_DONE interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrt10addr1Noack(pub u8);
impl IcTxAbrtSourceAbrt10addr1Noack {
    #[doc = "This abort is not generated"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Byte 1 of 10Bit Address not ACKed by any slave"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSlvflushTxfifo(pub u8);
impl IcTxAbrtSourceAbrtSlvflushTxfifo {
    #[doc = "Slave flushes existing data in TX-FIFO upon getting read command- scenario not present"]
    pub const ABRT_SLVFLUSH_TXFIFO_VOID: Self = Self(0);
    #[doc = "Slave flushes existing data in TX-FIFO upon getting read command"]
    pub const ABRT_SLVFLUSH_TXFIFO_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxDone(pub u8);
impl IcRawIntrStatRxDone {
    #[doc = "RX_DONE interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_DONE interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxEmpty(pub u8);
impl IcIntrStatRTxEmpty {
    #[doc = "R_TX_EMPTY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_EMPTY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRGenCall(pub u8);
impl IcIntrStatRGenCall {
    #[doc = "R_GEN_CALL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_GEN_CALL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConStopDetIfaddressed(pub u8);
impl IcConStopDetIfaddressed {
    #[doc = "slave issues STOP_DET intr always"]
    pub const DISABLED: Self = Self(0);
    #[doc = "slave issues STOP_DET intr only if addressed"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIc10bitaddrSlave(pub u8);
impl IcConIc10bitaddrSlave {
    #[doc = "Slave 7Bit addressing"]
    pub const ADDR_7BITS: Self = Self(0);
    #[doc = "Slave 10Bit addressing"]
    pub const ADDR_10BITS: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRestartDet(pub u8);
impl IcRawIntrStatRestartDet {
    #[doc = "RESTART_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RESTART_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConTxEmptyCtrl(pub u8);
impl IcConTxEmptyCtrl {
    #[doc = "Default behaviour of TX_EMPTY interrupt"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Controlled generation of TX_EMPTY interrupt"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusTfnf(pub u8);
impl IcStatusTfnf {
    #[doc = "Tx FIFO is full"]
    pub const FULL: Self = Self(0);
    #[doc = "Tx FIFO not full"]
    pub const NOT_FULL: Self = Self(0x01);
}
#[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTarSpecial(pub u8);
impl IcTarSpecial {
    #[doc = "Disables programming of GENERAL_CALL or START_BYTE transmission"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Enables programming of GENERAL_CALL or START_BYTE transmission"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTarGcOrStart(pub u8);
impl IcTarGcOrStart {
    #[doc = "GENERAL_CALL byte transmission"]
    pub const GENERAL_CALL: Self = Self(0);
    #[doc = "START byte transmission"]
    pub const START_BYTE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusMstActivity(pub u8);
impl IcStatusMstActivity {
    #[doc = "Master is idle"]
    pub const IDLE: Self = Self(0);
    #[doc = "Master not idle"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRStopDet(pub u8);
impl IcIntrStatRStopDet {
    #[doc = "R_STOP_DET interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_STOP_DET interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusRfne(pub u8);
impl IcStatusRfne {
    #[doc = "Rx FIFO is empty"]
    pub const EMPTY: Self = Self(0);
    #[doc = "Rx FIFO not empty"]
    pub const NOT_EMPTY: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxUnder(pub u8);
impl IcRawIntrStatRxUnder {
    #[doc = "RX_UNDER interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_UNDER interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtHsAckdet(pub u8);
impl IcTxAbrtSourceAbrtHsAckdet {
    #[doc = "HS Master code ACKed in HS Mode- scenario not present"]
    pub const ABRT_HS_ACK_VOID: Self = Self(0);
    #[doc = "HS Master code ACKed in HS Mode"]
    pub const ABRT_HS_ACK_GENERATED: Self = Self(0x01);
}
#[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcTxAbrtSourceAbrtSbyteNorstrt(pub u8);
impl IcTxAbrtSourceAbrtSbyteNorstrt {
    #[doc = "User trying to send START byte when RESTART disabled- scenario not present"]
    pub const ABRT_SBYTE_NORSTRT_VOID: Self = Self(0);
    #[doc = "User trying to send START byte when RESTART disabled"]
    pub const ABRT_SBYTE_NORSTRT_GENERATED: Self = Self(0x01);
}
#[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaCrTdmae(pub u8);
impl IcDmaCrTdmae {
    #[doc = "transmit FIFO DMA channel disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Transmit FIFO DMA channel enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatGenCall(pub u8);
impl IcRawIntrStatGenCall {
    #[doc = "GEN_CALL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "GEN_CALL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMActivity(pub u8);
impl IcIntrMaskMActivity {
    #[doc = "ACTIVITY interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "ACTIVITY interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrMaskMRdReq(pub u8);
impl IcIntrMaskMRdReq {
    #[doc = "RD_REQ interrupt is masked"]
    pub const ENABLED: Self = Self(0);
    #[doc = "RD_REQ interrupt is unmasked"]
    pub const DISABLED: Self = Self(0x01);
}
#[doc = "Generate Slave Data NACK Register The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect. A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0]
= 0) - Slave part is inactive (IC_STATUS[6]
= 0) Note: The IC_STATUS[6]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcSlvDataNackOnlyNack(pub u8);
impl IcSlvDataNackOnlyNack {
    #[doc = "Slave receiver generates NACK normally"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Slave receiver generates NACK upon data reception only"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcDmaCrRdmae(pub u8);
impl IcDmaCrRdmae {
    #[doc = "Receive FIFO DMA channel disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "Receive FIFO DMA channel enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRTxAbrt(pub u8);
impl IcIntrStatRTxAbrt {
    #[doc = "R_TX_ABRT interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_TX_ABRT interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatTxEmpty(pub u8);
impl IcRawIntrStatTxEmpty {
    #[doc = "TX_EMPTY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "TX_EMPTY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusTfe(pub u8);
impl IcStatusTfe {
    #[doc = "Tx FIFO not empty"]
    pub const NON_EMPTY: Self = Self(0);
    #[doc = "Tx FIFO is empty"]
    pub const EMPTY: Self = Self(0x01);
}
#[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcIntrStatRActivity(pub u8);
impl IcIntrStatRActivity {
    #[doc = "R_ACTIVITY interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "R_ACTIVITY interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcRawIntrStatRxFull(pub u8);
impl IcRawIntrStatRxFull {
    #[doc = "RX_FULL interrupt is inactive"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "RX_FULL interrupt is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Enable Register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableEnable(pub u8);
impl IcEnableEnable {
    #[doc = "I2C is disabled"]
    pub const DISABLED: Self = Self(0);
    #[doc = "I2C is enabled"]
    pub const ENABLED: Self = Self(0x01);
}
#[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcEnableStatusSlvDisabledWhileBusy(pub u8);
impl IcEnableStatusSlvDisabledWhileBusy {
    #[doc = "Slave is disabled when it is idle"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "Slave is disabled when it is active"]
    pub const ACTIVE: Self = Self(0x01);
}
#[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcConIc10bitaddrMaster(pub u8);
impl IcConIc10bitaddrMaster {
    #[doc = "Master 7Bit addressing mode"]
    pub const ADDR_7BITS: Self = Self(0);
    #[doc = "Master 10Bit addressing mode"]
    pub const ADDR_10BITS: Self = Self(0x01);
}
#[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IcStatusActivity(pub u8);
impl IcStatusActivity {
    #[doc = "I2C is idle"]
    pub const INACTIVE: Self = Self(0);
    #[doc = "I2C is active"]
    pub const ACTIVE: Self = Self(0x01);
}
