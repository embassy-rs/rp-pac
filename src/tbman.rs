#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbman {
    ptr: *mut u8,
}
unsafe impl Send for Tbman {}
unsafe impl Sync for Tbman {}
impl Tbman {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Indicates the type of platform in use"]
    #[inline(always)]
    pub const fn platform(self) -> crate::common::Reg<regs::Platform, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
}
pub mod regs;
