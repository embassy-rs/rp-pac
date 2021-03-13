use crate::generic::*;
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
#[derive(Copy, Clone)]
pub struct Tbman(pub *mut u8);
unsafe impl Send for Tbman {}
unsafe impl Sync for Tbman {}
impl Tbman {
    #[doc = "Indicates the type of platform in use"]
    pub fn platform(self) -> Reg<regs::Platform, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod regs;
