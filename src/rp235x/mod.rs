#![doc = "Peripheral access API (generated using chiptool v0.1.0 (689341a 2024-02-15))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - TIMER0_IRQ_0"]
    TIMER0_IRQ_0 = 0,
    #[doc = "1 - TIMER0_IRQ_1"]
    TIMER0_IRQ_1 = 1,
    #[doc = "2 - TIMER0_IRQ_2"]
    TIMER0_IRQ_2 = 2,
    #[doc = "3 - TIMER0_IRQ_3"]
    TIMER0_IRQ_3 = 3,
    #[doc = "4 - TIMER1_IRQ_0"]
    TIMER1_IRQ_0 = 4,
    #[doc = "5 - TIMER1_IRQ_1"]
    TIMER1_IRQ_1 = 5,
    #[doc = "6 - TIMER1_IRQ_2"]
    TIMER1_IRQ_2 = 6,
    #[doc = "7 - TIMER1_IRQ_3"]
    TIMER1_IRQ_3 = 7,
    #[doc = "8 - PWM_IRQ_WRAP_0"]
    PWM_IRQ_WRAP_0 = 8,
    #[doc = "9 - PWM_IRQ_WRAP_1"]
    PWM_IRQ_WRAP_1 = 9,
    #[doc = "10 - DMA_IRQ_0"]
    DMA_IRQ_0 = 10,
    #[doc = "11 - DMA_IRQ_1"]
    DMA_IRQ_1 = 11,
    #[doc = "12 - DMA_IRQ_2"]
    DMA_IRQ_2 = 12,
    #[doc = "13 - DMA_IRQ_3"]
    DMA_IRQ_3 = 13,
    #[doc = "14 - USBCTRL_IRQ"]
    USBCTRL_IRQ = 14,
    #[doc = "15 - PIO0_IRQ_0"]
    PIO0_IRQ_0 = 15,
    #[doc = "16 - PIO0_IRQ_1"]
    PIO0_IRQ_1 = 16,
    #[doc = "17 - PIO1_IRQ_0"]
    PIO1_IRQ_0 = 17,
    #[doc = "18 - PIO1_IRQ_1"]
    PIO1_IRQ_1 = 18,
    #[doc = "19 - PIO2_IRQ_0"]
    PIO2_IRQ_0 = 19,
    #[doc = "20 - PIO2_IRQ_1"]
    PIO2_IRQ_1 = 20,
    #[doc = "21 - IO_IRQ_BANK0"]
    IO_IRQ_BANK0 = 21,
    #[doc = "22 - IO_IRQ_BANK0_NS"]
    IO_IRQ_BANK0_NS = 22,
    #[doc = "23 - IO_IRQ_QSPI"]
    IO_IRQ_QSPI = 23,
    #[doc = "24 - IO_IRQ_QSPI_NS"]
    IO_IRQ_QSPI_NS = 24,
    #[doc = "25 - SIO_IRQ_FIFO"]
    SIO_IRQ_FIFO = 25,
    #[doc = "26 - SIO_IRQ_BELL"]
    SIO_IRQ_BELL = 26,
    #[doc = "27 - SIO_IRQ_FIFO_NS"]
    SIO_IRQ_FIFO_NS = 27,
    #[doc = "28 - SIO_IRQ_BELL_NS"]
    SIO_IRQ_BELL_NS = 28,
    #[doc = "29 - SIO_IRQ_MTIMECMP"]
    SIO_IRQ_MTIMECMP = 29,
    #[doc = "30 - CLOCKS_IRQ"]
    CLOCKS_IRQ = 30,
    #[doc = "31 - SPI0_IRQ"]
    SPI0_IRQ = 31,
    #[doc = "32 - SPI1_IRQ"]
    SPI1_IRQ = 32,
    #[doc = "33 - UART0_IRQ"]
    UART0_IRQ = 33,
    #[doc = "34 - UART1_IRQ"]
    UART1_IRQ = 34,
    #[doc = "35 - ADC_IRQ_FIFO"]
    ADC_IRQ_FIFO = 35,
    #[doc = "36 - I2C0_IRQ"]
    I2C0_IRQ = 36,
    #[doc = "37 - I2C1_IRQ"]
    I2C1_IRQ = 37,
    #[doc = "38 - OTP_IRQ"]
    OTP_IRQ = 38,
    #[doc = "39 - TRNG_IRQ"]
    TRNG_IRQ = 39,
    #[doc = "42 - PLL_SYS_IRQ"]
    PLL_SYS_IRQ = 42,
    #[doc = "43 - PLL_USB_IRQ"]
    PLL_USB_IRQ = 43,
    #[doc = "44 - POWMAN_IRQ_POW"]
    POWMAN_IRQ_POW = 44,
    #[doc = "45 - POWMAN_IRQ_TIMER"]
    POWMAN_IRQ_TIMER = 45,
    #[doc = "47 - SWI_IRQ_0"]
    SWI_IRQ_0 = 47,
    #[doc = "48 - SWI_IRQ_1"]
    SWI_IRQ_1 = 48,
    #[doc = "49 - SWI_IRQ_2"]
    SWI_IRQ_2 = 49,
    #[doc = "50 - SWI_IRQ_3"]
    SWI_IRQ_3 = 50,
    #[doc = "51 - SWI_IRQ_4"]
    SWI_IRQ_4 = 51,
    #[doc = "52 - SWI_IRQ_5"]
    SWI_IRQ_5 = 52,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
pub const SYSINFO: sysinfo::Sysinfo = unsafe { sysinfo::Sysinfo::from_ptr(0x4000_0000usize as _) };
#[doc = "Register block for various chip control signals"]
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4000_8000usize as _) };
pub const CLOCKS: clocks::Clocks = unsafe { clocks::Clocks::from_ptr(0x4001_0000usize as _) };
pub const PSM: psm::Psm = unsafe { psm::Psm::from_ptr(0x4001_8000usize as _) };
pub const RESETS: resets::Resets = unsafe { resets::Resets::from_ptr(0x4002_0000usize as _) };
pub const IO_BANK0: io::Io = unsafe { io::Io::from_ptr(0x4002_8000usize as _) };
pub const IO_QSPI: io::Io = unsafe { io::Io::from_ptr(0x4003_0000usize as _) };
pub const PADS_BANK0: pads::Pads = unsafe { pads::Pads::from_ptr(0x4003_8000usize as _) };
pub const PADS_QSPI: pads::Pads = unsafe { pads::Pads::from_ptr(0x4004_0000usize as _) };
#[doc = "Controls the crystal oscillator"]
pub const XOSC: xosc::Xosc = unsafe { xosc::Xosc::from_ptr(0x4004_8000usize as _) };
pub const PLL_SYS: pll::Pll = unsafe { pll::Pll::from_ptr(0x4005_0000usize as _) };
pub const PLL_USB: pll::Pll = unsafe { pll::Pll::from_ptr(0x4005_8000usize as _) };
#[doc = "Hardware access control registers"]
pub const ACCESSCTRL: accessctrl::Accessctrl =
    unsafe { accessctrl::Accessctrl::from_ptr(0x4006_0000usize as _) };
#[doc = "Register block for busfabric control signals and performance counters"]
pub const BUSCTRL: busctrl::Busctrl = unsafe { busctrl::Busctrl::from_ptr(0x4006_8000usize as _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4007_0000usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x4007_8000usize as _) };
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_0000usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4008_8000usize as _) };
#[doc = "DW_apb_i2c address block List of configuration constants for the Synopsys I2C hardware (you may see references to these in I2C register header; these are *fixed* values, set at hardware design time): IC_ULTRA_FAST_MODE ................ 0x0 IC_UFM_TBUF_CNT_DEFAULT ........... 0x8 IC_UFM_SCL_LOW_COUNT .............. 0x0008 IC_UFM_SCL_HIGH_COUNT ............. 0x0006 IC_TX_TL .......................... 0x0 IC_TX_CMD_BLOCK ................... 0x1 IC_HAS_DMA ........................ 0x1 IC_HAS_ASYNC_FIFO ................. 0x0 IC_SMBUS_ARP ...................... 0x0 IC_FIRST_DATA_BYTE_STATUS ......... 0x1 IC_INTR_IO ........................ 0x1 IC_MASTER_MODE .................... 0x1 IC_DEFAULT_ACK_GENERAL_CALL ....... 0x1 IC_INTR_POL ....................... 0x1 IC_OPTIONAL_SAR ................... 0x0 IC_DEFAULT_TAR_SLAVE_ADDR ......... 0x055 IC_DEFAULT_SLAVE_ADDR ............. 0x055 IC_DEFAULT_HS_SPKLEN .............. 0x1 IC_FS_SCL_HIGH_COUNT .............. 0x0006 IC_HS_SCL_LOW_COUNT ............... 0x0008 IC_DEVICE_ID_VALUE ................ 0x0 IC_10BITADDR_MASTER ............... 0x0 IC_CLK_FREQ_OPTIMIZATION .......... 0x0 IC_DEFAULT_FS_SPKLEN .............. 0x7 IC_ADD_ENCODED_PARAMS ............. 0x0 IC_DEFAULT_SDA_HOLD ............... 0x000001 IC_DEFAULT_SDA_SETUP .............. 0x64 IC_AVOID_RX_FIFO_FLUSH_ON_TX_ABRT . 0x0 IC_CLOCK_PERIOD ................... 100 IC_EMPTYFIFO_HOLD_MASTER_EN ....... 1 IC_RESTART_EN ..................... 0x1 IC_TX_CMD_BLOCK_DEFAULT ........... 0x0 IC_BUS_CLEAR_FEATURE .............. 0x0 IC_CAP_LOADING .................... 100 IC_FS_SCL_LOW_COUNT ............... 0x000d APB_DATA_WIDTH .................... 32 IC_SDA_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff IC_SLV_DATA_NACK_ONLY ............. 0x1 IC_10BITADDR_SLAVE ................ 0x0 IC_CLK_TYPE ....................... 0x0 IC_SMBUS_UDID_MSB ................. 0x0 IC_SMBUS_SUSPEND_ALERT ............ 0x0 IC_HS_SCL_HIGH_COUNT .............. 0x0006 IC_SLV_RESTART_DET_EN ............. 0x1 IC_SMBUS .......................... 0x0 IC_OPTIONAL_SAR_DEFAULT ........... 0x0 IC_PERSISTANT_SLV_ADDR_DEFAULT .... 0x0 IC_USE_COUNTS ..................... 0x0 IC_RX_BUFFER_DEPTH ................ 16 IC_SCL_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff IC_RX_FULL_HLD_BUS_EN ............. 0x1 IC_SLAVE_DISABLE .................. 0x1 IC_RX_TL .......................... 0x0 IC_DEVICE_ID ...................... 0x0 IC_HC_COUNT_VALUES ................ 0x0 I2C_DYNAMIC_TAR_UPDATE ............ 0 IC_SMBUS_CLK_LOW_MEXT_DEFAULT ..... 0xffffffff IC_SMBUS_CLK_LOW_SEXT_DEFAULT ..... 0xffffffff IC_HS_MASTER_CODE ................. 0x1 IC_SMBUS_RST_IDLE_CNT_DEFAULT ..... 0xffff IC_SMBUS_UDID_LSB_DEFAULT ......... 0xffffffff IC_SS_SCL_HIGH_COUNT .............. 0x0028 IC_SS_SCL_LOW_COUNT ............... 0x002f IC_MAX_SPEED_MODE ................. 0x2 IC_STAT_FOR_CLK_STRETCH ........... 0x0 IC_STOP_DET_IF_MASTER_ACTIVE ...... 0x0 IC_DEFAULT_UFM_SPKLEN ............. 0x1 IC_TX_BUFFER_DEPTH ................ 16"]
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4009_0000usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4009_8000usize as _) };
#[doc = "Control and data interface to SAR ADC"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x400a_0000usize as _) };
#[doc = "Simple PWM"]
pub const PWM: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x400a_8000usize as _) };
#[doc = "Controls time and alarms time is a 64 bit value indicating the time since power-on timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr An alarm is set by setting alarm_enable and writing to the corresponding alarm register When an alarm is pending, the corresponding alarm_running signal will be high An alarm can be cancelled before it has finished by clearing the alarm_enable When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared To clear the interrupt write a 1 to the corresponding alarm_irq The timer can be locked to prevent writing"]
pub const TIMER0: timer::Timer = unsafe { timer::Timer::from_ptr(0x400b_0000usize as _) };
pub const TIMER1: timer::Timer = unsafe { timer::Timer::from_ptr(0x400b_8000usize as _) };
#[doc = "Control interface to HSTX. For FIFO write access and status, see the HSTX_FIFO register block."]
pub const HSTX_CTRL: hstx_ctrl::HstxCtrl =
    unsafe { hstx_ctrl::HstxCtrl::from_ptr(0x400c_0000usize as _) };
#[doc = "QSPI flash execute-in-place block"]
pub const XIP_CTRL: xip_ctrl::XipCtrl =
    unsafe { xip_ctrl::XipCtrl::from_ptr(0x400c_8000usize as _) };
#[doc = "QSPI Memory Interface. Provides a memory-mapped interface to up to two SPI/DSPI/QSPI flash or PSRAM devices. Also provides a serial interface for programming and configuration of the external device."]
pub const QMI: qmi::Qmi = unsafe { qmi::Qmi::from_ptr(0x400d_0000usize as _) };
pub const WATCHDOG: watchdog::Watchdog =
    unsafe { watchdog::Watchdog::from_ptr(0x400d_8000usize as _) };
#[doc = "Additional registers mapped adjacent to the bootram, for use by the bootrom."]
pub const BOOTRAM: bootram::Bootram = unsafe { bootram::Bootram::from_ptr(0x400e_0000usize as _) };
pub const ROSC: rosc::Rosc = unsafe { rosc::Rosc::from_ptr(0x400e_8000usize as _) };
#[doc = "ARM TrustZone RNG register block"]
pub const TRNG: trng::Trng = unsafe { trng::Trng::from_ptr(0x400f_0000usize as _) };
#[doc = "SHA-256 hash function implementation"]
pub const SHA256: sha256::Sha256 = unsafe { sha256::Sha256::from_ptr(0x400f_8000usize as _) };
#[doc = "Controls vreg, bor, lposc, chip resets & xosc startup, powman and provides scratch register for general use and for bootcode use"]
pub const POWMAN: powman::Powman = unsafe { powman::Powman::from_ptr(0x4010_0000usize as _) };
pub const TICKS: ticks::Ticks = unsafe { ticks::Ticks::from_ptr(0x4010_8000usize as _) };
#[doc = "SNPS OTP control IF (SBPI and RPi wrapper control)"]
pub const OTP: otp::Otp = unsafe { otp::Otp::from_ptr(0x4012_0000usize as _) };
#[doc = "Predefined OTP data layout for RP2350"]
pub const OTP_DATA: otp_data::OtpData =
    unsafe { otp_data::OtpData::from_ptr(0x4013_0000usize as _) };
#[doc = "Predefined OTP data layout for RP2350"]
pub const OTP_DATA_RAW: otp_data_raw::OtpDataRaw =
    unsafe { otp_data_raw::OtpDataRaw::from_ptr(0x4013_4000usize as _) };
#[doc = "Glitch detector controls"]
pub const GLITCH_DETECTOR: glitch_detector::GlitchDetector =
    unsafe { glitch_detector::GlitchDetector::from_ptr(0x4015_8000usize as _) };
#[doc = "For managing simulation testbenches"]
pub const TBMAN: tbman::Tbman = unsafe { tbman::Tbman::from_ptr(0x4016_0000usize as _) };
#[doc = "DMA with separate read and write masters"]
pub const DMA: dma::Dma = unsafe { dma::Dma::from_ptr(0x5000_0000usize as _) };
#[doc = "DPRAM layout for USB device."]
pub const USB_DPRAM: usb_dpram::UsbDpram =
    unsafe { usb_dpram::UsbDpram::from_ptr(0x5010_0000usize as _) };
#[doc = "USB FS/LS controller device registers"]
pub const USB: usb::Usb = unsafe { usb::Usb::from_ptr(0x5011_0000usize as _) };
#[doc = "Programmable IO block"]
pub const PIO0: pio::Pio = unsafe { pio::Pio::from_ptr(0x5020_0000usize as _) };
pub const PIO1: pio::Pio = unsafe { pio::Pio::from_ptr(0x5030_0000usize as _) };
pub const PIO2: pio::Pio = unsafe { pio::Pio::from_ptr(0x5040_0000usize as _) };
#[doc = "Auxiliary DMA access to XIP FIFOs, via fast AHB bus access"]
pub const XIP_AUX: xip_aux::XipAux = unsafe { xip_aux::XipAux::from_ptr(0x5050_0000usize as _) };
#[doc = "FIFO status and write access for HSTX"]
pub const HSTX_FIFO: hstx_fifo::HstxFifo =
    unsafe { hstx_fifo::HstxFifo::from_ptr(0x5060_0000usize as _) };
#[doc = "Coresight block - RP specific registers"]
pub const CORESIGHT_TRACE: coresight_trace::CoresightTrace =
    unsafe { coresight_trace::CoresightTrace::from_ptr(0x5070_0000usize as _) };
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
pub const SIO: sio::Sio = unsafe { sio::Sio::from_ptr(0xd000_0000usize as _) };
pub const SIO_NS: sio::Sio = unsafe { sio::Sio::from_ptr(0xd002_0000usize as _) };
#[doc = "Cortex-M33 EPPB vendor register block for RP2350"]
pub const EPPB: eppb::Eppb = unsafe { eppb::Eppb::from_ptr(0xe008_0000usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 4;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags0R1(pub u32);
impl BootFlags0R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags0R1 {
    #[inline(always)]
    fn default() -> BootFlags0R1 {
        BootFlags0R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags0R2(pub u32);
impl BootFlags0R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags0R2 {
    #[inline(always)]
    fn default() -> BootFlags0R2 {
        BootFlags0R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags1R1(pub u32);
impl BootFlags1R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags1R1 {
    #[inline(always)]
    fn default() -> BootFlags1R1 {
        BootFlags1R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags1R2(pub u32);
impl BootFlags1R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags1R2 {
    #[inline(always)]
    fn default() -> BootFlags1R2 {
        BootFlags1R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey00(pub u32);
impl Bootkey00 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey00 {
    #[inline(always)]
    fn default() -> Bootkey00 {
        Bootkey00(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey01(pub u32);
impl Bootkey01 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey01 {
    #[inline(always)]
    fn default() -> Bootkey01 {
        Bootkey01(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey010(pub u32);
impl Bootkey010 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey010 {
    #[inline(always)]
    fn default() -> Bootkey010 {
        Bootkey010(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey011(pub u32);
impl Bootkey011 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey011 {
    #[inline(always)]
    fn default() -> Bootkey011 {
        Bootkey011(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey012(pub u32);
impl Bootkey012 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey012 {
    #[inline(always)]
    fn default() -> Bootkey012 {
        Bootkey012(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey013(pub u32);
impl Bootkey013 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey013 {
    #[inline(always)]
    fn default() -> Bootkey013 {
        Bootkey013(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey014(pub u32);
impl Bootkey014 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey014 {
    #[inline(always)]
    fn default() -> Bootkey014 {
        Bootkey014(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey015(pub u32);
impl Bootkey015 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey015 {
    #[inline(always)]
    fn default() -> Bootkey015 {
        Bootkey015(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey02(pub u32);
impl Bootkey02 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey02 {
    #[inline(always)]
    fn default() -> Bootkey02 {
        Bootkey02(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey03(pub u32);
impl Bootkey03 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey03 {
    #[inline(always)]
    fn default() -> Bootkey03 {
        Bootkey03(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey04(pub u32);
impl Bootkey04 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey04 {
    #[inline(always)]
    fn default() -> Bootkey04 {
        Bootkey04(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey05(pub u32);
impl Bootkey05 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey05 {
    #[inline(always)]
    fn default() -> Bootkey05 {
        Bootkey05(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey06(pub u32);
impl Bootkey06 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey06 {
    #[inline(always)]
    fn default() -> Bootkey06 {
        Bootkey06(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey07(pub u32);
impl Bootkey07 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey07 {
    #[inline(always)]
    fn default() -> Bootkey07 {
        Bootkey07(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey08(pub u32);
impl Bootkey08 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey08 {
    #[inline(always)]
    fn default() -> Bootkey08 {
        Bootkey08(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey09(pub u32);
impl Bootkey09 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey09 {
    #[inline(always)]
    fn default() -> Bootkey09 {
        Bootkey09(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey10(pub u32);
impl Bootkey10 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey10 {
    #[inline(always)]
    fn default() -> Bootkey10 {
        Bootkey10(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey11(pub u32);
impl Bootkey11 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey11 {
    #[inline(always)]
    fn default() -> Bootkey11 {
        Bootkey11(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey110(pub u32);
impl Bootkey110 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey110 {
    #[inline(always)]
    fn default() -> Bootkey110 {
        Bootkey110(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey111(pub u32);
impl Bootkey111 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey111 {
    #[inline(always)]
    fn default() -> Bootkey111 {
        Bootkey111(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey112(pub u32);
impl Bootkey112 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey112 {
    #[inline(always)]
    fn default() -> Bootkey112 {
        Bootkey112(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey113(pub u32);
impl Bootkey113 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey113 {
    #[inline(always)]
    fn default() -> Bootkey113 {
        Bootkey113(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey114(pub u32);
impl Bootkey114 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey114 {
    #[inline(always)]
    fn default() -> Bootkey114 {
        Bootkey114(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey115(pub u32);
impl Bootkey115 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey115 {
    #[inline(always)]
    fn default() -> Bootkey115 {
        Bootkey115(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey12(pub u32);
impl Bootkey12 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey12 {
    #[inline(always)]
    fn default() -> Bootkey12 {
        Bootkey12(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey13(pub u32);
impl Bootkey13 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey13 {
    #[inline(always)]
    fn default() -> Bootkey13 {
        Bootkey13(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey14(pub u32);
impl Bootkey14 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey14 {
    #[inline(always)]
    fn default() -> Bootkey14 {
        Bootkey14(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey15(pub u32);
impl Bootkey15 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey15 {
    #[inline(always)]
    fn default() -> Bootkey15 {
        Bootkey15(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey16(pub u32);
impl Bootkey16 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey16 {
    #[inline(always)]
    fn default() -> Bootkey16 {
        Bootkey16(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey17(pub u32);
impl Bootkey17 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey17 {
    #[inline(always)]
    fn default() -> Bootkey17 {
        Bootkey17(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey18(pub u32);
impl Bootkey18 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey18 {
    #[inline(always)]
    fn default() -> Bootkey18 {
        Bootkey18(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey19(pub u32);
impl Bootkey19 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey19 {
    #[inline(always)]
    fn default() -> Bootkey19 {
        Bootkey19(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey20(pub u32);
impl Bootkey20 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey20 {
    #[inline(always)]
    fn default() -> Bootkey20 {
        Bootkey20(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey21(pub u32);
impl Bootkey21 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey21 {
    #[inline(always)]
    fn default() -> Bootkey21 {
        Bootkey21(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey210(pub u32);
impl Bootkey210 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey210 {
    #[inline(always)]
    fn default() -> Bootkey210 {
        Bootkey210(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey211(pub u32);
impl Bootkey211 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey211 {
    #[inline(always)]
    fn default() -> Bootkey211 {
        Bootkey211(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey212(pub u32);
impl Bootkey212 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey212 {
    #[inline(always)]
    fn default() -> Bootkey212 {
        Bootkey212(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey213(pub u32);
impl Bootkey213 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey213 {
    #[inline(always)]
    fn default() -> Bootkey213 {
        Bootkey213(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey214(pub u32);
impl Bootkey214 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey214 {
    #[inline(always)]
    fn default() -> Bootkey214 {
        Bootkey214(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey215(pub u32);
impl Bootkey215 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey215 {
    #[inline(always)]
    fn default() -> Bootkey215 {
        Bootkey215(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey22(pub u32);
impl Bootkey22 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey22 {
    #[inline(always)]
    fn default() -> Bootkey22 {
        Bootkey22(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey23(pub u32);
impl Bootkey23 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey23 {
    #[inline(always)]
    fn default() -> Bootkey23 {
        Bootkey23(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey24(pub u32);
impl Bootkey24 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey24 {
    #[inline(always)]
    fn default() -> Bootkey24 {
        Bootkey24(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey25(pub u32);
impl Bootkey25 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey25 {
    #[inline(always)]
    fn default() -> Bootkey25 {
        Bootkey25(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey26(pub u32);
impl Bootkey26 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey26 {
    #[inline(always)]
    fn default() -> Bootkey26 {
        Bootkey26(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey27(pub u32);
impl Bootkey27 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey27 {
    #[inline(always)]
    fn default() -> Bootkey27 {
        Bootkey27(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey28(pub u32);
impl Bootkey28 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey28 {
    #[inline(always)]
    fn default() -> Bootkey28 {
        Bootkey28(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey29(pub u32);
impl Bootkey29 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey29 {
    #[inline(always)]
    fn default() -> Bootkey29 {
        Bootkey29(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey30(pub u32);
impl Bootkey30 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey30 {
    #[inline(always)]
    fn default() -> Bootkey30 {
        Bootkey30(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey31(pub u32);
impl Bootkey31 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey31 {
    #[inline(always)]
    fn default() -> Bootkey31 {
        Bootkey31(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey310(pub u32);
impl Bootkey310 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey310 {
    #[inline(always)]
    fn default() -> Bootkey310 {
        Bootkey310(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey311(pub u32);
impl Bootkey311 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey311 {
    #[inline(always)]
    fn default() -> Bootkey311 {
        Bootkey311(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey312(pub u32);
impl Bootkey312 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey312 {
    #[inline(always)]
    fn default() -> Bootkey312 {
        Bootkey312(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey313(pub u32);
impl Bootkey313 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey313 {
    #[inline(always)]
    fn default() -> Bootkey313 {
        Bootkey313(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey314(pub u32);
impl Bootkey314 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey314 {
    #[inline(always)]
    fn default() -> Bootkey314 {
        Bootkey314(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey315(pub u32);
impl Bootkey315 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey315 {
    #[inline(always)]
    fn default() -> Bootkey315 {
        Bootkey315(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey32(pub u32);
impl Bootkey32 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey32 {
    #[inline(always)]
    fn default() -> Bootkey32 {
        Bootkey32(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey33(pub u32);
impl Bootkey33 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey33 {
    #[inline(always)]
    fn default() -> Bootkey33 {
        Bootkey33(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey34(pub u32);
impl Bootkey34 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey34 {
    #[inline(always)]
    fn default() -> Bootkey34 {
        Bootkey34(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey35(pub u32);
impl Bootkey35 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey35 {
    #[inline(always)]
    fn default() -> Bootkey35 {
        Bootkey35(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey36(pub u32);
impl Bootkey36 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey36 {
    #[inline(always)]
    fn default() -> Bootkey36 {
        Bootkey36(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey37(pub u32);
impl Bootkey37 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey37 {
    #[inline(always)]
    fn default() -> Bootkey37 {
        Bootkey37(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey38(pub u32);
impl Bootkey38 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey38 {
    #[inline(always)]
    fn default() -> Bootkey38 {
        Bootkey38(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey39(pub u32);
impl Bootkey39 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey39 {
    #[inline(always)]
    fn default() -> Bootkey39 {
        Bootkey39(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid0(pub u32);
impl Chipid0 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid0 {
    #[inline(always)]
    fn default() -> Chipid0 {
        Chipid0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid1(pub u32);
impl Chipid1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid1 {
    #[inline(always)]
    fn default() -> Chipid1 {
        Chipid1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid2(pub u32);
impl Chipid2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid2 {
    #[inline(always)]
    fn default() -> Chipid2 {
        Chipid2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid3(pub u32);
impl Chipid3 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid3 {
    #[inline(always)]
    fn default() -> Chipid3 {
        Chipid3(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R1(pub u32);
impl Crit0R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R1 {
    #[inline(always)]
    fn default() -> Crit0R1 {
        Crit0R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R2(pub u32);
impl Crit0R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R2 {
    #[inline(always)]
    fn default() -> Crit0R2 {
        Crit0R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R3(pub u32);
impl Crit0R3 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R3 {
    #[inline(always)]
    fn default() -> Crit0R3 {
        Crit0R3(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R4(pub u32);
impl Crit0R4 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R4 {
    #[inline(always)]
    fn default() -> Crit0R4 {
        Crit0R4(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R5(pub u32);
impl Crit0R5 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R5 {
    #[inline(always)]
    fn default() -> Crit0R5 {
        Crit0R5(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R6(pub u32);
impl Crit0R6 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R6 {
    #[inline(always)]
    fn default() -> Crit0R6 {
        Crit0R6(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0R7(pub u32);
impl Crit0R7 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0R7 {
    #[inline(always)]
    fn default() -> Crit0R7 {
        Crit0R7(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R1(pub u32);
impl Crit1R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R1 {
    #[inline(always)]
    fn default() -> Crit1R1 {
        Crit1R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R2(pub u32);
impl Crit1R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R2 {
    #[inline(always)]
    fn default() -> Crit1R2 {
        Crit1R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R3(pub u32);
impl Crit1R3 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R3 {
    #[inline(always)]
    fn default() -> Crit1R3 {
        Crit1R3(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R4(pub u32);
impl Crit1R4 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R4 {
    #[inline(always)]
    fn default() -> Crit1R4 {
        Crit1R4(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R5(pub u32);
impl Crit1R5 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R5 {
    #[inline(always)]
    fn default() -> Crit1R5 {
        Crit1R5(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R6(pub u32);
impl Crit1R6 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R6 {
    #[inline(always)]
    fn default() -> Crit1R6 {
        Crit1R6(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1R7(pub u32);
impl Crit1R7 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1R7 {
    #[inline(always)]
    fn default() -> Crit1R7 {
        Crit1R7(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0(pub u32);
impl DefaultBootVersion0 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0 {
        DefaultBootVersion0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0R1(pub u32);
impl DefaultBootVersion0R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0R1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0R1 {
        DefaultBootVersion0R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0R2(pub u32);
impl DefaultBootVersion0R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0R2 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0R2 {
        DefaultBootVersion0R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1(pub u32);
impl DefaultBootVersion1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1 {
        DefaultBootVersion1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1R1(pub u32);
impl DefaultBootVersion1R1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1R1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1R1 {
        DefaultBootVersion1R1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1R2(pub u32);
impl DefaultBootVersion1R2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1R2 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1R2 {
        DefaultBootVersion1R2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashPartitionSlotSize(pub u32);
impl FlashPartitionSlotSize {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FlashPartitionSlotSize {
    #[inline(always)]
    fn default() -> FlashPartitionSlotSize {
        FlashPartitionSlotSize(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc0(pub u32);
impl InfoCrc0 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for InfoCrc0 {
    #[inline(always)]
    fn default() -> InfoCrc0 {
        InfoCrc0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc1(pub u32);
impl InfoCrc1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for InfoCrc1 {
    #[inline(always)]
    fn default() -> InfoCrc1 {
        InfoCrc1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key10(pub u32);
impl Key10 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key10 {
    #[inline(always)]
    fn default() -> Key10 {
        Key10(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key11(pub u32);
impl Key11 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key11 {
    #[inline(always)]
    fn default() -> Key11 {
        Key11(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key12(pub u32);
impl Key12 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key12 {
    #[inline(always)]
    fn default() -> Key12 {
        Key12(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key13(pub u32);
impl Key13 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key13 {
    #[inline(always)]
    fn default() -> Key13 {
        Key13(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key14(pub u32);
impl Key14 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key14 {
    #[inline(always)]
    fn default() -> Key14 {
        Key14(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key15(pub u32);
impl Key15 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key15 {
    #[inline(always)]
    fn default() -> Key15 {
        Key15(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key16(pub u32);
impl Key16 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key16 {
    #[inline(always)]
    fn default() -> Key16 {
        Key16(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key17(pub u32);
impl Key17 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key17 {
    #[inline(always)]
    fn default() -> Key17 {
        Key17(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key20(pub u32);
impl Key20 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key20 {
    #[inline(always)]
    fn default() -> Key20 {
        Key20(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key21(pub u32);
impl Key21 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key21 {
    #[inline(always)]
    fn default() -> Key21 {
        Key21(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key22(pub u32);
impl Key22 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key22 {
    #[inline(always)]
    fn default() -> Key22 {
        Key22(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key23(pub u32);
impl Key23 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key23 {
    #[inline(always)]
    fn default() -> Key23 {
        Key23(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key24(pub u32);
impl Key24 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key24 {
    #[inline(always)]
    fn default() -> Key24 {
        Key24(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key25(pub u32);
impl Key25 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key25 {
    #[inline(always)]
    fn default() -> Key25 {
        Key25(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key26(pub u32);
impl Key26 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key26 {
    #[inline(always)]
    fn default() -> Key26 {
        Key26(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key27(pub u32);
impl Key27 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key27 {
    #[inline(always)]
    fn default() -> Key27 {
        Key27(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key30(pub u32);
impl Key30 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key30 {
    #[inline(always)]
    fn default() -> Key30 {
        Key30(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key31(pub u32);
impl Key31 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key31 {
    #[inline(always)]
    fn default() -> Key31 {
        Key31(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key32(pub u32);
impl Key32 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key32 {
    #[inline(always)]
    fn default() -> Key32 {
        Key32(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key33(pub u32);
impl Key33 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key33 {
    #[inline(always)]
    fn default() -> Key33 {
        Key33(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key34(pub u32);
impl Key34 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key34 {
    #[inline(always)]
    fn default() -> Key34 {
        Key34(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key35(pub u32);
impl Key35 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key35 {
    #[inline(always)]
    fn default() -> Key35 {
        Key35(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key36(pub u32);
impl Key36 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key36 {
    #[inline(always)]
    fn default() -> Key36 {
        Key36(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key37(pub u32);
impl Key37 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key37 {
    #[inline(always)]
    fn default() -> Key37 {
        Key37(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key40(pub u32);
impl Key40 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key40 {
    #[inline(always)]
    fn default() -> Key40 {
        Key40(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key41(pub u32);
impl Key41 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key41 {
    #[inline(always)]
    fn default() -> Key41 {
        Key41(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key42(pub u32);
impl Key42 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key42 {
    #[inline(always)]
    fn default() -> Key42 {
        Key42(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key43(pub u32);
impl Key43 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key43 {
    #[inline(always)]
    fn default() -> Key43 {
        Key43(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key44(pub u32);
impl Key44 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key44 {
    #[inline(always)]
    fn default() -> Key44 {
        Key44(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key45(pub u32);
impl Key45 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key45 {
    #[inline(always)]
    fn default() -> Key45 {
        Key45(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key46(pub u32);
impl Key46 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key46 {
    #[inline(always)]
    fn default() -> Key46 {
        Key46(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key47(pub u32);
impl Key47 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key47 {
    #[inline(always)]
    fn default() -> Key47 {
        Key47(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key50(pub u32);
impl Key50 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key50 {
    #[inline(always)]
    fn default() -> Key50 {
        Key50(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key51(pub u32);
impl Key51 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key51 {
    #[inline(always)]
    fn default() -> Key51 {
        Key51(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key52(pub u32);
impl Key52 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key52 {
    #[inline(always)]
    fn default() -> Key52 {
        Key52(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key53(pub u32);
impl Key53 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key53 {
    #[inline(always)]
    fn default() -> Key53 {
        Key53(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key54(pub u32);
impl Key54 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key54 {
    #[inline(always)]
    fn default() -> Key54 {
        Key54(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key55(pub u32);
impl Key55 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key55 {
    #[inline(always)]
    fn default() -> Key55 {
        Key55(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key56(pub u32);
impl Key56 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key56 {
    #[inline(always)]
    fn default() -> Key56 {
        Key56(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key57(pub u32);
impl Key57 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key57 {
    #[inline(always)]
    fn default() -> Key57 {
        Key57(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key60(pub u32);
impl Key60 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key60 {
    #[inline(always)]
    fn default() -> Key60 {
        Key60(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key61(pub u32);
impl Key61 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key61 {
    #[inline(always)]
    fn default() -> Key61 {
        Key61(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key62(pub u32);
impl Key62 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key62 {
    #[inline(always)]
    fn default() -> Key62 {
        Key62(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key63(pub u32);
impl Key63 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key63 {
    #[inline(always)]
    fn default() -> Key63 {
        Key63(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key64(pub u32);
impl Key64 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key64 {
    #[inline(always)]
    fn default() -> Key64 {
        Key64(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key65(pub u32);
impl Key65 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key65 {
    #[inline(always)]
    fn default() -> Key65 {
        Key65(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key66(pub u32);
impl Key66 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key66 {
    #[inline(always)]
    fn default() -> Key66 {
        Key66(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key67(pub u32);
impl Key67 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key67 {
    #[inline(always)]
    fn default() -> Key67 {
        Key67(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LposcCalib(pub u32);
impl LposcCalib {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for LposcCalib {
    #[inline(always)]
    fn default() -> LposcCalib {
        LposcCalib(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumGpios(pub u32);
impl NumGpios {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for NumGpios {
    #[inline(always)]
    fn default() -> NumGpios {
        NumGpios(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst0(pub u32);
impl OtpbootDst0 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootDst0 {
    #[inline(always)]
    fn default() -> OtpbootDst0 {
        OtpbootDst0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst1(pub u32);
impl OtpbootDst1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootDst1 {
    #[inline(always)]
    fn default() -> OtpbootDst1 {
        OtpbootDst1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootLen(pub u32);
impl OtpbootLen {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootLen {
    #[inline(always)]
    fn default() -> OtpbootLen {
        OtpbootLen(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootSrc(pub u32);
impl OtpbootSrc {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootSrc {
    #[inline(always)]
    fn default() -> OtpbootSrc {
        OtpbootSrc(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid0(pub u32);
impl Randid0 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid0 {
    #[inline(always)]
    fn default() -> Randid0 {
        Randid0(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid1(pub u32);
impl Randid1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid1 {
    #[inline(always)]
    fn default() -> Randid1 {
        Randid1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid2(pub u32);
impl Randid2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid2 {
    #[inline(always)]
    fn default() -> Randid2 {
        Randid2(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid3(pub u32);
impl Randid3 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid3 {
    #[inline(always)]
    fn default() -> Randid3 {
        Randid3(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid4(pub u32);
impl Randid4 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid4 {
    #[inline(always)]
    fn default() -> Randid4 {
        Randid4(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid5(pub u32);
impl Randid5 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid5 {
    #[inline(always)]
    fn default() -> Randid5 {
        Randid5(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid6(pub u32);
impl Randid6 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid6 {
    #[inline(always)]
    fn default() -> Randid6 {
        Randid6(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid7(pub u32);
impl Randid7 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid7 {
    #[inline(always)]
    fn default() -> Randid7 {
        Randid7(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RoscCalib(pub u32);
impl RoscCalib {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for RoscCalib {
    #[inline(always)]
    fn default() -> RoscCalib {
        RoscCalib(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbBootFlagsR1(pub u32);
impl UsbBootFlagsR1 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for UsbBootFlagsR1 {
    #[inline(always)]
    fn default() -> UsbBootFlagsR1 {
        UsbBootFlagsR1(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbBootFlagsR2(pub u32);
impl UsbBootFlagsR2 {
    #[inline(always)]
    pub const fn val(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_val(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for UsbBootFlagsR2 {
    #[inline(always)]
    fn default() -> UsbBootFlagsR2 {
        UsbBootFlagsR2(0)
    }
}
pub mod accessctrl;
pub mod adc;
pub mod bootram;
pub mod busctrl;
pub mod clocks;
pub mod common;
pub mod coresight_trace;
pub mod dma;
pub mod eppb;
pub mod glitch_detector;
pub mod hstx_ctrl;
pub mod hstx_fifo;
pub mod i2c;
pub mod io;
pub mod otp;
pub mod otp_data;
pub mod otp_data_raw;
pub mod pads;
pub mod pio;
pub mod pll;
pub mod powman;
pub mod psm;
pub mod pwm;
pub mod qmi;
pub mod resets;
pub mod rosc;
pub mod sha256;
pub mod sio;
pub mod spi;
pub mod syscfg;
pub mod sysinfo;
pub mod tbman;
pub mod ticks;
pub mod timer;
pub mod trng;
pub mod uart;
pub mod usb;
pub mod usb_dpram;
pub mod watchdog;
pub mod xip_aux;
pub mod xip_ctrl;
pub mod xosc;
