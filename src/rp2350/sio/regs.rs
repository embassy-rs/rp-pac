#[doc = "Check and acknowledge doorbells posted to this core. This core's doorbell interrupt is asserted when any bit in this register is 1. Write 1 to each bit to clear that bit. The doorbell interrupt deasserts once all bits are cleared. Read to get status of doorbells currently asserted on this core."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DoorbellInClr(pub u32);
impl DoorbellInClr {
    #[inline(always)]
    pub const fn doorbell_in_clr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_doorbell_in_clr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DoorbellInClr {
    #[inline(always)]
    fn default() -> DoorbellInClr {
        DoorbellInClr(0)
    }
}
#[doc = "Write 1s to trigger doorbell interrupts on this core. Read to get status of doorbells currently asserted on this core."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DoorbellInSet(pub u32);
impl DoorbellInSet {
    #[inline(always)]
    pub const fn doorbell_in_set(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_doorbell_in_set(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DoorbellInSet {
    #[inline(always)]
    fn default() -> DoorbellInSet {
        DoorbellInSet(0)
    }
}
#[doc = "Clear doorbells which have been posted to the opposite core. This register is intended for debugging and initialisation purposes. Writing 1 to a bit in DOORBELL_OUT_CLR clears the corresponding bit in DOORBELL_IN on the opposite core. Clearing all bits will cause that core's doorbell interrupt to deassert. Since the usual order of events is for software to send events using DOORBELL_OUT_SET, and acknowledge incoming events by writing to DOORBELL_IN_CLR, this register should be used with caution to avoid race conditions. Reading returns the status of the doorbells currently asserted on the other core, i.e. is equivalent to that core reading its own DOORBELL_IN status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DoorbellOutClr(pub u32);
impl DoorbellOutClr {
    #[inline(always)]
    pub const fn doorbell_out_clr(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_doorbell_out_clr(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DoorbellOutClr {
    #[inline(always)]
    fn default() -> DoorbellOutClr {
        DoorbellOutClr(0)
    }
}
#[doc = "Trigger a doorbell interrupt on the opposite core. Write 1 to a bit to set the corresponding bit in DOORBELL_IN on the opposite core. This raises the opposite core's doorbell interrupt. Read to get the status of the doorbells currently asserted on the opposite core. This is equivalent to that core reading its own DOORBELL_IN status."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DoorbellOutSet(pub u32);
impl DoorbellOutSet {
    #[inline(always)]
    pub const fn doorbell_out_set(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_doorbell_out_set(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
}
impl Default for DoorbellOutSet {
    #[inline(always)]
    fn default() -> DoorbellOutSet {
        DoorbellOutSet(0)
    }
}
#[doc = "Status register for inter-core FIFOs (mailboxes). There is one FIFO in the core 0 -> core 1 direction, and one core 1 -> core 0. Both are 32 bits wide and 8 words deep. Core 0 can see the read side of the 1->0 FIFO (RX), and the write side of 0->1 FIFO (TX). Core 1 can see the read side of the 0->1 FIFO (RX), and the write side of 1->0 FIFO (TX). The SIO IRQ for each core is the logical OR of the VLD, WOF and ROE fields of its FIFO_ST register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FifoSt(pub u32);
impl FifoSt {
    #[doc = "Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    #[inline(always)]
    pub const fn vld(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Value is 1 if this core's RX FIFO is not empty (i.e. if FIFO_RD is valid)"]
    #[inline(always)]
    pub fn set_vld(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    #[inline(always)]
    pub const fn rdy(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Value is 1 if this core's TX FIFO is not full (i.e. if FIFO_WR is ready for more data)"]
    #[inline(always)]
    pub fn set_rdy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    pub const fn wof(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Sticky flag indicating the TX FIFO was written when full. This write was ignored by the FIFO."]
    #[inline(always)]
    pub fn set_wof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    pub const fn roe(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Sticky flag indicating the RX FIFO was read when empty. This read was ignored by the FIFO."]
    #[inline(always)]
    pub fn set_roe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for FifoSt {
    #[inline(always)]
    fn default() -> FifoSt {
        FifoSt(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp0accum0add(pub u32);
impl Interp0accum0add {
    #[inline(always)]
    pub const fn interp0_accum0_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_interp0_accum0_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Interp0accum0add {
    #[inline(always)]
    fn default() -> Interp0accum0add {
        Interp0accum0add(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp0accum1add(pub u32);
impl Interp0accum1add {
    #[inline(always)]
    pub const fn interp0_accum1_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_interp0_accum1_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Interp0accum1add {
    #[inline(always)]
    fn default() -> Interp0accum1add {
        Interp0accum1add(0)
    }
}
#[doc = "Control register for lane 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp0ctrlLane0(pub u32);
impl Interp0ctrlLane0 {
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[doc = "Only present on INTERP0 on each core. If BLEND mode is enabled: - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled by the 8 LSBs of lane 1 shift and mask value (a fractional number between 0 and 255/256ths) - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value) - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask) LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    #[inline(always)]
    pub const fn blend(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[doc = "Only present on INTERP0 on each core. If BLEND mode is enabled: - LANE1 result is a linear interpolation between BASE0 and BASE1, controlled by the 8 LSBs of lane 1 shift and mask value (a fractional number between 0 and 255/256ths) - LANE0 result does not have BASE0 added (yields only the 8 LSBs of lane 1 shift+mask value) - FULL result does not have lane 1 shift+mask value added (BASE2 + lane 0 shift+mask) LANE1 SIGNED flag controls whether the interpolation is signed or unsigned."]
    #[inline(always)]
    pub fn set_blend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    #[inline(always)]
    pub const fn overf0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    #[inline(always)]
    pub fn set_overf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    #[inline(always)]
    pub const fn overf1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    #[inline(always)]
    pub fn set_overf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    #[inline(always)]
    pub const fn overf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    #[inline(always)]
    pub fn set_overf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Interp0ctrlLane0 {
    #[inline(always)]
    fn default() -> Interp0ctrlLane0 {
        Interp0ctrlLane0(0)
    }
}
#[doc = "Control register for lane 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp0ctrlLane1(pub u32);
impl Interp0ctrlLane1 {
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
}
impl Default for Interp0ctrlLane1 {
    #[inline(always)]
    fn default() -> Interp0ctrlLane1 {
        Interp0ctrlLane1(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM0 Reading yields lane 0's raw shift and mask value (BASE0 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp1accum0add(pub u32);
impl Interp1accum0add {
    #[inline(always)]
    pub const fn interp1_accum0_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_interp1_accum0_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Interp1accum0add {
    #[inline(always)]
    fn default() -> Interp1accum0add {
        Interp1accum0add(0)
    }
}
#[doc = "Values written here are atomically added to ACCUM1 Reading yields lane 1's raw shift and mask value (BASE1 not added)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp1accum1add(pub u32);
impl Interp1accum1add {
    #[inline(always)]
    pub const fn interp1_accum1_add(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_interp1_accum1_add(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Interp1accum1add {
    #[inline(always)]
    fn default() -> Interp1accum1add {
        Interp1accum1add(0)
    }
}
#[doc = "Control register for lane 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp1ctrlLane0(pub u32);
impl Interp1ctrlLane0 {
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE0, and LANE0 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE0 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
    #[doc = "Only present on INTERP1 on each core. If CLAMP mode is enabled: - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of BASE0 and an upper bound of BASE1. - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED"]
    #[inline(always)]
    pub const fn clamp(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "Only present on INTERP1 on each core. If CLAMP mode is enabled: - LANE0 result is shifted and masked ACCUM0, clamped by a lower bound of BASE0 and an upper bound of BASE1. - Signedness of these comparisons is determined by LANE0_CTRL_SIGNED"]
    #[inline(always)]
    pub fn set_clamp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    #[inline(always)]
    pub const fn overf0(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM0 are set."]
    #[inline(always)]
    pub fn set_overf0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    #[inline(always)]
    pub const fn overf1(&self) -> bool {
        let val = (self.0 >> 24usize) & 0x01;
        val != 0
    }
    #[doc = "Indicates if any masked-off MSBs in ACCUM1 are set."]
    #[inline(always)]
    pub fn set_overf1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24usize)) | (((val as u32) & 0x01) << 24usize);
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    #[inline(always)]
    pub const fn overf(&self) -> bool {
        let val = (self.0 >> 25usize) & 0x01;
        val != 0
    }
    #[doc = "Set if either OVERF0 or OVERF1 is set."]
    #[inline(always)]
    pub fn set_overf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25usize)) | (((val as u32) & 0x01) << 25usize);
    }
}
impl Default for Interp1ctrlLane0 {
    #[inline(always)]
    fn default() -> Interp1ctrlLane0 {
        Interp1ctrlLane0(0)
    }
}
#[doc = "Control register for lane 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Interp1ctrlLane1(pub u32);
impl Interp1ctrlLane1 {
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to accumulator before masking. By appropriately configuring the masks, left and right shifts can be synthesised."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub const fn mask_lsb(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x1f;
        val as u8
    }
    #[doc = "The least-significant bit allowed to pass by the mask (inclusive)"]
    #[inline(always)]
    pub fn set_mask_lsb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 5usize)) | (((val as u32) & 0x1f) << 5usize);
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub const fn mask_msb(&self) -> u8 {
        let val = (self.0 >> 10usize) & 0x1f;
        val as u8
    }
    #[doc = "The most-significant bit allowed to pass by the mask (inclusive) Setting MSB < LSB may cause chip to turn inside-out"]
    #[inline(always)]
    pub fn set_mask_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 10usize)) | (((val as u32) & 0x1f) << 10usize);
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub const fn signed(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "If SIGNED is set, the shifted and masked accumulator value is sign-extended to 32 bits before adding to BASE1, and LANE1 PEEK/POP appear extended to 32 bits when read by processor."]
    #[inline(always)]
    pub fn set_signed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub const fn cross_input(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's accumulator into this lane's shift + mask hardware. Takes effect even if ADD_RAW is set (the CROSS_INPUT mux is before the shift+mask bypass)"]
    #[inline(always)]
    pub fn set_cross_input(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub const fn cross_result(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, feed the opposite lane's result into this lane's accumulator on POP."]
    #[inline(always)]
    pub fn set_cross_result(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub const fn add_raw(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, mask + shift is bypassed for LANE1 result. This does not affect FULL result."]
    #[inline(always)]
    pub fn set_add_raw(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub const fn force_msb(&self) -> u8 {
        let val = (self.0 >> 19usize) & 0x03;
        val as u8
    }
    #[doc = "ORed into bits 29:28 of the lane result presented to the processor on the bus. No effect on the internal 32-bit datapath. Handy for using a lane to generate sequence of pointers into flash or SRAM."]
    #[inline(always)]
    pub fn set_force_msb(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 19usize)) | (((val as u32) & 0x03) << 19usize);
    }
}
impl Default for Interp1ctrlLane1 {
    #[inline(always)]
    fn default() -> Interp1ctrlLane1 {
        Interp1ctrlLane1(0)
    }
}
#[doc = "Control register for the RISC-V 64-bit Machine-mode timer. This timer is only present in the Secure SIO, so is only accessible to an Arm core in Secure mode or a RISC-V core in Machine mode. Note whilst this timer follows the RISC-V privileged specification, it is equally usable by the Arm cores. The interrupts are routed to normal system-level interrupt lines as well as to the MIP.MTIP inputs on the RISC-V cores."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct MtimeCtrl(pub u32);
impl MtimeCtrl {
    #[doc = "Timer enable bit. When 0, the timer will not increment automatically."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Timer enable bit. When 0, the timer will not increment automatically."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
    #[inline(always)]
    pub const fn fullspeed(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, increment the timer every cycle (i.e. run directly from the system clock), rather than incrementing on the system-level timer tick input."]
    #[inline(always)]
    pub fn set_fullspeed(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "If 1, the timer pauses when core 0 is in the debug halt state."]
    #[inline(always)]
    pub const fn dbgpause_core0(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the timer pauses when core 0 is in the debug halt state."]
    #[inline(always)]
    pub fn set_dbgpause_core0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, the timer pauses when core 1 is in the debug halt state."]
    #[inline(always)]
    pub const fn dbgpause_core1(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, the timer pauses when core 1 is in the debug halt state."]
    #[inline(always)]
    pub fn set_dbgpause_core1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
}
impl Default for MtimeCtrl {
    #[inline(always)]
    fn default() -> MtimeCtrl {
        MtimeCtrl(0)
    }
}
#[doc = "Detach certain core-local peripherals from Secure SIO, and attach them to Non-secure SIO, so that Non-secure software can use them. Attempting to access one of these peripherals from the Secure SIO when it is attached to the Non-secure SIO, or vice versa, will generate a bus error. This register is per-core, and is only present on the Secure SIO. Most SIO hardware is duplicated across the Secure and Non-secure SIO, so is not listed in this register."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PeriNonsec(pub u32);
impl PeriNonsec {
    #[doc = "If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub const fn interp0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, detach interpolator 0 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn set_interp0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub const fn interp1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, detach interpolator 1 (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn set_interp1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub const fn tmds(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "IF 1, detach TMDS encoder (of this core) from the Secure SIO, and attach to the Non-secure SIO."]
    #[inline(always)]
    pub fn set_tmds(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
}
impl Default for PeriNonsec {
    #[inline(always)]
    fn default() -> PeriNonsec {
        PeriNonsec(0)
    }
}
#[doc = "Control the assertion of the standard software interrupt (MIP.MSIP) on the RISC-V cores. Unlike the RISC-V timer, this interrupt is not routed to a normal system-level interrupt line, so can not be used by the Arm cores. It is safe for both cores to write to this register on the same cycle. The set/clear effect is accumulated across both cores, and then applied. If a flag is both set and cleared on the same cycle, only the set takes effect."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RiscvSoftirq(pub u32);
impl RiscvSoftirq {
    #[doc = "Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub const fn core0_set(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to atomically set the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn set_core0_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub const fn core1_set(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to atomically set the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn set_core1_set(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub const fn core0_clr(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to atomically clear the core 0 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn set_core0_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub const fn core1_clr(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Write 1 to atomically clear the core 1 software interrupt flag. Read to get the status of this flag."]
    #[inline(always)]
    pub fn set_core1_clr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
}
impl Default for RiscvSoftirq {
    #[inline(always)]
    fn default() -> RiscvSoftirq {
        RiscvSoftirq(0)
    }
}
#[doc = "Control register for TMDS encoder."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct TmdsCtrl(pub u32);
impl TmdsCtrl {
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub const fn l0_rot(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 0 (blue) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), blue is bits 4:0, so should be right-rotated by 13 to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub fn set_l0_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
    #[inline(always)]
    pub const fn l1_rot(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0x0f;
        val as u8
    }
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 1 (green) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565, green is bits 10:5, so should be right-rotated by 3 bits to align with bits 7:2 of the encoder input."]
    #[inline(always)]
    pub fn set_l1_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 4usize)) | (((val as u32) & 0x0f) << 4usize);
    }
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub const fn l2_rot(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Right-rotate the 16 LSBs of the colour accumulator by 0-15 bits, in order to get the MSB of the lane 2 (red) colour data aligned with the MSB of the 8-bit encoder input. For example, for RGB565 (red most significant), red is bits 15:11, so should be right-rotated by 8 bits to align with bits 7:3 of the encoder input."]
    #[inline(always)]
    pub fn set_l2_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub const fn l0_nbits(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid colour MSBs for lane 0 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn set_l0_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub const fn l1_nbits(&self) -> u8 {
        let val = (self.0 >> 15usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid colour MSBs for lane 1 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn set_l1_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 15usize)) | (((val as u32) & 0x07) << 15usize);
    }
    #[doc = "Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub const fn l2_nbits(&self) -> u8 {
        let val = (self.0 >> 18usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid colour MSBs for lane 2 (1-8 bits, encoded as 0 through 7). Remaining LSBs are masked to 0 after the rotate."]
    #[inline(always)]
    pub fn set_l2_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 18usize)) | (((val as u32) & 0x07) << 18usize);
    }
    #[doc = "Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
    #[inline(always)]
    pub const fn interleave(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Enable lane interleaving for reads of PEEK_SINGLE/POP_SINGLE. When interleaving is disabled, each of the 3 symbols appears as a contiguous 10-bit field, with lane 0 being the least-significant and starting at bit 0 of the register. When interleaving is enabled, the symbols are packed into 5 chunks of 3 lanes times 2 bits (30 bits total). Each chunk contains two bits of a TMDS symbol per lane, with lane 0 being the least significant."]
    #[inline(always)]
    pub fn set_interleave(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
    #[doc = "Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
    #[inline(always)]
    pub const fn pix_shift(&self) -> super::vals::PixShift {
        let val = (self.0 >> 24usize) & 0x07;
        super::vals::PixShift::from_bits(val as u8)
    }
    #[doc = "Shift applied to the colour data register with each read of a POP alias register. Reading from the POP_SINGLE register, or reading from the POP_DOUBLE register with PIX2_NOSHIFT set (for pixel doubling), shifts by the indicated amount. Reading from a POP_DOUBLE register when PIX2_NOSHIFT is clear will shift by double the indicated amount. (Shift by 32 means no shift.)"]
    #[inline(always)]
    pub fn set_pix_shift(&mut self, val: super::vals::PixShift) {
        self.0 = (self.0 & !(0x07 << 24usize)) | (((val.to_bits() as u32) & 0x07) << 24usize);
    }
    #[doc = "When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
    #[inline(always)]
    pub const fn pix2_noshift(&self) -> bool {
        let val = (self.0 >> 27usize) & 0x01;
        val != 0
    }
    #[doc = "When encoding two pixels's worth of symbols in one cycle (a read of a PEEK/POP_DOUBLE register), the second encoder sees a shifted version of the colour data register. This control disables that shift, so that both encoder layers see the same pixel data. This is used for pixel doubling."]
    #[inline(always)]
    pub fn set_pix2_noshift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27usize)) | (((val as u32) & 0x01) << 27usize);
    }
    #[doc = "Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline."]
    #[inline(always)]
    pub const fn clear_balance(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Clear the running DC balance state of the TMDS encoders. This bit should be written once at the beginning of each scanline."]
    #[inline(always)]
    pub fn set_clear_balance(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
}
impl Default for TmdsCtrl {
    #[inline(always)]
    fn default() -> TmdsCtrl {
        TmdsCtrl(0)
    }
}
