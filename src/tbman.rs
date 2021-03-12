use crate::generic::*;
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
#[derive(Copy, Clone)]
pub struct Tbman(*mut u8);
unsafe impl Send for Tbman {}
unsafe impl Sync for Tbman {}
impl Tbman {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Indicates the type of platform in use"]
    pub fn platform(self) -> Reg<regs::Platform, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
}
pub mod regs;
