use crate::generic::*;
#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
#[derive(Copy, Clone)]
pub struct VregAndChipReset(*mut u8);
unsafe impl Send for VregAndChipReset {}
unsafe impl Sync for VregAndChipReset {}
impl VregAndChipReset {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Voltage regulator control and status"]
    pub fn vreg(self) -> Reg<fields::Vreg, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Vreg::from_bits(177)) }
    }
    #[doc = "brown-out detection control"]
    pub fn bod(self) -> Reg<fields::Bod, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Bod::from_bits(145)) }
    }
    #[doc = "Chip reset control and status"]
    pub fn chip_reset(self) -> Reg<fields::ChipReset, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::ChipReset::from_bits(0)) }
    }
}
pub mod fields;
