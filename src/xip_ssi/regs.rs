use crate::generic::*;
#[doc = "DMA TX data level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dmatdlr(pub u32);
impl Dmatdlr {
    #[doc = "Transmit data watermark level"]
    pub const fn dmatdl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Transmit data watermark level"]
    pub fn set_dmatdl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Dmatdlr {
    fn default() -> Dmatdlr {
        Dmatdlr(0)
    }
}
#[doc = "TX FIFO threshold level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Txftlr(pub u32);
impl Txftlr {
    #[doc = "Transmit FIFO threshold"]
    pub const fn tft(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO threshold"]
    pub fn set_tft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Txftlr {
    fn default() -> Txftlr {
        Txftlr(0)
    }
}
#[doc = "RX FIFO threshold level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rxftlr(pub u32);
impl Rxftlr {
    #[doc = "Receive FIFO threshold"]
    pub const fn rft(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO threshold"]
    pub fn set_rft(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Rxftlr {
    fn default() -> Rxftlr {
        Rxftlr(0)
    }
}
#[doc = "Control register 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrlr0(pub u32);
impl Ctrlr0 {
    #[doc = "Slave select toggle enable"]
    pub const fn sste(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Slave select toggle enable"]
    pub fn set_sste(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "SPI frame format"]
    pub const fn spi_frf(&self) -> super::vals::Ctrlr0SpiFrf {
        let val = (self.0 >> 21u32) & 0x03;
        super::vals::Ctrlr0SpiFrf(val as u8)
    }
    #[doc = "SPI frame format"]
    pub fn set_spi_frf(&mut self, val: super::vals::Ctrlr0SpiFrf) {
        self.0 = (self.0 & !(0x03 << 21u32)) | (((val.0 as u32) & 0x03) << 21u32);
    }
    #[doc = "Data frame size in 32b transfer mode Value of n -> n+1 clocks per frame."]
    pub const fn dfs_32(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x1f;
        val as u8
    }
    #[doc = "Data frame size in 32b transfer mode Value of n -> n+1 clocks per frame."]
    pub fn set_dfs_32(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16u32)) | (((val as u32) & 0x1f) << 16u32);
    }
    #[doc = "Control frame size Value of n -> n+1 clocks per frame."]
    pub const fn cfs(&self) -> u8 {
        let val = (self.0 >> 12u32) & 0x0f;
        val as u8
    }
    #[doc = "Control frame size Value of n -> n+1 clocks per frame."]
    pub fn set_cfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 12u32)) | (((val as u32) & 0x0f) << 12u32);
    }
    #[doc = "Shift register loop (test mode)"]
    pub const fn srl(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Shift register loop (test mode)"]
    pub fn set_srl(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Slave output enable"]
    pub const fn slv_oe(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Slave output enable"]
    pub fn set_slv_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Transfer mode"]
    pub const fn tmod(&self) -> super::vals::Ctrlr0Tmod {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::Ctrlr0Tmod(val as u8)
    }
    #[doc = "Transfer mode"]
    pub fn set_tmod(&mut self, val: super::vals::Ctrlr0Tmod) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "Serial clock polarity"]
    pub const fn scpol(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Serial clock polarity"]
    pub fn set_scpol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Serial clock phase"]
    pub const fn scph(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Serial clock phase"]
    pub fn set_scph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Frame format"]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x03;
        val as u8
    }
    #[doc = "Frame format"]
    pub fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val as u32) & 0x03) << 4u32);
    }
    #[doc = "Data frame size"]
    pub const fn dfs(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Data frame size"]
    pub fn set_dfs(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ctrlr0 {
    fn default() -> Ctrlr0 {
        Ctrlr0(0)
    }
}
#[doc = "SSI Enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ssienr(pub u32);
impl Ssienr {
    #[doc = "SSI enable"]
    pub const fn ssi_en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "SSI enable"]
    pub fn set_ssi_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ssienr {
    fn default() -> Ssienr {
        Ssienr(0)
    }
}
#[doc = "Slave enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ser(pub u32);
impl Ser {
    #[doc = "For each bit: 0 -> slave not selected 1 -> slave selected"]
    pub const fn ser(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "For each bit: 0 -> slave not selected 1 -> slave selected"]
    pub fn set_ser(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ser {
    fn default() -> Ser {
        Ser(0)
    }
}
#[doc = "RX FIFO overflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rxoicr(pub u32);
impl Rxoicr {
    #[doc = "Clear-on-read receive FIFO overflow interrupt"]
    pub const fn rxoicr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read receive FIFO overflow interrupt"]
    pub fn set_rxoicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Rxoicr {
    fn default() -> Rxoicr {
        Rxoicr(0)
    }
}
#[doc = "Multi-master interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Msticr(pub u32);
impl Msticr {
    #[doc = "Clear-on-read multi-master contention interrupt"]
    pub const fn msticr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read multi-master contention interrupt"]
    pub fn set_msticr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Msticr {
    fn default() -> Msticr {
        Msticr(0)
    }
}
#[doc = "TX FIFO overflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Txoicr(pub u32);
impl Txoicr {
    #[doc = "Clear-on-read transmit FIFO overflow interrupt"]
    pub const fn txoicr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read transmit FIFO overflow interrupt"]
    pub fn set_txoicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Txoicr {
    fn default() -> Txoicr {
        Txoicr(0)
    }
}
#[doc = "Microwire Control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Mwcr(pub u32);
impl Mwcr {
    #[doc = "Microwire handshaking"]
    pub const fn mhs(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Microwire handshaking"]
    pub fn set_mhs(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Microwire control"]
    pub const fn mdd(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Microwire control"]
    pub fn set_mdd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Microwire transfer mode"]
    pub const fn mwmod(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Microwire transfer mode"]
    pub fn set_mwmod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Mwcr {
    fn default() -> Mwcr {
        Mwcr(0)
    }
}
#[doc = "Status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sr(pub u32);
impl Sr {
    #[doc = "Data collision error"]
    pub const fn dcol(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Data collision error"]
    pub fn set_dcol(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Transmission error"]
    pub const fn txe(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "Transmission error"]
    pub fn set_txe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Receive FIFO full"]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full"]
    pub fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO not empty"]
    pub const fn rfne(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty"]
    pub fn set_rfne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Transmit FIFO empty"]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty"]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO not full"]
    pub const fn tfnf(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full"]
    pub fn set_tfnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "SSI busy flag"]
    pub const fn busy(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "SSI busy flag"]
    pub fn set_busy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sr {
    fn default() -> Sr {
        Sr(0)
    }
}
#[doc = "TX drive edge"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct TxdDriveEdge(pub u32);
impl TxdDriveEdge {
    #[doc = "TXD drive edge"]
    pub const fn tde(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "TXD drive edge"]
    pub fn set_tde(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for TxdDriveEdge {
    fn default() -> TxdDriveEdge {
        TxdDriveEdge(0)
    }
}
#[doc = "Interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Isr(pub u32);
impl Isr {
    #[doc = "Multi-master contention interrupt status"]
    pub const fn mstis(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention interrupt status"]
    pub fn set_mstis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Receive FIFO full interrupt status"]
    pub const fn rxfis(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full interrupt status"]
    pub fn set_rxfis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO overflow interrupt status"]
    pub const fn rxois(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow interrupt status"]
    pub fn set_rxois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Receive FIFO underflow interrupt status"]
    pub const fn rxuis(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow interrupt status"]
    pub fn set_rxuis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO overflow interrupt status"]
    pub const fn txois(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow interrupt status"]
    pub fn set_txois(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Transmit FIFO empty interrupt status"]
    pub const fn txeis(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty interrupt status"]
    pub fn set_txeis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Isr {
    fn default() -> Isr {
        Isr(0)
    }
}
#[doc = "RX FIFO level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rxflr(pub u32);
impl Rxflr {
    #[doc = "Receive FIFO level"]
    pub const fn rxtfl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Receive FIFO level"]
    pub fn set_rxtfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Rxflr {
    fn default() -> Rxflr {
        Rxflr(0)
    }
}
#[doc = "Baud rate"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Baudr(pub u32);
impl Baudr {
    #[doc = "SSI clock divider"]
    pub const fn sckdv(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "SSI clock divider"]
    pub fn set_sckdv(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Baudr {
    fn default() -> Baudr {
        Baudr(0)
    }
}
#[doc = "SPI control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SpiCtrlr0(pub u32);
impl SpiCtrlr0 {
    #[doc = "SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    pub const fn xip_cmd(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0xff;
        val as u8
    }
    #[doc = "SPI Command to send in XIP mode (INST_L = 8-bit) or to append to Address (INST_L = 0-bit)"]
    pub fn set_xip_cmd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24u32)) | (((val as u32) & 0xff) << 24u32);
    }
    #[doc = "Read data strobe enable"]
    pub const fn spi_rxds_en(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "Read data strobe enable"]
    pub fn set_spi_rxds_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "Instruction DDR transfer enable"]
    pub const fn inst_ddr_en(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "Instruction DDR transfer enable"]
    pub fn set_inst_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "SPI DDR transfer enable"]
    pub const fn spi_ddr_en(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "SPI DDR transfer enable"]
    pub fn set_spi_ddr_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    pub const fn wait_cycles(&self) -> u8 {
        let val = (self.0 >> 11u32) & 0x1f;
        val as u8
    }
    #[doc = "Wait cycles between control frame transmit and data reception (in SCLK cycles)"]
    pub fn set_wait_cycles(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 11u32)) | (((val as u32) & 0x1f) << 11u32);
    }
    #[doc = "Instruction length (0/4/8/16b)"]
    pub const fn inst_l(&self) -> super::vals::SpiCtrlr0InstL {
        let val = (self.0 >> 8u32) & 0x03;
        super::vals::SpiCtrlr0InstL(val as u8)
    }
    #[doc = "Instruction length (0/4/8/16b)"]
    pub fn set_inst_l(&mut self, val: super::vals::SpiCtrlr0InstL) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.0 as u32) & 0x03) << 8u32);
    }
    #[doc = "Address length (0b-60b in 4b increments)"]
    pub const fn addr_l(&self) -> u8 {
        let val = (self.0 >> 2u32) & 0x0f;
        val as u8
    }
    #[doc = "Address length (0b-60b in 4b increments)"]
    pub fn set_addr_l(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 2u32)) | (((val as u32) & 0x0f) << 2u32);
    }
    #[doc = "Address and instruction transfer format"]
    pub const fn trans_type(&self) -> super::vals::SpiCtrlr0TransType {
        let val = (self.0 >> 0u32) & 0x03;
        super::vals::SpiCtrlr0TransType(val as u8)
    }
    #[doc = "Address and instruction transfer format"]
    pub fn set_trans_type(&mut self, val: super::vals::SpiCtrlr0TransType) {
        self.0 = (self.0 & !(0x03 << 0u32)) | (((val.0 as u32) & 0x03) << 0u32);
    }
}
impl Default for SpiCtrlr0 {
    fn default() -> SpiCtrlr0 {
        SpiCtrlr0(0)
    }
}
#[doc = "TX FIFO level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Txflr(pub u32);
impl Txflr {
    #[doc = "Transmit FIFO level"]
    pub const fn tftfl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Transmit FIFO level"]
    pub fn set_tftfl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Txflr {
    fn default() -> Txflr {
        Txflr(0)
    }
}
#[doc = "Interrupt mask"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Imr(pub u32);
impl Imr {
    #[doc = "Multi-master contention interrupt mask"]
    pub const fn mstim(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention interrupt mask"]
    pub fn set_mstim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Receive FIFO full interrupt mask"]
    pub const fn rxfim(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full interrupt mask"]
    pub fn set_rxfim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO overflow interrupt mask"]
    pub const fn rxoim(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow interrupt mask"]
    pub fn set_rxoim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Receive FIFO underflow interrupt mask"]
    pub const fn rxuim(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow interrupt mask"]
    pub fn set_rxuim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO overflow interrupt mask"]
    pub const fn txoim(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow interrupt mask"]
    pub fn set_txoim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Transmit FIFO empty interrupt mask"]
    pub const fn txeim(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty interrupt mask"]
    pub fn set_txeim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Imr {
    fn default() -> Imr {
        Imr(0)
    }
}
#[doc = "Interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Icr(pub u32);
impl Icr {
    #[doc = "Clear-on-read all active interrupts"]
    pub const fn icr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read all active interrupts"]
    pub fn set_icr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Icr {
    fn default() -> Icr {
        Icr(0)
    }
}
#[doc = "RX sample delay"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct RxSampleDly(pub u32);
impl RxSampleDly {
    #[doc = "RXD sample delay (in SCLK cycles)"]
    pub const fn rsd(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "RXD sample delay (in SCLK cycles)"]
    pub fn set_rsd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for RxSampleDly {
    fn default() -> RxSampleDly {
        RxSampleDly(0)
    }
}
#[doc = "Raw interrupt status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Risr(pub u32);
impl Risr {
    #[doc = "Multi-master contention raw interrupt status"]
    pub const fn mstir(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "Multi-master contention raw interrupt status"]
    pub fn set_mstir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Receive FIFO full raw interrupt status"]
    pub const fn rxfir(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full raw interrupt status"]
    pub fn set_rxfir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO overflow raw interrupt status"]
    pub const fn rxoir(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO overflow raw interrupt status"]
    pub fn set_rxoir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Receive FIFO underflow raw interrupt status"]
    pub const fn rxuir(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO underflow raw interrupt status"]
    pub fn set_rxuir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO overflow raw interrupt status"]
    pub const fn txoir(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO overflow raw interrupt status"]
    pub fn set_txoir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Transmit FIFO empty raw interrupt status"]
    pub const fn txeir(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty raw interrupt status"]
    pub fn set_txeir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Risr {
    fn default() -> Risr {
        Risr(0)
    }
}
#[doc = "RX FIFO underflow interrupt clear"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Rxuicr(pub u32);
impl Rxuicr {
    #[doc = "Clear-on-read receive FIFO underflow interrupt"]
    pub const fn rxuicr(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clear-on-read receive FIFO underflow interrupt"]
    pub fn set_rxuicr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Rxuicr {
    fn default() -> Rxuicr {
        Rxuicr(0)
    }
}
#[doc = "Master Control register 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ctrlr1(pub u32);
impl Ctrlr1 {
    #[doc = "Number of data frames"]
    pub const fn ndf(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Number of data frames"]
    pub fn set_ndf(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ctrlr1 {
    fn default() -> Ctrlr1 {
        Ctrlr1(0)
    }
}
#[doc = "DMA control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dmacr(pub u32);
impl Dmacr {
    #[doc = "Transmit DMA enable"]
    pub const fn tdmae(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA enable"]
    pub fn set_tdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Receive DMA enable"]
    pub const fn rdmae(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA enable"]
    pub fn set_rdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Dmacr {
    fn default() -> Dmacr {
        Dmacr(0)
    }
}
#[doc = "DMA RX data level"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Dmardlr(pub u32);
impl Dmardlr {
    #[doc = "Receive data watermark level (DMARDLR+1)"]
    pub const fn dmardl(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Receive data watermark level (DMARDLR+1)"]
    pub fn set_dmardl(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Dmardlr {
    fn default() -> Dmardlr {
        Dmardlr(0)
    }
}
