use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Int(pub *mut u8);
unsafe impl Send for Int {}
unsafe impl Sync for Int {}
impl Int {
    #[doc = "Interrupt Enable for proc1"]
    pub fn inte(self, n: usize) -> Reg<regs::Int, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(0usize + n * 4usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn intf(self, n: usize) -> Reg<regs::Int, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(16usize + n * 4usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn ints(self, n: usize) -> Reg<regs::Int, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(32usize + n * 4usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Io(pub *mut u8);
unsafe impl Send for Io {}
unsafe impl Sync for Io {}
impl Io {
    pub fn gpio(self, n: usize) -> Gpio {
        assert!(n < 30usize);
        unsafe { Gpio(self.0.add(0usize + n * 8usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr(self, n: usize) -> Reg<regs::Int, RW> {
        assert!(n < 4usize);
        unsafe { Reg::from_ptr(self.0.add(240usize + n * 4usize)) }
    }
    pub fn int_dormant_wake(self) -> Int {
        unsafe { Int(self.0.add(352usize)) }
    }
    pub fn int_proc(self, n: usize) -> Int {
        assert!(n < 2usize);
        unsafe { Int(self.0.add(256usize + n * 48usize)) }
    }
}
#[derive(Copy, Clone)]
pub struct Gpio(pub *mut u8);
unsafe impl Send for Gpio {}
unsafe impl Sync for Gpio {}
impl Gpio {
    #[doc = "GPIO status"]
    pub fn status(self) -> Reg<regs::GpioStatus, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn ctrl(self) -> Reg<regs::GpioCtrl, RW> {
        unsafe { Reg::from_ptr(self.0.add(4usize)) }
    }
}
pub mod regs;
pub mod vals;
