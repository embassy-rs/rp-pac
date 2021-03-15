use crate::generic::*;
#[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspris(pub u32);
impl Sspris {
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    pub const fn txris(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPTXINTR interrupt"]
    pub fn set_txris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    pub const fn rxris(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRXINTR interrupt"]
    pub fn set_rxris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    pub const fn rtris(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRTINTR interrupt"]
    pub fn set_rtris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    pub const fn rorris(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the raw interrupt state, prior to masking, of the SSPRORINTR interrupt"]
    pub fn set_rorris(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspris {
    fn default() -> Sspris {
        Sspris(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspperiphid0(pub u32);
impl Sspperiphid0 {
    #[doc = "These bits read back as 0x22"]
    pub const fn partnumber0(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x22"]
    pub fn set_partnumber0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Sspperiphid0 {
    fn default() -> Sspperiphid0 {
        Sspperiphid0(0)
    }
}
#[doc = "Interrupt clear register, SSPICR on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspicr(pub u32);
impl Sspicr {
    #[doc = "Clears the SSPRTINTR interrupt"]
    pub const fn rtic(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRTINTR interrupt"]
    pub fn set_rtic(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Clears the SSPRORINTR interrupt"]
    pub const fn roric(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Clears the SSPRORINTR interrupt"]
    pub fn set_roric(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspicr {
    fn default() -> Sspicr {
        Sspicr(0)
    }
}
#[doc = "Control register 0, SSPCR0 on page 3-4"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspcr0(pub u32);
impl Sspcr0 {
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    pub const fn scr(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0xff;
        val as u8
    }
    #[doc = "Serial clock rate. The value SCR is used to generate the transmit and receive bit rate of the PrimeCell SSP. The bit rate is: F SSPCLK CPSDVSR x (1+SCR) where CPSDVSR is an even value from 2-254, programmed through the SSPCPSR register and SCR is a value from 0-255."]
    pub fn set_scr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub const fn sph(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT phase, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub fn set_sph(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub const fn spo(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "SSPCLKOUT polarity, applicable to Motorola SPI frame format only. See Motorola SPI frame format on page 2-10."]
    pub fn set_spo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    pub const fn frf(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x03;
        val as u8
    }
    #[doc = "Frame format: 00 Motorola SPI frame format. 01 TI synchronous serial frame format. 10 National Microwire frame format. 11 Reserved, undefined operation."]
    pub fn set_frf(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val as u32) & 0x03) << 4u32);
    }
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    pub const fn dss(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Data Size Select: 0000 Reserved, undefined operation. 0001 Reserved, undefined operation. 0010 Reserved, undefined operation. 0011 4-bit data. 0100 5-bit data. 0101 6-bit data. 0110 7-bit data. 0111 8-bit data. 1000 9-bit data. 1001 10-bit data. 1010 11-bit data. 1011 12-bit data. 1100 13-bit data. 1101 14-bit data. 1110 15-bit data. 1111 16-bit data."]
    pub fn set_dss(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Sspcr0 {
    fn default() -> Sspcr0 {
        Sspcr0(0)
    }
}
#[doc = "Control register 1, SSPCR1 on page 3-5"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspcr1(pub u32);
impl Sspcr1 {
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    pub const fn sod(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Slave-mode output disable. This bit is relevant only in the slave mode, MS=1. In multiple-slave systems, it is possible for an PrimeCell SSP master to broadcast a message to all slaves in the system while ensuring that only one slave drives data onto its serial output line. In such systems the RXD lines from multiple slaves could be tied together. To operate in such systems, the SOD bit can be set if the PrimeCell SSP slave is not supposed to drive the SSPTXD line: 0 SSP can drive the SSPTXD output in slave mode. 1 SSP must not drive the SSPTXD output in slave mode."]
    pub fn set_sod(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    pub const fn ms(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Master or slave mode select. This bit can be modified only when the PrimeCell SSP is disabled, SSE=0: 0 Device configured as master, default. 1 Device configured as slave."]
    pub fn set_ms(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    pub const fn sse(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Synchronous serial port enable: 0 SSP operation disabled. 1 SSP operation enabled."]
    pub fn set_sse(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    pub const fn lbm(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Loop back mode: 0 Normal serial port operation enabled. 1 Output of transmit serial shifter is connected to input of receive serial shifter internally."]
    pub fn set_lbm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspcr1 {
    fn default() -> Sspcr1 {
        Sspcr1(0)
    }
}
#[doc = "DMA control register, SSPDMACR on page 3-12"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspdmacr(pub u32);
impl Sspdmacr {
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    pub const fn txdmae(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit DMA Enable. If this bit is set to 1, DMA for the transmit FIFO is enabled."]
    pub fn set_txdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    pub const fn rxdmae(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Receive DMA Enable. If this bit is set to 1, DMA for the receive FIFO is enabled."]
    pub fn set_rxdmae(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspdmacr {
    fn default() -> Sspdmacr {
        Sspdmacr(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspperiphid2(pub u32);
impl Sspperiphid2 {
    #[doc = "These bits return the peripheral revision"]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x0f;
        val as u8
    }
    #[doc = "These bits return the peripheral revision"]
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4u32)) | (((val as u32) & 0x0f) << 4u32);
    }
    #[doc = "These bits read back as 0x4"]
    pub const fn designer1(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x4"]
    pub fn set_designer1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Sspperiphid2 {
    fn default() -> Sspperiphid2 {
        Sspperiphid2(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ssppcellid2(pub u32);
impl Ssppcellid2 {
    #[doc = "These bits read back as 0x05"]
    pub const fn ssppcellid2(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x05"]
    pub fn set_ssppcellid2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Ssppcellid2 {
    fn default() -> Ssppcellid2 {
        Ssppcellid2(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ssppcellid0(pub u32);
impl Ssppcellid0 {
    #[doc = "These bits read back as 0x0D"]
    pub const fn ssppcellid0(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x0D"]
    pub fn set_ssppcellid0(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Ssppcellid0 {
    fn default() -> Ssppcellid0 {
        Ssppcellid0(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ssppcellid3(pub u32);
impl Ssppcellid3 {
    #[doc = "These bits read back as 0xB1"]
    pub const fn ssppcellid3(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xB1"]
    pub fn set_ssppcellid3(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Ssppcellid3 {
    fn default() -> Ssppcellid3 {
        Ssppcellid3(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspperiphid1(pub u32);
impl Sspperiphid1 {
    #[doc = "These bits read back as 0x1"]
    pub const fn designer0(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x1"]
    pub fn set_designer0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4u32)) | (((val as u32) & 0x0f) << 4u32);
    }
    #[doc = "These bits read back as 0x0"]
    pub const fn partnumber1(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "These bits read back as 0x0"]
    pub fn set_partnumber1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Sspperiphid1 {
    fn default() -> Sspperiphid1 {
        Sspperiphid1(0)
    }
}
#[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ssppcellid1(pub u32);
impl Ssppcellid1 {
    #[doc = "These bits read back as 0xF0"]
    pub const fn ssppcellid1(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0xF0"]
    pub fn set_ssppcellid1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Ssppcellid1 {
    fn default() -> Ssppcellid1 {
        Ssppcellid1(0)
    }
}
#[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspperiphid3(pub u32);
impl Sspperiphid3 {
    #[doc = "These bits read back as 0x00"]
    pub const fn configuration(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "These bits read back as 0x00"]
    pub fn set_configuration(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Sspperiphid3 {
    fn default() -> Sspperiphid3 {
        Sspperiphid3(0)
    }
}
#[doc = "Clock prescale register, SSPCPSR on page 3-8"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspcpsr(pub u32);
impl Sspcpsr {
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    pub const fn cpsdvsr(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Clock prescale divisor. Must be an even number from 2-254, depending on the frequency of SSPCLK. The least significant bit always returns zero on reads."]
    pub fn set_cpsdvsr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Sspcpsr {
    fn default() -> Sspcpsr {
        Sspcpsr(0)
    }
}
#[doc = "Data register, SSPDR on page 3-6"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspdr(pub u32);
impl Sspdr {
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    pub const fn data(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    #[doc = "Transmit/Receive FIFO: Read Receive FIFO. Write Transmit FIFO. You must right-justify data when the PrimeCell SSP is programmed for a data size that is less than 16 bits. Unused bits at the top are ignored by transmit logic. The receive logic automatically right-justifies."]
    pub fn set_data(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Sspdr {
    fn default() -> Sspdr {
        Sspdr(0)
    }
}
#[doc = "Status register, SSPSR on page 3-7"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspsr(pub u32);
impl Sspsr {
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    pub const fn bsy(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "PrimeCell SSP busy flag, RO: 0 SSP is idle. 1 SSP is currently transmitting and/or receiving a frame or the transmit FIFO is not empty."]
    pub fn set_bsy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    pub const fn rff(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO full, RO: 0 Receive FIFO is not full. 1 Receive FIFO is full."]
    pub fn set_rff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    pub const fn rne(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO not empty, RO: 0 Receive FIFO is empty. 1 Receive FIFO is not empty."]
    pub fn set_rne(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    pub const fn tnf(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO not full, RO: 0 Transmit FIFO is full. 1 Transmit FIFO is not full."]
    pub fn set_tnf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    pub const fn tfe(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO empty, RO: 0 Transmit FIFO is not empty. 1 Transmit FIFO is empty."]
    pub fn set_tfe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspsr {
    fn default() -> Sspsr {
        Sspsr(0)
    }
}
#[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspimsc(pub u32);
impl Sspimsc {
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    pub const fn txim(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Transmit FIFO interrupt mask: 0 Transmit FIFO half empty or less condition interrupt is masked. 1 Transmit FIFO half empty or less condition interrupt is not masked."]
    pub fn set_txim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    pub const fn rxim(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Receive FIFO interrupt mask: 0 Receive FIFO half full or less condition interrupt is masked. 1 Receive FIFO half full or less condition interrupt is not masked."]
    pub fn set_rxim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    pub const fn rtim(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Receive timeout interrupt mask: 0 Receive FIFO not empty and no read prior to timeout period interrupt is masked. 1 Receive FIFO not empty and no read prior to timeout period interrupt is not masked."]
    pub fn set_rtim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    pub const fn rorim(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Receive overrun interrupt mask: 0 Receive FIFO written to while full condition interrupt is masked. 1 Receive FIFO written to while full condition interrupt is not masked."]
    pub fn set_rorim(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspimsc {
    fn default() -> Sspimsc {
        Sspimsc(0)
    }
}
#[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Sspmis(pub u32);
impl Sspmis {
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    pub const fn txmis(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the transmit FIFO masked interrupt state, after masking, of the SSPTXINTR interrupt"]
    pub fn set_txmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    pub const fn rxmis(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive FIFO masked interrupt state, after masking, of the SSPRXINTR interrupt"]
    pub fn set_rxmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    pub const fn rtmis(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive timeout masked interrupt state, after masking, of the SSPRTINTR interrupt"]
    pub fn set_rtmis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    pub const fn rormis(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Gives the receive over run masked interrupt status, after masking, of the SSPRORINTR interrupt"]
    pub fn set_rormis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Sspmis {
    fn default() -> Sspmis {
        Sspmis(0)
    }
}
