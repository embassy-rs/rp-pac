#[doc = "Predefined OTP data layout for RP2350"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpDataRaw {
    ptr: *mut u8,
}
unsafe impl Send for OtpDataRaw {}
unsafe impl Sync for OtpDataRaw {}
impl OtpDataRaw {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique."]
    #[inline(always)]
    pub const fn chipid0(self) -> crate::common::Reg<regs::Chipid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0usize) as _) }
    }
    #[doc = "Bits 31:16 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid1(self) -> crate::common::Reg<regs::Chipid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Bits 47:32 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid2(self) -> crate::common::Reg<regs::Chipid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Bits 63:48 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid3(self) -> crate::common::Reg<regs::Chipid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
    #[inline(always)]
    pub const fn randid0(self) -> crate::common::Reg<regs::Randid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Bits 31:16 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid1(self) -> crate::common::Reg<regs::Randid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Bits 47:32 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid2(self) -> crate::common::Reg<regs::Randid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(24usize) as _) }
    }
    #[doc = "Bits 63:48 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid3(self) -> crate::common::Reg<regs::Randid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(28usize) as _) }
    }
    #[doc = "Bits 79:64 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid4(self) -> crate::common::Reg<regs::Randid4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Bits 95:80 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid5(self) -> crate::common::Reg<regs::Randid5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(36usize) as _) }
    }
    #[doc = "Bits 111:96 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid6(self) -> crate::common::Reg<regs::Randid6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(40usize) as _) }
    }
    #[doc = "Bits 127:112 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid7(self) -> crate::common::Reg<regs::Randid7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(44usize) as _) }
    }
    #[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
    #[inline(always)]
    pub const fn rosc_calib(self) -> crate::common::Reg<regs::RoscCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(64usize) as _) }
    }
    #[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
    #[inline(always)]
    pub const fn lposc_calib(self) -> crate::common::Reg<regs::LposcCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(68usize) as _) }
    }
    #[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
    #[inline(always)]
    pub const fn num_gpios(self) -> crate::common::Reg<regs::NumGpios, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(96usize) as _) }
    }
    #[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
    #[inline(always)]
    pub const fn info_crc0(self) -> crate::common::Reg<regs::InfoCrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(216usize) as _) }
    }
    #[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
    #[inline(always)]
    pub const fn info_crc1(self) -> crate::common::Reg<regs::InfoCrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(220usize) as _) }
    }
    #[doc = "Page 0 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit0(self) -> crate::common::Reg<regs::Crit0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(224usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r1(self) -> crate::common::Reg<regs::Crit0r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(228usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r2(self) -> crate::common::Reg<regs::Crit0r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(232usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r3(self) -> crate::common::Reg<regs::Crit0r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(236usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r4(self) -> crate::common::Reg<regs::Crit0r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(240usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r5(self) -> crate::common::Reg<regs::Crit0r5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(244usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r6(self) -> crate::common::Reg<regs::Crit0r6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(248usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r7(self) -> crate::common::Reg<regs::Crit0r7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(252usize) as _) }
    }
    #[doc = "Page 1 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit1(self) -> crate::common::Reg<regs::Crit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r1(self) -> crate::common::Reg<regs::Crit1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r2(self) -> crate::common::Reg<regs::Crit1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r3(self) -> crate::common::Reg<regs::Crit1r3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r4(self) -> crate::common::Reg<regs::Crit1r4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r5(self) -> crate::common::Reg<regs::Crit1r5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r6(self) -> crate::common::Reg<regs::Crit1r6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r7(self) -> crate::common::Reg<regs::Crit1r7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags0(self) -> crate::common::Reg<regs::BootFlags0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r1(self) -> crate::common::Reg<regs::BootFlags0r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r2(self) -> crate::common::Reg<regs::BootFlags0r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags1(self) -> crate::common::Reg<regs::BootFlags1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r1(self) -> crate::common::Reg<regs::BootFlags1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r2(self) -> crate::common::Reg<regs::BootFlags1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "Default boot version thermometer counter, bits 23:0 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version0(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r1(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion0r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r2(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion0r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "Default boot version thermometer counter, bits 47:24 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version1(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r1(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion1r1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r2(
        self,
    ) -> crate::common::Reg<regs::DefaultBootVersion1r2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
    }
    #[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
    #[inline(always)]
    pub const fn flash_devinfo(self) -> crate::common::Reg<regs::FlashDevinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
    #[inline(always)]
    pub const fn flash_partition_slot_size(
        self,
    ) -> crate::common::Reg<regs::FlashPartitionSlotSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
    #[inline(always)]
    pub const fn bootsel_led_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselLedCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_pll_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselPllCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_xosc_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselXoscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "USB boot specific feature flags (RBIT-3)"]
    #[inline(always)]
    pub const fn usb_boot_flags(self) -> crate::common::Reg<regs::UsbBootFlags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r1(
        self,
    ) -> crate::common::Reg<regs::UsbBootFlagsR1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r2(
        self,
    ) -> crate::common::Reg<regs::UsbBootFlagsR2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
    #[inline(always)]
    pub const fn usb_white_label_addr(
        self,
    ) -> crate::common::Reg<regs::UsbWhiteLabelAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
    #[inline(always)]
    pub const fn otpboot_src(self) -> crate::common::Reg<regs::OtpbootSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
    #[inline(always)]
    pub const fn otpboot_len(self) -> crate::common::Reg<regs::OtpbootLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
    #[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst0(self) -> crate::common::Reg<regs::OtpbootDst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(384usize) as _) }
    }
    #[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst1(self) -> crate::common::Reg<regs::OtpbootDst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(388usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_0(self) -> crate::common::Reg<regs::Bootkey00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(512usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_1(self) -> crate::common::Reg<regs::Bootkey01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(516usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_2(self) -> crate::common::Reg<regs::Bootkey02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(520usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_3(self) -> crate::common::Reg<regs::Bootkey03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(524usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_4(self) -> crate::common::Reg<regs::Bootkey04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(528usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_5(self) -> crate::common::Reg<regs::Bootkey05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(532usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_6(self) -> crate::common::Reg<regs::Bootkey06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(536usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_7(self) -> crate::common::Reg<regs::Bootkey07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(540usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_8(self) -> crate::common::Reg<regs::Bootkey08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(544usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_9(self) -> crate::common::Reg<regs::Bootkey09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(548usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_10(self) -> crate::common::Reg<regs::Bootkey010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(552usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_11(self) -> crate::common::Reg<regs::Bootkey011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(556usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_12(self) -> crate::common::Reg<regs::Bootkey012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(560usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_13(self) -> crate::common::Reg<regs::Bootkey013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(564usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_14(self) -> crate::common::Reg<regs::Bootkey014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(568usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_15(self) -> crate::common::Reg<regs::Bootkey015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(572usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_0(self) -> crate::common::Reg<regs::Bootkey10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(576usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_1(self) -> crate::common::Reg<regs::Bootkey11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(580usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_2(self) -> crate::common::Reg<regs::Bootkey12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(584usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_3(self) -> crate::common::Reg<regs::Bootkey13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(588usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_4(self) -> crate::common::Reg<regs::Bootkey14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(592usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_5(self) -> crate::common::Reg<regs::Bootkey15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(596usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_6(self) -> crate::common::Reg<regs::Bootkey16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(600usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_7(self) -> crate::common::Reg<regs::Bootkey17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(604usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_8(self) -> crate::common::Reg<regs::Bootkey18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(608usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_9(self) -> crate::common::Reg<regs::Bootkey19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(612usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_10(self) -> crate::common::Reg<regs::Bootkey110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(616usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_11(self) -> crate::common::Reg<regs::Bootkey111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(620usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_12(self) -> crate::common::Reg<regs::Bootkey112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(624usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_13(self) -> crate::common::Reg<regs::Bootkey113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(628usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_14(self) -> crate::common::Reg<regs::Bootkey114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(632usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_15(self) -> crate::common::Reg<regs::Bootkey115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(636usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_0(self) -> crate::common::Reg<regs::Bootkey20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(640usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_1(self) -> crate::common::Reg<regs::Bootkey21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(644usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_2(self) -> crate::common::Reg<regs::Bootkey22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(648usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_3(self) -> crate::common::Reg<regs::Bootkey23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(652usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_4(self) -> crate::common::Reg<regs::Bootkey24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(656usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_5(self) -> crate::common::Reg<regs::Bootkey25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(660usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_6(self) -> crate::common::Reg<regs::Bootkey26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(664usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_7(self) -> crate::common::Reg<regs::Bootkey27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(668usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_8(self) -> crate::common::Reg<regs::Bootkey28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(672usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_9(self) -> crate::common::Reg<regs::Bootkey29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(676usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_10(self) -> crate::common::Reg<regs::Bootkey210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(680usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_11(self) -> crate::common::Reg<regs::Bootkey211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(684usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_12(self) -> crate::common::Reg<regs::Bootkey212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(688usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_13(self) -> crate::common::Reg<regs::Bootkey213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(692usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_14(self) -> crate::common::Reg<regs::Bootkey214, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(696usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_15(self) -> crate::common::Reg<regs::Bootkey215, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(700usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_0(self) -> crate::common::Reg<regs::Bootkey30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(704usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_1(self) -> crate::common::Reg<regs::Bootkey31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(708usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_2(self) -> crate::common::Reg<regs::Bootkey32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(712usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_3(self) -> crate::common::Reg<regs::Bootkey33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(716usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_4(self) -> crate::common::Reg<regs::Bootkey34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(720usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_5(self) -> crate::common::Reg<regs::Bootkey35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(724usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_6(self) -> crate::common::Reg<regs::Bootkey36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(728usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_7(self) -> crate::common::Reg<regs::Bootkey37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(732usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_8(self) -> crate::common::Reg<regs::Bootkey38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(736usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_9(self) -> crate::common::Reg<regs::Bootkey39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(740usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_10(self) -> crate::common::Reg<regs::Bootkey310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(744usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_11(self) -> crate::common::Reg<regs::Bootkey311, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(748usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_12(self) -> crate::common::Reg<regs::Bootkey312, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(752usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_13(self) -> crate::common::Reg<regs::Bootkey313, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(756usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_14(self) -> crate::common::Reg<regs::Bootkey314, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(760usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_15(self) -> crate::common::Reg<regs::Bootkey315, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(764usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_0(self) -> crate::common::Reg<regs::Key10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15648usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_1(self) -> crate::common::Reg<regs::Key11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15652usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_2(self) -> crate::common::Reg<regs::Key12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15656usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_3(self) -> crate::common::Reg<regs::Key13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15660usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_4(self) -> crate::common::Reg<regs::Key14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15664usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_5(self) -> crate::common::Reg<regs::Key15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15668usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_6(self) -> crate::common::Reg<regs::Key16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15672usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_7(self) -> crate::common::Reg<regs::Key17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15676usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_0(self) -> crate::common::Reg<regs::Key20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15680usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_1(self) -> crate::common::Reg<regs::Key21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15684usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_2(self) -> crate::common::Reg<regs::Key22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15688usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_3(self) -> crate::common::Reg<regs::Key23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15692usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_4(self) -> crate::common::Reg<regs::Key24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15696usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_5(self) -> crate::common::Reg<regs::Key25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15700usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_6(self) -> crate::common::Reg<regs::Key26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15704usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_7(self) -> crate::common::Reg<regs::Key27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15708usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_0(self) -> crate::common::Reg<regs::Key30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15712usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_1(self) -> crate::common::Reg<regs::Key31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15716usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_2(self) -> crate::common::Reg<regs::Key32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15720usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_3(self) -> crate::common::Reg<regs::Key33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15724usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_4(self) -> crate::common::Reg<regs::Key34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15728usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_5(self) -> crate::common::Reg<regs::Key35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15732usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_6(self) -> crate::common::Reg<regs::Key36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15736usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_7(self) -> crate::common::Reg<regs::Key37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15740usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_0(self) -> crate::common::Reg<regs::Key40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15744usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_1(self) -> crate::common::Reg<regs::Key41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15748usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_2(self) -> crate::common::Reg<regs::Key42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15752usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_3(self) -> crate::common::Reg<regs::Key43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15756usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_4(self) -> crate::common::Reg<regs::Key44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15760usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_5(self) -> crate::common::Reg<regs::Key45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15764usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_6(self) -> crate::common::Reg<regs::Key46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15768usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_7(self) -> crate::common::Reg<regs::Key47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15772usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_0(self) -> crate::common::Reg<regs::Key50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15776usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_1(self) -> crate::common::Reg<regs::Key51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15780usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_2(self) -> crate::common::Reg<regs::Key52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15784usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_3(self) -> crate::common::Reg<regs::Key53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15788usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_4(self) -> crate::common::Reg<regs::Key54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15792usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_5(self) -> crate::common::Reg<regs::Key55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15796usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_6(self) -> crate::common::Reg<regs::Key56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15800usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_7(self) -> crate::common::Reg<regs::Key57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15804usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_0(self) -> crate::common::Reg<regs::Key60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15808usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_1(self) -> crate::common::Reg<regs::Key61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15812usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_2(self) -> crate::common::Reg<regs::Key62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15816usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_3(self) -> crate::common::Reg<regs::Key63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15820usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_4(self) -> crate::common::Reg<regs::Key64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15824usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_5(self) -> crate::common::Reg<regs::Key65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15828usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_6(self) -> crate::common::Reg<regs::Key66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15832usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_7(self) -> crate::common::Reg<regs::Key67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15836usize) as _) }
    }
    #[doc = "Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key1_valid(self) -> crate::common::Reg<regs::Key1valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15844usize) as _) }
    }
    #[doc = "Valid flag for key 2. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key2_valid(self) -> crate::common::Reg<regs::Key2valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15848usize) as _) }
    }
    #[doc = "Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key3_valid(self) -> crate::common::Reg<regs::Key3valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15852usize) as _) }
    }
    #[doc = "Valid flag for key 4. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key4_valid(self) -> crate::common::Reg<regs::Key4valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15856usize) as _) }
    }
    #[doc = "Valid flag for key 5. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key5_valid(self) -> crate::common::Reg<regs::Key5valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15860usize) as _) }
    }
    #[doc = "Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key6_valid(self) -> crate::common::Reg<regs::Key6valid, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15864usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page0_lock0(self) -> crate::common::Reg<regs::Page0lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15872usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page0_lock1(self) -> crate::common::Reg<regs::Page0lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15876usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page1_lock0(self) -> crate::common::Reg<regs::Page1lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15880usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page1_lock1(self) -> crate::common::Reg<regs::Page1lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15884usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page2_lock0(self) -> crate::common::Reg<regs::Page2lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15888usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page2_lock1(self) -> crate::common::Reg<regs::Page2lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15892usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page3_lock0(self) -> crate::common::Reg<regs::Page3lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15896usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page3_lock1(self) -> crate::common::Reg<regs::Page3lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15900usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page4_lock0(self) -> crate::common::Reg<regs::Page4lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15904usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page4_lock1(self) -> crate::common::Reg<regs::Page4lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15908usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page5_lock0(self) -> crate::common::Reg<regs::Page5lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15912usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page5_lock1(self) -> crate::common::Reg<regs::Page5lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15916usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page6_lock0(self) -> crate::common::Reg<regs::Page6lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15920usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page6_lock1(self) -> crate::common::Reg<regs::Page6lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15924usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page7_lock0(self) -> crate::common::Reg<regs::Page7lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15928usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page7_lock1(self) -> crate::common::Reg<regs::Page7lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15932usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page8_lock0(self) -> crate::common::Reg<regs::Page8lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15936usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page8_lock1(self) -> crate::common::Reg<regs::Page8lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15940usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page9_lock0(self) -> crate::common::Reg<regs::Page9lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15944usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page9_lock1(self) -> crate::common::Reg<regs::Page9lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15948usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page10_lock0(self) -> crate::common::Reg<regs::Page10lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15952usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page10_lock1(self) -> crate::common::Reg<regs::Page10lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15956usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page11_lock0(self) -> crate::common::Reg<regs::Page11lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15960usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page11_lock1(self) -> crate::common::Reg<regs::Page11lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15964usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page12_lock0(self) -> crate::common::Reg<regs::Page12lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15968usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page12_lock1(self) -> crate::common::Reg<regs::Page12lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15972usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page13_lock0(self) -> crate::common::Reg<regs::Page13lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15976usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page13_lock1(self) -> crate::common::Reg<regs::Page13lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15980usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page14_lock0(self) -> crate::common::Reg<regs::Page14lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15984usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page14_lock1(self) -> crate::common::Reg<regs::Page14lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15988usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page15_lock0(self) -> crate::common::Reg<regs::Page15lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15992usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page15_lock1(self) -> crate::common::Reg<regs::Page15lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(15996usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page16_lock0(self) -> crate::common::Reg<regs::Page16lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16000usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page16_lock1(self) -> crate::common::Reg<regs::Page16lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16004usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page17_lock0(self) -> crate::common::Reg<regs::Page17lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16008usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page17_lock1(self) -> crate::common::Reg<regs::Page17lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16012usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page18_lock0(self) -> crate::common::Reg<regs::Page18lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16016usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page18_lock1(self) -> crate::common::Reg<regs::Page18lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16020usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page19_lock0(self) -> crate::common::Reg<regs::Page19lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16024usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page19_lock1(self) -> crate::common::Reg<regs::Page19lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16028usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page20_lock0(self) -> crate::common::Reg<regs::Page20lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16032usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page20_lock1(self) -> crate::common::Reg<regs::Page20lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16036usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page21_lock0(self) -> crate::common::Reg<regs::Page21lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16040usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page21_lock1(self) -> crate::common::Reg<regs::Page21lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16044usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page22_lock0(self) -> crate::common::Reg<regs::Page22lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16048usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page22_lock1(self) -> crate::common::Reg<regs::Page22lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16052usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page23_lock0(self) -> crate::common::Reg<regs::Page23lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16056usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page23_lock1(self) -> crate::common::Reg<regs::Page23lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16060usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page24_lock0(self) -> crate::common::Reg<regs::Page24lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16064usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page24_lock1(self) -> crate::common::Reg<regs::Page24lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16068usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page25_lock0(self) -> crate::common::Reg<regs::Page25lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16072usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page25_lock1(self) -> crate::common::Reg<regs::Page25lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16076usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page26_lock0(self) -> crate::common::Reg<regs::Page26lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16080usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page26_lock1(self) -> crate::common::Reg<regs::Page26lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16084usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page27_lock0(self) -> crate::common::Reg<regs::Page27lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16088usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page27_lock1(self) -> crate::common::Reg<regs::Page27lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16092usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page28_lock0(self) -> crate::common::Reg<regs::Page28lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16096usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page28_lock1(self) -> crate::common::Reg<regs::Page28lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16100usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page29_lock0(self) -> crate::common::Reg<regs::Page29lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16104usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page29_lock1(self) -> crate::common::Reg<regs::Page29lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16108usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page30_lock0(self) -> crate::common::Reg<regs::Page30lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16112usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page30_lock1(self) -> crate::common::Reg<regs::Page30lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16116usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page31_lock0(self) -> crate::common::Reg<regs::Page31lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16120usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page31_lock1(self) -> crate::common::Reg<regs::Page31lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16124usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page32_lock0(self) -> crate::common::Reg<regs::Page32lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16128usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page32_lock1(self) -> crate::common::Reg<regs::Page32lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16132usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page33_lock0(self) -> crate::common::Reg<regs::Page33lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16136usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page33_lock1(self) -> crate::common::Reg<regs::Page33lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16140usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page34_lock0(self) -> crate::common::Reg<regs::Page34lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16144usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page34_lock1(self) -> crate::common::Reg<regs::Page34lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16148usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page35_lock0(self) -> crate::common::Reg<regs::Page35lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16152usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page35_lock1(self) -> crate::common::Reg<regs::Page35lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16156usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page36_lock0(self) -> crate::common::Reg<regs::Page36lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16160usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page36_lock1(self) -> crate::common::Reg<regs::Page36lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16164usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page37_lock0(self) -> crate::common::Reg<regs::Page37lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16168usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page37_lock1(self) -> crate::common::Reg<regs::Page37lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16172usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page38_lock0(self) -> crate::common::Reg<regs::Page38lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16176usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page38_lock1(self) -> crate::common::Reg<regs::Page38lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16180usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page39_lock0(self) -> crate::common::Reg<regs::Page39lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16184usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page39_lock1(self) -> crate::common::Reg<regs::Page39lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16188usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page40_lock0(self) -> crate::common::Reg<regs::Page40lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16192usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page40_lock1(self) -> crate::common::Reg<regs::Page40lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16196usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page41_lock0(self) -> crate::common::Reg<regs::Page41lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16200usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page41_lock1(self) -> crate::common::Reg<regs::Page41lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16204usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page42_lock0(self) -> crate::common::Reg<regs::Page42lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16208usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page42_lock1(self) -> crate::common::Reg<regs::Page42lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16212usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page43_lock0(self) -> crate::common::Reg<regs::Page43lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16216usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page43_lock1(self) -> crate::common::Reg<regs::Page43lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16220usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page44_lock0(self) -> crate::common::Reg<regs::Page44lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16224usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page44_lock1(self) -> crate::common::Reg<regs::Page44lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16228usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page45_lock0(self) -> crate::common::Reg<regs::Page45lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16232usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page45_lock1(self) -> crate::common::Reg<regs::Page45lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16236usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page46_lock0(self) -> crate::common::Reg<regs::Page46lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16240usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page46_lock1(self) -> crate::common::Reg<regs::Page46lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16244usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page47_lock0(self) -> crate::common::Reg<regs::Page47lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16248usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page47_lock1(self) -> crate::common::Reg<regs::Page47lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16252usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page48_lock0(self) -> crate::common::Reg<regs::Page48lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16256usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page48_lock1(self) -> crate::common::Reg<regs::Page48lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16260usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page49_lock0(self) -> crate::common::Reg<regs::Page49lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16264usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page49_lock1(self) -> crate::common::Reg<regs::Page49lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16268usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page50_lock0(self) -> crate::common::Reg<regs::Page50lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16272usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page50_lock1(self) -> crate::common::Reg<regs::Page50lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16276usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page51_lock0(self) -> crate::common::Reg<regs::Page51lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16280usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page51_lock1(self) -> crate::common::Reg<regs::Page51lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16284usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page52_lock0(self) -> crate::common::Reg<regs::Page52lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16288usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page52_lock1(self) -> crate::common::Reg<regs::Page52lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16292usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page53_lock0(self) -> crate::common::Reg<regs::Page53lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16296usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page53_lock1(self) -> crate::common::Reg<regs::Page53lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16300usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page54_lock0(self) -> crate::common::Reg<regs::Page54lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16304usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page54_lock1(self) -> crate::common::Reg<regs::Page54lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16308usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page55_lock0(self) -> crate::common::Reg<regs::Page55lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16312usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page55_lock1(self) -> crate::common::Reg<regs::Page55lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16316usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page56_lock0(self) -> crate::common::Reg<regs::Page56lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16320usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page56_lock1(self) -> crate::common::Reg<regs::Page56lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16324usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page57_lock0(self) -> crate::common::Reg<regs::Page57lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16328usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page57_lock1(self) -> crate::common::Reg<regs::Page57lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16332usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page58_lock0(self) -> crate::common::Reg<regs::Page58lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16336usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page58_lock1(self) -> crate::common::Reg<regs::Page58lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16340usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page59_lock0(self) -> crate::common::Reg<regs::Page59lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16344usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page59_lock1(self) -> crate::common::Reg<regs::Page59lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16348usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page60_lock0(self) -> crate::common::Reg<regs::Page60lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16352usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page60_lock1(self) -> crate::common::Reg<regs::Page60lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16356usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page61_lock0(self) -> crate::common::Reg<regs::Page61lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16360usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page61_lock1(self) -> crate::common::Reg<regs::Page61lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16364usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page62_lock0(self) -> crate::common::Reg<regs::Page62lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16368usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page62_lock1(self) -> crate::common::Reg<regs::Page62lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16372usize) as _) }
    }
    #[doc = "Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page63_lock0(self) -> crate::common::Reg<regs::Page63lock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16376usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn page63_lock1(self) -> crate::common::Reg<regs::Page63lock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16380usize) as _) }
    }
}
pub mod regs;
pub mod vals;
