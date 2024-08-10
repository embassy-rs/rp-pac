#[doc = "SNPS OTP control IF (SBPI and RPi wrapper control)"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Otp {
    ptr: *mut u8,
}
unsafe impl Send for Otp {}
unsafe impl Sync for Otp {}
impl Otp {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Software lock register for page 0. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock0(self) -> crate::common::Reg<regs::SwLock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Software lock register for page 1. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock1(self) -> crate::common::Reg<regs::SwLock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Software lock register for page 2. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock2(self) -> crate::common::Reg<regs::SwLock2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Software lock register for page 3. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock3(self) -> crate::common::Reg<regs::SwLock3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Software lock register for page 4. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock4(self) -> crate::common::Reg<regs::SwLock4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Software lock register for page 5. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock5(self) -> crate::common::Reg<regs::SwLock5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Software lock register for page 6. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock6(self) -> crate::common::Reg<regs::SwLock6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Software lock register for page 7. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock7(self) -> crate::common::Reg<regs::SwLock7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Software lock register for page 8. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock8(self) -> crate::common::Reg<regs::SwLock8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Software lock register for page 9. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock9(self) -> crate::common::Reg<regs::SwLock9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Software lock register for page 10. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock10(self) -> crate::common::Reg<regs::SwLock10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Software lock register for page 11. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock11(self) -> crate::common::Reg<regs::SwLock11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Software lock register for page 12. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock12(self) -> crate::common::Reg<regs::SwLock12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Software lock register for page 13. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock13(self) -> crate::common::Reg<regs::SwLock13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Software lock register for page 14. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock14(self) -> crate::common::Reg<regs::SwLock14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Software lock register for page 15. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock15(self) -> crate::common::Reg<regs::SwLock15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Software lock register for page 16. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock16(self) -> crate::common::Reg<regs::SwLock16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Software lock register for page 17. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock17(self) -> crate::common::Reg<regs::SwLock17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Software lock register for page 18. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock18(self) -> crate::common::Reg<regs::SwLock18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Software lock register for page 19. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock19(self) -> crate::common::Reg<regs::SwLock19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Software lock register for page 20. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock20(self) -> crate::common::Reg<regs::SwLock20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Software lock register for page 21. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock21(self) -> crate::common::Reg<regs::SwLock21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Software lock register for page 22. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock22(self) -> crate::common::Reg<regs::SwLock22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Software lock register for page 23. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock23(self) -> crate::common::Reg<regs::SwLock23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Software lock register for page 24. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock24(self) -> crate::common::Reg<regs::SwLock24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Software lock register for page 25. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock25(self) -> crate::common::Reg<regs::SwLock25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Software lock register for page 26. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock26(self) -> crate::common::Reg<regs::SwLock26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Software lock register for page 27. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock27(self) -> crate::common::Reg<regs::SwLock27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Software lock register for page 28. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock28(self) -> crate::common::Reg<regs::SwLock28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Software lock register for page 29. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock29(self) -> crate::common::Reg<regs::SwLock29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Software lock register for page 30. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock30(self) -> crate::common::Reg<regs::SwLock30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Software lock register for page 31. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock31(self) -> crate::common::Reg<regs::SwLock31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "Software lock register for page 32. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock32(self) -> crate::common::Reg<regs::SwLock32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Software lock register for page 33. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock33(self) -> crate::common::Reg<regs::SwLock33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Software lock register for page 34. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock34(self) -> crate::common::Reg<regs::SwLock34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Software lock register for page 35. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock35(self) -> crate::common::Reg<regs::SwLock35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Software lock register for page 36. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock36(self) -> crate::common::Reg<regs::SwLock36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Software lock register for page 37. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock37(self) -> crate::common::Reg<regs::SwLock37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "Software lock register for page 38. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock38(self) -> crate::common::Reg<regs::SwLock38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "Software lock register for page 39. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock39(self) -> crate::common::Reg<regs::SwLock39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "Software lock register for page 40. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock40(self) -> crate::common::Reg<regs::SwLock40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Software lock register for page 41. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock41(self) -> crate::common::Reg<regs::SwLock41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "Software lock register for page 42. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock42(self) -> crate::common::Reg<regs::SwLock42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "Software lock register for page 43. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock43(self) -> crate::common::Reg<regs::SwLock43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "Software lock register for page 44. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock44(self) -> crate::common::Reg<regs::SwLock44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Software lock register for page 45. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock45(self) -> crate::common::Reg<regs::SwLock45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "Software lock register for page 46. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock46(self) -> crate::common::Reg<regs::SwLock46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "Software lock register for page 47. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock47(self) -> crate::common::Reg<regs::SwLock47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "Software lock register for page 48. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock48(self) -> crate::common::Reg<regs::SwLock48, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Software lock register for page 49. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock49(self) -> crate::common::Reg<regs::SwLock49, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Software lock register for page 50. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock50(self) -> crate::common::Reg<regs::SwLock50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "Software lock register for page 51. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock51(self) -> crate::common::Reg<regs::SwLock51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "Software lock register for page 52. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock52(self) -> crate::common::Reg<regs::SwLock52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Software lock register for page 53. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock53(self) -> crate::common::Reg<regs::SwLock53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "Software lock register for page 54. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock54(self) -> crate::common::Reg<regs::SwLock54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "Software lock register for page 55. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock55(self) -> crate::common::Reg<regs::SwLock55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(220usize) as _) }
    }
    #[doc = "Software lock register for page 56. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock56(self) -> crate::common::Reg<regs::SwLock56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Software lock register for page 57. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock57(self) -> crate::common::Reg<regs::SwLock57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Software lock register for page 58. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock58(self) -> crate::common::Reg<regs::SwLock58, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "Software lock register for page 59. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock59(self) -> crate::common::Reg<regs::SwLock59, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(236usize) as _) }
    }
    #[doc = "Software lock register for page 60. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock60(self) -> crate::common::Reg<regs::SwLock60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "Software lock register for page 61. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock61(self) -> crate::common::Reg<regs::SwLock61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "Software lock register for page 62. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock62(self) -> crate::common::Reg<regs::SwLock62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "Software lock register for page 63. Locks are initialised from the OTP lock pages at reset. This register can be written to further advance the lock state of each page (until next reset), and read to check the current lock state of a page."]
    #[inline(always)]
    pub const fn sw_lock63(self) -> crate::common::Reg<regs::SwLock63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "Dispatch instructions to the SBPI interface, used for programming the OTP fuses."]
    #[inline(always)]
    pub const fn sbpi_instr(self) -> crate::common::Reg<regs::SbpiInstr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "SBPI write payload bytes 3..0"]
    #[inline(always)]
    pub const fn sbpi_wdata_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "SBPI write payload bytes 7..4"]
    #[inline(always)]
    pub const fn sbpi_wdata_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "SBPI write payload bytes 11..8"]
    #[inline(always)]
    pub const fn sbpi_wdata_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "SBPI write payload bytes 15..12"]
    #[inline(always)]
    pub const fn sbpi_wdata_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "Read payload bytes 3..0. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "Read payload bytes 7..4. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "Read payload bytes 11..8. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "Read payload bytes 15..12. Once read, the data in the register will automatically clear to 0."]
    #[inline(always)]
    pub const fn sbpi_rdata_3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[inline(always)]
    pub const fn sbpi_status(self) -> crate::common::Reg<regs::SbpiStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "Controls for APB data read interface (USER interface)"]
    #[inline(always)]
    pub const fn usr(self) -> crate::common::Reg<regs::Usr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "Debug for OTP power-on state machine"]
    #[inline(always)]
    pub const fn dbg(self) -> crate::common::Reg<regs::Dbg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "During BIST, count address locations that have at least one leaky bit"]
    #[inline(always)]
    pub const fn bist(self) -> crate::common::Reg<regs::Bist, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "Word 0 (bits 31..0) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "Word 1 (bits 63..32) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "Word 2 (bits 95..64) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "Word 3 (bits 127..96) of the key. Write only, read returns 0x0"]
    #[inline(always)]
    pub const fn crt_key_w3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Quickly check values of critical flags read during boot up"]
    #[inline(always)]
    pub const fn critical(self) -> crate::common::Reg<regs::Critical, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "Which keys were valid (enrolled) at boot time"]
    #[inline(always)]
    pub const fn key_valid(self) -> crate::common::Reg<regs::KeyValid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
    }
    #[doc = "Enable a debug feature that has been disabled. Debug features are disabled if one of the relevant critical boot flags is set in OTP (DEBUG_DISABLE or SECURE_DEBUG_DISABLE), OR if a debug key is marked valid in OTP, and the matching key value has not been supplied over SWD. Specifically: - The DEBUG_DISABLE flag disables all debug features. This can be fully overridden by setting all bits of this register. - The SECURE_DEBUG_DISABLE flag disables secure processor debug. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If a single debug key has been registered, and no matching key value has been supplied over SWD, then all debug features are disabled. This can be fully overridden by setting all bits of this register. - If both debug keys have been registered, and the Non-secure key's value (key 6) has been supplied over SWD, secure processor debug is disabled. This can be fully overridden by setting the PROC0_SECURE and PROC1_SECURE bits of this register. - If both debug keys have been registered, and the Secure key's value (key 5) has been supplied over SWD, then no debug features are disabled by the key mechanism. However, note that in this case debug features may still be disabled by the critical boot flags."]
    #[inline(always)]
    pub const fn debugen(self) -> crate::common::Reg<regs::Debugen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "Write 1s to lock corresponding bits in DEBUGEN. This register is reset by the processor cold reset."]
    #[inline(always)]
    pub const fn debugen_lock(self) -> crate::common::Reg<regs::DebugenLock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Architecture select (Arm/RISC-V). The default and allowable values of this register are constrained by the critical boot flags. This register is reset by the earliest reset in the switched core power domain (before a processor cold reset). Cores sample their architecture select signal on a warm reset. The source of the warm reset could be the system power-up state machine, the watchdog timer, Arm SYSRESETREQ or from RISC-V hartresetreq. Note that when an Arm core is deselected, its cold reset domain is also held in reset, since in particular the SYSRESETREQ bit becomes inaccessible once the core is deselected. Note also the RISC-V cores do not have a cold reset domain, since their corresponding controls are located in the Debug Module."]
    #[inline(always)]
    pub const fn archsel(self) -> crate::common::Reg<regs::Archsel, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Get the current architecture select state of each core. Cores sample the current value of the ARCHSEL register when their warm reset is released, at which point the corresponding bit in this register will also update."]
    #[inline(always)]
    pub const fn archsel_status(
        self,
    ) -> crate::common::Reg<regs::ArchselStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "Tell the bootrom to ignore scratch register boot vectors (both power manager and watchdog) on the next power up. If an early boot stage has soft-locked some OTP pages in order to protect their contents from later stages, there is a risk that Secure code running at a later stage can unlock the pages by performing a watchdog reset that resets the OTP. This register can be used to ensure that the bootloader runs as normal on the next power up, preventing Secure code at a later stage from accessing OTP in its unlocked state. Should be used in conjunction with the power manager BOOTDIS register."]
    #[inline(always)]
    pub const fn bootdis(self) -> crate::common::Reg<regs::Bootdis, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self) -> crate::common::Reg<regs::Intr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Interrupt Enable"]
    #[inline(always)]
    pub const fn inte(self) -> crate::common::Reg<regs::Inte, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "Interrupt Force"]
    #[inline(always)]
    pub const fn intf(self) -> crate::common::Reg<regs::Intf, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing"]
    #[inline(always)]
    pub const fn ints(self) -> crate::common::Reg<regs::Ints, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
}
pub mod regs;
pub mod vals;
