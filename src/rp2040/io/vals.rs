#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Gpio0ctrlFuncsel {
    JTAG_TCK = 0,
    SPI0_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_0 = 0x04,
    SIO_0 = 0x05,
    PIO0_0 = 0x06,
    PIO1_0 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_10 = 0x05,
    PIO0_10 = 0x06,
    PIO1_10 = 0x07,
    USB_MUXING_EXTPHY_VM = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_11 = 0x05,
    PIO0_11 = 0x06,
    PIO1_11 = 0x07,
    USB_MUXING_EXTPHY_SUSPND = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI1_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_6 = 0x04,
    SIO_12 = 0x05,
    PIO0_12 = 0x06,
    PIO1_12 = 0x07,
    USB_MUXING_EXTPHY_SPEED = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI1_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_6 = 0x04,
    SIO_13 = 0x05,
    PIO0_13 = 0x06,
    PIO1_13 = 0x07,
    USB_MUXING_EXTPHY_VPO = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI1_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_7 = 0x04,
    SIO_14 = 0x05,
    PIO0_14 = 0x06,
    PIO1_14 = 0x07,
    USB_MUXING_EXTPHY_VMO = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI1_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_7 = 0x04,
    SIO_15 = 0x05,
    PIO0_15 = 0x06,
    PIO1_15 = 0x07,
    USB_MUXING_DIGITAL_DP = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART0_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_0 = 0x04,
    SIO_16 = 0x05,
    PIO0_16 = 0x06,
    PIO1_16 = 0x07,
    USB_MUXING_DIGITAL_DM = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI0_SS_N = 0x01,
    UART0_RX = 0x02,
    I2C0_SCL = 0x03,
    PWM_B_0 = 0x04,
    SIO_17 = 0x05,
    PIO0_17 = 0x06,
    PIO1_17 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI0_SCLK = 0x01,
    UART0_CTS = 0x02,
    I2C1_SDA = 0x03,
    PWM_A_1 = 0x04,
    SIO_18 = 0x05,
    PIO0_18 = 0x06,
    PIO1_18 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    _RESERVED_0 = 0,
    SPI0_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_1 = 0x04,
    SIO_19 = 0x05,
    PIO0_19 = 0x06,
    PIO1_19 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_1 = 0x05,
    PIO0_1 = 0x06,
    PIO1_1 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_20 = 0x05,
    PIO0_20 = 0x06,
    PIO1_20 = 0x07,
    CLOCKS_GPIN_0 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_21 = 0x05,
    PIO0_21 = 0x06,
    PIO1_21 = 0x07,
    CLOCKS_GPOUT_0 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_22 = 0x05,
    PIO0_22 = 0x06,
    PIO1_22 = 0x07,
    CLOCKS_GPIN_1 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_23 = 0x05,
    PIO0_23 = 0x06,
    PIO1_23 = 0x07,
    CLOCKS_GPOUT_1 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_24 = 0x05,
    PIO0_24 = 0x06,
    PIO1_24 = 0x07,
    CLOCKS_GPOUT_2 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_25 = 0x05,
    PIO0_25 = 0x06,
    PIO1_25 = 0x07,
    CLOCKS_GPOUT_3 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_26 = 0x05,
    PIO0_26 = 0x06,
    PIO1_26 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_27 = 0x05,
    PIO0_27 = 0x06,
    PIO1_27 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_28 = 0x05,
    PIO0_28 = 0x06,
    PIO1_28 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_29 = 0x05,
    PIO0_29 = 0x06,
    PIO1_29 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_2 = 0x05,
    PIO0_2 = 0x06,
    PIO1_2 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
pub enum Gpio3ctrlFuncsel {
    JTAG_TDO = 0,
    SPI0_TX = 0x01,
    UART0_RTS = 0x02,
    I2C1_SCL = 0x03,
    PWM_B_1 = 0x04,
    SIO_3 = 0x05,
    PIO0_3 = 0x06,
    PIO1_3 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
pub enum Gpio4ctrlFuncsel {
    _RESERVED_0 = 0,
    SPI0_RX = 0x01,
    UART1_TX = 0x02,
    I2C0_SDA = 0x03,
    PWM_A_2 = 0x04,
    SIO_4 = 0x05,
    PIO0_4 = 0x06,
    PIO1_4 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_5 = 0x05,
    PIO0_5 = 0x06,
    PIO1_5 = 0x07,
    _RESERVED_8 = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_6 = 0x05,
    PIO0_6 = 0x06,
    PIO1_6 = 0x07,
    USB_MUXING_EXTPHY_SOFTCON = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_7 = 0x05,
    PIO0_7 = 0x06,
    PIO1_7 = 0x07,
    USB_MUXING_EXTPHY_OE_N = 0x08,
    USB_MUXING_VBUS_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_8 = 0x05,
    PIO0_8 = 0x06,
    PIO1_8 = 0x07,
    USB_MUXING_EXTPHY_RCV = 0x08,
    USB_MUXING_VBUS_EN = 0x09,
    _RESERVED_a = 0x0a,
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
    SIO_9 = 0x05,
    PIO0_9 = 0x06,
    PIO1_9 = 0x07,
    USB_MUXING_EXTPHY_VP = 0x08,
    USB_MUXING_OVERCURR_DETECT = 0x09,
    _RESERVED_a = 0x0a,
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
