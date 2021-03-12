use crate::generic::*;
#[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIspr(u32);
impl NvicIspr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIspr {
        NvicIspr(val)
    }
    #[doc = "Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
    pub const fn setpend(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt set-pending bits. Write: 0 = No effect. 1 = Changes interrupt state to pending. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending. Note: Writing 1 to the NVIC_ISPR bit corresponding to: An interrupt that is pending has no effect. A disabled interrupt sets the state of that interrupt to pending."]
    pub fn set_setpend(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
    }
}
impl Default for NvicIspr {
    fn default() -> NvicIspr {
        NvicIspr(0)
    }
}
#[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SystRvr(u32);
impl SystRvr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> SystRvr {
        SystRvr(val)
    }
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0."]
    pub const fn reload(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Value to load into the SysTick Current Value Register when the counter reaches 0."]
    pub fn set_reload(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for SystRvr {
    fn default() -> SystRvr {
        SystRvr(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr3(u32);
impl NvicIpr3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr3 {
        NvicIpr3(val)
    }
    #[doc = "Priority of interrupt 15"]
    pub const fn ip_15(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 15"]
    pub fn set_ip_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 14"]
    pub const fn ip_14(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 14"]
    pub fn set_ip_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 13"]
    pub const fn ip_13(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 13"]
    pub fn set_ip_13(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 12"]
    pub const fn ip_12(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 12"]
    pub fn set_ip_12(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr3 {
    fn default() -> NvicIpr3 {
        NvicIpr3(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr4(u32);
impl NvicIpr4 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr4 {
        NvicIpr4(val)
    }
    #[doc = "Priority of interrupt 19"]
    pub const fn ip_19(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 19"]
    pub fn set_ip_19(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 18"]
    pub const fn ip_18(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 18"]
    pub fn set_ip_18(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 17"]
    pub const fn ip_17(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 17"]
    pub fn set_ip_17(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 16"]
    pub const fn ip_16(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 16"]
    pub fn set_ip_16(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr4 {
    fn default() -> NvicIpr4 {
        NvicIpr4(0)
    }
}
#[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SystCsr(u32);
impl SystCsr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> SystCsr {
        SystCsr(val)
    }
    #[doc = "Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
    pub const fn countflag(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Returns 1 if timer counted to 0 since last time this was read. Clears on read by application or debugger."]
    pub fn set_countflag(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "SysTick clock source. Always reads as one if SYST_CALIB reports NOREF. Selects the SysTick timer clock source: 0 = External reference clock. 1 = Processor clock."]
    pub const fn clksource(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "SysTick clock source. Always reads as one if SYST_CALIB reports NOREF. Selects the SysTick timer clock source: 0 = External reference clock. 1 = Processor clock."]
    pub fn set_clksource(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Enables SysTick exception request: 0 = Counting down to zero does not assert the SysTick exception request. 1 = Counting down to zero to asserts the SysTick exception request."]
    pub const fn tickint(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Enables SysTick exception request: 0 = Counting down to zero does not assert the SysTick exception request. 1 = Counting down to zero to asserts the SysTick exception request."]
    pub fn set_tickint(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable SysTick counter: 0 = Counter disabled. 1 = Counter enabled."]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable SysTick counter: 0 = Counter disabled. 1 = Counter enabled."]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SystCsr {
    fn default() -> SystCsr {
        SystCsr(0)
    }
}
#[doc = "The VTOR holds the vector table offset address."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Vtor(u32);
impl Vtor {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Vtor {
        Vtor(val)
    }
    #[doc = "Bits [31:8]
of the indicate the vector table offset address."]
    pub const fn tbloff(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Bits [31:8]
of the indicate the vector table offset address."]
    pub fn set_tbloff(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
}
impl Default for Vtor {
    fn default() -> Vtor {
        Vtor(0)
    }
}
#[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Shcsr(u32);
impl Shcsr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Shcsr {
        Shcsr(val)
    }
    #[doc = "Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    pub const fn svcallpended(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "Reads as 1 if SVCall is Pending. Write 1 to set pending SVCall, write 0 to clear pending SVCall."]
    pub fn set_svcallpended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
}
impl Default for Shcsr {
    fn default() -> Shcsr {
        Shcsr(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr2(u32);
impl NvicIpr2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr2 {
        NvicIpr2(val)
    }
    #[doc = "Priority of interrupt 11"]
    pub const fn ip_11(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 11"]
    pub fn set_ip_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 10"]
    pub const fn ip_10(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 10"]
    pub fn set_ip_10(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 9"]
    pub const fn ip_9(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 9"]
    pub fn set_ip_9(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 8"]
    pub const fn ip_8(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 8"]
    pub fn set_ip_8(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr2 {
    fn default() -> NvicIpr2 {
        NvicIpr2(0)
    }
}
#[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MpuRasr(u32);
impl MpuRasr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MpuRasr {
        MpuRasr(val)
    }
    #[doc = "The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
    pub const fn attrs(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    #[doc = "The MPU Region Attribute field. Use to define the region attribute control. 28 = XN: Instruction access disable bit: 0 = Instruction fetches enabled. 1 = Instruction fetches disabled. 26:24 = AP: Access permission field 18 = S: Shareable bit 17 = C: Cacheable bit 16 = B: Bufferable bit"]
    pub fn set_attrs(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    pub const fn srd(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0xff;
        val as u8
    }
    #[doc = "Subregion Disable. For regions of 256 bytes or larger, each bit of this field controls whether one of the eight equal subregions is enabled."]
    pub fn set_srd(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
    }
    #[doc = "Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    pub const fn size(&self) -> u8 {
        let val = (self.0 >> 1u32) & 0x1f;
        val as u8
    }
    #[doc = "Indicates the region size. Region size in bytes = 2^(SIZE+1). The minimum permitted value is 7 (b00111) = 256Bytes"]
    pub fn set_size(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 1u32)) | (((val as u32) & 0x1f) << 1u32);
    }
    #[doc = "Enables the region."]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enables the region."]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for MpuRasr {
    fn default() -> MpuRasr {
        MpuRasr(0)
    }
}
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Shpr2(u32);
impl Shpr2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Shpr2 {
        Shpr2(val)
    }
    #[doc = "Priority of system handler 11, SVCall"]
    pub const fn pri_11(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of system handler 11, SVCall"]
    pub fn set_pri_11(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
}
impl Default for Shpr2 {
    fn default() -> Shpr2 {
        Shpr2(0)
    }
}
#[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SystCalib(u32);
impl SystCalib {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> SystCalib {
        SystCalib(val)
    }
    #[doc = "If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
    pub const fn noref(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "If reads as 1, the Reference clock is not provided - the CLKSOURCE bit of the SysTick Control and Status register will be forced to 1 and cannot be cleared to 0."]
    pub fn set_noref(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
    pub const fn skew(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "If reads as 1, the calibration value for 10ms is inexact (due to clock frequency)."]
    pub fn set_skew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
    pub const fn tenms(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "An optional Reload value to be used for 10ms (100Hz) timing, subject to system clock skew errors. If the value reads as 0, the calibration value is not known."]
    pub fn set_tenms(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for SystCalib {
    fn default() -> SystCalib {
        SystCalib(0)
    }
}
#[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SystCvr(u32);
impl SystCvr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> SystCvr {
        SystCvr(val)
    }
    #[doc = "Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
    pub const fn current(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Reads return the current value of the SysTick counter. This register is write-clear. Writing to it with any value clears the register to 0. Clearing this register also clears the COUNTFLAG bit of the SysTick Control and Status Register."]
    pub fn set_current(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0u32)) | (((val as u32) & 0x00ff_ffff) << 0u32);
    }
}
impl Default for SystCvr {
    fn default() -> SystCvr {
        SystCvr(0)
    }
}
#[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MpuType(u32);
impl MpuType {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MpuType {
        MpuType(val)
    }
    #[doc = "Instruction region. Reads as zero as ARMv6-M only supports a unified MPU."]
    pub const fn iregion(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0xff;
        val as u8
    }
    #[doc = "Instruction region. Reads as zero as ARMv6-M only supports a unified MPU."]
    pub fn set_iregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16u32)) | (((val as u32) & 0xff) << 16u32);
    }
    #[doc = "Number of regions supported by the MPU."]
    pub const fn dregion(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0xff;
        val as u8
    }
    #[doc = "Number of regions supported by the MPU."]
    pub fn set_dregion(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8u32)) | (((val as u32) & 0xff) << 8u32);
    }
    #[doc = "Indicates support for separate instruction and data address maps. Reads as 0 as ARMv6-M only supports a unified MPU."]
    pub const fn separate(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates support for separate instruction and data address maps. Reads as 0 as ARMv6-M only supports a unified MPU."]
    pub fn set_separate(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for MpuType {
    fn default() -> MpuType {
        MpuType(0)
    }
}
#[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MpuRnr(u32);
impl MpuRnr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MpuRnr {
        MpuRnr(val)
    }
    #[doc = "Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Indicates the MPU region referenced by the MPU_RBAR and MPU_RASR registers. The MPU supports 8 memory regions, so the permitted values of this field are 0-7."]
    pub fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for MpuRnr {
    fn default() -> MpuRnr {
        MpuRnr(0)
    }
}
#[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Shpr3(u32);
impl Shpr3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Shpr3 {
        Shpr3(val)
    }
    #[doc = "Priority of system handler 15, SysTick"]
    pub const fn pri_15(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of system handler 15, SysTick"]
    pub fn set_pri_15(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of system handler 14, PendSV"]
    pub const fn pri_14(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of system handler 14, PendSV"]
    pub fn set_pri_14(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
}
impl Default for Shpr3 {
    fn default() -> Shpr3 {
        Shpr3(0)
    }
}
#[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MpuCtrl(u32);
impl MpuCtrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MpuCtrl {
        MpuCtrl(val)
    }
    #[doc = "Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear. 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault. 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses. When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    pub const fn privdefena(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Controls whether the default memory map is enabled as a background region for privileged accesses. This bit is ignored when ENABLE is clear. 0 = If the MPU is enabled, disables use of the default memory map. Any memory access to a location not covered by any enabled region causes a fault. 1 = If the MPU is enabled, enables use of the default memory map as a background region for privileged software accesses. When enabled, the background region acts as if it is region number -1. Any region that is defined and enabled has priority over this default map."]
    pub fn set_privdefena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour. When the MPU is enabled: 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit. 1 = the MPU is enabled during HardFault and NMI handlers."]
    pub const fn hfnmiena(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Controls the use of the MPU for HardFaults and NMIs. Setting this bit when ENABLE is clear results in UNPREDICTABLE behaviour. When the MPU is enabled: 0 = MPU is disabled during HardFault and NMI handlers, regardless of the value of the ENABLE bit. 1 = the MPU is enabled during HardFault and NMI handlers."]
    pub fn set_hfnmiena(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map. 0 = MPU disabled. 1 = MPU enabled."]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enables the MPU. If the MPU is disabled, privileged and unprivileged accesses use the default memory map. 0 = MPU disabled. 1 = MPU enabled."]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for MpuCtrl {
    fn default() -> MpuCtrl {
        MpuCtrl(0)
    }
}
#[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ccr(u32);
impl Ccr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ccr {
        Ccr(val)
    }
    #[doc = "Always reads as one, indicates 8-byte stack alignment on exception entry. On exception entry, the processor uses bit[9]
of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
    pub const fn stkalign(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "Always reads as one, indicates 8-byte stack alignment on exception entry. On exception entry, the processor uses bit[9]
of the stacked PSR to indicate the stack alignment. On return from the exception it uses this stacked bit to restore the correct stack alignment."]
    pub fn set_stkalign(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "Always reads as one, indicates that all unaligned accesses generate a HardFault."]
    pub const fn unalign_trp(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Always reads as one, indicates that all unaligned accesses generate a HardFault."]
    pub fn set_unalign_trp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
}
impl Default for Ccr {
    fn default() -> Ccr {
        Ccr(0)
    }
}
#[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIcer(u32);
impl NvicIcer {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIcer {
        NvicIcer(val)
    }
    #[doc = "Interrupt clear-enable bits. Write: 0 = No effect. 1 = Disable interrupt. Read: 0 = Interrupt disabled. 1 = Interrupt enabled."]
    pub const fn clrena(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt clear-enable bits. Write: 0 = No effect. 1 = Disable interrupt. Read: 0 = Interrupt disabled. 1 = Interrupt enabled."]
    pub fn set_clrena(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
    }
}
impl Default for NvicIcer {
    fn default() -> NvicIcer {
        NvicIcer(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr6(u32);
impl NvicIpr6 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr6 {
        NvicIpr6(val)
    }
    #[doc = "Priority of interrupt 27"]
    pub const fn ip_27(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 27"]
    pub fn set_ip_27(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 26"]
    pub const fn ip_26(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 26"]
    pub fn set_ip_26(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 25"]
    pub const fn ip_25(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 25"]
    pub fn set_ip_25(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 24"]
    pub const fn ip_24(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 24"]
    pub fn set_ip_24(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr6 {
    fn default() -> NvicIpr6 {
        NvicIpr6(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr7(u32);
impl NvicIpr7 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr7 {
        NvicIpr7(val)
    }
    #[doc = "Priority of interrupt 31"]
    pub const fn ip_31(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 31"]
    pub fn set_ip_31(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 30"]
    pub const fn ip_30(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 30"]
    pub fn set_ip_30(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 29"]
    pub const fn ip_29(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 29"]
    pub fn set_ip_29(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 28"]
    pub const fn ip_28(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 28"]
    pub fn set_ip_28(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr7 {
    fn default() -> NvicIpr7 {
        NvicIpr7(0)
    }
}
#[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIcpr(u32);
impl NvicIcpr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIcpr {
        NvicIcpr(val)
    }
    #[doc = "Interrupt clear-pending bits. Write: 0 = No effect. 1 = Removes pending state and interrupt. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending."]
    pub const fn clrpend(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt clear-pending bits. Write: 0 = No effect. 1 = Removes pending state and interrupt. Read: 0 = Interrupt is not pending. 1 = Interrupt is pending."]
    pub fn set_clrpend(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
    }
}
impl Default for NvicIcpr {
    fn default() -> NvicIcpr {
        NvicIcpr(0)
    }
}
#[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Icsr(u32);
impl Icsr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Icsr {
        Icsr(val)
    }
    #[doc = "Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    pub const fn nmipendset(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Setting this bit will activate an NMI. Since NMI is the highest priority exception, it will activate as soon as it is registered. NMI set-pending bit. Write: 0 = No effect. 1 = Changes NMI exception state to pending. Read: 0 = NMI exception is not pending. 1 = NMI exception is pending. Because NMI is the highest-priority exception, normally the processor enters the NMI exception handler as soon as it detects a write of 1 to this bit. Entering the handler then clears this bit to 0. This means a read of this bit by the NMI exception handler returns 1 only if the NMI signal is reasserted while the processor is executing that handler."]
    pub fn set_nmipendset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    pub const fn pendsvset(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "PendSV set-pending bit. Write: 0 = No effect. 1 = Changes PendSV exception state to pending. Read: 0 = PendSV exception is not pending. 1 = PendSV exception is pending. Writing 1 to this bit is the only way to set the PendSV exception state to pending."]
    pub fn set_pendsvset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
    pub const fn pendsvclr(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    #[doc = "PendSV clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the PendSV exception."]
    pub fn set_pendsvclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    #[doc = "SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
    pub const fn pendstset(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception set-pending bit. Write: 0 = No effect. 1 = Changes SysTick exception state to pending. Read: 0 = SysTick exception is not pending. 1 = SysTick exception is pending."]
    pub fn set_pendstset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
    pub const fn pendstclr(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "SysTick exception clear-pending bit. Write: 0 = No effect. 1 = Removes the pending state from the SysTick exception. This bit is WO. On a register read its value is Unknown."]
    pub fn set_pendstclr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "The system can only access this bit when the core is halted. It indicates that a pending interrupt is to be taken in the next running cycle. If C_MASKINTS is clear in the Debug Halting Control and Status Register, the interrupt is serviced."]
    pub const fn isrpreempt(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    #[doc = "The system can only access this bit when the core is halted. It indicates that a pending interrupt is to be taken in the next running cycle. If C_MASKINTS is clear in the Debug Halting Control and Status Register, the interrupt is serviced."]
    pub fn set_isrpreempt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    #[doc = "External interrupt pending flag"]
    pub const fn isrpending(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    #[doc = "External interrupt pending flag"]
    pub fn set_isrpending(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    #[doc = "Indicates the exception number for the highest priority pending exception: 0 = no pending exceptions. Non zero = The pending state includes the effect of memory-mapped enable and mask registers. It does not include the PRIMASK special-purpose register qualifier."]
    pub const fn vectpending(&self) -> u16 {
        let val = (self.0 >> 12u32) & 0x01ff;
        val as u16
    }
    #[doc = "Indicates the exception number for the highest priority pending exception: 0 = no pending exceptions. Non zero = The pending state includes the effect of memory-mapped enable and mask registers. It does not include the PRIMASK special-purpose register qualifier."]
    pub fn set_vectpending(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 12u32)) | (((val as u32) & 0x01ff) << 12u32);
    }
    #[doc = "Active exception number field. Reset clears the VECTACTIVE field."]
    pub const fn vectactive(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x01ff;
        val as u16
    }
    #[doc = "Active exception number field. Reset clears the VECTACTIVE field."]
    pub fn set_vectactive(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0u32)) | (((val as u32) & 0x01ff) << 0u32);
    }
}
impl Default for Icsr {
    fn default() -> Icsr {
        Icsr(0)
    }
}
#[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Scr(u32);
impl Scr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Scr {
        Scr(val)
    }
    #[doc = "Send Event on Pending bit: 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded. 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event."]
    pub const fn sevonpend(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Send Event on Pending bit: 0 = Only enabled interrupts or events can wakeup the processor, disabled interrupts are excluded. 1 = Enabled events and all interrupts, including disabled interrupts, can wakeup the processor. When an event or interrupt becomes pending, the event signal wakes up the processor from WFE. If the processor is not waiting for an event, the event is registered and affects the next WFE. The processor also wakes up on execution of an SEV instruction or an external event."]
    pub fn set_sevonpend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode: 0 = Sleep. 1 = Deep sleep."]
    pub const fn sleepdeep(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Controls whether the processor uses sleep or deep sleep as its low power mode: 0 = Sleep. 1 = Deep sleep."]
    pub fn set_sleepdeep(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode: 0 = Do not sleep when returning to Thread mode. 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    pub const fn sleeponexit(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Indicates sleep-on-exit when returning from Handler mode to Thread mode: 0 = Do not sleep when returning to Thread mode. 1 = Enter sleep, or deep sleep, on return from an ISR to Thread mode. Setting this bit to 1 enables an interrupt driven application to avoid returning to an empty main application."]
    pub fn set_sleeponexit(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
}
impl Default for Scr {
    fn default() -> Scr {
        Scr(0)
    }
}
#[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Aircr(u32);
impl Aircr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Aircr {
        Aircr(val)
    }
    #[doc = "Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    pub const fn vectkey(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    #[doc = "Register key: Reads as Unknown On writes, write 0x05FA to VECTKEY, otherwise the write is ignored."]
    pub fn set_vectkey(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    #[doc = "Data endianness implemented: 0 = Little-endian."]
    pub const fn endianess(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "Data endianness implemented: 0 = Little-endian."]
    pub fn set_endianess(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    pub const fn sysresetreq(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Writing 1 to this bit causes the SYSRESETREQ signal to the outer system to be asserted to request a reset. The intention is to force a large system reset of all major components except for debug. The C_HALT bit in the DHCSR is cleared as a result of the system reset requested. The debugger does not lose contact with the device."]
    pub fn set_sysresetreq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    pub const fn vectclractive(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Clears all active state information for fixed and configurable exceptions. This bit: is self-clearing, can only be set by the DAP when the core is halted. When set: clears all active exception status of the processor, forces a return to Thread mode, forces an IPSR of 0. A debugger must re-initialize the stack."]
    pub fn set_vectclractive(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
}
impl Default for Aircr {
    fn default() -> Aircr {
        Aircr(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr0(u32);
impl NvicIpr0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr0 {
        NvicIpr0(val)
    }
    #[doc = "Priority of interrupt 3"]
    pub const fn ip_3(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 3"]
    pub fn set_ip_3(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 2"]
    pub const fn ip_2(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 2"]
    pub fn set_ip_2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 1"]
    pub const fn ip_1(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 1"]
    pub fn set_ip_1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 0"]
    pub const fn ip_0(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 0"]
    pub fn set_ip_0(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr0 {
    fn default() -> NvicIpr0 {
        NvicIpr0(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr5(u32);
impl NvicIpr5 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr5 {
        NvicIpr5(val)
    }
    #[doc = "Priority of interrupt 23"]
    pub const fn ip_23(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 23"]
    pub fn set_ip_23(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 22"]
    pub const fn ip_22(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 22"]
    pub fn set_ip_22(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 21"]
    pub const fn ip_21(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 21"]
    pub fn set_ip_21(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 20"]
    pub const fn ip_20(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 20"]
    pub fn set_ip_20(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr5 {
    fn default() -> NvicIpr5 {
        NvicIpr5(0)
    }
}
#[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Cpuid(u32);
impl Cpuid {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Cpuid {
        Cpuid(val)
    }
    #[doc = "Implementor code: 0x41 = ARM"]
    pub const fn implementer(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0xff;
        val as u8
    }
    #[doc = "Implementor code: 0x41 = ARM"]
    pub fn set_implementer(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 24u32)) | (((val as u32) & 0xff) << 24u32);
    }
    #[doc = "Major revision number n in the rnpm revision status: 0x0 = Revision 0."]
    pub const fn variant(&self) -> u8 {
        let val = (self.0 >> 20u32) & 0x0f;
        val as u8
    }
    #[doc = "Major revision number n in the rnpm revision status: 0x0 = Revision 0."]
    pub fn set_variant(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 20u32)) | (((val as u32) & 0x0f) << 20u32);
    }
    #[doc = "Constant that defines the architecture of the processor: 0xC = ARMv6-M architecture."]
    pub const fn architecture(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x0f;
        val as u8
    }
    #[doc = "Constant that defines the architecture of the processor: 0xC = ARMv6-M architecture."]
    pub fn set_architecture(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16u32)) | (((val as u32) & 0x0f) << 16u32);
    }
    #[doc = "Number of processor within family: 0xC60 = Cortex-M0+"]
    pub const fn partno(&self) -> u16 {
        let val = (self.0 >> 4u32) & 0x0fff;
        val as u16
    }
    #[doc = "Number of processor within family: 0xC60 = Cortex-M0+"]
    pub fn set_partno(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 4u32)) | (((val as u32) & 0x0fff) << 4u32);
    }
    #[doc = "Minor revision number m in the rnpm revision status: 0x1 = Patch 1."]
    pub const fn revision(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "Minor revision number m in the rnpm revision status: 0x1 = Patch 1."]
    pub fn set_revision(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Cpuid {
    fn default() -> Cpuid {
        Cpuid(0)
    }
}
#[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIpr1(u32);
impl NvicIpr1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIpr1 {
        NvicIpr1(val)
    }
    #[doc = "Priority of interrupt 7"]
    pub const fn ip_7(&self) -> u8 {
        let val = (self.0 >> 30u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 7"]
    pub fn set_ip_7(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 30u32)) | (((val as u32) & 0x03) << 30u32);
    }
    #[doc = "Priority of interrupt 6"]
    pub const fn ip_6(&self) -> u8 {
        let val = (self.0 >> 22u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 6"]
    pub fn set_ip_6(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 22u32)) | (((val as u32) & 0x03) << 22u32);
    }
    #[doc = "Priority of interrupt 5"]
    pub const fn ip_5(&self) -> u8 {
        let val = (self.0 >> 14u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 5"]
    pub fn set_ip_5(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 14u32)) | (((val as u32) & 0x03) << 14u32);
    }
    #[doc = "Priority of interrupt 4"]
    pub const fn ip_4(&self) -> u8 {
        let val = (self.0 >> 6u32) & 0x03;
        val as u8
    }
    #[doc = "Priority of interrupt 4"]
    pub fn set_ip_4(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 6u32)) | (((val as u32) & 0x03) << 6u32);
    }
}
impl Default for NvicIpr1 {
    fn default() -> NvicIpr1 {
        NvicIpr1(0)
    }
}
#[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MpuRbar(u32);
impl MpuRbar {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> MpuRbar {
        MpuRbar(val)
    }
    #[doc = "Base address of the region."]
    pub const fn addr(&self) -> u32 {
        let val = (self.0 >> 8u32) & 0x00ff_ffff;
        val as u32
    }
    #[doc = "Base address of the region."]
    pub fn set_addr(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 8u32)) | (((val as u32) & 0x00ff_ffff) << 8u32);
    }
    #[doc = "On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "On writes, indicates whether the write must update the base address of the region identified by the REGION field, updating the MPU_RNR to indicate this new region. Write: 0 = MPU_RNR not changed, and the processor: Updates the base address for the region specified in the MPU_RNR. Ignores the value of the REGION field. 1 = The processor: Updates the value of the MPU_RNR to the value of the REGION field. Updates the base address for the region specified in the REGION field. Always reads as zero."]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits [3:0]
of MPU_RNR."]
    pub const fn region(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    #[doc = "On writes, specifies the number of the region whose base address to update provided VALID is set written as 1. On reads, returns bits [3:0]
of MPU_RNR."]
    pub fn set_region(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for MpuRbar {
    fn default() -> MpuRbar {
        MpuRbar(0)
    }
}
#[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NvicIser(u32);
impl NvicIser {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> NvicIser {
        NvicIser(val)
    }
    #[doc = "Interrupt set-enable bits. Write: 0 = No effect. 1 = Enable interrupt. Read: 0 = Interrupt disabled. 1 = Interrupt enabled."]
    pub const fn setena(&self) -> u32 {
        let val = (self.0 >> 0u32) & 0xffff_ffff;
        val as u32
    }
    #[doc = "Interrupt set-enable bits. Write: 0 = No effect. 1 = Enable interrupt. Read: 0 = Interrupt disabled. 1 = Interrupt enabled."]
    pub fn set_setena(&mut self, val: u32) {
        self.0 = (self.0 & !(0xffff_ffff << 0u32)) | (((val as u32) & 0xffff_ffff) << 0u32);
    }
}
impl Default for NvicIser {
    fn default() -> NvicIser {
        NvicIser(0)
    }
}
