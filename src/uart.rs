#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Uart(pub *mut u8);
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[doc = "Data Register, UARTDR"]
    #[inline(always)]
    pub fn uartdr(self) -> crate::common::Reg<regs::Uartdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    #[inline(always)]
    pub fn uartrsr(self) -> crate::common::Reg<regs::Uartrsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Flag Register, UARTFR"]
    #[inline(always)]
    pub fn uartfr(self) -> crate::common::Reg<regs::Uartfr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "IrDA Low-Power Counter Register, UARTILPR"]
    #[inline(always)]
    pub fn uartilpr(self) -> crate::common::Reg<regs::Uartilpr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Integer Baud Rate Register, UARTIBRD"]
    #[inline(always)]
    pub fn uartibrd(self) -> crate::common::Reg<regs::Uartibrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Fractional Baud Rate Register, UARTFBRD"]
    #[inline(always)]
    pub fn uartfbrd(self) -> crate::common::Reg<regs::Uartfbrd, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Line Control Register, UARTLCR_H"]
    #[inline(always)]
    pub fn uartlcr_h(self) -> crate::common::Reg<regs::UartlcrH, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Control Register, UARTCR"]
    #[inline(always)]
    pub fn uartcr(self) -> crate::common::Reg<regs::Uartcr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
    #[inline(always)]
    pub fn uartifls(self) -> crate::common::Reg<regs::Uartifls, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
    #[inline(always)]
    pub fn uartimsc(self) -> crate::common::Reg<regs::Uartimsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Raw Interrupt Status Register, UARTRIS"]
    #[inline(always)]
    pub fn uartris(self) -> crate::common::Reg<regs::Uartris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Masked Interrupt Status Register, UARTMIS"]
    #[inline(always)]
    pub fn uartmis(self) -> crate::common::Reg<regs::Uartmis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Interrupt Clear Register, UARTICR"]
    #[inline(always)]
    pub fn uarticr(self) -> crate::common::Reg<regs::Uarticr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "DMA Control Register, UARTDMACR"]
    #[inline(always)]
    pub fn uartdmacr(self) -> crate::common::Reg<regs::Uartdmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "UARTPeriphID0 Register"]
    #[inline(always)]
    pub fn uartperiphid0(self) -> crate::common::Reg<regs::Uartperiphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4064usize)) }
    }
    #[doc = "UARTPeriphID1 Register"]
    #[inline(always)]
    pub fn uartperiphid1(self) -> crate::common::Reg<regs::Uartperiphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4068usize)) }
    }
    #[doc = "UARTPeriphID2 Register"]
    #[inline(always)]
    pub fn uartperiphid2(self) -> crate::common::Reg<regs::Uartperiphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4072usize)) }
    }
    #[doc = "UARTPeriphID3 Register"]
    #[inline(always)]
    pub fn uartperiphid3(self) -> crate::common::Reg<regs::Uartperiphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4076usize)) }
    }
    #[doc = "UARTPCellID0 Register"]
    #[inline(always)]
    pub fn uartpcellid0(self) -> crate::common::Reg<regs::Uartpcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4080usize)) }
    }
    #[doc = "UARTPCellID1 Register"]
    #[inline(always)]
    pub fn uartpcellid1(self) -> crate::common::Reg<regs::Uartpcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4084usize)) }
    }
    #[doc = "UARTPCellID2 Register"]
    #[inline(always)]
    pub fn uartpcellid2(self) -> crate::common::Reg<regs::Uartpcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4088usize)) }
    }
    #[doc = "UARTPCellID3 Register"]
    #[inline(always)]
    pub fn uartpcellid3(self) -> crate::common::Reg<regs::Uartpcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
