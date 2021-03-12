use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Spi0(*mut u8);
unsafe impl Send for Spi0 {}
unsafe impl Sync for Spi0 {}
impl Spi0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Control register 0, SSPCR0 on page 3-4"]
    pub fn sspcr0(self) -> Reg<regs::Sspcr0, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    pub fn sspcr1(self) -> Reg<regs::Sspcr1, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    pub fn sspdr(self) -> Reg<regs::Sspdr, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    pub fn sspsr(self) -> Reg<regs::Sspsr, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    pub fn sspcpsr(self) -> Reg<regs::Sspcpsr, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub fn sspimsc(self) -> Reg<regs::Sspimsc, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    pub fn sspris(self) -> Reg<regs::Sspris, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    pub fn sspmis(self) -> Reg<regs::Sspmis, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    pub fn sspicr(self) -> Reg<regs::Sspicr, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    pub fn sspdmacr(self) -> Reg<regs::Sspdmacr, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid0(self) -> Reg<regs::Sspperiphid0, RW> {
        unsafe { Reg::new(self.0.add(4064usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid1(self) -> Reg<regs::Sspperiphid1, RW> {
        unsafe { Reg::new(self.0.add(4068usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid2(self) -> Reg<regs::Sspperiphid2, RW> {
        unsafe { Reg::new(self.0.add(4072usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid3(self) -> Reg<regs::Sspperiphid3, RW> {
        unsafe { Reg::new(self.0.add(4076usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid0(self) -> Reg<regs::Ssppcellid0, RW> {
        unsafe { Reg::new(self.0.add(4080usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid1(self) -> Reg<regs::Ssppcellid1, RW> {
        unsafe { Reg::new(self.0.add(4084usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid2(self) -> Reg<regs::Ssppcellid2, RW> {
        unsafe { Reg::new(self.0.add(4088usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid3(self) -> Reg<regs::Ssppcellid3, RW> {
        unsafe { Reg::new(self.0.add(4092usize)) }
    }
}
pub mod regs;
