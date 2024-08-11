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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
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
    #[doc = "Interrupt Enable for proc1"]
    #[inline(always)]
    pub const fn inte(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt Force for proc1"]
    #[inline(always)]
    pub const fn intf(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize + n * 4usize) as _) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    #[inline(always)]
    pub const fn ints(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize + n * 4usize) as _) }
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
        unsafe { Gpio::from_ptr(self.ptr.add(0usize + n * 8usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_secure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc0secure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_secure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc0secure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_nonsecure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc0nonsecure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc0_nonsecure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc0nonsecure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_secure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc1secure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_secure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc1secure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_nonsecure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc1nonsecure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_proc1_nonsecure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryProc1nonsecure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_secure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryDormantWakeSecure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_secure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryDormantWakeSecure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(548usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_nonsecure0(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryDormantWakeNonsecure0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(552usize) as _) }
    }
    #[inline(always)]
    pub const fn irqsummary_dormant_wake_nonsecure1(
        self,
    ) -> crate::common::Reg<regs::IrqsummaryDormantWakeNonsecure1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(556usize) as _) }
    }
    #[doc = "Raw Interrupts"]
    #[inline(always)]
    pub const fn intr(self, n: usize) -> crate::common::Reg<regs::Int, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn int_proc(self, n: usize) -> Int {
        assert!(n < 2usize);
        unsafe { Int::from_ptr(self.ptr.add(584usize + n * 72usize) as _) }
    }
    #[inline(always)]
    pub const fn int_dormant_wake(self) -> Int {
        unsafe { Int::from_ptr(self.ptr.add(728usize) as _) }
    }
}
pub mod regs;
pub mod vals;
