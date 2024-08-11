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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2048usize) as _) }
    }
    #[doc = "This registers always ORs writes into its current contents. Once a bit is set, it can only be cleared by a reset."]
    #[inline(always)]
    pub const fn write_once1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2052usize) as _) }
    }
    #[doc = "Bootlock status register. 1=unclaimed, 0=claimed. These locks function identically to the SIO spinlocks, but are reserved for bootrom use."]
    #[inline(always)]
    pub const fn bootlock_stat(self) -> crate::common::Reg<regs::BootlockStat, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2056usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2060usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2064usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2068usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2072usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2076usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2080usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2084usize) as _) }
    }
    #[doc = "Read to claim and check. Write to unclaim. The value returned on successful claim is 1 << n, and on failed claim is zero."]
    #[inline(always)]
    pub const fn bootlock7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2088usize) as _) }
    }
}
pub mod regs;
