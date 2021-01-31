use crate::generic::*;
#[doc = "DW_apb_i2c address block"]
#[derive(Copy, Clone)]
pub struct I2c0(*mut u8);
unsafe impl Send for I2c0 {}
unsafe impl Sync for I2c0 {}
impl I2c0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "I2C Control Register. This register can be written only when the DW_apb_i2c is disabled, which corresponds to the IC_ENABLE[0]
register being set to 0. Writes at other times have no effect. Read/Write Access: - bit 10 is read only. - bit 11 is read only - bit 16 is read only - bit 17 is read only - bits 18 and 19 are read only."]
    pub fn ic_con(self) -> Reg<fields::IcCon, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::IcCon::from_bits(101)) }
    }
    #[doc = "I2C Target Address Register This register is 12 bits wide, and bits 31:12 are reserved. This register can be written to only when IC_ENABLE[0]
is set to 0. Note: If the software or application is aware that the DW_apb_i2c is not using the TAR address for the pending commands in the Tx FIFO, then it is possible to update the TAR address even while the Tx FIFO has entries (IC_STATUS[2]= 0). - It is not necessary to perform any write to this register if DW_apb_i2c is enabled as an I2C slave only."]
    pub fn ic_tar(self) -> Reg<fields::IcTar, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::IcTar::from_bits(85)) }
    }
    #[doc = "I2C Slave Address Register"]
    pub fn ic_sar(self) -> Reg<fields::IcSar, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::IcSar::from_bits(85)) }
    }
    #[doc = "I2C Rx/Tx Data Buffer and Command Register; this is the register the CPU writes to when filling the TX FIFO and the CPU reads from when retrieving bytes from RX FIFO. The size of the register changes as follows: Write: - 11 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=1 - 9 bits when IC_EMPTYFIFO_HOLD_MASTER_EN=0 Read: - 12 bits when IC_FIRST_DATA_BYTE_STATUS = 1 - 8 bits when IC_FIRST_DATA_BYTE_STATUS = 0 Note: In order for the DW_apb_i2c to continue acknowledging reads, a read command should be written for every byte that is to be received; otherwise the DW_apb_i2c will stop acknowledging."]
    pub fn ic_data_cmd(self) -> Reg<fields::IcDataCmd, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::IcDataCmd::from_bits(0)) }
    }
    #[doc = "Standard Speed I2C Clock SCL High Count Register"]
    pub fn ic_ss_scl_hcnt(self) -> Reg<fields::IcSsSclHcnt, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::IcSsSclHcnt::from_bits(40)) }
    }
    #[doc = "Standard Speed I2C Clock SCL Low Count Register"]
    pub fn ic_ss_scl_lcnt(self) -> Reg<fields::IcSsSclLcnt, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::IcSsSclLcnt::from_bits(47)) }
    }
    #[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL High Count Register"]
    pub fn ic_fs_scl_hcnt(self) -> Reg<fields::IcFsSclHcnt, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::IcFsSclHcnt::from_bits(6)) }
    }
    #[doc = "Fast Mode or Fast Mode Plus I2C Clock SCL Low Count Register"]
    pub fn ic_fs_scl_lcnt(self) -> Reg<fields::IcFsSclLcnt, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::IcFsSclLcnt::from_bits(13)) }
    }
    #[doc = "I2C Interrupt Status Register Each bit in this register has a corresponding mask bit in the IC_INTR_MASK register. These bits are cleared by reading the matching interrupt clear register. The unmasked raw versions of these bits are available in the IC_RAW_INTR_STAT register."]
    pub fn ic_intr_stat(self) -> Reg<fields::IcIntrStat, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::IcIntrStat::from_bits(0)) }
    }
    #[doc = "I2C Interrupt Mask Register. These bits mask their corresponding interrupt status bits. This register is active low; a value of 0 masks the interrupt, whereas a value of 1 unmasks the interrupt."]
    pub fn ic_intr_mask(self) -> Reg<fields::IcIntrMask, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::IcIntrMask::from_bits(2303)) }
    }
    #[doc = "I2C Raw Interrupt Status Register Unlike the IC_INTR_STAT register, these bits are not masked so they always show the true status of the DW_apb_i2c."]
    pub fn ic_raw_intr_stat(self) -> Reg<fields::IcRawIntrStat, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::IcRawIntrStat::from_bits(0)) }
    }
    #[doc = "I2C Receive FIFO Threshold Register"]
    pub fn ic_rx_tl(self) -> Reg<fields::IcRxTl, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::IcRxTl::from_bits(0)) }
    }
    #[doc = "I2C Transmit FIFO Threshold Register"]
    pub fn ic_tx_tl(self) -> Reg<fields::IcTxTl, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::IcTxTl::from_bits(0)) }
    }
    #[doc = "Clear Combined and Individual Interrupt Register"]
    pub fn ic_clr_intr(self) -> Reg<fields::IcClrIntr, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::IcClrIntr::from_bits(0)) }
    }
    #[doc = "Clear RX_UNDER Interrupt Register"]
    pub fn ic_clr_rx_under(self) -> Reg<fields::IcClrRxUnder, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::IcClrRxUnder::from_bits(0)) }
    }
    #[doc = "Clear RX_OVER Interrupt Register"]
    pub fn ic_clr_rx_over(self) -> Reg<fields::IcClrRxOver, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::IcClrRxOver::from_bits(0)) }
    }
    #[doc = "Clear TX_OVER Interrupt Register"]
    pub fn ic_clr_tx_over(self) -> Reg<fields::IcClrTxOver, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::IcClrTxOver::from_bits(0)) }
    }
    #[doc = "Clear RD_REQ Interrupt Register"]
    pub fn ic_clr_rd_req(self) -> Reg<fields::IcClrRdReq, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::IcClrRdReq::from_bits(0)) }
    }
    #[doc = "Clear TX_ABRT Interrupt Register"]
    pub fn ic_clr_tx_abrt(self) -> Reg<fields::IcClrTxAbrt, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::IcClrTxAbrt::from_bits(0)) }
    }
    #[doc = "Clear RX_DONE Interrupt Register"]
    pub fn ic_clr_rx_done(self) -> Reg<fields::IcClrRxDone, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::IcClrRxDone::from_bits(0)) }
    }
    #[doc = "Clear ACTIVITY Interrupt Register"]
    pub fn ic_clr_activity(self) -> Reg<fields::IcClrActivity, RW> {
        unsafe { Reg::new(self.0.add(92usize), fields::IcClrActivity::from_bits(0)) }
    }
    #[doc = "Clear STOP_DET Interrupt Register"]
    pub fn ic_clr_stop_det(self) -> Reg<fields::IcClrStopDet, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::IcClrStopDet::from_bits(0)) }
    }
    #[doc = "Clear START_DET Interrupt Register"]
    pub fn ic_clr_start_det(self) -> Reg<fields::IcClrStartDet, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::IcClrStartDet::from_bits(0)) }
    }
    #[doc = "Clear GEN_CALL Interrupt Register"]
    pub fn ic_clr_gen_call(self) -> Reg<fields::IcClrGenCall, RW> {
        unsafe { Reg::new(self.0.add(104usize), fields::IcClrGenCall::from_bits(0)) }
    }
    #[doc = "I2C Enable Register"]
    pub fn ic_enable(self) -> Reg<fields::IcEnable, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::IcEnable::from_bits(0)) }
    }
    #[doc = "I2C Status Register This is a read-only register used to indicate the current transfer status and FIFO status. The status register may be read at any time. None of the bits in this register request an interrupt. When the I2C is disabled by writing 0 in bit 0 of the IC_ENABLE register: - Bits 1 and 2 are set to 1 - Bits 3 and 10 are set to 0 When the master or slave state machines goes to idle and ic_en=0: - Bits 5 and 6 are set to 0"]
    pub fn ic_status(self) -> Reg<fields::IcStatus, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::IcStatus::from_bits(6)) }
    }
    #[doc = "I2C Transmit FIFO Level Register This register contains the number of valid data entries in the transmit FIFO buffer. It is cleared whenever: - The I2C is disabled - There is a transmit abort - that is, TX_ABRT bit is set in the IC_RAW_INTR_STAT register - The slave bulk transmit mode is aborted The register increments whenever data is placed into the transmit FIFO and decrements when data is taken from the transmit FIFO."]
    pub fn ic_txflr(self) -> Reg<fields::IcTxflr, RW> {
        unsafe { Reg::new(self.0.add(116usize), fields::IcTxflr::from_bits(0)) }
    }
    #[doc = "I2C Receive FIFO Level Register This register contains the number of valid data entries in the receive FIFO buffer. It is cleared whenever: - The I2C is disabled - Whenever there is a transmit abort caused by any of the events tracked in IC_TX_ABRT_SOURCE The register increments whenever data is placed into the receive FIFO and decrements when data is taken from the receive FIFO."]
    pub fn ic_rxflr(self) -> Reg<fields::IcRxflr, RW> {
        unsafe { Reg::new(self.0.add(120usize), fields::IcRxflr::from_bits(0)) }
    }
    #[doc = "I2C SDA Hold Time Length Register The bits [15:0]
of this register are used to control the hold time of SDA during transmit in both slave and master mode (after SCL goes from HIGH to LOW). The bits [23:16]
of this register are used to extend the SDA transition (if any) whenever SCL is HIGH in the receiver in either master or slave mode. Writes to this register succeed only when IC_ENABLE[0]=0. The values in this register are in units of ic_clk period. The value programmed in IC_SDA_TX_HOLD must be greater than the minimum hold time in each mode one cycle in master mode, seven cycles in slave mode for the value to be implemented. The programmed SDA hold time during transmit (IC_SDA_TX_HOLD) cannot exceed at any time the duration of the low part of scl. Therefore the programmed value cannot be larger than N_SCL_LOW-2, where N_SCL_LOW is the duration of the low part of the scl period measured in ic_clk cycles."]
    pub fn ic_sda_hold(self) -> Reg<fields::IcSdaHold, RW> {
        unsafe { Reg::new(self.0.add(124usize), fields::IcSdaHold::from_bits(1)) }
    }
    #[doc = "I2C Transmit Abort Source Register This register has 32 bits that indicate the source of the TX_ABRT bit. Except for Bit 9, this register is cleared whenever the IC_CLR_TX_ABRT register or the IC_CLR_INTR register is read. To clear Bit 9, the source of the ABRT_SBYTE_NORSTRT must be fixed first; RESTART must be enabled (IC_CON[5]=1), the SPECIAL bit must be cleared (IC_TAR[11]), or the GC_OR_START bit must be cleared (IC_TAR[10]). Once the source of the ABRT_SBYTE_NORSTRT is fixed, then this bit can be cleared in the same manner as other bits in this register. If the source of the ABRT_SBYTE_NORSTRT is not fixed before attempting to clear this bit, Bit 9 clears for one cycle and is then re-asserted."]
    pub fn ic_tx_abrt_source(self) -> Reg<fields::IcTxAbrtSource, RW> {
        unsafe { Reg::new(self.0.add(128usize), fields::IcTxAbrtSource::from_bits(0)) }
    }
    #[doc = "Generate Slave Data NACK Register The register is used to generate a NACK for the data part of a transfer when DW_apb_i2c is acting as a slave-receiver. This register only exists when the IC_SLV_DATA_NACK_ONLY parameter is set to 1. When this parameter disabled, this register does not exist and writing to the register's address has no effect. A write can occur on this register if both of the following conditions are met: - DW_apb_i2c is disabled (IC_ENABLE[0]
= 0) - Slave part is inactive (IC_STATUS[6]
= 0) Note: The IC_STATUS[6]
is a register read-back location for the internal slv_activity signal; the user should poll this before writing the ic_slv_data_nack_only bit."]
    pub fn ic_slv_data_nack_only(self) -> Reg<fields::IcSlvDataNackOnly, RW> {
        unsafe {
            Reg::new(
                self.0.add(132usize),
                fields::IcSlvDataNackOnly::from_bits(0),
            )
        }
    }
    #[doc = "DMA Control Register The register is used to enable the DMA Controller interface operation. There is a separate bit for transmit and receive. This can be programmed regardless of the state of IC_ENABLE."]
    pub fn ic_dma_cr(self) -> Reg<fields::IcDmaCr, RW> {
        unsafe { Reg::new(self.0.add(136usize), fields::IcDmaCr::from_bits(0)) }
    }
    #[doc = "DMA Transmit Data Level Register"]
    pub fn ic_dma_tdlr(self) -> Reg<fields::IcDmaTdlr, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::IcDmaTdlr::from_bits(0)) }
    }
    #[doc = "I2C Receive Data Level Register"]
    pub fn ic_dma_rdlr(self) -> Reg<fields::IcDmaRdlr, RW> {
        unsafe { Reg::new(self.0.add(144usize), fields::IcDmaRdlr::from_bits(0)) }
    }
    #[doc = "I2C SDA Setup Register This register controls the amount of time delay (in terms of number of ic_clk clock periods) introduced in the rising edge of SCL - relative to SDA changing - when DW_apb_i2c services a read request in a slave-transmitter operation. The relevant I2C requirement is tSU:DAT (note 4) as detailed in the I2C Bus Specification. This register must be programmed with a value equal to or greater than 2. Writes to this register succeed only when IC_ENABLE[0]
= 0. Note: The length of setup time is calculated using [(IC_SDA_SETUP - 1) * (ic_clk_period)], so if the user requires 10 ic_clk periods of setup time, they should program a value of 11. The IC_SDA_SETUP register is only used by the DW_apb_i2c when operating as a slave transmitter."]
    pub fn ic_sda_setup(self) -> Reg<fields::IcSdaSetup, RW> {
        unsafe { Reg::new(self.0.add(148usize), fields::IcSdaSetup::from_bits(100)) }
    }
    #[doc = "I2C ACK General Call Register The register controls whether DW_apb_i2c responds with a ACK or NACK when it receives an I2C General Call address. This register is applicable only when the DW_apb_i2c is in slave mode."]
    pub fn ic_ack_general_call(self) -> Reg<fields::IcAckGeneralCall, RW> {
        unsafe { Reg::new(self.0.add(152usize), fields::IcAckGeneralCall::from_bits(1)) }
    }
    #[doc = "I2C Enable Status Register The register is used to report the DW_apb_i2c hardware status when the IC_ENABLE[0]
register is set from 1 to 0; that is, when DW_apb_i2c is disabled. If IC_ENABLE[0]
has been set to 1, bits 2:1 are forced to 0, and bit 0 is forced to 1. If IC_ENABLE[0]
has been set to 0, bits 2:1 is only be valid as soon as bit 0 is read as '0'. Note: When IC_ENABLE[0]
has been set to 0, a delay occurs for bit 0 to be read as 0 because disabling the DW_apb_i2c depends on I2C bus activities."]
    pub fn ic_enable_status(self) -> Reg<fields::IcEnableStatus, RW> {
        unsafe { Reg::new(self.0.add(156usize), fields::IcEnableStatus::from_bits(0)) }
    }
    #[doc = "I2C SS, FS or FM+ spike suppression limit This register is used to store the duration, measured in ic_clk cycles, of the longest spike that is filtered out by the spike suppression logic when the component is operating in SS, FS or FM+ modes. The relevant I2C requirement is tSP (table 4) as detailed in the I2C Bus Specification. This register must be programmed with a minimum value of 1."]
    pub fn ic_fs_spklen(self) -> Reg<fields::IcFsSpklen, RW> {
        unsafe { Reg::new(self.0.add(160usize), fields::IcFsSpklen::from_bits(7)) }
    }
    #[doc = "Clear RESTART_DET Interrupt Register"]
    pub fn ic_clr_restart_det(self) -> Reg<fields::IcClrRestartDet, RW> {
        unsafe { Reg::new(self.0.add(168usize), fields::IcClrRestartDet::from_bits(0)) }
    }
    #[doc = "Component Parameter Register 1 Note This register is not implemented and therefore reads as 0. If it was implemented it would be a constant read-only register that contains encoded information about the component's parameter settings. Fields shown below are the settings for those parameters"]
    pub fn ic_comp_param_1(self) -> Reg<fields::IcCompParam1, RW> {
        unsafe { Reg::new(self.0.add(244usize), fields::IcCompParam1::from_bits(0)) }
    }
    #[doc = "I2C Component Version Register"]
    pub fn ic_comp_version(self) -> Reg<fields::IcCompVersion, RW> {
        unsafe {
            Reg::new(
                self.0.add(248usize),
                fields::IcCompVersion::from_bits(842019114),
            )
        }
    }
    #[doc = "I2C Component Type Register"]
    pub fn ic_comp_type(self) -> Reg<fields::IcCompType, RW> {
        unsafe {
            Reg::new(
                self.0.add(252usize),
                fields::IcCompType::from_bits(1146552640),
            )
        }
    }
}
pub mod fields;
pub mod values;
