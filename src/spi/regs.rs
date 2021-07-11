#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid0(pub u32);
impl periphid0 {
    #[doc = "These bits read back as 0x22"]
    pub const fn partnumber0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x22"]
    pub fn set_partnumber0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for periphid0 {
    fn default() -> periphid0 {
        periphid0(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid2(pub u32);
impl periphid2 {
    #[doc = "These bits read back as 0x4"]
    pub const fn designer1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x4"]
    pub fn set_designer1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits return the peripheral revision"]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits return the peripheral revision"]
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for periphid2 {
    fn default() -> periphid2 {
        periphid2(0)
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct icr(pub u32);
impl icr {
    #[doc = "Clears the SSPRORINTR interrupt"]
    pub const fn roric(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRORINTR interrupt"]
    pub fn set_roric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Clears the SSPRTINTR interrupt"]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRTINTR interrupt"]
    pub fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for icr {
    fn default() -> icr {
        icr(0)
    }
}
#[doc = "Control register 1, SSPCR1 on page 3-5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cr1(pub u32);
impl cr1 {
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    pub const fn lbm(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    pub fn set_lbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    pub const fn sse(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    pub fn set_sse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    pub const fn ms(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    pub fn set_ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    pub fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for cr1 {
    fn default() -> cr1 {
        cr1(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid0(pub u32);
impl pcellid0 {
    #[doc = "These bits read back as 0x0D"]
    pub const fn ssppcellid0(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x0D"]
    pub fn set_ssppcellid0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid0 {
    fn default() -> pcellid0 {
        pcellid0(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid3(pub u32);
impl pcellid3 {
    #[doc = "These bits read back as 0xB1"]
    pub const fn ssppcellid3(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xB1"]
    pub fn set_ssppcellid3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid3 {
    fn default() -> pcellid3 {
        pcellid3(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid1(pub u32);
impl periphid1 {
    #[doc = "These bits read back as 0x0"]
    pub const fn partnumber1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x0"]
    pub fn set_partnumber1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "These bits read back as 0x1"]
    pub const fn designer0(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x1"]
    pub fn set_designer0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
}
impl Default for periphid1 {
    fn default() -> periphid1 {
        periphid1(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid1(pub u32);
impl pcellid1 {
    #[doc = "These bits read back as 0xF0"]
    pub const fn ssppcellid1(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xF0"]
    pub fn set_ssppcellid1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid1 {
    fn default() -> pcellid1 {
        pcellid1(0)
    }
}
#[doc = "Data register, SSPDR on page 3-6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct dr(pub u32);
impl dr {
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for dr {
    fn default() -> dr {
        dr(0)
    }
}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct mis(pub u32);
impl mis {
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    pub const fn rormis(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    pub fn set_rormis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    pub fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    pub fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    pub fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for mis {
    fn default() -> mis {
        mis(0)
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cr0(pub u32);
impl cr0 {
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    pub const fn dss(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    pub fn set_dss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x03;
        val as u8
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    pub fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val as u32) & 0x03) << 4usize);
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub const fn spo(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub fn set_spo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub const fn sph(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub fn set_sph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    pub const fn scr(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    pub fn set_scr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
}
impl Default for cr0 {
    fn default() -> cr0 {
        cr0(0)
    }
}
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ris(pub u32);
impl ris {
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    pub const fn rorris(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    pub fn set_rorris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    pub fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    pub fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    pub fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for ris {
    fn default() -> ris {
        ris(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct pcellid2(pub u32);
impl pcellid2 {
    #[doc = "These bits read back as 0x05"]
    pub const fn ssppcellid2(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x05"]
    pub fn set_ssppcellid2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for pcellid2 {
    fn default() -> pcellid2 {
        pcellid2(0)
    }
}
#[doc = "Status register, SSPSR on page 3-7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct sr(pub u32);
impl sr {
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    pub const fn tnf(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    pub fn set_tnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    pub const fn rne(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    pub fn set_rne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    pub fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
}
impl Default for sr {
    fn default() -> sr {
        sr(0)
    }
}
#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct cpsr(pub u32);
impl cpsr {
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    pub const fn cpsdvsr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    pub fn set_cpsdvsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for cpsr {
    fn default() -> cpsr {
        cpsr(0)
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct imsc(pub u32);
impl imsc {
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    pub const fn rorim(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    pub fn set_rorim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    pub fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    pub fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    pub fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for imsc {
    fn default() -> imsc {
        imsc(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct periphid3(pub u32);
impl periphid3 {
    #[doc = "These bits read back as 0x00"]
    pub const fn configuration(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x00"]
    pub fn set_configuration(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for periphid3 {
    fn default() -> periphid3 {
        periphid3(0)
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct dmacr(pub u32);
impl dmacr {
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    pub const fn rxdmae(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    pub fn set_rxdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    pub const fn txdmae(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    pub fn set_txdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for dmacr {
    fn default() -> dmacr {
        dmacr(0)
    }
}
