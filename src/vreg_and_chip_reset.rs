#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
#[derive(Copy, Clone)]
pub struct VregAndChipReset(pub *mut u8);
unsafe impl Send for VregAndChipReset {}
unsafe impl Sync for VregAndChipReset {}
impl VregAndChipReset {
    #[doc = "Voltage regulator control and status"]
    pub fn vreg(self) -> crate::common::Reg<regs::Vreg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(0usize)) }
    }
    #[doc = "brown-out detection control"]
    pub fn bod(self) -> crate::common::Reg<regs::Bod, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(4usize)) }
    }
    #[doc = "Chip reset control and status"]
    pub fn chip_reset(self) -> crate::common::Reg<regs::ChipReset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.0.add(8usize)) }
    }
}
pub mod regs;
