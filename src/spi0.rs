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
    pub fn sspcr0(self) -> Reg<fields::Sspcr0, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Sspcr0::from_bits(0)) }
    }
    #[doc = "Control register 1, SSPCR1 on page 3-5"]
    pub fn sspcr1(self) -> Reg<fields::Sspcr1, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Sspcr1::from_bits(0)) }
    }
    #[doc = "Data register, SSPDR on page 3-6"]
    pub fn sspdr(self) -> Reg<fields::Sspdr, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Sspdr::from_bits(0)) }
    }
    #[doc = "Status register, SSPSR on page 3-7"]
    pub fn sspsr(self) -> Reg<fields::Sspsr, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Sspsr::from_bits(3)) }
    }
    #[doc = "Clock prescale register, SSPCPSR on page 3-8"]
    pub fn sspcpsr(self) -> Reg<fields::Sspcpsr, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Sspcpsr::from_bits(0)) }
    }
    #[doc = "Interrupt mask set or clear register, SSPIMSC on page 3-9"]
    pub fn sspimsc(self) -> Reg<fields::Sspimsc, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Sspimsc::from_bits(0)) }
    }
    #[doc = "Raw interrupt status register, SSPRIS on page 3-10"]
    pub fn sspris(self) -> Reg<fields::Sspris, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Sspris::from_bits(8)) }
    }
    #[doc = "Masked interrupt status register, SSPMIS on page 3-11"]
    pub fn sspmis(self) -> Reg<fields::Sspmis, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Sspmis::from_bits(0)) }
    }
    #[doc = "Interrupt clear register, SSPICR on page 3-11"]
    pub fn sspicr(self) -> Reg<fields::Sspicr, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Sspicr::from_bits(0)) }
    }
    #[doc = "DMA control register, SSPDMACR on page 3-12"]
    pub fn sspdmacr(self) -> Reg<fields::Sspdmacr, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Sspdmacr::from_bits(0)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid0(self) -> Reg<fields::Sspperiphid0, RW> {
        unsafe { Reg::new(self.0.add(4064usize), fields::Sspperiphid0::from_bits(34)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid1(self) -> Reg<fields::Sspperiphid1, RW> {
        unsafe { Reg::new(self.0.add(4068usize), fields::Sspperiphid1::from_bits(16)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid2(self) -> Reg<fields::Sspperiphid2, RW> {
        unsafe { Reg::new(self.0.add(4072usize), fields::Sspperiphid2::from_bits(52)) }
    }
    #[doc = "Peripheral identification registers, SSPPeriphID0-3 on page 3-13"]
    pub fn sspperiphid3(self) -> Reg<fields::Sspperiphid3, RW> {
        unsafe { Reg::new(self.0.add(4076usize), fields::Sspperiphid3::from_bits(0)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid0(self) -> Reg<fields::Ssppcellid0, RW> {
        unsafe { Reg::new(self.0.add(4080usize), fields::Ssppcellid0::from_bits(13)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid1(self) -> Reg<fields::Ssppcellid1, RW> {
        unsafe { Reg::new(self.0.add(4084usize), fields::Ssppcellid1::from_bits(240)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid2(self) -> Reg<fields::Ssppcellid2, RW> {
        unsafe { Reg::new(self.0.add(4088usize), fields::Ssppcellid2::from_bits(5)) }
    }
    #[doc = "PrimeCell identification registers, SSPPCellID0-3 on page 3-16"]
    pub fn ssppcellid3(self) -> Reg<fields::Ssppcellid3, RW> {
        unsafe { Reg::new(self.0.add(4092usize), fields::Ssppcellid3::from_bits(177)) }
    }
}
pub mod fields;
