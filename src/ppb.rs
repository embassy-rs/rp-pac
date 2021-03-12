use crate::generic::*;
#[derive(Copy, Clone)]
pub struct Ppb(*mut u8);
unsafe impl Send for Ppb {}
unsafe impl Sync for Ppb {}
impl Ppb {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "Use the SysTick Control and Status Register to enable the SysTick features."]
    pub fn syst_csr(self) -> Reg<regs::SystCsr, RW> {
        unsafe { Reg::new(self.0.add(57360usize)) }
    }
    #[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    pub fn syst_rvr(self) -> Reg<regs::SystRvr, RW> {
        unsafe { Reg::new(self.0.add(57364usize)) }
    }
    #[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    pub fn syst_cvr(self) -> Reg<regs::SystCvr, RW> {
        unsafe { Reg::new(self.0.add(57368usize)) }
    }
    #[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    pub fn syst_calib(self) -> Reg<regs::SystCalib, RW> {
        unsafe { Reg::new(self.0.add(57372usize)) }
    }
    #[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
    pub fn nvic_iser(self) -> Reg<regs::NvicIser, RW> {
        unsafe { Reg::new(self.0.add(57600usize)) }
    }
    #[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
    pub fn nvic_icer(self) -> Reg<regs::NvicIcer, RW> {
        unsafe { Reg::new(self.0.add(57728usize)) }
    }
    #[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
    pub fn nvic_ispr(self) -> Reg<regs::NvicIspr, RW> {
        unsafe { Reg::new(self.0.add(57856usize)) }
    }
    #[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
    pub fn nvic_icpr(self) -> Reg<regs::NvicIcpr, RW> {
        unsafe { Reg::new(self.0.add(57984usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible"]
    pub fn nvic_ipr0(self) -> Reg<regs::NvicIpr0, RW> {
        unsafe { Reg::new(self.0.add(58368usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr1(self) -> Reg<regs::NvicIpr1, RW> {
        unsafe { Reg::new(self.0.add(58372usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr2(self) -> Reg<regs::NvicIpr2, RW> {
        unsafe { Reg::new(self.0.add(58376usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr3(self) -> Reg<regs::NvicIpr3, RW> {
        unsafe { Reg::new(self.0.add(58380usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr4(self) -> Reg<regs::NvicIpr4, RW> {
        unsafe { Reg::new(self.0.add(58384usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr5(self) -> Reg<regs::NvicIpr5, RW> {
        unsafe { Reg::new(self.0.add(58388usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr6(self) -> Reg<regs::NvicIpr6, RW> {
        unsafe { Reg::new(self.0.add(58392usize)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr7(self) -> Reg<regs::NvicIpr7, RW> {
        unsafe { Reg::new(self.0.add(58396usize)) }
    }
    #[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
    pub fn cpuid(self) -> Reg<regs::Cpuid, RW> {
        unsafe { Reg::new(self.0.add(60672usize)) }
    }
    #[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
    pub fn icsr(self) -> Reg<regs::Icsr, RW> {
        unsafe { Reg::new(self.0.add(60676usize)) }
    }
    #[doc = "The VTOR holds the vector table offset address."]
    pub fn vtor(self) -> Reg<regs::Vtor, RW> {
        unsafe { Reg::new(self.0.add(60680usize)) }
    }
    #[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
    pub fn aircr(self) -> Reg<regs::Aircr, RW> {
        unsafe { Reg::new(self.0.add(60684usize)) }
    }
    #[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
    pub fn scr(self) -> Reg<regs::Scr, RW> {
        unsafe { Reg::new(self.0.add(60688usize)) }
    }
    #[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
    pub fn ccr(self) -> Reg<regs::Ccr, RW> {
        unsafe { Reg::new(self.0.add(60692usize)) }
    }
    #[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
    pub fn shpr2(self) -> Reg<regs::Shpr2, RW> {
        unsafe { Reg::new(self.0.add(60700usize)) }
    }
    #[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
    pub fn shpr3(self) -> Reg<regs::Shpr3, RW> {
        unsafe { Reg::new(self.0.add(60704usize)) }
    }
    #[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
    pub fn shcsr(self) -> Reg<regs::Shcsr, RW> {
        unsafe { Reg::new(self.0.add(60708usize)) }
    }
    #[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
    pub fn mpu_type(self) -> Reg<regs::MpuType, RW> {
        unsafe { Reg::new(self.0.add(60816usize)) }
    }
    #[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
    pub fn mpu_ctrl(self) -> Reg<regs::MpuCtrl, RW> {
        unsafe { Reg::new(self.0.add(60820usize)) }
    }
    #[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
    pub fn mpu_rnr(self) -> Reg<regs::MpuRnr, RW> {
        unsafe { Reg::new(self.0.add(60824usize)) }
    }
    #[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
    pub fn mpu_rbar(self) -> Reg<regs::MpuRbar, RW> {
        unsafe { Reg::new(self.0.add(60828usize)) }
    }
    #[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
    pub fn mpu_rasr(self) -> Reg<regs::MpuRasr, RW> {
        unsafe { Reg::new(self.0.add(60832usize)) }
    }
}
pub mod regs;
