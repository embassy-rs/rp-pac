#[doc = "SHA-256 hash function implementation"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Sha256 {
    ptr: *mut u8,
}
unsafe impl Send for Sha256 {}
unsafe impl Sync for Sha256 {}
impl Sha256 {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Control and status register"]
    #[inline(always)]
    pub const fn csr(self) -> crate::common::Reg<regs::Csr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Write data register"]
    #[inline(always)]
    pub const fn wdata(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "256-bit checksum result. Contents are undefined when CSR_SUM_VLD is 0."]
    #[inline(always)]
    pub const fn sum7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
}
pub mod regs;
pub mod vals;
