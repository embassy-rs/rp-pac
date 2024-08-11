#[doc = "Additional registers mapped adjacent to the bootram, for use by the bootrom."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootram {
    ptr: *mut u8,
}
unsafe impl Send for Bootram {}
unsafe impl Sync for Bootram {}
impl Bootram {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
    #[inline(always)]
    pub const fn write_once0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0800usize) as _) }
    }
    #[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
    #[inline(always)]
    pub const fn write_once1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0804usize) as _) }
    }
    #[doc = "Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use."]
    #[inline(always)]
    pub const fn bootlock_stat(self) -> crate::common::Reg<regs::BootlockStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0808usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock(self, n: usize) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 8usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x080cusize + n * 4usize) as _) }
    }
}
pub mod regs;
