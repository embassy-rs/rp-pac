use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Uart(pub *mut u8);
unsafe impl Send for Uart {}
unsafe impl Sync for Uart {}
impl Uart {
    #[doc = "Data Register, UARTDR"]
    pub fn uartdr(self) -> Reg<regs::Uartdr, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    pub fn uartrsr(self) -> Reg<regs::Uartrsr, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Flag Register, UARTFR"]
    pub fn uartfr(self) -> Reg<regs::Uartfr, RW> {
        unsafe { Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "IrDA Low-Power Counter Register, UARTILPR"]
    pub fn uartilpr(self) -> Reg<regs::Uartilpr, RW> {
        unsafe { Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "Integer Baud Rate Register, UARTIBRD"]
    pub fn uartibrd(self) -> Reg<regs::Uartibrd, RW> {
        unsafe { Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Fractional Baud Rate Register, UARTFBRD"]
    pub fn uartfbrd(self) -> Reg<regs::Uartfbrd, RW> {
        unsafe { Reg::from_ptr(self.0.add(40usize)) }
    }
    #[doc = "Line Control Register, UARTLCR_H"]
    pub fn uartlcr_h(self) -> Reg<regs::UartlcrH, RW> {
        unsafe { Reg::from_ptr(self.0.add(44usize)) }
    }
    #[doc = "Control Register, UARTCR"]
    pub fn uartcr(self) -> Reg<regs::Uartcr, RW> {
        unsafe { Reg::from_ptr(self.0.add(48usize)) }
    }
    #[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
    pub fn uartifls(self) -> Reg<regs::Uartifls, RW> {
        unsafe { Reg::from_ptr(self.0.add(52usize)) }
    }
    #[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
    pub fn uartimsc(self) -> Reg<regs::Uartimsc, RW> {
        unsafe { Reg::from_ptr(self.0.add(56usize)) }
    }
    #[doc = "Raw Interrupt Status Register, UARTRIS"]
    pub fn uartris(self) -> Reg<regs::Uartris, RW> {
        unsafe { Reg::from_ptr(self.0.add(60usize)) }
    }
    #[doc = "Masked Interrupt Status Register, UARTMIS"]
    pub fn uartmis(self) -> Reg<regs::Uartmis, RW> {
        unsafe { Reg::from_ptr(self.0.add(64usize)) }
    }
    #[doc = "Interrupt Clear Register, UARTICR"]
    pub fn uarticr(self) -> Reg<regs::Uarticr, RW> {
        unsafe { Reg::from_ptr(self.0.add(68usize)) }
    }
    #[doc = "DMA Control Register, UARTDMACR"]
    pub fn uartdmacr(self) -> Reg<regs::Uartdmacr, RW> {
        unsafe { Reg::from_ptr(self.0.add(72usize)) }
    }
    #[doc = "UARTPeriphID0 Register"]
    pub fn uartperiphid0(self) -> Reg<regs::Uartperiphid0, RW> {
        unsafe { Reg::from_ptr(self.0.add(4064usize)) }
    }
    #[doc = "UARTPeriphID1 Register"]
    pub fn uartperiphid1(self) -> Reg<regs::Uartperiphid1, RW> {
        unsafe { Reg::from_ptr(self.0.add(4068usize)) }
    }
    #[doc = "UARTPeriphID2 Register"]
    pub fn uartperiphid2(self) -> Reg<regs::Uartperiphid2, RW> {
        unsafe { Reg::from_ptr(self.0.add(4072usize)) }
    }
    #[doc = "UARTPeriphID3 Register"]
    pub fn uartperiphid3(self) -> Reg<regs::Uartperiphid3, RW> {
        unsafe { Reg::from_ptr(self.0.add(4076usize)) }
    }
    #[doc = "UARTPCellID0 Register"]
    pub fn uartpcellid0(self) -> Reg<regs::Uartpcellid0, RW> {
        unsafe { Reg::from_ptr(self.0.add(4080usize)) }
    }
    #[doc = "UARTPCellID1 Register"]
    pub fn uartpcellid1(self) -> Reg<regs::Uartpcellid1, RW> {
        unsafe { Reg::from_ptr(self.0.add(4084usize)) }
    }
    #[doc = "UARTPCellID2 Register"]
    pub fn uartpcellid2(self) -> Reg<regs::Uartpcellid2, RW> {
        unsafe { Reg::from_ptr(self.0.add(4088usize)) }
    }
    #[doc = "UARTPCellID3 Register"]
    pub fn uartpcellid3(self) -> Reg<regs::Uartpcellid3, RW> {
        unsafe { Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
