#[doc = "DW_apb_ssi has the following features: * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation. * APB3 and APB4 protocol support. * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits. * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices. * Programmable Dual/Quad/Octal SPI support in Master Mode. * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation. * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes. * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes. * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests. * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently. * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus. * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains. * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates. * Programmable features: - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire. - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation. - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer. * Configured features: - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits. - 1 slave select output. - Hardware slave-select – Dedicated hardware slave-select line. - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller. - Interrupt polarity – active high interrupt lines. - Serial clock polarity – low serial-clock polarity directly after reset. - Serial clock phase – capture on first edge of serial-clock directly after reset."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct XipSsi {
    ptr: *mut u8,
}
unsafe impl Send for XipSsi {}
unsafe impl Sync for XipSsi {}
impl XipSsi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 0"]
    #[inline(always)]
    pub const fn ctrlr0(self) -> crate::common::Reg<regs::Ctrlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Master Control register 1"]
    #[inline(always)]
    pub const fn ctrlr1(self) -> crate::common::Reg<regs::Ctrlr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "SSI Enable"]
    #[inline(always)]
    pub const fn ssienr(self) -> crate::common::Reg<regs::Ssienr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Microwire Control"]
    #[inline(always)]
    pub const fn mwcr(self) -> crate::common::Reg<regs::Mwcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Slave enable"]
    #[inline(always)]
    pub const fn ser(self) -> crate::common::Reg<regs::Ser, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Baud rate"]
    #[inline(always)]
    pub const fn baudr(self) -> crate::common::Reg<regs::Baudr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "TX FIFO threshold level"]
    #[inline(always)]
    pub const fn txftlr(self) -> crate::common::Reg<regs::Txftlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "RX FIFO threshold level"]
    #[inline(always)]
    pub const fn rxftlr(self) -> crate::common::Reg<regs::Rxftlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "TX FIFO level"]
    #[inline(always)]
    pub const fn txflr(self) -> crate::common::Reg<regs::Txflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "RX FIFO level"]
    #[inline(always)]
    pub const fn rxflr(self) -> crate::common::Reg<regs::Rxflr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Status register"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Interrupt mask"]
    #[inline(always)]
    pub const fn imr(self) -> crate::common::Reg<regs::Imr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Interrupt status"]
    #[inline(always)]
    pub const fn isr(self) -> crate::common::Reg<regs::Isr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Raw interrupt status"]
    #[inline(always)]
    pub const fn risr(self) -> crate::common::Reg<regs::Risr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "TX FIFO overflow interrupt clear"]
    #[inline(always)]
    pub const fn txoicr(self) -> crate::common::Reg<regs::Txoicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "RX FIFO overflow interrupt clear"]
    #[inline(always)]
    pub const fn rxoicr(self) -> crate::common::Reg<regs::Rxoicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "RX FIFO underflow interrupt clear"]
    #[inline(always)]
    pub const fn rxuicr(self) -> crate::common::Reg<regs::Rxuicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Multi-master interrupt clear"]
    #[inline(always)]
    pub const fn msticr(self) -> crate::common::Reg<regs::Msticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Interrupt clear"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "DMA control"]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "DMA TX data level"]
    #[inline(always)]
    pub const fn dmatdlr(self) -> crate::common::Reg<regs::Dmatdlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "DMA RX data level"]
    #[inline(always)]
    pub const fn dmardlr(self) -> crate::common::Reg<regs::Dmardlr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Identification register"]
    #[inline(always)]
    pub const fn idr(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Version ID"]
    #[inline(always)]
    pub const fn ssi_version_id(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Data Register 0 (of 36)"]
    #[inline(always)]
    pub const fn dr0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "RX sample delay"]
    #[inline(always)]
    pub const fn rx_sample_dly(self) -> crate::common::Reg<regs::RxSampleDly, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "SPI control"]
    #[inline(always)]
    pub const fn spi_ctrlr0(self) -> crate::common::Reg<regs::SpiCtrlr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "TX drive edge"]
    #[inline(always)]
    pub const fn txd_drive_edge(self) -> crate::common::Reg<regs::TxdDriveEdge, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
}
pub mod regs;
pub mod vals;
