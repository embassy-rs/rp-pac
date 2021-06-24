#[derive(Copy, Clone)]
pub struct Spi(pub *mut u8);
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[doc = "Control register 0, SSPCR0 on page 3-4"]
    pub fn sspcr0(self) -> crate::common::Reg<regs::Sspcr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    pub fn sspcr1(self) -> crate::common::Reg<regs::Sspcr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    pub fn sspdr(self) -> crate::common::Reg<regs::Sspdr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    pub fn sspsr(self) -> crate::common::Reg<regs::Sspsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    pub fn sspcpsr(self) -> crate::common::Reg<regs::Sspcpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub fn sspimsc(self) -> crate::common::Reg<regs::Sspimsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    pub fn sspris(self) -> crate::common::Reg<regs::Sspris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    pub fn sspmis(self) -> crate::common::Reg<regs::Sspmis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    pub fn sspicr(self) -> crate::common::Reg<regs::Sspicr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    pub fn sspdmacr(self) -> crate::common::Reg<regs::Sspdmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid0(self) -> crate::common::Reg<regs::Sspperiphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4064usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid1(self) -> crate::common::Reg<regs::Sspperiphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4068usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid2(self) -> crate::common::Reg<regs::Sspperiphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4072usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid3(self) -> crate::common::Reg<regs::Sspperiphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4076usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid0(self) -> crate::common::Reg<regs::Ssppcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4080usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid1(self) -> crate::common::Reg<regs::Ssppcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4084usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid2(self) -> crate::common::Reg<regs::Ssppcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4088usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid3(self) -> crate::common::Reg<regs::Ssppcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
