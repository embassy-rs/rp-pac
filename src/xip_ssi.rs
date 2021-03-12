use crate::generic::*;
#[doc = "DW_apb_ssi has the following features: * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation. * APB3 and APB4 protocol support. * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits. * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices. * Programmable Dual/Quad/Octal SPI support in Master Mode. * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation. * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes. * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes. * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests. * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently. * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus. * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains. * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates. * Programmable features: - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire. - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation. - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer. * Configured features: - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits. - 1 slave select output. - Hardware slave-select – Dedicated hardware slave-select line. - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller. - Interrupt polarity – active high interrupt lines. - Serial clock polarity – low serial-clock polarity directly after reset. - Serial clock phase – capture on first edge of serial-clock directly after reset."]
#[derive(Copy, Clone)]
pub struct XipSsi(*mut u8);
unsafe impl Send for XipSsi {}
unsafe impl Sync for XipSsi {}
impl XipSsi {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Control register 0"]
    pub fn ctrlr0(self) -> Reg<regs::Ctrlr0, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Master Control register 1"]
    pub fn ctrlr1(self) -> Reg<regs::Ctrlr1, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "SSI Enable"]
    pub fn ssienr(self) -> Reg<regs::Ssienr, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Microwire Control"]
    pub fn mwcr(self) -> Reg<regs::Mwcr, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Slave enable"]
    pub fn ser(self) -> Reg<regs::Ser, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Baud rate"]
    pub fn baudr(self) -> Reg<regs::Baudr, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "TX FIFO threshold level"]
    pub fn txftlr(self) -> Reg<regs::Txftlr, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "RX FIFO threshold level"]
    pub fn rxftlr(self) -> Reg<regs::Rxftlr, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "TX FIFO level"]
    pub fn txflr(self) -> Reg<regs::Txflr, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "RX FIFO level"]
    pub fn rxflr(self) -> Reg<regs::Rxflr, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Status register"]
    pub fn sr(self) -> Reg<regs::Sr, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "Interrupt mask"]
    pub fn imr(self) -> Reg<regs::Imr, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "Interrupt status"]
    pub fn isr(self) -> Reg<regs::Isr, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "Raw interrupt status"]
    pub fn risr(self) -> Reg<regs::Risr, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "TX FIFO overflow interrupt clear"]
    pub fn txoicr(self) -> Reg<regs::Txoicr, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "RX FIFO overflow interrupt clear"]
    pub fn rxoicr(self) -> Reg<regs::Rxoicr, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "RX FIFO underflow interrupt clear"]
    pub fn rxuicr(self) -> Reg<regs::Rxuicr, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "Multi-master interrupt clear"]
    pub fn msticr(self) -> Reg<regs::Msticr, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "Interrupt clear"]
    pub fn icr(self) -> Reg<regs::Icr, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "DMA control"]
    pub fn dmacr(self) -> Reg<regs::Dmacr, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "DMA TX data level"]
    pub fn dmatdlr(self) -> Reg<regs::Dmatdlr, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "DMA RX data level"]
    pub fn dmardlr(self) -> Reg<regs::Dmardlr, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "Identification register"]
    pub fn idr(self) -> Reg<regs::Idr, RW> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "Version ID"]
    pub fn ssi_version_id(self) -> Reg<regs::SsiVersionId, RW> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "Data Register 0 (of 36)"]
    pub fn dr0(self) -> Reg<regs::Dr0, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "RX sample delay"]
    pub fn rx_sample_dly(self) -> Reg<regs::RxSampleDly, RW> {
        unsafe { Reg::new(self.0.add(240usize)) }
    }
    #[doc = "SPI control"]
    pub fn spi_ctrlr0(self) -> Reg<regs::SpiCtrlr0, RW> {
        unsafe { Reg::new(self.0.add(244usize)) }
    }
    #[doc = "TX drive edge"]
    pub fn txd_drive_edge(self) -> Reg<regs::TxdDriveEdge, RW> {
        unsafe { Reg::new(self.0.add(248usize)) }
    }
}
pub mod regs;
pub mod vals;
