#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Spi {
    ptr: *mut u8,
}
unsafe impl Send for Spi {}
unsafe impl Sync for Spi {}
impl Spi {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control register 0, SSPCR0 on page 3-4"]
    #[inline(always)]
    pub const fn cr0(self) -> crate::common::Reg<regs::Cr0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    #[inline(always)]
    pub const fn cr1(self) -> crate::common::Reg<regs::Cr1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    #[inline(always)]
    pub const fn dr(self) -> crate::common::Reg<regs::Dr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    #[inline(always)]
    pub const fn sr(self) -> crate::common::Reg<regs::Sr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    #[inline(always)]
    pub const fn cpsr(self) -> crate::common::Reg<regs::Cpsr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    #[inline(always)]
    pub const fn imsc(self) -> crate::common::Reg<regs::Imsc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    #[inline(always)]
    pub const fn ris(self) -> crate::common::Reg<regs::Ris, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    #[inline(always)]
    pub const fn mis(self) -> crate::common::Reg<regs::Mis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    #[inline(always)]
    pub const fn icr(self) -> crate::common::Reg<regs::Icr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    #[inline(always)]
    pub const fn dmacr(self) -> crate::common::Reg<regs::Dmacr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub const fn periphid0(self) -> crate::common::Reg<regs::Periphid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4064usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub const fn periphid1(self) -> crate::common::Reg<regs::Periphid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4068usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub const fn periphid2(self) -> crate::common::Reg<regs::Periphid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4072usize) as _) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    #[inline(always)]
    pub const fn periphid3(self) -> crate::common::Reg<regs::Periphid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4076usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub const fn pcellid0(self) -> crate::common::Reg<regs::Pcellid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4080usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub const fn pcellid1(self) -> crate::common::Reg<regs::Pcellid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4084usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub const fn pcellid2(self) -> crate::common::Reg<regs::Pcellid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4088usize) as _) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    #[inline(always)]
    pub const fn pcellid3(self) -> crate::common::Reg<regs::Pcellid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4092usize) as _) }
    }
}
pub mod regs;
