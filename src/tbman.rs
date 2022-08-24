#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Tbman(pub *mut u8);
unsafe impl Send for Tbman {}
unsafe impl Sync for Tbman {}
impl Tbman {
    #[doc = "Indicates the type of platform in use"]
    #[inline(always)]
    pub fn platform(self) -> crate::common::Reg<regs::Platform, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
}
pub mod regs;
