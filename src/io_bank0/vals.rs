use crate::generic::*;
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio11CtrlFuncsel(u8);
impl Gpio11CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio11CtrlFuncsel {
        Gpio11CtrlFuncsel(val)
    }
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_5: Self = Self(0x04);
    pub const SIO_11: Self = Self(0x05);
    pub const PIO0_11: Self = Self(0x06);
    pub const PIO1_11: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SUSPND: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio5CtrlFuncsel(u8);
impl Gpio5CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio5CtrlFuncsel {
        Gpio5CtrlFuncsel(val)
    }
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_2: Self = Self(0x04);
    pub const SIO_5: Self = Self(0x05);
    pub const PIO0_5: Self = Self(0x06);
    pub const PIO1_5: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio0CtrlFuncsel(u8);
impl Gpio0CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio0CtrlFuncsel {
        Gpio0CtrlFuncsel(val)
    }
    pub const JTAG_TCK: Self = Self(0);
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_0: Self = Self(0x04);
    pub const SIO_0: Self = Self(0x05);
    pub const PIO0_0: Self = Self(0x06);
    pub const PIO1_0: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio22CtrlFuncsel(u8);
impl Gpio22CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio22CtrlFuncsel {
        Gpio22CtrlFuncsel(val)
    }
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_3: Self = Self(0x04);
    pub const SIO_22: Self = Self(0x05);
    pub const PIO0_22: Self = Self(0x06);
    pub const PIO1_22: Self = Self(0x07);
    pub const CLOCKS_GPIN_1: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio10CtrlFuncsel(u8);
impl Gpio10CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio10CtrlFuncsel {
        Gpio10CtrlFuncsel(val)
    }
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_5: Self = Self(0x04);
    pub const SIO_10: Self = Self(0x05);
    pub const PIO0_10: Self = Self(0x06);
    pub const PIO1_10: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VM: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio15CtrlFuncsel(u8);
impl Gpio15CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio15CtrlFuncsel {
        Gpio15CtrlFuncsel(val)
    }
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_7: Self = Self(0x04);
    pub const SIO_15: Self = Self(0x05);
    pub const PIO0_15: Self = Self(0x06);
    pub const PIO1_15: Self = Self(0x07);
    pub const USB_MUXING_DIGITAL_DP: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio18CtrlFuncsel(u8);
impl Gpio18CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio18CtrlFuncsel {
        Gpio18CtrlFuncsel(val)
    }
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_1: Self = Self(0x04);
    pub const SIO_18: Self = Self(0x05);
    pub const PIO0_18: Self = Self(0x06);
    pub const PIO1_18: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio26CtrlFuncsel(u8);
impl Gpio26CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio26CtrlFuncsel {
        Gpio26CtrlFuncsel(val)
    }
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_5: Self = Self(0x04);
    pub const SIO_26: Self = Self(0x05);
    pub const PIO0_26: Self = Self(0x06);
    pub const PIO1_26: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio23CtrlFuncsel(u8);
impl Gpio23CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio23CtrlFuncsel {
        Gpio23CtrlFuncsel(val)
    }
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_3: Self = Self(0x04);
    pub const SIO_23: Self = Self(0x05);
    pub const PIO0_23: Self = Self(0x06);
    pub const PIO1_23: Self = Self(0x07);
    pub const CLOCKS_GPOUT_1: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio9CtrlFuncsel(u8);
impl Gpio9CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio9CtrlFuncsel {
        Gpio9CtrlFuncsel(val)
    }
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_4: Self = Self(0x04);
    pub const SIO_9: Self = Self(0x05);
    pub const PIO0_9: Self = Self(0x06);
    pub const PIO1_9: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VP: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio20CtrlFuncsel(u8);
impl Gpio20CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio20CtrlFuncsel {
        Gpio20CtrlFuncsel(val)
    }
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_2: Self = Self(0x04);
    pub const SIO_20: Self = Self(0x05);
    pub const PIO0_20: Self = Self(0x06);
    pub const PIO1_20: Self = Self(0x07);
    pub const CLOCKS_GPIN_0: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio6CtrlFuncsel(u8);
impl Gpio6CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio6CtrlFuncsel {
        Gpio6CtrlFuncsel(val)
    }
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART1_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_3: Self = Self(0x04);
    pub const SIO_6: Self = Self(0x05);
    pub const PIO0_6: Self = Self(0x06);
    pub const PIO1_6: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SOFTCON: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio7CtrlFuncsel(u8);
impl Gpio7CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio7CtrlFuncsel {
        Gpio7CtrlFuncsel(val)
    }
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_3: Self = Self(0x04);
    pub const SIO_7: Self = Self(0x05);
    pub const PIO0_7: Self = Self(0x06);
    pub const PIO1_7: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_OE_N: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio4CtrlFuncsel(u8);
impl Gpio4CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio4CtrlFuncsel {
        Gpio4CtrlFuncsel(val)
    }
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_2: Self = Self(0x04);
    pub const SIO_4: Self = Self(0x05);
    pub const PIO0_4: Self = Self(0x06);
    pub const PIO1_4: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio21CtrlFuncsel(u8);
impl Gpio21CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio21CtrlFuncsel {
        Gpio21CtrlFuncsel(val)
    }
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_2: Self = Self(0x04);
    pub const SIO_21: Self = Self(0x05);
    pub const PIO0_21: Self = Self(0x06);
    pub const PIO1_21: Self = Self(0x07);
    pub const CLOCKS_GPOUT_0: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio2CtrlFuncsel(u8);
impl Gpio2CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio2CtrlFuncsel {
        Gpio2CtrlFuncsel(val)
    }
    pub const JTAG_TDI: Self = Self(0);
    pub const SPI0_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_1: Self = Self(0x04);
    pub const SIO_2: Self = Self(0x05);
    pub const PIO0_2: Self = Self(0x06);
    pub const PIO1_2: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio28CtrlFuncsel(u8);
impl Gpio28CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio28CtrlFuncsel {
        Gpio28CtrlFuncsel(val)
    }
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_6: Self = Self(0x04);
    pub const SIO_28: Self = Self(0x05);
    pub const PIO0_28: Self = Self(0x06);
    pub const PIO1_28: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio1CtrlFuncsel(u8);
impl Gpio1CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio1CtrlFuncsel {
        Gpio1CtrlFuncsel(val)
    }
    pub const JTAG_TMS: Self = Self(0);
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_0: Self = Self(0x04);
    pub const SIO_1: Self = Self(0x05);
    pub const PIO0_1: Self = Self(0x06);
    pub const PIO1_1: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio17CtrlFuncsel(u8);
impl Gpio17CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio17CtrlFuncsel {
        Gpio17CtrlFuncsel(val)
    }
    pub const SPI0_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_0: Self = Self(0x04);
    pub const SIO_17: Self = Self(0x05);
    pub const PIO0_17: Self = Self(0x06);
    pub const PIO1_17: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio3CtrlFuncsel(u8);
impl Gpio3CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio3CtrlFuncsel {
        Gpio3CtrlFuncsel(val)
    }
    pub const JTAG_TDO: Self = Self(0);
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_1: Self = Self(0x04);
    pub const SIO_3: Self = Self(0x05);
    pub const PIO0_3: Self = Self(0x06);
    pub const PIO1_3: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio12CtrlFuncsel(u8);
impl Gpio12CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio12CtrlFuncsel {
        Gpio12CtrlFuncsel(val)
    }
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_6: Self = Self(0x04);
    pub const SIO_12: Self = Self(0x05);
    pub const PIO0_12: Self = Self(0x06);
    pub const PIO1_12: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_SPEED: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio16CtrlFuncsel(u8);
impl Gpio16CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio16CtrlFuncsel {
        Gpio16CtrlFuncsel(val)
    }
    pub const SPI0_RX: Self = Self(0x01);
    pub const UART0_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_0: Self = Self(0x04);
    pub const SIO_16: Self = Self(0x05);
    pub const PIO0_16: Self = Self(0x06);
    pub const PIO1_16: Self = Self(0x07);
    pub const USB_MUXING_DIGITAL_DM: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio25CtrlFuncsel(u8);
impl Gpio25CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio25CtrlFuncsel {
        Gpio25CtrlFuncsel(val)
    }
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART1_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_4: Self = Self(0x04);
    pub const SIO_25: Self = Self(0x05);
    pub const PIO0_25: Self = Self(0x06);
    pub const PIO1_25: Self = Self(0x07);
    pub const CLOCKS_GPOUT_3: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio27CtrlFuncsel(u8);
impl Gpio27CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio27CtrlFuncsel {
        Gpio27CtrlFuncsel(val)
    }
    pub const SPI1_TX: Self = Self(0x01);
    pub const UART1_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_5: Self = Self(0x04);
    pub const SIO_27: Self = Self(0x05);
    pub const PIO0_27: Self = Self(0x06);
    pub const PIO1_27: Self = Self(0x07);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio8CtrlFuncsel(u8);
impl Gpio8CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio8CtrlFuncsel {
        Gpio8CtrlFuncsel(val)
    }
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_4: Self = Self(0x04);
    pub const SIO_8: Self = Self(0x05);
    pub const PIO0_8: Self = Self(0x06);
    pub const PIO1_8: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_RCV: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio13CtrlFuncsel(u8);
impl Gpio13CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio13CtrlFuncsel {
        Gpio13CtrlFuncsel(val)
    }
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_6: Self = Self(0x04);
    pub const SIO_13: Self = Self(0x05);
    pub const PIO0_13: Self = Self(0x06);
    pub const PIO1_13: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VPO: Self = Self(0x08);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio19CtrlFuncsel(u8);
impl Gpio19CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio19CtrlFuncsel {
        Gpio19CtrlFuncsel(val)
    }
    pub const SPI0_TX: Self = Self(0x01);
    pub const UART0_RTS: Self = Self(0x02);
    pub const I2C1_SCL: Self = Self(0x03);
    pub const PWM_B_1: Self = Self(0x04);
    pub const SIO_19: Self = Self(0x05);
    pub const PIO0_19: Self = Self(0x06);
    pub const PIO1_19: Self = Self(0x07);
    pub const USB_MUXING_VBUS_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio14CtrlFuncsel(u8);
impl Gpio14CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio14CtrlFuncsel {
        Gpio14CtrlFuncsel(val)
    }
    pub const SPI1_SCLK: Self = Self(0x01);
    pub const UART0_CTS: Self = Self(0x02);
    pub const I2C1_SDA: Self = Self(0x03);
    pub const PWM_A_7: Self = Self(0x04);
    pub const SIO_14: Self = Self(0x05);
    pub const PIO0_14: Self = Self(0x06);
    pub const PIO1_14: Self = Self(0x07);
    pub const USB_MUXING_EXTPHY_VMO: Self = Self(0x08);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio29CtrlFuncsel(u8);
impl Gpio29CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio29CtrlFuncsel {
        Gpio29CtrlFuncsel(val)
    }
    pub const SPI1_SS_N: Self = Self(0x01);
    pub const UART0_RX: Self = Self(0x02);
    pub const I2C0_SCL: Self = Self(0x03);
    pub const PWM_B_6: Self = Self(0x04);
    pub const SIO_29: Self = Self(0x05);
    pub const PIO0_29: Self = Self(0x06);
    pub const PIO1_29: Self = Self(0x07);
    pub const USB_MUXING_VBUS_EN: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio24CtrlFuncsel(u8);
impl Gpio24CtrlFuncsel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> Gpio24CtrlFuncsel {
        Gpio24CtrlFuncsel(val)
    }
    pub const SPI1_RX: Self = Self(0x01);
    pub const UART1_TX: Self = Self(0x02);
    pub const I2C0_SDA: Self = Self(0x03);
    pub const PWM_A_4: Self = Self(0x04);
    pub const SIO_24: Self = Self(0x05);
    pub const PIO0_24: Self = Self(0x06);
    pub const PIO1_24: Self = Self(0x07);
    pub const CLOCKS_GPOUT_2: Self = Self(0x08);
    pub const USB_MUXING_OVERCURR_DETECT: Self = Self(0x09);
    pub const NULL: Self = Self(0x1f);
}
