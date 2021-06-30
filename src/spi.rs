#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi(pub *mut u8);
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[doc = "Control register 0, SSPCR0 on page 3-4"]
    pub fn cr0(self) -> crate::common::Reg<regs::cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    pub fn cr1(self) -> crate::common::Reg<regs::cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    pub fn dr(self) -> crate::common::Reg<regs::dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    pub fn sr(self) -> crate::common::Reg<regs::sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    pub fn cpsr(self) -> crate::common::Reg<regs::cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub fn imsc(self) -> crate::common::Reg<regs::imsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    pub fn ris(self) -> crate::common::Reg<regs::ris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    pub fn mis(self) -> crate::common::Reg<regs::mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    pub fn icr(self) -> crate::common::Reg<regs::icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    pub fn dmacr(self) -> crate::common::Reg<regs::dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn periphid0(self) -> crate::common::Reg<regs::periphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4064usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn periphid1(self) -> crate::common::Reg<regs::periphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4068usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn periphid2(self) -> crate::common::Reg<regs::periphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4072usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn periphid3(self) -> crate::common::Reg<regs::periphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4076usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn pcellid0(self) -> crate::common::Reg<regs::pcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4080usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn pcellid1(self) -> crate::common::Reg<regs::pcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4084usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn pcellid2(self) -> crate::common::Reg<regs::pcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4088usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn pcellid3(self) -> crate::common::Reg<regs::pcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
