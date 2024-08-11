#[doc = "Hardware access control registers"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Accessctrl {
    ptr: *mut u8,
}
unsafe impl Send for Accessctrl {}
unsafe impl Sync for Accessctrl {}
impl Accessctrl {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Once a LOCK bit is written to 1, ACCESSCTRL silently ignores writes from that master. LOCK is writable only by a Secure, Privileged processor or debugger. LOCK bits are only writable when their value is zero. Once set, they can never be cleared, except by a full reset of ACCESSCTRL Setting the LOCK bit does not affect whether an access raises a bus error. Unprivileged writes, or writes from the DMA, will continue to raise bus errors. All other accesses will continue not to."]
    #[inline(always)]
    pub const fn lock(self) -> crate::common::Reg<regs::Lock, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Force core 1's bus accesses to always be Non-secure, no matter the core's internal state. Useful for schemes where one core is designated as the Non-secure core, since some peripherals may filter individual registers internally based on security state but not on master ID."]
    #[inline(always)]
    pub const fn force_core_ns(self) -> crate::common::Reg<regs::ForceCoreNs, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Write 1 to reset all ACCESSCTRL configuration, except for the LOCK and FORCE_CORE_NS registers. This bit is used in the RP2350 bootrom to quickly restore ACCESSCTRL to a known state during the boot path. Note that, like all registers in ACCESSCTRL, this register is not writable when the writer's corresponding LOCK bit is set, therefore a master which has been locked out of ACCESSCTRL can not use the CFGRESET register to disturb its contents."]
    #[inline(always)]
    pub const fn cfgreset(self) -> crate::common::Reg<regs::Cfgreset, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Control whether GPIO0...31 are accessible to Non-secure code. Writable only by a Secure, Privileged processor or debugger. 0 -> Secure access only 1 -> Secure + Non-secure access"]
    #[inline(always)]
    pub const fn gpio_nsmask0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Control whether GPIO32..47 are accessible to Non-secure code, and whether QSPI and USB bitbang are accessible through the Non-secure SIO. Writable only by a Secure, Privileged processor or debugger."]
    #[inline(always)]
    pub const fn gpio_nsmask1(self) -> crate::common::Reg<regs::GpioNsmask1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROM, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rom(self) -> crate::common::Reg<regs::Rom, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_MAIN, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_main(self) -> crate::common::Reg<regs::XipMain, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM0, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram0(self) -> crate::common::Reg<regs::Sram0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM1, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram1(self) -> crate::common::Reg<regs::Sram1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM2, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram2(self) -> crate::common::Reg<regs::Sram2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM3, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram3(self) -> crate::common::Reg<regs::Sram3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM4, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram4(self) -> crate::common::Reg<regs::Sram4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM5, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram5(self) -> crate::common::Reg<regs::Sram5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM6, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram6(self) -> crate::common::Reg<regs::Sram6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(52usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM7, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram7(self) -> crate::common::Reg<regs::Sram7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(56usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM8, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram8(self) -> crate::common::Reg<regs::Sram8, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(60usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SRAM9, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sram9(self) -> crate::common::Reg<regs::Sram9, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access DMA, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn dma(self) -> crate::common::Reg<regs::Dma, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access USBCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn usbctrl(self) -> crate::common::Reg<regs::Usbctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(72usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio0(self) -> crate::common::Reg<regs::Pio0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(76usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio1(self) -> crate::common::Reg<regs::Pio1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(80usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PIO2, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pio2(self) -> crate::common::Reg<regs::Pio2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(84usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_TRACE, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn coresight_trace(
        self,
    ) -> crate::common::Reg<regs::CoresightTrace, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(88usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access CORESIGHT_PERIPH, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn coresight_periph(
        self,
    ) -> crate::common::Reg<regs::CoresightPeriph, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(92usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSINFO, and at what security/privilege levels they can do so. Defaults to fully open access. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sysinfo(self) -> crate::common::Reg<regs::Sysinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access RESETS, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn resets(self) -> crate::common::Reg<regs::Resets, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(100usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn io_bank0(self) -> crate::common::Reg<regs::IoBank0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(104usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access IO_BANK1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn io_bank1(self) -> crate::common::Reg<regs::IoBank1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_BANK0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pads_bank0(self) -> crate::common::Reg<regs::PadsBank0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(112usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PADS_QSPI, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pads_qspi(self) -> crate::common::Reg<regs::PadsQspi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(116usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access BUSCTRL, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn busctrl(self) -> crate::common::Reg<regs::Busctrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(120usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access ADC0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn adc0(self) -> crate::common::Reg<regs::Adc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(124usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access HSTX, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn hstx(self) -> crate::common::Reg<regs::Hstx, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(128usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn i2c0(self) -> crate::common::Reg<regs::I2c0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(132usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access I2C1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn i2c1(self) -> crate::common::Reg<regs::I2c1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(136usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PWM, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pwm(self) -> crate::common::Reg<regs::Pwm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(140usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn spi0(self) -> crate::common::Reg<regs::Spi0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(144usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SPI1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn spi1(self) -> crate::common::Reg<regs::Spi1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(148usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn timer0(self) -> crate::common::Reg<regs::Timer0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(152usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access TIMER1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn timer1(self) -> crate::common::Reg<regs::Timer1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(156usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART0, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn uart0(self) -> crate::common::Reg<regs::Uart0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(160usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access UART1, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn uart1(self) -> crate::common::Reg<regs::Uart1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(164usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access OTP, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn otp(self) -> crate::common::Reg<regs::Otp, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access TBMAN, and at what security/privilege levels they can do so. Defaults to Secure access from any master. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn tbman(self) -> crate::common::Reg<regs::Tbman, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access POWMAN, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn powman(self) -> crate::common::Reg<regs::Powman, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access TRNG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn trng(self) -> crate::common::Reg<regs::Trng, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(180usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SHA256, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn sha256(self) -> crate::common::Reg<regs::Sha256, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access SYSCFG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn syscfg(self) -> crate::common::Reg<regs::Syscfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access CLOCKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn clocks(self) -> crate::common::Reg<regs::Clocks, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access XOSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xosc(self) -> crate::common::Reg<regs::Xosc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(196usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access ROSC, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rosc(self) -> crate::common::Reg<regs::Rosc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(200usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_SYS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pll_sys(self) -> crate::common::Reg<regs::PllSys, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(204usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access PLL_USB, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn pll_usb(self) -> crate::common::Reg<regs::PllUsb, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(208usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access TICKS, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn ticks(self) -> crate::common::Reg<regs::Ticks, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(212usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access WATCHDOG, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn watchdog(self) -> crate::common::Reg<regs::Watchdog, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access RSM, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn rsm(self) -> crate::common::Reg<regs::Rsm, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(220usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_CTRL, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_ctrl(self) -> crate::common::Reg<regs::XipCtrl, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_QMI, and at what security/privilege levels they can do so. Defaults to Secure, Privileged processor or debug access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_qmi(self) -> crate::common::Reg<regs::XipQmi, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Control whether debugger, DMA, core 0 and core 1 can access XIP_AUX, and at what security/privilege levels they can do so. Defaults to Secure, Privileged access only. This register is writable only from a Secure, Privileged processor or debugger, with the exception of the NSU bit, which becomes Non-secure-Privileged-writable when the NSP bit is set."]
    #[inline(always)]
    pub const fn xip_aux(self) -> crate::common::Reg<regs::XipAux, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
}
pub mod regs;
