#![no_std]
#![doc = include_str!("../README.md")]
#![allow(non_camel_case_types)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Interrupt {
    #[doc = "0 - TIMER_IRQ_0"]
    TIMER_IRQ_0 = 0,
    #[doc = "1 - TIMER_IRQ_1"]
    TIMER_IRQ_1 = 1,
    #[doc = "2 - TIMER_IRQ_2"]
    TIMER_IRQ_2 = 2,
    #[doc = "3 - TIMER_IRQ_3"]
    TIMER_IRQ_3 = 3,
    #[doc = "4 - PWM_IRQ_WRAP"]
    PWM_IRQ_WRAP = 4,
    #[doc = "5 - USBCTRL_IRQ"]
    USBCTRL_IRQ = 5,
    #[doc = "6 - XIP_IRQ"]
    XIP_IRQ = 6,
    #[doc = "7 - PIO0_IRQ_0"]
    PIO0_IRQ_0 = 7,
    #[doc = "8 - PIO0_IRQ_1"]
    PIO0_IRQ_1 = 8,
    #[doc = "9 - PIO1_IRQ_0"]
    PIO1_IRQ_0 = 9,
    #[doc = "10 - PIO1_IRQ_1"]
    PIO1_IRQ_1 = 10,
    #[doc = "11 - DMA_IRQ_0"]
    DMA_IRQ_0 = 11,
    #[doc = "12 - DMA_IRQ_1"]
    DMA_IRQ_1 = 12,
    #[doc = "13 - IO_IRQ_BANK0"]
    IO_IRQ_BANK0 = 13,
    #[doc = "14 - IO_IRQ_QSPI"]
    IO_IRQ_QSPI = 14,
    #[doc = "15 - SIO_IRQ_PROC0"]
    SIO_IRQ_PROC0 = 15,
    #[doc = "16 - SIO_IRQ_PROC1"]
    SIO_IRQ_PROC1 = 16,
    #[doc = "17 - CLOCKS_IRQ"]
    CLOCKS_IRQ = 17,
    #[doc = "18 - SPI0_IRQ"]
    SPI0_IRQ = 18,
    #[doc = "19 - SPI1_IRQ"]
    SPI1_IRQ = 19,
    #[doc = "20 - UART0_IRQ"]
    UART0_IRQ = 20,
    #[doc = "21 - UART1_IRQ"]
    UART1_IRQ = 21,
    #[doc = "22 - ADC_IRQ_FIFO"]
    ADC_IRQ_FIFO = 22,
    #[doc = "23 - I2C0_IRQ"]
    I2C0_IRQ = 23,
    #[doc = "24 - I2C1_IRQ"]
    I2C1_IRQ = 24,
    #[doc = "25 - RTC_IRQ"]
    RTC_IRQ = 25,
    #[doc = "26 - SWI_IRQ_0"]
    SWI_IRQ_0 = 26,
    #[doc = "27 - SWI_IRQ_1"]
    SWI_IRQ_1 = 27,
    #[doc = "28 - SWI_IRQ_2"]
    SWI_IRQ_2 = 28,
    #[doc = "29 - SWI_IRQ_3"]
    SWI_IRQ_3 = 29,
    #[doc = "30 - SWI_IRQ_4"]
    SWI_IRQ_4 = 30,
    #[doc = "31 - SWI_IRQ_5"]
    SWI_IRQ_5 = 31,
}
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[cfg(feature = "rt")]
mod _vectors;
#[doc = "QSPI flash execute-in-place block"]
pub const XIP_CTRL: xip_ctrl::XipCtrl =
    unsafe { xip_ctrl::XipCtrl::from_ptr(0x1400_0000 as usize as _) };
#[doc = "DW_apb_ssi has the following features: * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation. * APB3 and APB4 protocol support. * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits. * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices. * Programmable Dual/Quad/Octal SPI support in Master Mode. * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation. * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes. * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes. * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests. * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently. * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus. * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains. * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates. * Programmable features: - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire. - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation. - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer. * Configured features: - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits. - 1 slave select output. - Hardware slave-select – Dedicated hardware slave-select line. - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller. - Interrupt polarity – active high interrupt lines. - Serial clock polarity – low serial-clock polarity directly after reset. - Serial clock phase – capture on first edge of serial-clock directly after reset."]
pub const XIP_SSI: xip_ssi::XipSsi =
    unsafe { xip_ssi::XipSsi::from_ptr(0x1800_0000 as usize as _) };
pub const SYSINFO: sysinfo::Sysinfo =
    unsafe { sysinfo::Sysinfo::from_ptr(0x4000_0000 as usize as _) };
#[doc = "Register block for various chip control signals"]
pub const SYSCFG: syscfg::Syscfg = unsafe { syscfg::Syscfg::from_ptr(0x4000_4000 as usize as _) };
pub const CLOCKS: clocks::Clocks = unsafe { clocks::Clocks::from_ptr(0x4000_8000 as usize as _) };
pub const RESETS: resets::Resets = unsafe { resets::Resets::from_ptr(0x4000_c000 as usize as _) };
pub const PSM: psm::Psm = unsafe { psm::Psm::from_ptr(0x4001_0000 as usize as _) };
pub const IO_BANK0: io::Io = unsafe { io::Io::from_ptr(0x4001_4000 as usize as _) };
pub const IO_QSPI: io::Io = unsafe { io::Io::from_ptr(0x4001_8000 as usize as _) };
pub const PADS_BANK0: pads::Pads = unsafe { pads::Pads::from_ptr(0x4001_c000 as usize as _) };
pub const PADS_QSPI: pads::Pads = unsafe { pads::Pads::from_ptr(0x4002_0000 as usize as _) };
#[doc = "Controls the crystal oscillator"]
pub const XOSC: xosc::Xosc = unsafe { xosc::Xosc::from_ptr(0x4002_4000 as usize as _) };
pub const PLL_SYS: pll::Pll = unsafe { pll::Pll::from_ptr(0x4002_8000 as usize as _) };
pub const PLL_USB: pll::Pll = unsafe { pll::Pll::from_ptr(0x4002_c000 as usize as _) };
#[doc = "Register block for busfabric control signals and performance counters"]
pub const BUSCTRL: busctrl::Busctrl =
    unsafe { busctrl::Busctrl::from_ptr(0x4003_0000 as usize as _) };
pub const UART0: uart::Uart = unsafe { uart::Uart::from_ptr(0x4003_4000 as usize as _) };
pub const UART1: uart::Uart = unsafe { uart::Uart::from_ptr(0x4003_8000 as usize as _) };
pub const SPI0: spi::Spi = unsafe { spi::Spi::from_ptr(0x4003_c000 as usize as _) };
pub const SPI1: spi::Spi = unsafe { spi::Spi::from_ptr(0x4004_0000 as usize as _) };
#[doc = "DW_apb_i2c address block List of configuration constants for the Synopsys I2C hardware (you may see references to these in I2C register header; these are *fixed* values, set at hardware design time): IC_ULTRA_FAST_MODE ................ 0x0 IC_UFM_TBUF_CNT_DEFAULT ........... 0x8 IC_UFM_SCL_LOW_COUNT .............. 0x0008 IC_UFM_SCL_HIGH_COUNT ............. 0x0006 IC_TX_TL .......................... 0x0 IC_TX_CMD_BLOCK ................... 0x1 IC_HAS_DMA ........................ 0x1 IC_HAS_ASYNC_FIFO ................. 0x0 IC_SMBUS_ARP ...................... 0x0 IC_FIRST_DATA_BYTE_STATUS ......... 0x1 IC_INTR_IO ........................ 0x1 IC_MASTER_MODE .................... 0x1 IC_DEFAULT_ACK_GENERAL_CALL ....... 0x1 IC_INTR_POL ....................... 0x1 IC_OPTIONAL_SAR ................... 0x0 IC_DEFAULT_TAR_SLAVE_ADDR ......... 0x055 IC_DEFAULT_SLAVE_ADDR ............. 0x055 IC_DEFAULT_HS_SPKLEN .............. 0x1 IC_FS_SCL_HIGH_COUNT .............. 0x0006 IC_HS_SCL_LOW_COUNT ............... 0x0008 IC_DEVICE_ID_VALUE ................ 0x0 IC_10BITADDR_MASTER ............... 0x0 IC_CLK_FREQ_OPTIMIZATION .......... 0x0 IC_DEFAULT_FS_SPKLEN .............. 0x7 IC_ADD_ENCODED_PARAMS ............. 0x0 IC_DEFAULT_SDA_HOLD ............... 0x000001 IC_DEFAULT_SDA_SETUP .............. 0x64 IC_AVOID_RX_FIFO_FLUSH_ON_TX_ABRT . 0x0 IC_CLOCK_PERIOD ................... 100 IC_EMPTYFIFO_HOLD_MASTER_EN ....... 1 IC_RESTART_EN ..................... 0x1 IC_TX_CMD_BLOCK_DEFAULT ........... 0x0 IC_BUS_CLEAR_FEATURE .............. 0x0 IC_CAP_LOADING .................... 100 IC_FS_SCL_LOW_COUNT ............... 0x000d APB_DATA_WIDTH .................... 32 IC_SDA_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff IC_SLV_DATA_NACK_ONLY ............. 0x1 IC_10BITADDR_SLAVE ................ 0x0 IC_CLK_TYPE ....................... 0x0 IC_SMBUS_UDID_MSB ................. 0x0 IC_SMBUS_SUSPEND_ALERT ............ 0x0 IC_HS_SCL_HIGH_COUNT .............. 0x0006 IC_SLV_RESTART_DET_EN ............. 0x1 IC_SMBUS .......................... 0x0 IC_OPTIONAL_SAR_DEFAULT ........... 0x0 IC_PERSISTANT_SLV_ADDR_DEFAULT .... 0x0 IC_USE_COUNTS ..................... 0x0 IC_RX_BUFFER_DEPTH ................ 16 IC_SCL_STUCK_TIMEOUT_DEFAULT ...... 0xffffffff IC_RX_FULL_HLD_BUS_EN ............. 0x1 IC_SLAVE_DISABLE .................. 0x1 IC_RX_TL .......................... 0x0 IC_DEVICE_ID ...................... 0x0 IC_HC_COUNT_VALUES ................ 0x0 I2C_DYNAMIC_TAR_UPDATE ............ 0 IC_SMBUS_CLK_LOW_MEXT_DEFAULT ..... 0xffffffff IC_SMBUS_CLK_LOW_SEXT_DEFAULT ..... 0xffffffff IC_HS_MASTER_CODE ................. 0x1 IC_SMBUS_RST_IDLE_CNT_DEFAULT ..... 0xffff IC_SMBUS_UDID_LSB_DEFAULT ......... 0xffffffff IC_SS_SCL_HIGH_COUNT .............. 0x0028 IC_SS_SCL_LOW_COUNT ............... 0x002f IC_MAX_SPEED_MODE ................. 0x2 IC_STAT_FOR_CLK_STRETCH ........... 0x0 IC_STOP_DET_IF_MASTER_ACTIVE ...... 0x0 IC_DEFAULT_UFM_SPKLEN ............. 0x1 IC_TX_BUFFER_DEPTH ................ 16"]
pub const I2C0: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4004_4000 as usize as _) };
pub const I2C1: i2c::I2c = unsafe { i2c::I2c::from_ptr(0x4004_8000 as usize as _) };
#[doc = "Control and data interface to SAR ADC"]
pub const ADC: adc::Adc = unsafe { adc::Adc::from_ptr(0x4004_c000 as usize as _) };
#[doc = "Simple PWM"]
pub const PWM: pwm::Pwm = unsafe { pwm::Pwm::from_ptr(0x4005_0000 as usize as _) };
#[doc = "Controls time and alarms time is a 64 bit value indicating the time in usec since power-on timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr An alarm is set by setting alarm_enable and writing to the corresponding alarm register When an alarm is pending, the corresponding alarm_running signal will be high An alarm can be cancelled before it has finished by clearing the alarm_enable When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared To clear the interrupt write a 1 to the corresponding alarm_irq"]
pub const TIMER: timer::Timer = unsafe { timer::Timer::from_ptr(0x4005_4000 as usize as _) };
pub const WATCHDOG: watchdog::Watchdog =
    unsafe { watchdog::Watchdog::from_ptr(0x4005_8000 as usize as _) };
#[doc = "Register block to control RTC"]
pub const RTC: rtc::Rtc = unsafe { rtc::Rtc::from_ptr(0x4005_c000 as usize as _) };
pub const ROSC: rosc::Rosc = unsafe { rosc::Rosc::from_ptr(0x4006_0000 as usize as _) };
#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
pub const VREG_AND_CHIP_RESET: vreg_and_chip_reset::VregAndChipReset =
    unsafe { vreg_and_chip_reset::VregAndChipReset::from_ptr(0x4006_4000 as usize as _) };
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
pub const TBMAN: tbman::Tbman = unsafe { tbman::Tbman::from_ptr(0x4006_c000 as usize as _) };
#[doc = "DMA with separate read and write masters"]
pub const DMA: dma::Dma = unsafe { dma::Dma::from_ptr(0x5000_0000 as usize as _) };
#[doc = "DPRAM layout for USB device."]
pub const USBCTRL_DPRAM: usb_dpram::UsbDpram =
    unsafe { usb_dpram::UsbDpram::from_ptr(0x5010_0000 as usize as _) };
#[doc = "USB FS/LS controller device registers"]
pub const USBCTRL_REGS: usb::Usb = unsafe { usb::Usb::from_ptr(0x5011_0000 as usize as _) };
#[doc = "Programmable IO block"]
pub const PIO0: pio::Pio = unsafe { pio::Pio::from_ptr(0x5020_0000 as usize as _) };
pub const PIO1: pio::Pio = unsafe { pio::Pio::from_ptr(0x5030_0000 as usize as _) };
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
pub const SIO: sio::Sio = unsafe { sio::Sio::from_ptr(0xd000_0000 as usize as _) };
#[doc = r" Number available in the NVIC for configuring priority"]
#[cfg(feature = "rt")]
pub const NVIC_PRIO_BITS: u8 = 2;
#[cfg(feature = "rt")]
pub use cortex_m_rt::interrupt;
#[cfg(feature = "rt")]
pub use Interrupt as interrupt;
pub mod adc;
pub mod busctrl;
pub mod clocks;
pub mod common;
pub mod dma;
pub mod i2c;
pub mod io;
pub mod pads;
pub mod pio;
pub mod pll;
pub mod psm;
pub mod pwm;
pub mod resets;
pub mod rosc;
pub mod rtc;
pub mod sio;
pub mod spi;
pub mod syscfg;
pub mod sysinfo;
pub mod tbman;
pub mod timer;
pub mod uart;
pub mod usb;
pub mod usb_dpram;
pub mod vreg_and_chip_reset;
pub mod watchdog;
pub mod xip_ctrl;
pub mod xip_ssi;
pub mod xosc;
