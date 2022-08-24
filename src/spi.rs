#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi(pub *mut u8);
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[doc = "Control register 0, SSPCR0 on page 3-4"]
    #[inline(always)]
    pub fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    #[inline(always)]
    pub fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    #[inline(always)]
    pub fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    #[inline(always)]
    pub fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(12usize)) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    #[inline(always)]
    pub fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(16usize)) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    #[inline(always)]
    pub fn imsc(self) -> crate::common::Reg<regs::Imsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(20usize)) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    #[inline(always)]
    pub fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(24usize)) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    #[inline(always)]
    pub fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(28usize)) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    #[inline(always)]
    pub fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(32usize)) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    #[inline(always)]
    pub fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(36usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub fn periphid0(self) -> crate::common::Reg<regs::Periphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4064usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub fn periphid1(self) -> crate::common::Reg<regs::Periphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4068usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub fn periphid2(self) -> crate::common::Reg<regs::Periphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4072usize)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub fn periphid3(self) -> crate::common::Reg<regs::Periphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4076usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub fn pcellid0(self) -> crate::common::Reg<regs::Pcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4080usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub fn pcellid1(self) -> crate::common::Reg<regs::Pcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4084usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub fn pcellid2(self) -> crate::common::Reg<regs::Pcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4088usize)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub fn pcellid3(self) -> crate::common::Reg<regs::Pcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4092usize)) }
    }
}
pub mod regs;
