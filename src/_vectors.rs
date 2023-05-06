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
pub static __INTERRUPTS: [Vector; 32] = [
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
