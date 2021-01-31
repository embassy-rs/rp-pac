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
    pub fn syst_csr(self) -> Reg<fields::SystCsr, RW> {
        unsafe { Reg::new(self.0.add(57360usize), fields::SystCsr::from_bits(0)) }
    }
    #[doc = "Use the SysTick Reload Value Register to specify the start value to load into the current value register when the counter reaches 0. It can be any value between 0 and 0x00FFFFFF. A start value of 0 is possible, but has no effect because the SysTick interrupt and COUNTFLAG are activated when counting from 1 to 0. The reset value of this register is UNKNOWN. To generate a multi-shot timer with a period of N processor clock cycles, use a RELOAD value of N-1. For example, if the SysTick interrupt is required every 100 clock pulses, set RELOAD to 99."]
    pub fn syst_rvr(self) -> Reg<fields::SystRvr, RW> {
        unsafe { Reg::new(self.0.add(57364usize), fields::SystRvr::from_bits(0)) }
    }
    #[doc = "Use the SysTick Current Value Register to find the current value in the register. The reset value of this register is UNKNOWN."]
    pub fn syst_cvr(self) -> Reg<fields::SystCvr, RW> {
        unsafe { Reg::new(self.0.add(57368usize), fields::SystCvr::from_bits(0)) }
    }
    #[doc = "Use the SysTick Calibration Value Register to enable software to scale to any required speed using divide and multiply."]
    pub fn syst_calib(self) -> Reg<fields::SystCalib, RW> {
        unsafe { Reg::new(self.0.add(57372usize), fields::SystCalib::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Set-Enable Register to enable interrupts and determine which interrupts are currently enabled. If a pending interrupt is enabled, the NVIC activates the interrupt based on its priority. If an interrupt is not enabled, asserting its interrupt signal changes the interrupt state to pending, but the NVIC never activates the interrupt, regardless of its priority."]
    pub fn nvic_iser(self) -> Reg<fields::NvicIser, RW> {
        unsafe { Reg::new(self.0.add(57600usize), fields::NvicIser::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Clear-Enable Registers to disable interrupts and determine which interrupts are currently enabled."]
    pub fn nvic_icer(self) -> Reg<fields::NvicIcer, RW> {
        unsafe { Reg::new(self.0.add(57728usize), fields::NvicIcer::from_bits(0)) }
    }
    #[doc = "The NVIC_ISPR forces interrupts into the pending state, and shows which interrupts are pending."]
    pub fn nvic_ispr(self) -> Reg<fields::NvicIspr, RW> {
        unsafe { Reg::new(self.0.add(57856usize), fields::NvicIspr::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Clear-Pending Register to clear pending interrupts and determine which interrupts are currently pending."]
    pub fn nvic_icpr(self) -> Reg<fields::NvicIcpr, RW> {
        unsafe { Reg::new(self.0.add(57984usize), fields::NvicIcpr::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest. Note: Writing 1 to an NVIC_ICPR bit does not affect the active state of the corresponding interrupt. These registers are only word-accessible"]
    pub fn nvic_ipr0(self) -> Reg<fields::NvicIpr0, RW> {
        unsafe { Reg::new(self.0.add(58368usize), fields::NvicIpr0::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr1(self) -> Reg<fields::NvicIpr1, RW> {
        unsafe { Reg::new(self.0.add(58372usize), fields::NvicIpr1::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr2(self) -> Reg<fields::NvicIpr2, RW> {
        unsafe { Reg::new(self.0.add(58376usize), fields::NvicIpr2::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr3(self) -> Reg<fields::NvicIpr3, RW> {
        unsafe { Reg::new(self.0.add(58380usize), fields::NvicIpr3::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr4(self) -> Reg<fields::NvicIpr4, RW> {
        unsafe { Reg::new(self.0.add(58384usize), fields::NvicIpr4::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr5(self) -> Reg<fields::NvicIpr5, RW> {
        unsafe { Reg::new(self.0.add(58388usize), fields::NvicIpr5::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr6(self) -> Reg<fields::NvicIpr6, RW> {
        unsafe { Reg::new(self.0.add(58392usize), fields::NvicIpr6::from_bits(0)) }
    }
    #[doc = "Use the Interrupt Priority Registers to assign a priority from 0 to 3 to each of the available interrupts. 0 is the highest priority, and 3 is the lowest."]
    pub fn nvic_ipr7(self) -> Reg<fields::NvicIpr7, RW> {
        unsafe { Reg::new(self.0.add(58396usize), fields::NvicIpr7::from_bits(0)) }
    }
    #[doc = "Read the CPU ID Base Register to determine: the ID number of the processor core, the version number of the processor core, the implementation details of the processor core."]
    pub fn cpuid(self) -> Reg<fields::Cpuid, RW> {
        unsafe { Reg::new(self.0.add(60672usize), fields::Cpuid::from_bits(1091356161)) }
    }
    #[doc = "Use the Interrupt Control State Register to set a pending Non-Maskable Interrupt (NMI), set or clear a pending PendSV, set or clear a pending SysTick, check for pending exceptions, check the vector number of the highest priority pended exception, check the vector number of the active exception."]
    pub fn icsr(self) -> Reg<fields::Icsr, RW> {
        unsafe { Reg::new(self.0.add(60676usize), fields::Icsr::from_bits(0)) }
    }
    #[doc = "The VTOR holds the vector table offset address."]
    pub fn vtor(self) -> Reg<fields::Vtor, RW> {
        unsafe { Reg::new(self.0.add(60680usize), fields::Vtor::from_bits(0)) }
    }
    #[doc = "Use the Application Interrupt and Reset Control Register to: determine data endianness, clear all active state information from debug halt mode, request a system reset."]
    pub fn aircr(self) -> Reg<fields::Aircr, RW> {
        unsafe { Reg::new(self.0.add(60684usize), fields::Aircr::from_bits(0)) }
    }
    #[doc = "System Control Register. Use the System Control Register for power-management functions: signal to the system when the processor can enter a low power state, control how the processor enters and exits low power states."]
    pub fn scr(self) -> Reg<fields::Scr, RW> {
        unsafe { Reg::new(self.0.add(60688usize), fields::Scr::from_bits(0)) }
    }
    #[doc = "The Configuration and Control Register permanently enables stack alignment and causes unaligned accesses to result in a Hard Fault."]
    pub fn ccr(self) -> Reg<fields::Ccr, RW> {
        unsafe { Reg::new(self.0.add(60692usize), fields::Ccr::from_bits(0)) }
    }
    #[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 2 to set the priority of SVCall."]
    pub fn shpr2(self) -> Reg<fields::Shpr2, RW> {
        unsafe { Reg::new(self.0.add(60700usize), fields::Shpr2::from_bits(0)) }
    }
    #[doc = "System handlers are a special class of exception handler that can have their priority set to any of the priority levels. Use the System Handler Priority Register 3 to set the priority of PendSV and SysTick."]
    pub fn shpr3(self) -> Reg<fields::Shpr3, RW> {
        unsafe { Reg::new(self.0.add(60704usize), fields::Shpr3::from_bits(0)) }
    }
    #[doc = "Use the System Handler Control and State Register to determine or clear the pending status of SVCall."]
    pub fn shcsr(self) -> Reg<fields::Shcsr, RW> {
        unsafe { Reg::new(self.0.add(60708usize), fields::Shcsr::from_bits(0)) }
    }
    #[doc = "Read the MPU Type Register to determine if the processor implements an MPU, and how many regions the MPU supports."]
    pub fn mpu_type(self) -> Reg<fields::MpuType, RW> {
        unsafe { Reg::new(self.0.add(60816usize), fields::MpuType::from_bits(2048)) }
    }
    #[doc = "Use the MPU Control Register to enable and disable the MPU, and to control whether the default memory map is enabled as a background region for privileged accesses, and whether the MPU is enabled for HardFaults and NMIs."]
    pub fn mpu_ctrl(self) -> Reg<fields::MpuCtrl, RW> {
        unsafe { Reg::new(self.0.add(60820usize), fields::MpuCtrl::from_bits(0)) }
    }
    #[doc = "Use the MPU Region Number Register to select the region currently accessed by MPU_RBAR and MPU_RASR."]
    pub fn mpu_rnr(self) -> Reg<fields::MpuRnr, RW> {
        unsafe { Reg::new(self.0.add(60824usize), fields::MpuRnr::from_bits(0)) }
    }
    #[doc = "Read the MPU Region Base Address Register to determine the base address of the region identified by MPU_RNR. Write to update the base address of said region or that of a specified region, with whose number MPU_RNR will also be updated."]
    pub fn mpu_rbar(self) -> Reg<fields::MpuRbar, RW> {
        unsafe { Reg::new(self.0.add(60828usize), fields::MpuRbar::from_bits(0)) }
    }
    #[doc = "Use the MPU Region Attribute and Size Register to define the size, access behaviour and memory type of the region identified by MPU_RNR, and enable that region."]
    pub fn mpu_rasr(self) -> Reg<fields::MpuRasr, RW> {
        unsafe { Reg::new(self.0.add(60832usize), fields::MpuRasr::from_bits(0)) }
    }
}
pub mod fields;
