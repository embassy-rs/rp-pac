extern "C" {
    fn TIMER0_IRQ_0();
    fn TIMER0_IRQ_1();
    fn TIMER0_IRQ_2();
    fn TIMER0_IRQ_3();
    fn TIMER1_IRQ_0();
    fn TIMER1_IRQ_1();
    fn TIMER1_IRQ_2();
    fn TIMER1_IRQ_3();
    fn PWM_IRQ_WRAP_0();
    fn PWM_IRQ_WRAP_1();
    fn DMA_IRQ_0();
    fn DMA_IRQ_1();
    fn DMA_IRQ_2();
    fn DMA_IRQ_3();
    fn USBCTRL_IRQ();
    fn PIO0_IRQ_0();
    fn PIO0_IRQ_1();
    fn PIO1_IRQ_0();
    fn PIO1_IRQ_1();
    fn PIO2_IRQ_0();
    fn PIO2_IRQ_1();
    fn IO_IRQ_BANK0();
    fn IO_IRQ_BANK0_NS();
    fn IO_IRQ_QSPI();
    fn IO_IRQ_QSPI_NS();
    fn SIO_IRQ_FIFO();
    fn SIO_IRQ_BELL();
    fn SIO_IRQ_FIFO_NS();
    fn SIO_IRQ_BELL_NS();
    fn SIO_IRQ_MTIMECMP();
    fn CLOCKS_IRQ();
    fn SPI0_IRQ();
    fn SPI1_IRQ();
    fn UART0_IRQ();
    fn UART1_IRQ();
    fn ADC_IRQ_FIFO();
    fn I2C0_IRQ();
    fn I2C1_IRQ();
    fn OTP_IRQ();
    fn TRNG_IRQ();
    fn PLL_SYS_IRQ();
    fn PLL_USB_IRQ();
    fn POWMAN_IRQ_POW();
    fn POWMAN_IRQ_TIMER();
    fn SWI_IRQ_0();
    fn SWI_IRQ_1();
    fn SWI_IRQ_2();
    fn SWI_IRQ_3();
    fn SWI_IRQ_4();
    fn SWI_IRQ_5();
}
pub union Vector {
    _handler: unsafe extern "C" fn(),
    _reserved: u32,
}
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static __INTERRUPTS: [Vector; 53] = [
    Vector {
        _handler: TIMER0_IRQ_0,
    },
    Vector {
        _handler: TIMER0_IRQ_1,
    },
    Vector {
        _handler: TIMER0_IRQ_2,
    },
    Vector {
        _handler: TIMER0_IRQ_3,
    },
    Vector {
        _handler: TIMER1_IRQ_0,
    },
    Vector {
        _handler: TIMER1_IRQ_1,
    },
    Vector {
        _handler: TIMER1_IRQ_2,
    },
    Vector {
        _handler: TIMER1_IRQ_3,
    },
    Vector {
        _handler: PWM_IRQ_WRAP_0,
    },
    Vector {
        _handler: PWM_IRQ_WRAP_1,
    },
    Vector {
        _handler: DMA_IRQ_0,
    },
    Vector {
        _handler: DMA_IRQ_1,
    },
    Vector {
        _handler: DMA_IRQ_2,
    },
    Vector {
        _handler: DMA_IRQ_3,
    },
    Vector {
        _handler: USBCTRL_IRQ,
    },
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
        _handler: PIO2_IRQ_0,
    },
    Vector {
        _handler: PIO2_IRQ_1,
    },
    Vector {
        _handler: IO_IRQ_BANK0,
    },
    Vector {
        _handler: IO_IRQ_BANK0_NS,
    },
    Vector {
        _handler: IO_IRQ_QSPI,
    },
    Vector {
        _handler: IO_IRQ_QSPI_NS,
    },
    Vector {
        _handler: SIO_IRQ_FIFO,
    },
    Vector {
        _handler: SIO_IRQ_BELL,
    },
    Vector {
        _handler: SIO_IRQ_FIFO_NS,
    },
    Vector {
        _handler: SIO_IRQ_BELL_NS,
    },
    Vector {
        _handler: SIO_IRQ_MTIMECMP,
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
    Vector { _handler: OTP_IRQ },
    Vector { _handler: TRNG_IRQ },
    Vector { _reserved: 0 },
    Vector { _reserved: 0 },
    Vector {
        _handler: PLL_SYS_IRQ,
    },
    Vector {
        _handler: PLL_USB_IRQ,
    },
    Vector {
        _handler: POWMAN_IRQ_POW,
    },
    Vector {
        _handler: POWMAN_IRQ_TIMER,
    },
    Vector { _reserved: 0 },
    Vector {
        _handler: SWI_IRQ_0,
    },
    Vector {
        _handler: SWI_IRQ_1,
    },
    Vector {
        _handler: SWI_IRQ_2,
    },
    Vector {
        _handler: SWI_IRQ_3,
    },
    Vector {
        _handler: SWI_IRQ_4,
    },
    Vector {
        _handler: SWI_IRQ_5,
    },
];
