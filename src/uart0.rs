use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Uart0(*mut u8);
unsafe impl Send for Uart0 {}
unsafe impl Sync for Uart0 {}
impl Uart0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Data Register, UARTDR"]
    pub fn uartdr(self) -> Reg<fields::Uartdr, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Uartdr::from_bits(0)) }
    }
    #[doc = "Receive Status Register/Error Clear Register, UARTRSR/UARTECR"]
    pub fn uartrsr(self) -> Reg<fields::Uartrsr, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Uartrsr::from_bits(0)) }
    }
    #[doc = "Flag Register, UARTFR"]
    pub fn uartfr(self) -> Reg<fields::Uartfr, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Uartfr::from_bits(144)) }
    }
    #[doc = "IrDA Low-Power Counter Register, UARTILPR"]
    pub fn uartilpr(self) -> Reg<fields::Uartilpr, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Uartilpr::from_bits(0)) }
    }
    #[doc = "Integer Baud Rate Register, UARTIBRD"]
    pub fn uartibrd(self) -> Reg<fields::Uartibrd, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Uartibrd::from_bits(0)) }
    }
    #[doc = "Fractional Baud Rate Register, UARTFBRD"]
    pub fn uartfbrd(self) -> Reg<fields::Uartfbrd, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::Uartfbrd::from_bits(0)) }
    }
    #[doc = "Line Control Register, UARTLCR_H"]
    pub fn uartlcr_h(self) -> Reg<fields::UartlcrH, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::UartlcrH::from_bits(0)) }
    }
    #[doc = "Control Register, UARTCR"]
    pub fn uartcr(self) -> Reg<fields::Uartcr, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Uartcr::from_bits(768)) }
    }
    #[doc = "Interrupt FIFO Level Select Register, UARTIFLS"]
    pub fn uartifls(self) -> Reg<fields::Uartifls, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Uartifls::from_bits(18)) }
    }
    #[doc = "Interrupt Mask Set/Clear Register, UARTIMSC"]
    pub fn uartimsc(self) -> Reg<fields::Uartimsc, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Uartimsc::from_bits(0)) }
    }
    #[doc = "Raw Interrupt Status Register, UARTRIS"]
    pub fn uartris(self) -> Reg<fields::Uartris, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Uartris::from_bits(0)) }
    }
    #[doc = "Masked Interrupt Status Register, UARTMIS"]
    pub fn uartmis(self) -> Reg<fields::Uartmis, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Uartmis::from_bits(0)) }
    }
    #[doc = "Interrupt Clear Register, UARTICR"]
    pub fn uarticr(self) -> Reg<fields::Uarticr, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::Uarticr::from_bits(0)) }
    }
    #[doc = "DMA Control Register, UARTDMACR"]
    pub fn uartdmacr(self) -> Reg<fields::Uartdmacr, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::Uartdmacr::from_bits(0)) }
    }
    #[doc = "UARTPeriphID0 Register"]
    pub fn uartperiphid0(self) -> Reg<fields::Uartperiphid0, RW> {
        unsafe { Reg::new(self.0.add(4064usize), fields::Uartperiphid0::from_bits(17)) }
    }
    #[doc = "UARTPeriphID1 Register"]
    pub fn uartperiphid1(self) -> Reg<fields::Uartperiphid1, RW> {
        unsafe { Reg::new(self.0.add(4068usize), fields::Uartperiphid1::from_bits(16)) }
    }
    #[doc = "UARTPeriphID2 Register"]
    pub fn uartperiphid2(self) -> Reg<fields::Uartperiphid2, RW> {
        unsafe { Reg::new(self.0.add(4072usize), fields::Uartperiphid2::from_bits(52)) }
    }
    #[doc = "UARTPeriphID3 Register"]
    pub fn uartperiphid3(self) -> Reg<fields::Uartperiphid3, RW> {
        unsafe { Reg::new(self.0.add(4076usize), fields::Uartperiphid3::from_bits(0)) }
    }
    #[doc = "UARTPCellID0 Register"]
    pub fn uartpcellid0(self) -> Reg<fields::Uartpcellid0, RW> {
        unsafe { Reg::new(self.0.add(4080usize), fields::Uartpcellid0::from_bits(13)) }
    }
    #[doc = "UARTPCellID1 Register"]
    pub fn uartpcellid1(self) -> Reg<fields::Uartpcellid1, RW> {
        unsafe { Reg::new(self.0.add(4084usize), fields::Uartpcellid1::from_bits(240)) }
    }
    #[doc = "UARTPCellID2 Register"]
    pub fn uartpcellid2(self) -> Reg<fields::Uartpcellid2, RW> {
        unsafe { Reg::new(self.0.add(4088usize), fields::Uartpcellid2::from_bits(5)) }
    }
    #[doc = "UARTPCellID3 Register"]
    pub fn uartpcellid3(self) -> Reg<fields::Uartpcellid3, RW> {
        unsafe { Reg::new(self.0.add(4092usize), fields::Uartpcellid3::from_bits(177)) }
    }
}
pub mod fields;
