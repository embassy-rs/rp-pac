use crate::generic::*;
#[doc = "DW_apb_i2c address block"]
#[derive(Copy, Clone)]
pub struct I2c(pub *mut u8);
unsafe impl Send for I2c {}
unsafe impl Sync for I2c {}
impl I2c {
    #[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
    pub fn ic_con(self) -> Reg<regs::IcCon, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
    pub fn ic_tar(self) -> Reg<regs::IcTar, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "I2C Slave Address Register"]
    pub fn ic_sar(self) -> Reg<regs::IcSar, RW> {
        unsafe { Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
    pub fn ic_data_cmd(self) -> Reg<regs::IcDataCmd, RW> {
        unsafe { Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Standard Speed I2C Clock SCL High Count Register"]
    pub fn ic_ss_scl_hcnt(self) -> Reg<regs::IcSsSclHcnt, RW> {
        unsafe { Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Standard Speed I2C Clock SCL Low Count Register"]
    pub fn ic_ss_scl_lcnt(self) -> Reg<regs::IcSsSclLcnt, RW> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
    pub fn ic_fs_scl_hcnt(self) -> Reg<regs::IcFsSclHcnt, RW> {
        unsafe { Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
    pub fn ic_fs_scl_lcnt(self) -> Reg<regs::IcFsSclLcnt, RW> {
        unsafe { Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
    pub fn ic_intr_stat(self) -> Reg<regs::IcIntrStat, RW> {
        unsafe { Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
    pub fn ic_intr_mask(self) -> Reg<regs::IcIntrMask, RW> {
        unsafe { Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
    pub fn ic_raw_intr_stat(self) -> Reg<regs::IcRawIntrStat, RW> {
        unsafe { Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "I2C Receive FIFO Threshold Register"]
    pub fn ic_rx_tl(self) -> Reg<regs::IcRxTl, RW> {
        unsafe { Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "I2C Transmit FIFO Threshold Register"]
    pub fn ic_tx_tl(self) -> Reg<regs::IcTxTl, RW> {
        unsafe { Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Clear Combined and Individual Interrupt Register"]
    pub fn ic_clr_intr(self) -> Reg<regs::IcClrIntr, RW> {
        unsafe { Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Clear RX_UNDER Interrupt Register"]
    pub fn ic_clr_rx_under(self) -> Reg<regs::IcClrRxUnder, RW> {
        unsafe { Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "Clear RX_OVER Interrupt Register"]
    pub fn ic_clr_rx_over(self) -> Reg<regs::IcClrRxOver, RW> {
        unsafe { Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "Clear TX_OVER Interrupt Register"]
    pub fn ic_clr_tx_over(self) -> Reg<regs::IcClrTxOver, RW> {
        unsafe { Reg::from_ptr(self.0.add(76usize)) }
    }
    #[doc = "Clear RD_REQ Interrupt Register"]
    pub fn ic_clr_rd_req(self) -> Reg<regs::IcClrRdReq, RW> {
        unsafe { Reg::from_ptr(self.0.add(80usize)) }
    }
    #[doc = "Clear TX_ABRT Interrupt Register"]
    pub fn ic_clr_tx_abrt(self) -> Reg<regs::IcClrTxAbrt, RW> {
        unsafe { Reg::from_ptr(self.0.add(84usize)) }
    }
    #[doc = "Clear RX_DONE Interrupt Register"]
    pub fn ic_clr_rx_done(self) -> Reg<regs::IcClrRxDone, RW> {
        unsafe { Reg::from_ptr(self.0.add(88usize)) }
    }
    #[doc = "Clear ACTIVITY Interrupt Register"]
    pub fn ic_clr_activity(self) -> Reg<regs::IcClrActivity, RW> {
        unsafe { Reg::from_ptr(self.0.add(92usize)) }
    }
    #[doc = "Clear STOP_DET Interrupt Register"]
    pub fn ic_clr_stop_det(self) -> Reg<regs::IcClrStopDet, RW> {
        unsafe { Reg::from_ptr(self.0.add(96usize)) }
    }
    #[doc = "Clear START_DET Interrupt Register"]
    pub fn ic_clr_start_det(self) -> Reg<regs::IcClrStartDet, RW> {
        unsafe { Reg::from_ptr(self.0.add(100usize)) }
    }
    #[doc = "Clear GEN_CALL Interrupt Register"]
    pub fn ic_clr_gen_call(self) -> Reg<regs::IcClrGenCall, RW> {
        unsafe { Reg::from_ptr(self.0.add(104usize)) }
    }
    #[doc = "I2C Enable Register"]
    pub fn ic_enable(self) -> Reg<regs::IcEnable, RW> {
        unsafe { Reg::from_ptr(self.0.add(108usize)) }
    }
    #[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
    pub fn ic_status(self) -> Reg<regs::IcStatus, RW> {
        unsafe { Reg::from_ptr(self.0.add(112usize)) }
    }
    #[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
    pub fn ic_txflr(self) -> Reg<regs::IcTxflr, RW> {
        unsafe { Reg::from_ptr(self.0.add(116usize)) }
    }
    #[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
    pub fn ic_rxflr(self) -> Reg<regs::IcRxflr, RW> {
        unsafe { Reg::from_ptr(self.0.add(120usize)) }
    }
    #[doc = "I2C SDA Hold Time Length Register The bits [15:0]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW). The bits [23:16]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode. Writes to this register succeed only when IC_ENABLE[0]=0. The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented. The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
    pub fn ic_sda_hold(self) -> Reg<regs::IcSdaHold, RW> {
        unsafe { Reg::from_ptr(self.0.add(124usize)) }
    }
    #[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
    pub fn ic_tx_abrt_source(self) -> Reg<regs::IcTxAbrtSource, RW> {
        unsafe { Reg::from_ptr(self.0.add(128usize)) }
    }
    #[doc = "Generate Slave Data NACK Register The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect. A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0]
= 0) - Slave part is inactive (IC_STATUS[6]
= 0) Note: The IC_STATUS[6]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
    pub fn ic_slv_data_nack_only(self) -> Reg<regs::IcSlvDataNackOnly, RW> {
        unsafe { Reg::from_ptr(self.0.add(132usize)) }
    }
    #[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
    pub fn ic_dma_cr(self) -> Reg<regs::IcDmaCr, RW> {
        unsafe { Reg::from_ptr(self.0.add(136usize)) }
    }
    #[doc = "DMA Transmit Data Level Register"]
    pub fn ic_dma_tdlr(self) -> Reg<regs::IcDmaTdlr, RW> {
        unsafe { Reg::from_ptr(self.0.add(140usize)) }
    }
    #[doc = "I2C Receive Data Level Register"]
    pub fn ic_dma_rdlr(self) -> Reg<regs::IcDmaRdlr, RW> {
        unsafe { Reg::from_ptr(self.0.add(144usize)) }
    }
    #[doc = "I2C SDA Setup Register This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. Writes to this register succeed only when IC_ENABLE[0]
= 0. Note: The length of setup time is calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
    pub fn ic_sda_setup(self) -> Reg<regs::IcSdaSetup, RW> {
        unsafe { Reg::from_ptr(self.0.add(148usize)) }
    }
    #[doc = "I2C ACK General Call Register The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address. This register is applicable only when the DW_apb_i2c is in slave mode."]
    pub fn ic_ack_general_call(self) -> Reg<regs::IcAckGeneralCall, RW> {
        unsafe { Reg::from_ptr(self.0.add(152usize)) }
    }
    #[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
    pub fn ic_enable_status(self) -> Reg<regs::IcEnableStatus, RW> {
        unsafe { Reg::from_ptr(self.0.add(156usize)) }
    }
    #[doc = "I2C SS, FS or FM+ spike suppression limit This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
    pub fn ic_fs_spklen(self) -> Reg<regs::IcFsSpklen, RW> {
        unsafe { Reg::from_ptr(self.0.add(160usize)) }
    }
    #[doc = "Clear RESTART_DET Interrupt Register"]
    pub fn ic_clr_restart_det(self) -> Reg<regs::IcClrRestartDet, RW> {
        unsafe { Reg::from_ptr(self.0.add(168usize)) }
    }
    #[doc = "Component Parameter Register 1 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
    pub fn ic_comp_param_1(self) -> Reg<regs::IcCompParam1, RW> {
        unsafe { Reg::from_ptr(self.0.add(244usize)) }
    }
    #[doc = "I2C Component Version Register"]
    pub fn ic_comp_version(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(248usize)) }
    }
    #[doc = "I2C Component Type Register"]
    pub fn ic_comp_type(self) -> Reg<u32, RW> {
        unsafe { Reg::from_ptr(self.0.add(252usize)) }
    }
}
pub mod regs;
pub mod vals;
