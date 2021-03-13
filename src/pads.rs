use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Pads(pub *mut u8);
unsafe impl Send for Pads {}
unsafe impl Sync for Pads {}
impl Pads {
    #[doc = "Voltage select. Per bank control"]
    pub fn voltage_select(self) -> Reg<regs::VoltageSelect, RW> {
        unsafe { Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "Pad control register"]
    pub fn gpio(self, n: usize) -> Reg<regs::GpioCtrl, RW> {
        assert!(n < 32usize);
        unsafe { Reg::from_ptr(self.0.add(4usize + n * 4usize)) }
    }
}
pub mod regs;
pub mod vals;
