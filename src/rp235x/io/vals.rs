#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio0ctrlFuncsel {
    JTAG_TCK = 0,
    SPI0_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_0 = 0x04,
    SIOB_PROC_0 = 0x05,
    PIO0_0 = 0x06,
    PIO1_0 = 0x07,
    PIO2_0 = 0x08,
    XIP_SS_N_1 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio0ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio0ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio0ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio0ctrlFuncsel {
        Gpio0ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio0ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio0ctrlFuncsel) -> u8 {
        Gpio0ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio10ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_5 = 0x04,
    SIOB_PROC_10 = 0x05,
    PIO0_10 = 0x06,
    PIO1_10 = 0x07,
    PIO2_10 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio10ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio10ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio10ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio10ctrlFuncsel {
        Gpio10ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio10ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio10ctrlFuncsel) -> u8 {
        Gpio10ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio11ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_5 = 0x04,
    SIOB_PROC_11 = 0x05,
    PIO0_11 = 0x06,
    PIO1_11 = 0x07,
    PIO2_11 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio11ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio11ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio11ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio11ctrlFuncsel {
        Gpio11ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio11ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio11ctrlFuncsel) -> u8 {
        Gpio11ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio12ctrlFuncsel {
    HSTX_0 = 0,
    SPI1_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_6 = 0x04,
    SIOB_PROC_12 = 0x05,
    PIO0_12 = 0x06,
    PIO1_12 = 0x07,
    PIO2_12 = 0x08,
    CLOCKS_GPIN_0 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio12ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio12ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio12ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio12ctrlFuncsel {
        Gpio12ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio12ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio12ctrlFuncsel) -> u8 {
        Gpio12ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio13ctrlFuncsel {
    HSTX_1 = 0,
    SPI1_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_6 = 0x04,
    SIOB_PROC_13 = 0x05,
    PIO0_13 = 0x06,
    PIO1_13 = 0x07,
    PIO2_13 = 0x08,
    CLOCKS_GPOUT_0 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio13ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio13ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio13ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio13ctrlFuncsel {
        Gpio13ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio13ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio13ctrlFuncsel) -> u8 {
        Gpio13ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio14ctrlFuncsel {
    HSTX_2 = 0,
    SPI1_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_7 = 0x04,
    SIOB_PROC_14 = 0x05,
    PIO0_14 = 0x06,
    PIO1_14 = 0x07,
    PIO2_14 = 0x08,
    CLOCKS_GPIN_1 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio14ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio14ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio14ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio14ctrlFuncsel {
        Gpio14ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio14ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio14ctrlFuncsel) -> u8 {
        Gpio14ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio15ctrlFuncsel {
    HSTX_3 = 0,
    SPI1_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_7 = 0x04,
    SIOB_PROC_15 = 0x05,
    PIO0_15 = 0x06,
    PIO1_15 = 0x07,
    PIO2_15 = 0x08,
    CLOCKS_GPOUT_1 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio15ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio15ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio15ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio15ctrlFuncsel {
        Gpio15ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio15ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio15ctrlFuncsel) -> u8 {
        Gpio15ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio16ctrlFuncsel {
    HSTX_4 = 0,
    SPI0_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_0 = 0x04,
    SIOB_PROC_16 = 0x05,
    PIO0_16 = 0x06,
    PIO1_16 = 0x07,
    PIO2_16 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio16ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio16ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio16ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio16ctrlFuncsel {
        Gpio16ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio16ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio16ctrlFuncsel) -> u8 {
        Gpio16ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio17ctrlFuncsel {
    HSTX_5 = 0,
    SPI0_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_0 = 0x04,
    SIOB_PROC_17 = 0x05,
    PIO0_17 = 0x06,
    PIO1_17 = 0x07,
    PIO2_17 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio17ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio17ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio17ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio17ctrlFuncsel {
        Gpio17ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio17ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio17ctrlFuncsel) -> u8 {
        Gpio17ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio18ctrlFuncsel {
    HSTX_6 = 0,
    SPI0_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_1 = 0x04,
    SIOB_PROC_18 = 0x05,
    PIO0_18 = 0x06,
    PIO1_18 = 0x07,
    PIO2_18 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio18ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio18ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio18ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio18ctrlFuncsel {
        Gpio18ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio18ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio18ctrlFuncsel) -> u8 {
        Gpio18ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio19ctrlFuncsel {
    HSTX_7 = 0,
    SPI0_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_1 = 0x04,
    SIOB_PROC_19 = 0x05,
    PIO0_19 = 0x06,
    PIO1_19 = 0x07,
    PIO2_19 = 0x08,
    XIP_SS_N_1 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio19ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio19ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio19ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio19ctrlFuncsel {
        Gpio19ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio19ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio19ctrlFuncsel) -> u8 {
        Gpio19ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio1ctrlFuncsel {
    JTAG_TMS = 0,
    SPI0_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_0 = 0x04,
    SIOB_PROC_1 = 0x05,
    PIO0_1 = 0x06,
    PIO1_1 = 0x07,
    PIO2_1 = 0x08,
    CORESIGHT_TRACECLK = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio1ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio1ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio1ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio1ctrlFuncsel {
        Gpio1ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio1ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio1ctrlFuncsel) -> u8 {
        Gpio1ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio20ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_2 = 0x04,
    SIOB_PROC_20 = 0x05,
    PIO0_20 = 0x06,
    PIO1_20 = 0x07,
    PIO2_20 = 0x08,
    CLOCKS_GPIN_0 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio20ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio20ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio20ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio20ctrlFuncsel {
        Gpio20ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio20ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio20ctrlFuncsel) -> u8 {
        Gpio20ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio21ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_2 = 0x04,
    SIOB_PROC_21 = 0x05,
    PIO0_21 = 0x06,
    PIO1_21 = 0x07,
    PIO2_21 = 0x08,
    CLOCKS_GPOUT_0 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio21ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio21ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio21ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio21ctrlFuncsel {
        Gpio21ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio21ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio21ctrlFuncsel) -> u8 {
        Gpio21ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio22ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_3 = 0x04,
    SIOB_PROC_22 = 0x05,
    PIO0_22 = 0x06,
    PIO1_22 = 0x07,
    PIO2_22 = 0x08,
    CLOCKS_GPIN_1 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio22ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio22ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio22ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio22ctrlFuncsel {
        Gpio22ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio22ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio22ctrlFuncsel) -> u8 {
        Gpio22ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio23ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_3 = 0x04,
    SIOB_PROC_23 = 0x05,
    PIO0_23 = 0x06,
    PIO1_23 = 0x07,
    PIO2_23 = 0x08,
    CLOCKS_GPOUT_1 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio23ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio23ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio23ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio23ctrlFuncsel {
        Gpio23ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio23ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio23ctrlFuncsel) -> u8 {
        Gpio23ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio24ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_4 = 0x04,
    SIOB_PROC_24 = 0x05,
    PIO0_24 = 0x06,
    PIO1_24 = 0x07,
    PIO2_24 = 0x08,
    CLOCKS_GPOUT_2 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio24ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio24ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio24ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio24ctrlFuncsel {
        Gpio24ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio24ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio24ctrlFuncsel) -> u8 {
        Gpio24ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio25ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_4 = 0x04,
    SIOB_PROC_25 = 0x05,
    PIO0_25 = 0x06,
    PIO1_25 = 0x07,
    PIO2_25 = 0x08,
    CLOCKS_GPOUT_3 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio25ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio25ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio25ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio25ctrlFuncsel {
        Gpio25ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio25ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio25ctrlFuncsel) -> u8 {
        Gpio25ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio26ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_5 = 0x04,
    SIOB_PROC_26 = 0x05,
    PIO0_26 = 0x06,
    PIO1_26 = 0x07,
    PIO2_26 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio26ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio26ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio26ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio26ctrlFuncsel {
        Gpio26ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio26ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio26ctrlFuncsel) -> u8 {
        Gpio26ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio27ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_5 = 0x04,
    SIOB_PROC_27 = 0x05,
    PIO0_27 = 0x06,
    PIO1_27 = 0x07,
    PIO2_27 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio27ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio27ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio27ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio27ctrlFuncsel {
        Gpio27ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio27ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio27ctrlFuncsel) -> u8 {
        Gpio27ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio28ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_6 = 0x04,
    SIOB_PROC_28 = 0x05,
    PIO0_28 = 0x06,
    PIO1_28 = 0x07,
    PIO2_28 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio28ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio28ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio28ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio28ctrlFuncsel {
        Gpio28ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio28ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio28ctrlFuncsel) -> u8 {
        Gpio28ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio29ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_6 = 0x04,
    SIOB_PROC_29 = 0x05,
    PIO0_29 = 0x06,
    PIO1_29 = 0x07,
    PIO2_29 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio29ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio29ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio29ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio29ctrlFuncsel {
        Gpio29ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio29ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio29ctrlFuncsel) -> u8 {
        Gpio29ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio2ctrlFuncsel {
    JTAG_TDI = 0,
    SPI0_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_1 = 0x04,
    SIOB_PROC_2 = 0x05,
    PIO0_2 = 0x06,
    PIO1_2 = 0x07,
    PIO2_2 = 0x08,
    CORESIGHT_TRACEDATA_0 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio2ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio2ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio2ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio2ctrlFuncsel {
        Gpio2ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio2ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio2ctrlFuncsel) -> u8 {
        Gpio2ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio30ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_7 = 0x04,
    SIOB_PROC_30 = 0x05,
    PIO0_30 = 0x06,
    PIO1_30 = 0x07,
    PIO2_30 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio30ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio30ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio30ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio30ctrlFuncsel {
        Gpio30ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio30ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio30ctrlFuncsel) -> u8 {
        Gpio30ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio31ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_7 = 0x04,
    SIOB_PROC_31 = 0x05,
    PIO0_31 = 0x06,
    PIO1_31 = 0x07,
    PIO2_31 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio31ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio31ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio31ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio31ctrlFuncsel {
        Gpio31ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio31ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio31ctrlFuncsel) -> u8 {
        Gpio31ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio32ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_8 = 0x04,
    SIOB_PROC_32 = 0x05,
    PIO0_32 = 0x06,
    PIO1_32 = 0x07,
    PIO2_32 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio32ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio32ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio32ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio32ctrlFuncsel {
        Gpio32ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio32ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio32ctrlFuncsel) -> u8 {
        Gpio32ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio33ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_8 = 0x04,
    SIOB_PROC_33 = 0x05,
    PIO0_33 = 0x06,
    PIO1_33 = 0x07,
    PIO2_33 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio33ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio33ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio33ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio33ctrlFuncsel {
        Gpio33ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio33ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio33ctrlFuncsel) -> u8 {
        Gpio33ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio34ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_9 = 0x04,
    SIOB_PROC_34 = 0x05,
    PIO0_34 = 0x06,
    PIO1_34 = 0x07,
    PIO2_34 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio34ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio34ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio34ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio34ctrlFuncsel {
        Gpio34ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio34ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio34ctrlFuncsel) -> u8 {
        Gpio34ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio35ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_9 = 0x04,
    SIOB_PROC_35 = 0x05,
    PIO0_35 = 0x06,
    PIO1_35 = 0x07,
    PIO2_35 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio35ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio35ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio35ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio35ctrlFuncsel {
        Gpio35ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio35ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio35ctrlFuncsel) -> u8 {
        Gpio35ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio36ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_10 = 0x04,
    SIOB_PROC_36 = 0x05,
    PIO0_36 = 0x06,
    PIO1_36 = 0x07,
    PIO2_36 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio36ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio36ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio36ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio36ctrlFuncsel {
        Gpio36ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio36ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio36ctrlFuncsel) -> u8 {
        Gpio36ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio37ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_10 = 0x04,
    SIOB_PROC_37 = 0x05,
    PIO0_37 = 0x06,
    PIO1_37 = 0x07,
    PIO2_37 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio37ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio37ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio37ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio37ctrlFuncsel {
        Gpio37ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio37ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio37ctrlFuncsel) -> u8 {
        Gpio37ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio38ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_11 = 0x04,
    SIOB_PROC_38 = 0x05,
    PIO0_38 = 0x06,
    PIO1_38 = 0x07,
    PIO2_38 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio38ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio38ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio38ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio38ctrlFuncsel {
        Gpio38ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio38ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio38ctrlFuncsel) -> u8 {
        Gpio38ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio39ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_11 = 0x04,
    SIOB_PROC_39 = 0x05,
    PIO0_39 = 0x06,
    PIO1_39 = 0x07,
    PIO2_39 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio39ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio39ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio39ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio39ctrlFuncsel {
        Gpio39ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio39ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio39ctrlFuncsel) -> u8 {
        Gpio39ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio3ctrlFuncsel {
    JTAG_TDO = 0,
    SPI0_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_1 = 0x04,
    SIOB_PROC_3 = 0x05,
    PIO0_3 = 0x06,
    PIO1_3 = 0x07,
    PIO2_3 = 0x08,
    CORESIGHT_TRACEDATA_1 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio3ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio3ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio3ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio3ctrlFuncsel {
        Gpio3ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio3ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio3ctrlFuncsel) -> u8 {
        Gpio3ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio40ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_8 = 0x04,
    SIOB_PROC_40 = 0x05,
    PIO0_40 = 0x06,
    PIO1_40 = 0x07,
    PIO2_40 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio40ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio40ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio40ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio40ctrlFuncsel {
        Gpio40ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio40ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio40ctrlFuncsel) -> u8 {
        Gpio40ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio41ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_8 = 0x04,
    SIOB_PROC_41 = 0x05,
    PIO0_41 = 0x06,
    PIO1_41 = 0x07,
    PIO2_41 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio41ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio41ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio41ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio41ctrlFuncsel {
        Gpio41ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio41ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio41ctrlFuncsel) -> u8 {
        Gpio41ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio42ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_9 = 0x04,
    SIOB_PROC_42 = 0x05,
    PIO0_42 = 0x06,
    PIO1_42 = 0x07,
    PIO2_42 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio42ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio42ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio42ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio42ctrlFuncsel {
        Gpio42ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio42ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio42ctrlFuncsel) -> u8 {
        Gpio42ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio43ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_9 = 0x04,
    SIOB_PROC_43 = 0x05,
    PIO0_43 = 0x06,
    PIO1_43 = 0x07,
    PIO2_43 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio43ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio43ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio43ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio43ctrlFuncsel {
        Gpio43ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio43ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio43ctrlFuncsel) -> u8 {
        Gpio43ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio44ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_10 = 0x04,
    SIOB_PROC_44 = 0x05,
    PIO0_44 = 0x06,
    PIO1_44 = 0x07,
    PIO2_44 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio44ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio44ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio44ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio44ctrlFuncsel {
        Gpio44ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio44ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio44ctrlFuncsel) -> u8 {
        Gpio44ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio45ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_10 = 0x04,
    SIOB_PROC_45 = 0x05,
    PIO0_45 = 0x06,
    PIO1_45 = 0x07,
    PIO2_45 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio45ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio45ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio45ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio45ctrlFuncsel {
        Gpio45ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio45ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio45ctrlFuncsel) -> u8 {
        Gpio45ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio46ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_11 = 0x04,
    SIOB_PROC_46 = 0x05,
    PIO0_46 = 0x06,
    PIO1_46 = 0x07,
    PIO2_46 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART0_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio46ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio46ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio46ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio46ctrlFuncsel {
        Gpio46ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio46ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio46ctrlFuncsel) -> u8 {
        Gpio46ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio47ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_11 = 0x04,
    SIOB_PROC_47 = 0x05,
    PIO0_47 = 0x06,
    PIO1_47 = 0x07,
    PIO2_47 = 0x08,
    XIP_SS_N_1 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    UART0_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio47ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio47ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio47ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio47ctrlFuncsel {
        Gpio47ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio47ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio47ctrlFuncsel) -> u8 {
        Gpio47ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio4ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_2 = 0x04,
    SIOB_PROC_4 = 0x05,
    PIO0_4 = 0x06,
    PIO1_4 = 0x07,
    PIO2_4 = 0x08,
    CORESIGHT_TRACEDATA_2 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio4ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio4ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio4ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio4ctrlFuncsel {
        Gpio4ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio4ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio4ctrlFuncsel) -> u8 {
        Gpio4ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio5ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_2 = 0x04,
    SIOB_PROC_5 = 0x05,
    PIO0_5 = 0x06,
    PIO1_5 = 0x07,
    PIO2_5 = 0x08,
    CORESIGHT_TRACEDATA_3 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio5ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio5ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio5ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio5ctrlFuncsel {
        Gpio5ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio5ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio5ctrlFuncsel) -> u8 {
        Gpio5ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio6ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_SCLK = 0x01,
    UART1_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_3 = 0x04,
    SIOB_PROC_6 = 0x05,
    PIO0_6 = 0x06,
    PIO1_6 = 0x07,
    PIO2_6 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    UART1_TX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio6ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio6ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio6ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio6ctrlFuncsel {
        Gpio6ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio6ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio6ctrlFuncsel) -> u8 {
        Gpio6ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio7ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_TX = 0x01,
    UART1_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_3 = 0x04,
    SIOB_PROC_7 = 0x05,
    PIO0_7 = 0x06,
    PIO1_7 = 0x07,
    PIO2_7 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_VBUS_DETECT = 0x0a,
    UART1_RX = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio7ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio7ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio7ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio7ctrlFuncsel {
        Gpio7ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio7ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio7ctrlFuncsel) -> u8 {
        Gpio7ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio8ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_4 = 0x04,
    SIOB_PROC_8 = 0x05,
    PIO0_8 = 0x06,
    PIO1_8 = 0x07,
    PIO2_8 = 0x08,
    XIP_SS_N_1 = 0x09,
    USB_MUXING_VBUS_EN = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio8ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio8ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio8ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio8ctrlFuncsel {
        Gpio8ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio8ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio8ctrlFuncsel) -> u8 {
        Gpio8ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio9ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART1_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_4 = 0x04,
    SIOB_PROC_9 = 0x05,
    PIO0_9 = 0x06,
    PIO1_9 = 0x07,
    PIO2_9 = 0x08,
    _RESERVED_9 = 0x09,
    USB_MUXING_OVERCURR_DETECT = 0x0a,
    _RESERVED_b = 0x0b,
    _RESERVED_c = 0x0c,
    _RESERVED_d = 0x0d,
    _RESERVED_e = 0x0e,
    _RESERVED_f = 0x0f,
    _RESERVED_10 = 0x10,
    _RESERVED_11 = 0x11,
    _RESERVED_12 = 0x12,
    _RESERVED_13 = 0x13,
    _RESERVED_14 = 0x14,
    _RESERVED_15 = 0x15,
    _RESERVED_16 = 0x16,
    _RESERVED_17 = 0x17,
    _RESERVED_18 = 0x18,
    _RESERVED_19 = 0x19,
    _RESERVED_1a = 0x1a,
    _RESERVED_1b = 0x1b,
    _RESERVED_1c = 0x1c,
    _RESERVED_1d = 0x1d,
    _RESERVED_1e = 0x1e,
    NULL = 0x1f,
}
impl Gpio9ctrlFuncsel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Gpio9ctrlFuncsel {
        unsafe { core::mem::transmute(val & 0x1f) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Gpio9ctrlFuncsel {
    #[inline(always)]
    fn from(val: u8) -> Gpio9ctrlFuncsel {
        Gpio9ctrlFuncsel::from_bits(val)
    }
}
impl From<Gpio9ctrlFuncsel> for u8 {
    #[inline(always)]
    fn from(val: Gpio9ctrlFuncsel) -> u8 {
        Gpio9ctrlFuncsel::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Inover {
    #[doc = "don't invert the peri input"]
    NORMAL = 0,
    #[doc = "invert the peri input"]
    INVERT = 0x01,
    #[doc = "drive peri input low"]
    LOW = 0x02,
    #[doc = "drive peri input high"]
    HIGH = 0x03,
}
impl Inover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Inover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Inover {
    #[inline(always)]
    fn from(val: u8) -> Inover {
        Inover::from_bits(val)
    }
}
impl From<Inover> for u8 {
    #[inline(always)]
    fn from(val: Inover) -> u8 {
        Inover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Irqover {
    #[doc = "don't invert the interrupt"]
    NORMAL = 0,
    #[doc = "invert the interrupt"]
    INVERT = 0x01,
    #[doc = "drive interrupt low"]
    LOW = 0x02,
    #[doc = "drive interrupt high"]
    HIGH = 0x03,
}
impl Irqover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Irqover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Irqover {
    #[inline(always)]
    fn from(val: u8) -> Irqover {
        Irqover::from_bits(val)
    }
}
impl From<Irqover> for u8 {
    #[inline(always)]
    fn from(val: Irqover) -> u8 {
        Irqover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Oeover {
    #[doc = "drive output enable from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "drive output enable from inverse of peripheral signal selected by funcsel"]
    INVERT = 0x01,
    #[doc = "disable output"]
    DISABLE = 0x02,
    #[doc = "enable output"]
    ENABLE = 0x03,
}
impl Oeover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Oeover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Oeover {
    #[inline(always)]
    fn from(val: u8) -> Oeover {
        Oeover::from_bits(val)
    }
}
impl From<Oeover> for u8 {
    #[inline(always)]
    fn from(val: Oeover) -> u8 {
        Oeover::to_bits(val)
    }
}
#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Outover {
    #[doc = "drive output from peripheral signal selected by funcsel"]
    NORMAL = 0,
    #[doc = "drive output from inverse of peripheral signal selected by funcsel"]
    INVERT = 0x01,
    #[doc = "drive output low"]
    LOW = 0x02,
    #[doc = "drive output high"]
    HIGH = 0x03,
}
impl Outover {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> Outover {
        unsafe { core::mem::transmute(val & 0x03) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for Outover {
    #[inline(always)]
    fn from(val: u8) -> Outover {
        Outover::from_bits(val)
    }
}
impl From<Outover> for u8 {
    #[inline(always)]
    fn from(val: Outover) -> u8 {
        Outover::to_bits(val)
    }
}
