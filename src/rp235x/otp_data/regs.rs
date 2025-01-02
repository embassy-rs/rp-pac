#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselLedCfg(pub u16);
impl BootselLedCfg {
    #[doc = "GPIO index to use for bootloader activity LED."]
    #[inline(always)]
    pub const fn pin(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "GPIO index to use for bootloader activity LED."]
    #[inline(always)]
    pub fn set_pin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub const fn activelow(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub fn set_activelow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u16) & 0x01) << 8usize);
    }
}
impl Default for BootselLedCfg {
    #[inline(always)]
    fn default() -> BootselLedCfg {
        BootselLedCfg(0)
    }
}
impl core::fmt::Debug for BootselLedCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootselLedCfg")
            .field("pin", &self.pin())
            .field("activelow", &self.activelow())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootselLedCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BootselLedCfg {
            pin: u8,
            activelow: bool,
        }
        let proxy = BootselLedCfg {
            pin: self.pin(),
            activelow: self.activelow(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselPllCfg(pub u16);
impl BootselPllCfg {
    #[doc = "PLL feedback divisor, in the range 16..320 inclusive."]
    #[inline(always)]
    pub const fn fbdiv(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x01ff;
        val as u16
    }
    #[doc = "PLL feedback divisor, in the range 16..320 inclusive."]
    #[inline(always)]
    pub fn set_fbdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u16) & 0x01ff) << 0usize);
    }
    #[doc = "PLL post-divide 1 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub const fn postdiv1(&self) -> u8 {
        let val = (self.0 >> 9usize) & 0x07;
        val as u8
    }
    #[doc = "PLL post-divide 1 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn set_postdiv1(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u16) & 0x07) << 9usize);
    }
    #[doc = "PLL post-divide 2 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub const fn postdiv2(&self) -> u8 {
        let val = (self.0 >> 12usize) & 0x07;
        val as u8
    }
    #[doc = "PLL post-divide 2 divisor, in the range 1..7 inclusive."]
    #[inline(always)]
    pub fn set_postdiv2(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u16) & 0x07) << 12usize);
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub const fn refdiv(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub fn set_refdiv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u16) & 0x01) << 15usize);
    }
}
impl Default for BootselPllCfg {
    #[inline(always)]
    fn default() -> BootselPllCfg {
        BootselPllCfg(0)
    }
}
impl core::fmt::Debug for BootselPllCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootselPllCfg")
            .field("fbdiv", &self.fbdiv())
            .field("postdiv1", &self.postdiv1())
            .field("postdiv2", &self.postdiv2())
            .field("refdiv", &self.refdiv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootselPllCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BootselPllCfg {
            fbdiv: u16,
            postdiv1: u8,
            postdiv2: u8,
            refdiv: bool,
        }
        let proxy = BootselPllCfg {
            fbdiv: self.fbdiv(),
            postdiv1: self.postdiv1(),
            postdiv2: self.postdiv2(),
            refdiv: self.refdiv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselXoscCfg(pub u16);
impl BootselXoscCfg {
    #[doc = "Value of the XOSC_STARTUP register"]
    #[inline(always)]
    pub const fn startup(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0x3fff;
        val as u16
    }
    #[doc = "Value of the XOSC_STARTUP register"]
    #[inline(always)]
    pub fn set_startup(&mut self, val: u16) {
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u16) & 0x3fff) << 0usize);
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 14usize) & 0x03;
        super::vals::Range::from_bits(val as u8)
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x03 << 14usize)) | (((val.to_bits() as u16) & 0x03) << 14usize);
    }
}
impl Default for BootselXoscCfg {
    #[inline(always)]
    fn default() -> BootselXoscCfg {
        BootselXoscCfg(0)
    }
}
impl core::fmt::Debug for BootselXoscCfg {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootselXoscCfg")
            .field("startup", &self.startup())
            .field("range", &self.range())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootselXoscCfg {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BootselXoscCfg {
            startup: u16,
            range: super::vals::Range,
        }
        let proxy = BootselXoscCfg {
            startup: self.startup(),
            range: self.range(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashDevinfo(pub u16);
impl FlashDevinfo {
    #[doc = "Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
    #[inline(always)]
    pub const fn cs1_gpio(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x3f;
        val as u8
    }
    #[doc = "Indicate a GPIO number to be used for the secondary flash chip select (CS1), which selects the external QSPI device mapped at system addresses 0x11000000 through 0x11ffffff. There is no such configuration for CS0, as the primary chip select has a dedicated pin. On RP2350 the permissible GPIO numbers are 0, 8, 19 and 47. Ignored if CS1_size is zero. If CS1_SIZE is nonzero, the bootrom will automatically configure this GPIO as a second chip select upon entering the flash boot path, or entering any other path that may use the QSPI flash interface, such as BOOTSEL mode (nsboot)."]
    #[inline(always)]
    pub fn set_cs1_gpio(&mut self, val: u8) {
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u16) & 0x3f) << 0usize);
    }
    #[doc = "If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
    #[inline(always)]
    pub const fn d8h_erase_supported(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "If true, all attached devices are assumed to support (or ignore, in the case of PSRAM) a block erase command with a command prefix of D8h, an erase size of 64 kiB, and a 24-bit address. Almost all 25-series flash devices support this command. If set, the bootrom will use the D8h erase command where it is able, to accelerate bulk erase operations. This makes flash programming faster. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, this field defaults to false."]
    #[inline(always)]
    pub fn set_d8h_erase_supported(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u16) & 0x01) << 7usize);
    }
    #[doc = "The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
    #[inline(always)]
    pub const fn cs0_size(&self) -> super::vals::Cs0size {
        let val = (self.0 >> 8usize) & 0x0f;
        super::vals::Cs0size::from_bits(val as u8)
    }
    #[doc = "The size of the flash/PSRAM device on chip select 0 (addressable at 0x10000000 through 0x10ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS0_SIZE. For example, four megabytes is encoded with a CS0_SIZE value of 10, and 16 megabytes is encoded with a CS0_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of 12 (16 MiB) is used."]
    #[inline(always)]
    pub fn set_cs0_size(&mut self, val: super::vals::Cs0size) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u16) & 0x0f) << 8usize);
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub const fn cs1_size(&self) -> super::vals::Cs1size {
        let val = (self.0 >> 12usize) & 0x0f;
        super::vals::Cs1size::from_bits(val as u8)
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub fn set_cs1_size(&mut self, val: super::vals::Cs1size) {
        self.0 = (self.0 & !(0x0f << 12usize)) | (((val.to_bits() as u16) & 0x0f) << 12usize);
    }
}
impl Default for FlashDevinfo {
    #[inline(always)]
    fn default() -> FlashDevinfo {
        FlashDevinfo(0)
    }
}
impl core::fmt::Debug for FlashDevinfo {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FlashDevinfo")
            .field("cs1_gpio", &self.cs1_gpio())
            .field("d8h_erase_supported", &self.d8h_erase_supported())
            .field("cs0_size", &self.cs0_size())
            .field("cs1_size", &self.cs1_size())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for FlashDevinfo {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct FlashDevinfo {
            cs1_gpio: u8,
            d8h_erase_supported: bool,
            cs0_size: super::vals::Cs0size,
            cs1_size: super::vals::Cs1size,
        }
        let proxy = FlashDevinfo {
            cs1_gpio: self.cs1_gpio(),
            d8h_erase_supported: self.d8h_erase_supported(),
            cs0_size: self.cs0_size(),
            cs1_size: self.cs1_size(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumGpios(pub u16);
impl NumGpios {
    #[inline(always)]
    pub const fn num_gpios(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_num_gpios(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u16) & 0xff) << 0usize);
    }
}
impl Default for NumGpios {
    #[inline(always)]
    fn default() -> NumGpios {
        NumGpios(0)
    }
}
impl core::fmt::Debug for NumGpios {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("NumGpios")
            .field("num_gpios", &self.num_gpios())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for NumGpios {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct NumGpios {
            num_gpios: u8,
        }
        let proxy = NumGpios {
            num_gpios: self.num_gpios(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbWhiteLabelAddr(pub u16);
impl UsbWhiteLabelAddr {
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> super::vals::UsbWhiteLabelAddr {
        let val = (self.0 >> 0usize) & 0xffff;
        super::vals::UsbWhiteLabelAddr::from_bits(val as u16)
    }
    #[inline(always)]
    pub fn set_usb_white_label_addr(&mut self, val: super::vals::UsbWhiteLabelAddr) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val.to_bits() as u16) & 0xffff) << 0usize);
    }
}
impl Default for UsbWhiteLabelAddr {
    #[inline(always)]
    fn default() -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr(0)
    }
}
impl core::fmt::Debug for UsbWhiteLabelAddr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbWhiteLabelAddr")
            .field("usb_white_label_addr", &self.usb_white_label_addr())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbWhiteLabelAddr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbWhiteLabelAddr {
            usb_white_label_addr: super::vals::UsbWhiteLabelAddr,
        }
        let proxy = UsbWhiteLabelAddr {
            usb_white_label_addr: self.usb_white_label_addr(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
