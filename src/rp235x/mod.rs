#![doc = "Peripheral access API (generated using chiptool v0.1.0 (e09c27d 2025-01-02))"]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
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
