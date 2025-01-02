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
    #[inline(always)]
    pub const fn status(self) -> crate::common::Reg<regs::GpioStatus, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[inline(always)]
    pub const fn ctrl(self) -> crate::common::Reg<regs::GpioCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
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
    #[doc = "Interrupt Enable for proc1"]
    #[inline(always)]
    pub const fn inte(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Force for proc1"]
    #[inline(always)]
    pub const fn intf(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    #[inline(always)]
    pub const fn ints(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize + n * 4usize) as _) }
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
        assert!(n < 48usize);
        unsafe { Gpio::from_ptr(self.ptr.add(0x0usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_secure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0200usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_nonsecure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0208usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_secure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0210usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_nonsecure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0218usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_secure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0220usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_nonsecure(
        self,
        n: usize,
    ) -> crate::common::Reg<u32, crate::common::RW> {
        assert!(n < 2usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0228usize + n * 4usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0230usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn int_proc(self, n: usize) -> Int {
        assert!(n < 2usize);
        unsafe { Int::from_ptr(self.ptr.add(0x0248usize + n * 72usize) as _) }
    }
    #[inline(always)]
    pub const fn int_dormant_wake(self) -> Int {
        unsafe { Int::from_ptr(self.ptr.add(0x02d8usize) as _) }
    }
}
pub mod regs;
pub mod vals;
