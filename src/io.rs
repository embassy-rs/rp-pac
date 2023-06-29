#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Gpio {
    ptr: *mut u8,
}
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "GPIO status"]
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::GpioStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "GPIO control including function select and overrides."]
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int {
    ptr: *mut u8,
}
unsafe impl Send for Int {}
unsafe impl Sync for Int {}
impl Int {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Interrupt Enable for proc0"]
    #[inline(always)]
    pub const fn inte(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Force for proc0"]
    #[inline(always)]
    pub const fn intf(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    #[inline(always)]
    pub const fn ints(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize + n * 4usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Io {
    ptr: *mut u8,
}
unsafe impl Send for Io {}
unsafe impl Sync for Io {}
impl Io {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[inline(always)]
    pub const fn gpio(self, n: usize) -> Gpio {
        assert!(n < 30usize);
        unsafe { Gpio::from_ptr(self.ptr.add(0usize + n * 8usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 4usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn int_proc(self, n: usize) -> Int {
        assert!(n < 2usize);
        unsafe { Int::from_ptr(self.ptr.add(256usize + n * 48usize) as _) }
    }
    #[inline(always)]
    pub const fn int_dormant_wake(self) -> Int {
        unsafe { Int::from_ptr(self.ptr.add(352usize) as _) }
    }
}
pub mod regs;
pub mod vals;
