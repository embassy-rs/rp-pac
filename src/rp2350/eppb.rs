#[doc = "Cortex-M33 EPPB vendor register block for RP2350"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Eppb {
    ptr: *mut u8,
}
unsafe impl Send for Eppb {}
unsafe impl Sync for Eppb {}
impl Eppb {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "NMI mask for IRQs 0 through 31. This register is core-local, and is reset by a processor warm reset."]
    #[inline(always)]
    pub const fn nmi_mask0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "NMI mask for IRQs 0 though 51. This register is core-local, and is reset by a processor warm reset."]
    #[inline(always)]
    pub const fn nmi_mask1(self) -> crate::common::Reg<regs::NmiMask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Nonstandard sleep control register"]
    #[inline(always)]
    pub const fn sleepctrl(self) -> crate::common::Reg<regs::Sleepctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
}
pub mod regs;
