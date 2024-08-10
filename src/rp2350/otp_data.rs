#[doc = "Predefined OTP data layout for RP2350"]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpData {
    ptr: *mut u8,
}
unsafe impl Send for OtpData {}
unsafe impl Sync for OtpData {}
impl OtpData {
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
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(2usize) as _) }
    }
    #[doc = "Bits 47:32 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid2(self) -> crate::common::Reg<regs::Chipid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(4usize) as _) }
    }
    #[doc = "Bits 63:48 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid3(self) -> crate::common::Reg<regs::Chipid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(6usize) as _) }
    }
    #[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
    #[inline(always)]
    pub const fn randid0(self) -> crate::common::Reg<regs::Randid0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(8usize) as _) }
    }
    #[doc = "Bits 31:16 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid1(self) -> crate::common::Reg<regs::Randid1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(10usize) as _) }
    }
    #[doc = "Bits 47:32 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid2(self) -> crate::common::Reg<regs::Randid2, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(12usize) as _) }
    }
    #[doc = "Bits 63:48 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid3(self) -> crate::common::Reg<regs::Randid3, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(14usize) as _) }
    }
    #[doc = "Bits 79:64 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid4(self) -> crate::common::Reg<regs::Randid4, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(16usize) as _) }
    }
    #[doc = "Bits 95:80 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid5(self) -> crate::common::Reg<regs::Randid5, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(18usize) as _) }
    }
    #[doc = "Bits 111:96 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid6(self) -> crate::common::Reg<regs::Randid6, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(20usize) as _) }
    }
    #[doc = "Bits 127:112 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid7(self) -> crate::common::Reg<regs::Randid7, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(22usize) as _) }
    }
    #[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
    #[inline(always)]
    pub const fn rosc_calib(self) -> crate::common::Reg<regs::RoscCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(32usize) as _) }
    }
    #[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
    #[inline(always)]
    pub const fn lposc_calib(self) -> crate::common::Reg<regs::LposcCalib, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(34usize) as _) }
    }
    #[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
    #[inline(always)]
    pub const fn num_gpios(self) -> crate::common::Reg<regs::NumGpios, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(48usize) as _) }
    }
    #[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
    #[inline(always)]
    pub const fn info_crc0(self) -> crate::common::Reg<regs::InfoCrc0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(108usize) as _) }
    }
    #[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
    #[inline(always)]
    pub const fn info_crc1(self) -> crate::common::Reg<regs::InfoCrc1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(110usize) as _) }
    }
    #[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
    #[inline(always)]
    pub const fn flash_devinfo(self) -> crate::common::Reg<regs::FlashDevinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(168usize) as _) }
    }
    #[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
    #[inline(always)]
    pub const fn flash_partition_slot_size(
        self,
    ) -> crate::common::Reg<regs::FlashPartitionSlotSize, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(170usize) as _) }
    }
    #[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
    #[inline(always)]
    pub const fn bootsel_led_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselLedCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(172usize) as _) }
    }
    #[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_pll_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselPllCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(174usize) as _) }
    }
    #[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_xosc_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselXoscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(176usize) as _) }
    }
    #[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
    #[inline(always)]
    pub const fn usb_white_label_addr(
        self,
    ) -> crate::common::Reg<regs::UsbWhiteLabelAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(184usize) as _) }
    }
    #[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
    #[inline(always)]
    pub const fn otpboot_src(self) -> crate::common::Reg<regs::OtpbootSrc, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(188usize) as _) }
    }
    #[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
    #[inline(always)]
    pub const fn otpboot_len(self) -> crate::common::Reg<regs::OtpbootLen, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(190usize) as _) }
    }
    #[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst0(self) -> crate::common::Reg<regs::OtpbootDst0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(192usize) as _) }
    }
    #[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst1(self) -> crate::common::Reg<regs::OtpbootDst1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(194usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_0(self) -> crate::common::Reg<regs::Bootkey00, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(256usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_1(self) -> crate::common::Reg<regs::Bootkey01, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(258usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_2(self) -> crate::common::Reg<regs::Bootkey02, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(260usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_3(self) -> crate::common::Reg<regs::Bootkey03, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(262usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_4(self) -> crate::common::Reg<regs::Bootkey04, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(264usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_5(self) -> crate::common::Reg<regs::Bootkey05, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(266usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_6(self) -> crate::common::Reg<regs::Bootkey06, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(268usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_7(self) -> crate::common::Reg<regs::Bootkey07, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(270usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_8(self) -> crate::common::Reg<regs::Bootkey08, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(272usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_9(self) -> crate::common::Reg<regs::Bootkey09, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(274usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_10(self) -> crate::common::Reg<regs::Bootkey010, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(276usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_11(self) -> crate::common::Reg<regs::Bootkey011, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(278usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_12(self) -> crate::common::Reg<regs::Bootkey012, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(280usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_13(self) -> crate::common::Reg<regs::Bootkey013, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(282usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_14(self) -> crate::common::Reg<regs::Bootkey014, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(284usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
    #[inline(always)]
    pub const fn bootkey0_15(self) -> crate::common::Reg<regs::Bootkey015, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(286usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_0(self) -> crate::common::Reg<regs::Bootkey10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(288usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_1(self) -> crate::common::Reg<regs::Bootkey11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(290usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_2(self) -> crate::common::Reg<regs::Bootkey12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(292usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_3(self) -> crate::common::Reg<regs::Bootkey13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(294usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_4(self) -> crate::common::Reg<regs::Bootkey14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(296usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_5(self) -> crate::common::Reg<regs::Bootkey15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(298usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_6(self) -> crate::common::Reg<regs::Bootkey16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(300usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_7(self) -> crate::common::Reg<regs::Bootkey17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(302usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_8(self) -> crate::common::Reg<regs::Bootkey18, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(304usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_9(self) -> crate::common::Reg<regs::Bootkey19, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(306usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_10(self) -> crate::common::Reg<regs::Bootkey110, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(308usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_11(self) -> crate::common::Reg<regs::Bootkey111, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(310usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_12(self) -> crate::common::Reg<regs::Bootkey112, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(312usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_13(self) -> crate::common::Reg<regs::Bootkey113, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(314usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_14(self) -> crate::common::Reg<regs::Bootkey114, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(316usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
    #[inline(always)]
    pub const fn bootkey1_15(self) -> crate::common::Reg<regs::Bootkey115, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(318usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_0(self) -> crate::common::Reg<regs::Bootkey20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(320usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_1(self) -> crate::common::Reg<regs::Bootkey21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(322usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_2(self) -> crate::common::Reg<regs::Bootkey22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(324usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_3(self) -> crate::common::Reg<regs::Bootkey23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(326usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_4(self) -> crate::common::Reg<regs::Bootkey24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(328usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_5(self) -> crate::common::Reg<regs::Bootkey25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(330usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_6(self) -> crate::common::Reg<regs::Bootkey26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(332usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_7(self) -> crate::common::Reg<regs::Bootkey27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(334usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_8(self) -> crate::common::Reg<regs::Bootkey28, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(336usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_9(self) -> crate::common::Reg<regs::Bootkey29, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(338usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_10(self) -> crate::common::Reg<regs::Bootkey210, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(340usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_11(self) -> crate::common::Reg<regs::Bootkey211, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(342usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_12(self) -> crate::common::Reg<regs::Bootkey212, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(344usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_13(self) -> crate::common::Reg<regs::Bootkey213, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(346usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_14(self) -> crate::common::Reg<regs::Bootkey214, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(348usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
    #[inline(always)]
    pub const fn bootkey2_15(self) -> crate::common::Reg<regs::Bootkey215, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(350usize) as _) }
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_0(self) -> crate::common::Reg<regs::Bootkey30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(352usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_1(self) -> crate::common::Reg<regs::Bootkey31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(354usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_2(self) -> crate::common::Reg<regs::Bootkey32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(356usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_3(self) -> crate::common::Reg<regs::Bootkey33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(358usize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_4(self) -> crate::common::Reg<regs::Bootkey34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(360usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_5(self) -> crate::common::Reg<regs::Bootkey35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(362usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_6(self) -> crate::common::Reg<regs::Bootkey36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(364usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_7(self) -> crate::common::Reg<regs::Bootkey37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(366usize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_8(self) -> crate::common::Reg<regs::Bootkey38, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(368usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_9(self) -> crate::common::Reg<regs::Bootkey39, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(370usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_10(self) -> crate::common::Reg<regs::Bootkey310, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(372usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_11(self) -> crate::common::Reg<regs::Bootkey311, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(374usize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_12(self) -> crate::common::Reg<regs::Bootkey312, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(376usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_13(self) -> crate::common::Reg<regs::Bootkey313, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(378usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_14(self) -> crate::common::Reg<regs::Bootkey314, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(380usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn bootkey3_15(self) -> crate::common::Reg<regs::Bootkey315, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(382usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_0(self) -> crate::common::Reg<regs::Key10, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7824usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_1(self) -> crate::common::Reg<regs::Key11, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7826usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_2(self) -> crate::common::Reg<regs::Key12, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7828usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_3(self) -> crate::common::Reg<regs::Key13, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7830usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_4(self) -> crate::common::Reg<regs::Key14, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7832usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_5(self) -> crate::common::Reg<regs::Key15, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7834usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_6(self) -> crate::common::Reg<regs::Key16, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7836usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
    #[inline(always)]
    pub const fn key1_7(self) -> crate::common::Reg<regs::Key17, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7838usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_0(self) -> crate::common::Reg<regs::Key20, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7840usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_1(self) -> crate::common::Reg<regs::Key21, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7842usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_2(self) -> crate::common::Reg<regs::Key22, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7844usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_3(self) -> crate::common::Reg<regs::Key23, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7846usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_4(self) -> crate::common::Reg<regs::Key24, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7848usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_5(self) -> crate::common::Reg<regs::Key25, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7850usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_6(self) -> crate::common::Reg<regs::Key26, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7852usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
    #[inline(always)]
    pub const fn key2_7(self) -> crate::common::Reg<regs::Key27, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7854usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_0(self) -> crate::common::Reg<regs::Key30, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7856usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_1(self) -> crate::common::Reg<regs::Key31, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7858usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_2(self) -> crate::common::Reg<regs::Key32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7860usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_3(self) -> crate::common::Reg<regs::Key33, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7862usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_4(self) -> crate::common::Reg<regs::Key34, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7864usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_5(self) -> crate::common::Reg<regs::Key35, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7866usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_6(self) -> crate::common::Reg<regs::Key36, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7868usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
    #[inline(always)]
    pub const fn key3_7(self) -> crate::common::Reg<regs::Key37, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7870usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_0(self) -> crate::common::Reg<regs::Key40, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7872usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_1(self) -> crate::common::Reg<regs::Key41, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7874usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_2(self) -> crate::common::Reg<regs::Key42, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7876usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_3(self) -> crate::common::Reg<regs::Key43, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7878usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_4(self) -> crate::common::Reg<regs::Key44, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7880usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_5(self) -> crate::common::Reg<regs::Key45, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7882usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_6(self) -> crate::common::Reg<regs::Key46, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7884usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
    #[inline(always)]
    pub const fn key4_7(self) -> crate::common::Reg<regs::Key47, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7886usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_0(self) -> crate::common::Reg<regs::Key50, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7888usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_1(self) -> crate::common::Reg<regs::Key51, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7890usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_2(self) -> crate::common::Reg<regs::Key52, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7892usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_3(self) -> crate::common::Reg<regs::Key53, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7894usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_4(self) -> crate::common::Reg<regs::Key54, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7896usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_5(self) -> crate::common::Reg<regs::Key55, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7898usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_6(self) -> crate::common::Reg<regs::Key56, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7900usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
    #[inline(always)]
    pub const fn key5_7(self) -> crate::common::Reg<regs::Key57, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7902usize) as _) }
    }
    #[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_0(self) -> crate::common::Reg<regs::Key60, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7904usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_1(self) -> crate::common::Reg<regs::Key61, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7906usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_2(self) -> crate::common::Reg<regs::Key62, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7908usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_3(self) -> crate::common::Reg<regs::Key63, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7910usize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_4(self) -> crate::common::Reg<regs::Key64, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7912usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_5(self) -> crate::common::Reg<regs::Key65, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7914usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_6(self) -> crate::common::Reg<regs::Key66, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7916usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn key6_7(self) -> crate::common::Reg<regs::Key67, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(7918usize) as _) }
    }
}
pub mod regs;
pub mod vals;
