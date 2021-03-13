use crate::generic::*;
#[doc = "Control register for lane 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp0CtrlLane0(pub u32);
impl Interp0CtrlLane0 {
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    pub const fn overf(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    pub fn set_overf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    pub const fn overf1(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    pub fn set_overf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    pub const fn overf0(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    pub fn set_overf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    #[doc = "Only present on INTERP0 on each core. If BLEND mode is enabled: - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled by the 8 LSBs of lane 1 shift and mask value (a fractional number between 0 and 255/256ths) - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value) - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask) LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    pub const fn blend(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    #[doc = "Only present on INTERP0 on each core. If BLEND mode is enabled: - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled by the 8 LSBs of lane 1 shift and mask value (a fractional number between 0 and 255/256ths) - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value) - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask) LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    pub fn set_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19u32) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19u32)) | (((val as u32) & 0x03) << 19u32);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10u32) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10u32)) | (((val as u32) & 0x1f) << 10u32);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5u32) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5u32)) | (((val as u32) & 0x1f) << 5u32);
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Interp0CtrlLane0 {
    fn default() -> Interp0CtrlLane0 {
        Interp0CtrlLane0(0)
    }
}
#[doc = "Control register for lane 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp1CtrlLane1(pub u32);
impl Interp1CtrlLane1 {
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19u32) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19u32)) | (((val as u32) & 0x03) << 19u32);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10u32) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10u32)) | (((val as u32) & 0x1f) << 10u32);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5u32) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5u32)) | (((val as u32) & 0x1f) << 5u32);
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Interp1CtrlLane1 {
    fn default() -> Interp1CtrlLane1 {
        Interp1CtrlLane1(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp1Accum1Add(pub u32);
impl Interp1Accum1Add {
    pub const fn interp1_accum1_add(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    pub fn set_interp1_accum1_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Interp1Accum1Add {
    fn default() -> Interp1Accum1Add {
        Interp1Accum1Add(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp0Accum0Add(pub u32);
impl Interp0Accum0Add {
    pub const fn interp0_accum0_add(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    pub fn set_interp0_accum0_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Interp0Accum0Add {
    fn default() -> Interp0Accum0Add {
        Interp0Accum0Add(0)
    }
}
#[doc = "Control register for lane 0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp1CtrlLane0(pub u32);
impl Interp1CtrlLane0 {
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    pub const fn overf(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    pub fn set_overf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    pub const fn overf1(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    pub fn set_overf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    pub const fn overf0(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    pub fn set_overf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    #[doc = "Only present on INTERP1 on each core. If CLAMP mode is enabled: - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of BASE0 and an upper bound of BASE1. - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED"]
    pub const fn clamp(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    #[doc = "Only present on INTERP1 on each core. If CLAMP mode is enabled: - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of BASE0 and an upper bound of BASE1. - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED"]
    pub fn set_clamp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19u32) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19u32)) | (((val as u32) & 0x03) << 19u32);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10u32) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10u32)) | (((val as u32) & 0x1f) << 10u32);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5u32) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5u32)) | (((val as u32) & 0x1f) << 5u32);
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Interp1CtrlLane0 {
    fn default() -> Interp1CtrlLane0 {
        Interp1CtrlLane0(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp1Accum0Add(pub u32);
impl Interp1Accum0Add {
    pub const fn interp1_accum0_add(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    pub fn set_interp1_accum0_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Interp1Accum0Add {
    fn default() -> Interp1Accum0Add {
        Interp1Accum0Add(0)
    }
}
#[doc = "Control register for lane 1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp0CtrlLane1(pub u32);
impl Interp0CtrlLane1 {
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19u32) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19u32)) | (((val as u32) & 0x03) << 19u32);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10u32) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10u32)) | (((val as u32) & 0x1f) << 10u32);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5u32) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5u32)) | (((val as u32) & 0x1f) << 5u32);
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Logical right-shift applied to accumulator before masking"]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for Interp0CtrlLane1 {
    fn default() -> Interp0CtrlLane1 {
        Interp0CtrlLane1(0)
    }
}
#[doc = "Control and status register for divider."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DivCsr(pub u32);
impl DivCsr {
    #[doc = "Changes to 1 when any register is written, and back to 0 when QUOTIENT is read. Software can use this flag to make save/restore more efficient (skip if not DIRTY). If the flag is used in this way, it's recommended to either read QUOTIENT only, or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
    pub const fn dirty(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Changes to 1 when any register is written, and back to 0 when QUOTIENT is read. Software can use this flag to make save/restore more efficient (skip if not DIRTY). If the flag is used in this way, it's recommended to either read QUOTIENT only, or REMAINDER and then QUOTIENT, to prevent data loss on context switch."]
    pub fn set_dirty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Reads as 0 when a calculation is in progress, 1 otherwise. Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no matter if one is already in progress. Writing to a result register will immediately terminate any in-progress calculation and set the READY and DIRTY flags."]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Reads as 0 when a calculation is in progress, 1 otherwise. Writing an operand (xDIVIDEND, xDIVISOR) will immediately start a new calculation, no matter if one is already in progress. Writing to a result register will immediately terminate any in-progress calculation and set the READY and DIRTY flags."]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for DivCsr {
    fn default() -> DivCsr {
        DivCsr(0)
    }
}
#[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct FifoSt(pub u32);
impl FifoSt {
    #[doc = "Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    pub const fn roe(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    pub fn set_roe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    pub const fn wof(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    pub fn set_wof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    pub fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    pub fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for FifoSt {
    fn default() -> FifoSt {
        FifoSt(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Interp0Accum1Add(pub u32);
impl Interp0Accum1Add {
    pub const fn interp0_accum1_add(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    pub fn set_interp0_accum1_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for Interp0Accum1Add {
    fn default() -> Interp0Accum1Add {
        Interp0Accum1Add(0)
    }
}
