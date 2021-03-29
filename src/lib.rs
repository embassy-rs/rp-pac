#![no_std]
#![doc = "Peripheral access API (generated using svd2rust v0.17.0 (426f4c8 2021-03-17))"]
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
}
#[cfg(feature = "rt")]
extern "C" {
    fn TIMER_IRQ_0();
    fn TIMER_IRQ_1();
    fn TIMER_IRQ_2();
    fn TIMER_IRQ_3();
    fn PWM_IRQ_WRAP();
    fn USBCTRL_IRQ();
    fn XIP_IRQ();
    fn PIO0_IRQ_0();
    fn PIO0_IRQ_1();
    fn PIO1_IRQ_0();
    fn PIO1_IRQ_1();
    fn DMA_IRQ_0();
    fn DMA_IRQ_1();
    fn IO_IRQ_BANK0();
    fn IO_IRQ_QSPI();
    fn SIO_IRQ_PROC0();
    fn SIO_IRQ_PROC1();
    fn CLOCKS_IRQ();
    fn SPI0_IRQ();
    fn SPI1_IRQ();
    fn UART0_IRQ();
    fn UART1_IRQ();
    fn ADC_IRQ_FIFO();
    fn I2C0_IRQ();
    fn I2C1_IRQ();
    fn RTC_IRQ();
}
#[doc(hidden)]
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[cfg(feature = "rt")]
#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 26] = [
    Vector {
        _handler: TIMER_IRQ_0,
    },
    Vector {
        _handler: TIMER_IRQ_1,
    },
    Vector {
        _handler: TIMER_IRQ_2,
    },
    Vector {
        _handler: TIMER_IRQ_3,
    },
    Vector {
        _handler: PWM_IRQ_WRAP,
    },
    Vector {
        _handler: USBCTRL_IRQ,
    },
    Vector { _handler: XIP_IRQ },
    Vector {
        _handler: PIO0_IRQ_0,
    },
    Vector {
        _handler: PIO0_IRQ_1,
    },
    Vector {
        _handler: PIO1_IRQ_0,
    },
    Vector {
        _handler: PIO1_IRQ_1,
    },
    Vector {
        _handler: DMA_IRQ_0,
    },
    Vector {
        _handler: DMA_IRQ_1,
    },
    Vector {
        _handler: IO_IRQ_BANK0,
    },
    Vector {
        _handler: IO_IRQ_QSPI,
    },
    Vector {
        _handler: SIO_IRQ_PROC0,
    },
    Vector {
        _handler: SIO_IRQ_PROC1,
    },
    Vector {
        _handler: CLOCKS_IRQ,
    },
    Vector { _handler: SPI0_IRQ },
    Vector { _handler: SPI1_IRQ },
    Vector {
        _handler: UART0_IRQ,
    },
    Vector {
        _handler: UART1_IRQ,
    },
    Vector {
        _handler: ADC_IRQ_FIFO,
    },
    Vector { _handler: I2C0_IRQ },
    Vector { _handler: I2C1_IRQ },
    Vector { _handler: RTC_IRQ },
];
unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
    #[inline(always)]
    fn number(self) -> u16 {
        self as u16
    }
}
#[doc = "QSPI flash execute-in-place block"]
pub const XIP_CTRL: xip_ctrl::XipCtrl = xip_ctrl::XipCtrl(0x1400_0000 as u32 as _);
#[doc = "DW_apb_ssi has the following features: * APB interface – Allows for easy integration into a DesignWare Synthesizable Components for AMBA 2 implementation. * APB3 and APB4 protocol support. * Scalable APB data bus width – Supports APB data bus widths of 8, 16, and 32 bits. * Serial-master or serial-slave operation – Enables serial communication with serial-master or serial-slave peripheral devices. * Programmable Dual/Quad/Octal SPI support in Master Mode. * Dual Data Rate (DDR) and Read Data Strobe (RDS) Support - Enables the DW_apb_ssi master to perform operations with the device in DDR and RDS modes when working in Dual/Quad/Octal mode of operation. * Data Mask Support - Enables the DW_apb_ssi to selectively update the bytes in the device. This feature is applicable only in enhanced SPI modes. * eXecute-In-Place (XIP) support - Enables the DW_apb_ssi master to behave as a memory mapped I/O and fetches the data from the device based on the APB read request. This feature is applicable only in enhanced SPI modes. * DMA Controller Interface – Enables the DW_apb_ssi to interface to a DMA controller over the bus using a handshaking interface for transfer requests. * Independent masking of interrupts – Master collision, transmit FIFO overflow, transmit FIFO empty, receive FIFO full, receive FIFO underflow, and receive FIFO overflow interrupts can all be masked independently. * Multi-master contention detection – Informs the processor of multiple serial-master accesses on the serial bus. * Bypass of meta-stability flip-flops for synchronous clocks – When the APB clock (pclk) and the DW_apb_ssi serial clock (ssi_clk) are synchronous, meta-stable flip-flops are not used when transferring control signals across these clock domains. * Programmable delay on the sample time of the received serial data bit (rxd); enables programmable control of routing delays resulting in higher serial data-bit rates. * Programmable features: - Serial interface operation – Choice of Motorola SPI, Texas Instruments Synchronous Serial Protocol or National Semiconductor Microwire. - Clock bit-rate – Dynamic control of the serial bit rate of the data transfer; used in only serial-master mode of operation. - Data Item size (4 to 32 bits) – Item size of each data transfer under the control of the programmer. * Configured features: - FIFO depth – 16 words deep. The FIFO width is fixed at 32 bits. - 1 slave select output. - Hardware slave-select – Dedicated hardware slave-select line. - Combined interrupt line - one combined interrupt line from the DW_apb_ssi to the interrupt controller. - Interrupt polarity – active high interrupt lines. - Serial clock polarity – low serial-clock polarity directly after reset. - Serial clock phase – capture on first edge of serial-clock directly after reset."]
pub const XIP_SSI: xip_ssi::XipSsi = xip_ssi::XipSsi(0x1800_0000 as u32 as _);
pub const SYSINFO: sysinfo::Sysinfo = sysinfo::Sysinfo(0x4000_0000 as u32 as _);
#[doc = "Register block for various chip control signals"]
pub const SYSCFG: syscfg::Syscfg = syscfg::Syscfg(0x4000_4000 as u32 as _);
pub const CLOCKS: clocks::Clocks = clocks::Clocks(0x4000_8000 as u32 as _);
pub const RESETS: resets::Resets = resets::Resets(0x4000_c000 as u32 as _);
pub const PSM: psm::Psm = psm::Psm(0x4001_0000 as u32 as _);
pub const IO_BANK0: io::Io = io::Io(0x4001_4000 as u32 as _);
pub const IO_QSPI: io::Io = io::Io(0x4001_8000 as u32 as _);
pub const PADS_BANK0: pads::Pads = pads::Pads(0x4001_c000 as u32 as _);
pub const PADS_QSPI: pads::Pads = pads::Pads(0x4002_0000 as u32 as _);
#[doc = "Controls the crystal oscillator"]
pub const XOSC: xosc::Xosc = xosc::Xosc(0x4002_4000 as u32 as _);
pub const PLL_SYS: pll::Pll = pll::Pll(0x4002_8000 as u32 as _);
pub const PLL_USB: pll::Pll = pll::Pll(0x4002_c000 as u32 as _);
#[doc = "Register block for busfabric control signals and performance counters"]
pub const BUSCTRL: busctrl::Busctrl = busctrl::Busctrl(0x4003_0000 as u32 as _);
pub const UART0: uart::Uart = uart::Uart(0x4003_4000 as u32 as _);
pub const UART1: uart::Uart = uart::Uart(0x4003_8000 as u32 as _);
pub const SPI0: spi::Spi = spi::Spi(0x4003_c000 as u32 as _);
pub const SPI1: spi::Spi = spi::Spi(0x4004_0000 as u32 as _);
#[doc = "DW_apb_i2c address block"]
pub const I2C0: i2c::I2c = i2c::I2c(0x4004_4000 as u32 as _);
pub const I2C1: i2c::I2c = i2c::I2c(0x4004_8000 as u32 as _);
#[doc = "Control and data interface to SAR ADC"]
pub const ADC: adc::Adc = adc::Adc(0x4004_c000 as u32 as _);
#[doc = "Simple PWM"]
pub const PWM: pwm::Pwm = pwm::Pwm(0x4005_0000 as u32 as _);
#[doc = "Controls time and alarms time is a 64 bit value indicating the time in usec since power-on timeh is the top 32 bits of time & timel is the bottom 32 bits to change time write to timelw before timehw to read time read from timelr before timehr An alarm is set by setting alarm_enable and writing to the corresponding alarm register When an alarm is pending, the corresponding alarm_running signal will be high An alarm can be cancelled before it has finished by clearing the alarm_enable When an alarm fires, the corresponding alarm_irq is set and alarm_running is cleared To clear the interrupt write a 1 to the corresponding alarm_irq"]
pub const TIMER: timer::Timer = timer::Timer(0x4005_4000 as u32 as _);
pub const WATCHDOG: watchdog::Watchdog = watchdog::Watchdog(0x4005_8000 as u32 as _);
#[doc = "Register block to control RTC"]
pub const RTC: rtc::Rtc = rtc::Rtc(0x4005_c000 as u32 as _);
pub const ROSC: rosc::Rosc = rosc::Rosc(0x4006_0000 as u32 as _);
#[doc = "control and status for on-chip voltage regulator and chip level reset subsystem"]
pub const VREG_AND_CHIP_RESET: vreg_and_chip_reset::VregAndChipReset =
    vreg_and_chip_reset::VregAndChipReset(0x4006_4000 as u32 as _);
#[doc = "Testbench manager. Allows the programmer to know what platform their software is running on."]
pub const TBMAN: tbman::Tbman = tbman::Tbman(0x4006_c000 as u32 as _);
#[doc = "DMA with separate read and write masters"]
pub const DMA: dma::Dma = dma::Dma(0x5000_0000 as u32 as _);
#[doc = "USB FS/LS controller device registers"]
pub const USBCTRL_REGS: usb::Usb = usb::Usb(0x5011_0000 as u32 as _);
#[doc = "Programmable IO block"]
pub const PIO0: pio::Pio = pio::Pio(0x5020_0000 as u32 as _);
pub const PIO1: pio::Pio = pio::Pio(0x5030_0000 as u32 as _);
#[doc = "Single-cycle IO block Provides core-local and inter-core hardware for the two processors, with single-cycle access."]
pub const SIO: sio::Sio = sio::Sio(0xd000_0000 as u32 as _);
#[doc = r"Number available in the NVIC for configuring priority"]
pub const NVIC_PRIO_BITS: u8 = 2;
pub mod adc;
pub mod busctrl;
pub mod clocks;
pub mod dma;
pub mod generic;
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
pub mod vreg_and_chip_reset;
pub mod watchdog;
pub mod xip_ctrl;
pub mod xip_ssi;
pub mod xosc;
