#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags0(pub u32);
impl BootFlags0 {
    #[inline(always)]
    pub const fn disable_bootsel_exec2(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_bootsel_exec2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable bootloader activity LED. If set, bootsel_led_cfg is assumed to be valid"]
    #[inline(always)]
    pub const fn enable_bootsel_led(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable bootloader activity LED. If set, bootsel_led_cfg is assumed to be valid"]
    #[inline(always)]
    pub fn set_enable_bootsel_led(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable loading of the non-default XOSC and PLL configuration before entering BOOTSEL mode. Ensure that BOOTSEL_XOSC_CFG and BOOTSEL_PLL_CFG are correctly programmed before setting this bit. If this bit is set, user software may use the contents of BOOTSEL_PLL_CFG to calculated the expected XOSC frequency based on the fixed USB boot frequency of 48 MHz."]
    #[inline(always)]
    pub const fn enable_bootsel_non_default_pll_xosc_cfg(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Enable loading of the non-default XOSC and PLL configuration before entering BOOTSEL mode. Ensure that BOOTSEL_XOSC_CFG and BOOTSEL_PLL_CFG are correctly programmed before setting this bit. If this bit is set, user software may use the contents of BOOTSEL_PLL_CFG to calculated the expected XOSC frequency based on the fixed USB boot frequency of 48 MHz."]
    #[inline(always)]
    pub fn set_enable_bootsel_non_default_pll_xosc_cfg(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "If 1, configure the QSPI pads for 1.8 V operation when accessing flash for the first time from the bootrom, using the VOLTAGE_SELECT register for the QSPI pads bank. This slightly improves the input timing of the pads at low voltages, but does not affect their output characteristics. If 0, leave VOLTAGE_SELECT in its reset state (suitable for operation at and above 2.5 V)"]
    #[inline(always)]
    pub const fn flash_io_voltage_1v8(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "If 1, configure the QSPI pads for 1.8 V operation when accessing flash for the first time from the bootrom, using the VOLTAGE_SELECT register for the QSPI pads bank. This slightly improves the input timing of the pads at low voltages, but does not affect their output characteristics. If 0, leave VOLTAGE_SELECT in its reset state (suitable for operation at and above 2.5 V)"]
    #[inline(always)]
    pub fn set_flash_io_voltage_1v8(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Enable quartering of ROSC divisor during signature check, to reduce secure boot time"]
    #[inline(always)]
    pub const fn fast_sigcheck_rosc_div(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable quartering of ROSC divisor during signature check, to reduce secure boot time"]
    #[inline(always)]
    pub fn set_fast_sigcheck_rosc_div(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Mark FLASH_DEVINFO as containing valid, ECC'd data which describes external flash devices."]
    #[inline(always)]
    pub const fn flash_devinfo_enable(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "Mark FLASH_DEVINFO as containing valid, ECC'd data which describes external flash devices."]
    #[inline(always)]
    pub fn set_flash_devinfo_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "Override the limit for default flash metadata scanning. The value is specified in FLASH_PARTITION_SLOT_SIZE. Make sure FLASH_PARTITION_SLOT_SIZE is valid before setting this bit"]
    #[inline(always)]
    pub const fn override_flash_partition_slot_size(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Override the limit for default flash metadata scanning. The value is specified in FLASH_PARTITION_SLOT_SIZE. Make sure FLASH_PARTITION_SLOT_SIZE is valid before setting this bit"]
    #[inline(always)]
    pub fn set_override_flash_partition_slot_size(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Restrict flash boot path to use of a single binary at the start of flash"]
    #[inline(always)]
    pub const fn single_flash_binary(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Restrict flash boot path to use of a single binary at the start of flash"]
    #[inline(always)]
    pub fn set_single_flash_binary(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Disable auto-switch of CPU architecture on boot when the (only) binary to be booted is for the other Arm/RISC-V architecture and both architectures are enabled"]
    #[inline(always)]
    pub const fn disable_auto_switch_arch(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Disable auto-switch of CPU architecture on boot when the (only) binary to be booted is for the other Arm/RISC-V architecture and both architectures are enabled"]
    #[inline(always)]
    pub fn set_disable_auto_switch_arch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Require a partition table to be signed"]
    #[inline(always)]
    pub const fn secure_partition_table(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "Require a partition table to be signed"]
    #[inline(always)]
    pub fn set_secure_partition_table(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "Require a partition table to be hashed (if not signed)"]
    #[inline(always)]
    pub const fn hashed_partition_table(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "Require a partition table to be hashed (if not signed)"]
    #[inline(always)]
    pub fn set_hashed_partition_table(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "Require binaries to have a rollback version. Set automatically the first time a binary with a rollback version is booted."]
    #[inline(always)]
    pub const fn rollback_required(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Require binaries to have a rollback version. Set automatically the first time a binary with a rollback version is booted."]
    #[inline(always)]
    pub fn set_rollback_required(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[inline(always)]
    pub const fn disable_flash_boot(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_flash_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "Takes precedence over ENABLE_OTP_BOOT."]
    #[inline(always)]
    pub const fn disable_otp_boot(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "Takes precedence over ENABLE_OTP_BOOT."]
    #[inline(always)]
    pub fn set_disable_otp_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "Enable OTP boot. A number of OTP rows specified by OTPBOOT_LEN will be loaded, starting from OTPBOOT_SRC, into the SRAM location specified by OTPBOOT_DST1 and OTPBOOT_DST0. The loaded program image is stored with ECC, 16 bits per row, and must contain a valid IMAGE_DEF. Do not set this bit without first programming an image into OTP and configuring OTPBOOT_LEN, OTPBOOT_SRC, OTPBOOT_DST0 and OTPBOOT_DST1. Note that OTPBOOT_LEN and OTPBOOT_SRC must be even numbers of OTP rows. Equivalently, the image must be a multiple of 32 bits in size, and must start at a 32-bit-aligned address in the ECC read data address window."]
    #[inline(always)]
    pub const fn enable_otp_boot(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "Enable OTP boot. A number of OTP rows specified by OTPBOOT_LEN will be loaded, starting from OTPBOOT_SRC, into the SRAM location specified by OTPBOOT_DST1 and OTPBOOT_DST0. The loaded program image is stored with ECC, 16 bits per row, and must contain a valid IMAGE_DEF. Do not set this bit without first programming an image into OTP and configuring OTPBOOT_LEN, OTPBOOT_SRC, OTPBOOT_DST0 and OTPBOOT_DST1. Note that OTPBOOT_LEN and OTPBOOT_SRC must be even numbers of OTP rows. Equivalently, the image must be a multiple of 32 bits in size, and must start at a 32-bit-aligned address in the ECC read data address window."]
    #[inline(always)]
    pub fn set_enable_otp_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[inline(always)]
    pub const fn disable_power_scratch(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_power_scratch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[inline(always)]
    pub const fn disable_watchdog_scratch(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_watchdog_scratch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[inline(always)]
    pub const fn disable_bootsel_usb_msd_ifc(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_bootsel_usb_msd_ifc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn disable_bootsel_usb_picoboot_ifc(&self) -> bool {
        let val = (self.0 >> 18usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_bootsel_usb_picoboot_ifc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18usize)) | (((val as u32) & 0x01) << 18usize);
    }
    #[inline(always)]
    pub const fn disable_bootsel_uart_boot(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_bootsel_uart_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
    #[doc = "Disable all access to XIP after entering an SRAM binary. Note that this will cause bootrom APIs that access XIP to fail, including APIs that interact with the partition table."]
    #[inline(always)]
    pub const fn disable_xip_access_on_sram_entry(&self) -> bool {
        let val = (self.0 >> 20usize) & 0x01;
        val != 0
    }
    #[doc = "Disable all access to XIP after entering an SRAM binary. Note that this will cause bootrom APIs that access XIP to fail, including APIs that interact with the partition table."]
    #[inline(always)]
    pub fn set_disable_xip_access_on_sram_entry(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20usize)) | (((val as u32) & 0x01) << 20usize);
    }
    #[inline(always)]
    pub const fn disable_sram_window_boot(&self) -> bool {
        let val = (self.0 >> 21usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_disable_sram_window_boot(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21usize)) | (((val as u32) & 0x01) << 21usize);
    }
}
impl Default for BootFlags0 {
    #[inline(always)]
    fn default() -> BootFlags0 {
        BootFlags0(0)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags0r1(pub u32);
impl BootFlags0r1 {
    #[inline(always)]
    pub const fn boot_flags0_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_boot_flags0_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags0r1 {
    #[inline(always)]
    fn default() -> BootFlags0r1 {
        BootFlags0r1(0)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags0r2(pub u32);
impl BootFlags0r2 {
    #[inline(always)]
    pub const fn boot_flags0_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_boot_flags0_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags0r2 {
    #[inline(always)]
    fn default() -> BootFlags0r2 {
        BootFlags0r2(0)
    }
}
#[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags1(pub u32);
impl BootFlags1 {
    #[doc = "Mark each of the possible boot keys as valid. The bootrom will check signatures against all valid boot keys, and ignore invalid boot keys. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. A KEY_VALID bit is ignored if the corresponding KEY_INVALID bit is set. Boot keys are considered valid only when KEY_VALID is set and KEY_INVALID is clear. Do not mark a boot key as KEY_VALID if it does not contain a valid SHA-256 hash of your secp256k1 public key. Verify keys after programming, before setting the KEY_VALID bits -- a boot key with uncorrectable ECC faults will render your device unbootable if secure boot is enabled. Do not enable secure boot without first installing a valid key. This will render your device unbootable."]
    #[inline(always)]
    pub const fn key_valid(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[doc = "Mark each of the possible boot keys as valid. The bootrom will check signatures against all valid boot keys, and ignore invalid boot keys. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. A KEY_VALID bit is ignored if the corresponding KEY_INVALID bit is set. Boot keys are considered valid only when KEY_VALID is set and KEY_INVALID is clear. Do not mark a boot key as KEY_VALID if it does not contain a valid SHA-256 hash of your secp256k1 public key. Verify keys after programming, before setting the KEY_VALID bits -- a boot key with uncorrectable ECC faults will render your device unbootable if secure boot is enabled. Do not enable secure boot without first installing a valid key. This will render your device unbootable."]
    #[inline(always)]
    pub fn set_key_valid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[doc = "Mark a boot key as invalid, or prevent it from ever becoming valid. The bootrom will ignore any boot key marked as invalid during secure boot signature checks. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. When provisioning boot keys, it's recommended to mark any boot key slots you don't intend to use as KEY_INVALID, so that spurious keys can not be installed at a later time."]
    #[inline(always)]
    pub const fn key_invalid(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x0f;
        val as u8
    }
    #[doc = "Mark a boot key as invalid, or prevent it from ever becoming valid. The bootrom will ignore any boot key marked as invalid during secure boot signature checks. Each bit in this field corresponds to one of the four 256-bit boot key hashes that may be stored in page 2 of the OTP. When provisioning boot keys, it's recommended to mark any boot key slots you don't intend to use as KEY_INVALID, so that spurious keys can not be installed at a later time."]
    #[inline(always)]
    pub fn set_key_invalid(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val as u32) & 0x0f) << 8usize);
    }
    #[doc = "Adjust how long to wait for a second reset when double tap BOOTSEL mode is enabled via DOUBLE_TAP. The minimum is 50 milliseconds, and each unit of this field adds an additional 50 milliseconds. For example, settings this field to its maximum value of 7 will cause the chip to wait for 400 milliseconds at boot to check for a second reset which requests entry to BOOTSEL mode. 200 milliseconds (DOUBLE_TAP_DELAY=3) is a good intermediate value."]
    #[inline(always)]
    pub const fn double_tap_delay(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x07;
        val as u8
    }
    #[doc = "Adjust how long to wait for a second reset when double tap BOOTSEL mode is enabled via DOUBLE_TAP. The minimum is 50 milliseconds, and each unit of this field adds an additional 50 milliseconds. For example, settings this field to its maximum value of 7 will cause the chip to wait for 400 milliseconds at boot to check for a second reset which requests entry to BOOTSEL mode. 200 milliseconds (DOUBLE_TAP_DELAY=3) is a good intermediate value."]
    #[inline(always)]
    pub fn set_double_tap_delay(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 16usize)) | (((val as u32) & 0x07) << 16usize);
    }
    #[doc = "Enable entering BOOTSEL mode via double-tap of the RUN/RSTn pin. Adds a significant delay to boot time, as configured by DOUBLE_TAP_DELAY. This functions by waiting at startup (i.e. following a reset) to see if a second reset is applied soon afterward. The second reset is detected by the bootrom with help of the POWMAN_CHIP_RESET_DOUBLE_TAP flag, which is not reset by the external reset pin, and the bootrom enters BOOTSEL mode (NSBOOT) to await further instruction over USB or UART."]
    #[inline(always)]
    pub const fn double_tap(&self) -> bool {
        let val = (self.0 >> 19usize) & 0x01;
        val != 0
    }
    #[doc = "Enable entering BOOTSEL mode via double-tap of the RUN/RSTn pin. Adds a significant delay to boot time, as configured by DOUBLE_TAP_DELAY. This functions by waiting at startup (i.e. following a reset) to see if a second reset is applied soon afterward. The second reset is detected by the bootrom with help of the POWMAN_CHIP_RESET_DOUBLE_TAP flag, which is not reset by the external reset pin, and the bootrom enters BOOTSEL mode (NSBOOT) to await further instruction over USB or UART."]
    #[inline(always)]
    pub fn set_double_tap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19usize)) | (((val as u32) & 0x01) << 19usize);
    }
}
impl Default for BootFlags1 {
    #[inline(always)]
    fn default() -> BootFlags1 {
        BootFlags1(0)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags1r1(pub u32);
impl BootFlags1r1 {
    #[inline(always)]
    pub const fn boot_flags1_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_boot_flags1_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags1r1 {
    #[inline(always)]
    fn default() -> BootFlags1r1 {
        BootFlags1r1(0)
    }
}
#[doc = "Redundant copy of BOOT_FLAGS1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootFlags1r2(pub u32);
impl BootFlags1r2 {
    #[inline(always)]
    pub const fn boot_flags1_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_boot_flags1_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for BootFlags1r2 {
    #[inline(always)]
    fn default() -> BootFlags1r2 {
        BootFlags1r2(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey00(pub u32);
impl Bootkey00 {
    #[inline(always)]
    pub const fn bootkey0_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey00 {
    #[inline(always)]
    fn default() -> Bootkey00 {
        Bootkey00(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey01(pub u32);
impl Bootkey01 {
    #[inline(always)]
    pub const fn bootkey0_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey01 {
    #[inline(always)]
    fn default() -> Bootkey01 {
        Bootkey01(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey010(pub u32);
impl Bootkey010 {
    #[inline(always)]
    pub const fn bootkey0_10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey010 {
    #[inline(always)]
    fn default() -> Bootkey010 {
        Bootkey010(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey011(pub u32);
impl Bootkey011 {
    #[inline(always)]
    pub const fn bootkey0_11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey011 {
    #[inline(always)]
    fn default() -> Bootkey011 {
        Bootkey011(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey012(pub u32);
impl Bootkey012 {
    #[inline(always)]
    pub const fn bootkey0_12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey012 {
    #[inline(always)]
    fn default() -> Bootkey012 {
        Bootkey012(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey013(pub u32);
impl Bootkey013 {
    #[inline(always)]
    pub const fn bootkey0_13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey013 {
    #[inline(always)]
    fn default() -> Bootkey013 {
        Bootkey013(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey014(pub u32);
impl Bootkey014 {
    #[inline(always)]
    pub const fn bootkey0_14(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey014 {
    #[inline(always)]
    fn default() -> Bootkey014 {
        Bootkey014(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey015(pub u32);
impl Bootkey015 {
    #[inline(always)]
    pub const fn bootkey0_15(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey015 {
    #[inline(always)]
    fn default() -> Bootkey015 {
        Bootkey015(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey02(pub u32);
impl Bootkey02 {
    #[inline(always)]
    pub const fn bootkey0_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey02 {
    #[inline(always)]
    fn default() -> Bootkey02 {
        Bootkey02(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey03(pub u32);
impl Bootkey03 {
    #[inline(always)]
    pub const fn bootkey0_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey03 {
    #[inline(always)]
    fn default() -> Bootkey03 {
        Bootkey03(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey04(pub u32);
impl Bootkey04 {
    #[inline(always)]
    pub const fn bootkey0_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey04 {
    #[inline(always)]
    fn default() -> Bootkey04 {
        Bootkey04(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey05(pub u32);
impl Bootkey05 {
    #[inline(always)]
    pub const fn bootkey0_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey05 {
    #[inline(always)]
    fn default() -> Bootkey05 {
        Bootkey05(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey06(pub u32);
impl Bootkey06 {
    #[inline(always)]
    pub const fn bootkey0_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey06 {
    #[inline(always)]
    fn default() -> Bootkey06 {
        Bootkey06(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey07(pub u32);
impl Bootkey07 {
    #[inline(always)]
    pub const fn bootkey0_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey07 {
    #[inline(always)]
    fn default() -> Bootkey07 {
        Bootkey07(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey08(pub u32);
impl Bootkey08 {
    #[inline(always)]
    pub const fn bootkey0_8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey08 {
    #[inline(always)]
    fn default() -> Bootkey08 {
        Bootkey08(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 0 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey09(pub u32);
impl Bootkey09 {
    #[inline(always)]
    pub const fn bootkey0_9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey0_9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey09 {
    #[inline(always)]
    fn default() -> Bootkey09 {
        Bootkey09(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey10(pub u32);
impl Bootkey10 {
    #[inline(always)]
    pub const fn bootkey1_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey10 {
    #[inline(always)]
    fn default() -> Bootkey10 {
        Bootkey10(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey11(pub u32);
impl Bootkey11 {
    #[inline(always)]
    pub const fn bootkey1_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey11 {
    #[inline(always)]
    fn default() -> Bootkey11 {
        Bootkey11(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey110(pub u32);
impl Bootkey110 {
    #[inline(always)]
    pub const fn bootkey1_10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey110 {
    #[inline(always)]
    fn default() -> Bootkey110 {
        Bootkey110(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey111(pub u32);
impl Bootkey111 {
    #[inline(always)]
    pub const fn bootkey1_11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey111 {
    #[inline(always)]
    fn default() -> Bootkey111 {
        Bootkey111(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey112(pub u32);
impl Bootkey112 {
    #[inline(always)]
    pub const fn bootkey1_12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey112 {
    #[inline(always)]
    fn default() -> Bootkey112 {
        Bootkey112(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey113(pub u32);
impl Bootkey113 {
    #[inline(always)]
    pub const fn bootkey1_13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey113 {
    #[inline(always)]
    fn default() -> Bootkey113 {
        Bootkey113(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey114(pub u32);
impl Bootkey114 {
    #[inline(always)]
    pub const fn bootkey1_14(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey114 {
    #[inline(always)]
    fn default() -> Bootkey114 {
        Bootkey114(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey115(pub u32);
impl Bootkey115 {
    #[inline(always)]
    pub const fn bootkey1_15(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey115 {
    #[inline(always)]
    fn default() -> Bootkey115 {
        Bootkey115(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey12(pub u32);
impl Bootkey12 {
    #[inline(always)]
    pub const fn bootkey1_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey12 {
    #[inline(always)]
    fn default() -> Bootkey12 {
        Bootkey12(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey13(pub u32);
impl Bootkey13 {
    #[inline(always)]
    pub const fn bootkey1_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey13 {
    #[inline(always)]
    fn default() -> Bootkey13 {
        Bootkey13(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey14(pub u32);
impl Bootkey14 {
    #[inline(always)]
    pub const fn bootkey1_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey14 {
    #[inline(always)]
    fn default() -> Bootkey14 {
        Bootkey14(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey15(pub u32);
impl Bootkey15 {
    #[inline(always)]
    pub const fn bootkey1_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey15 {
    #[inline(always)]
    fn default() -> Bootkey15 {
        Bootkey15(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey16(pub u32);
impl Bootkey16 {
    #[inline(always)]
    pub const fn bootkey1_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey16 {
    #[inline(always)]
    fn default() -> Bootkey16 {
        Bootkey16(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey17(pub u32);
impl Bootkey17 {
    #[inline(always)]
    pub const fn bootkey1_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey17 {
    #[inline(always)]
    fn default() -> Bootkey17 {
        Bootkey17(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey18(pub u32);
impl Bootkey18 {
    #[inline(always)]
    pub const fn bootkey1_8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey18 {
    #[inline(always)]
    fn default() -> Bootkey18 {
        Bootkey18(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey19(pub u32);
impl Bootkey19 {
    #[inline(always)]
    pub const fn bootkey1_9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey1_9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey19 {
    #[inline(always)]
    fn default() -> Bootkey19 {
        Bootkey19(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey20(pub u32);
impl Bootkey20 {
    #[inline(always)]
    pub const fn bootkey2_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey20 {
    #[inline(always)]
    fn default() -> Bootkey20 {
        Bootkey20(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey21(pub u32);
impl Bootkey21 {
    #[inline(always)]
    pub const fn bootkey2_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey21 {
    #[inline(always)]
    fn default() -> Bootkey21 {
        Bootkey21(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey210(pub u32);
impl Bootkey210 {
    #[inline(always)]
    pub const fn bootkey2_10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey210 {
    #[inline(always)]
    fn default() -> Bootkey210 {
        Bootkey210(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey211(pub u32);
impl Bootkey211 {
    #[inline(always)]
    pub const fn bootkey2_11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey211 {
    #[inline(always)]
    fn default() -> Bootkey211 {
        Bootkey211(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey212(pub u32);
impl Bootkey212 {
    #[inline(always)]
    pub const fn bootkey2_12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey212 {
    #[inline(always)]
    fn default() -> Bootkey212 {
        Bootkey212(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey213(pub u32);
impl Bootkey213 {
    #[inline(always)]
    pub const fn bootkey2_13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey213 {
    #[inline(always)]
    fn default() -> Bootkey213 {
        Bootkey213(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey214(pub u32);
impl Bootkey214 {
    #[inline(always)]
    pub const fn bootkey2_14(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey214 {
    #[inline(always)]
    fn default() -> Bootkey214 {
        Bootkey214(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey215(pub u32);
impl Bootkey215 {
    #[inline(always)]
    pub const fn bootkey2_15(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey215 {
    #[inline(always)]
    fn default() -> Bootkey215 {
        Bootkey215(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey22(pub u32);
impl Bootkey22 {
    #[inline(always)]
    pub const fn bootkey2_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey22 {
    #[inline(always)]
    fn default() -> Bootkey22 {
        Bootkey22(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey23(pub u32);
impl Bootkey23 {
    #[inline(always)]
    pub const fn bootkey2_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey23 {
    #[inline(always)]
    fn default() -> Bootkey23 {
        Bootkey23(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey24(pub u32);
impl Bootkey24 {
    #[inline(always)]
    pub const fn bootkey2_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey24 {
    #[inline(always)]
    fn default() -> Bootkey24 {
        Bootkey24(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey25(pub u32);
impl Bootkey25 {
    #[inline(always)]
    pub const fn bootkey2_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey25 {
    #[inline(always)]
    fn default() -> Bootkey25 {
        Bootkey25(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey26(pub u32);
impl Bootkey26 {
    #[inline(always)]
    pub const fn bootkey2_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey26 {
    #[inline(always)]
    fn default() -> Bootkey26 {
        Bootkey26(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey27(pub u32);
impl Bootkey27 {
    #[inline(always)]
    pub const fn bootkey2_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey27 {
    #[inline(always)]
    fn default() -> Bootkey27 {
        Bootkey27(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey28(pub u32);
impl Bootkey28 {
    #[inline(always)]
    pub const fn bootkey2_8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey28 {
    #[inline(always)]
    fn default() -> Bootkey28 {
        Bootkey28(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey29(pub u32);
impl Bootkey29 {
    #[inline(always)]
    pub const fn bootkey2_9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey2_9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey29 {
    #[inline(always)]
    fn default() -> Bootkey29 {
        Bootkey29(0)
    }
}
#[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey30(pub u32);
impl Bootkey30 {
    #[inline(always)]
    pub const fn bootkey3_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey30 {
    #[inline(always)]
    fn default() -> Bootkey30 {
        Bootkey30(0)
    }
}
#[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey31(pub u32);
impl Bootkey31 {
    #[inline(always)]
    pub const fn bootkey3_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey31 {
    #[inline(always)]
    fn default() -> Bootkey31 {
        Bootkey31(0)
    }
}
#[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey310(pub u32);
impl Bootkey310 {
    #[inline(always)]
    pub const fn bootkey3_10(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_10(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey310 {
    #[inline(always)]
    fn default() -> Bootkey310 {
        Bootkey310(0)
    }
}
#[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey311(pub u32);
impl Bootkey311 {
    #[inline(always)]
    pub const fn bootkey3_11(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_11(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey311 {
    #[inline(always)]
    fn default() -> Bootkey311 {
        Bootkey311(0)
    }
}
#[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey312(pub u32);
impl Bootkey312 {
    #[inline(always)]
    pub const fn bootkey3_12(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_12(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey312 {
    #[inline(always)]
    fn default() -> Bootkey312 {
        Bootkey312(0)
    }
}
#[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey313(pub u32);
impl Bootkey313 {
    #[inline(always)]
    pub const fn bootkey3_13(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_13(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey313 {
    #[inline(always)]
    fn default() -> Bootkey313 {
        Bootkey313(0)
    }
}
#[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey314(pub u32);
impl Bootkey314 {
    #[inline(always)]
    pub const fn bootkey3_14(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_14(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey314 {
    #[inline(always)]
    fn default() -> Bootkey314 {
        Bootkey314(0)
    }
}
#[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey315(pub u32);
impl Bootkey315 {
    #[inline(always)]
    pub const fn bootkey3_15(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_15(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey315 {
    #[inline(always)]
    fn default() -> Bootkey315 {
        Bootkey315(0)
    }
}
#[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey32(pub u32);
impl Bootkey32 {
    #[inline(always)]
    pub const fn bootkey3_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey32 {
    #[inline(always)]
    fn default() -> Bootkey32 {
        Bootkey32(0)
    }
}
#[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey33(pub u32);
impl Bootkey33 {
    #[inline(always)]
    pub const fn bootkey3_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey33 {
    #[inline(always)]
    fn default() -> Bootkey33 {
        Bootkey33(0)
    }
}
#[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey34(pub u32);
impl Bootkey34 {
    #[inline(always)]
    pub const fn bootkey3_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey34 {
    #[inline(always)]
    fn default() -> Bootkey34 {
        Bootkey34(0)
    }
}
#[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey35(pub u32);
impl Bootkey35 {
    #[inline(always)]
    pub const fn bootkey3_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey35 {
    #[inline(always)]
    fn default() -> Bootkey35 {
        Bootkey35(0)
    }
}
#[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey36(pub u32);
impl Bootkey36 {
    #[inline(always)]
    pub const fn bootkey3_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey36 {
    #[inline(always)]
    fn default() -> Bootkey36 {
        Bootkey36(0)
    }
}
#[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey37(pub u32);
impl Bootkey37 {
    #[inline(always)]
    pub const fn bootkey3_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey37 {
    #[inline(always)]
    fn default() -> Bootkey37 {
        Bootkey37(0)
    }
}
#[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey38(pub u32);
impl Bootkey38 {
    #[inline(always)]
    pub const fn bootkey3_8(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_8(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey38 {
    #[inline(always)]
    fn default() -> Bootkey38 {
        Bootkey38(0)
    }
}
#[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bootkey39(pub u32);
impl Bootkey39 {
    #[inline(always)]
    pub const fn bootkey3_9(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_bootkey3_9(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Bootkey39 {
    #[inline(always)]
    fn default() -> Bootkey39 {
        Bootkey39(0)
    }
}
#[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselLedCfg(pub u32);
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
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub const fn activelow(&self) -> u16 {
        let val = (self.0 >> 8usize) & 0xffff;
        val as u16
    }
    #[doc = "LED is active-low. (Default: active-high.)"]
    #[inline(always)]
    pub fn set_activelow(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8usize)) | (((val as u32) & 0xffff) << 8usize);
    }
}
impl Default for BootselLedCfg {
    #[inline(always)]
    fn default() -> BootselLedCfg {
        BootselLedCfg(0)
    }
}
#[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselPllCfg(pub u32);
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
        self.0 = (self.0 & !(0x01ff << 0usize)) | (((val as u32) & 0x01ff) << 0usize);
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
        self.0 = (self.0 & !(0x07 << 9usize)) | (((val as u32) & 0x07) << 9usize);
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
        self.0 = (self.0 & !(0x07 << 12usize)) | (((val as u32) & 0x07) << 12usize);
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub const fn refdiv(&self) -> u16 {
        let val = (self.0 >> 15usize) & 0x01ff;
        val as u16
    }
    #[doc = "PLL reference divisor, minus one. Programming a value of 0 means a reference divisor of 1. Programming a value of 1 means a reference divisor of 2 (for exceptionally fast XIN inputs)"]
    #[inline(always)]
    pub fn set_refdiv(&mut self, val: u16) {
        self.0 = (self.0 & !(0x01ff << 15usize)) | (((val as u32) & 0x01ff) << 15usize);
    }
}
impl Default for BootselPllCfg {
    #[inline(always)]
    fn default() -> BootselPllCfg {
        BootselPllCfg(0)
    }
}
#[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootselXoscCfg(pub u32);
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
        self.0 = (self.0 & !(0x3fff << 0usize)) | (((val as u32) & 0x3fff) << 0usize);
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub const fn range(&self) -> super::vals::Range {
        let val = (self.0 >> 14usize) & 0x03ff;
        super::vals::Range::from_bits(val as u16)
    }
    #[doc = "Value of the XOSC_CTRL_FREQ_RANGE register."]
    #[inline(always)]
    pub fn set_range(&mut self, val: super::vals::Range) {
        self.0 = (self.0 & !(0x03ff << 14usize)) | (((val.to_bits() as u32) & 0x03ff) << 14usize);
    }
}
impl Default for BootselXoscCfg {
    #[inline(always)]
    fn default() -> BootselXoscCfg {
        BootselXoscCfg(0)
    }
}
#[doc = "Bits 15:0 of public device ID. (ECC) The CHIPID0..3 rows contain a 64-bit random identifier for this chip, which can be read from the USB bootloader PICOBOOT interface or from the get_sys_info ROM API. The number of random bits makes the occurrence of twins exceedingly unlikely: for example, a fleet of a hundred million devices has a 99.97% probability of no twinned IDs. This is estimated to be lower than the occurrence of process errors in the assignment of sequential random IDs, and for practical purposes CHIPID may be treated as unique."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid0(pub u32);
impl Chipid0 {
    #[inline(always)]
    pub const fn chipid0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_chipid0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid0 {
    #[inline(always)]
    fn default() -> Chipid0 {
        Chipid0(0)
    }
}
#[doc = "Bits 31:16 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid1(pub u32);
impl Chipid1 {
    #[inline(always)]
    pub const fn chipid1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_chipid1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid1 {
    #[inline(always)]
    fn default() -> Chipid1 {
        Chipid1(0)
    }
}
#[doc = "Bits 47:32 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid2(pub u32);
impl Chipid2 {
    #[inline(always)]
    pub const fn chipid2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_chipid2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid2 {
    #[inline(always)]
    fn default() -> Chipid2 {
        Chipid2(0)
    }
}
#[doc = "Bits 63:48 of public device ID (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Chipid3(pub u32);
impl Chipid3 {
    #[inline(always)]
    pub const fn chipid3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_chipid3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Chipid3 {
    #[inline(always)]
    fn default() -> Chipid3 {
        Chipid3(0)
    }
}
#[doc = "Page 0 critical boot flags (RBIT-8)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0(pub u32);
impl Crit0 {
    #[doc = "Permanently disable ARM processors (Cortex-M33)"]
    #[inline(always)]
    pub const fn arm_disable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Permanently disable ARM processors (Cortex-M33)"]
    #[inline(always)]
    pub fn set_arm_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Permanently disable RISC-V processors (Hazard3)"]
    #[inline(always)]
    pub const fn riscv_disable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Permanently disable RISC-V processors (Hazard3)"]
    #[inline(always)]
    pub fn set_riscv_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
}
impl Default for Crit0 {
    #[inline(always)]
    fn default() -> Crit0 {
        Crit0(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r1(pub u32);
impl Crit0r1 {
    #[inline(always)]
    pub const fn crit0_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r1 {
    #[inline(always)]
    fn default() -> Crit0r1 {
        Crit0r1(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r2(pub u32);
impl Crit0r2 {
    #[inline(always)]
    pub const fn crit0_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r2 {
    #[inline(always)]
    fn default() -> Crit0r2 {
        Crit0r2(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r3(pub u32);
impl Crit0r3 {
    #[inline(always)]
    pub const fn crit0_r3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r3 {
    #[inline(always)]
    fn default() -> Crit0r3 {
        Crit0r3(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r4(pub u32);
impl Crit0r4 {
    #[inline(always)]
    pub const fn crit0_r4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r4 {
    #[inline(always)]
    fn default() -> Crit0r4 {
        Crit0r4(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r5(pub u32);
impl Crit0r5 {
    #[inline(always)]
    pub const fn crit0_r5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r5 {
    #[inline(always)]
    fn default() -> Crit0r5 {
        Crit0r5(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r6(pub u32);
impl Crit0r6 {
    #[inline(always)]
    pub const fn crit0_r6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r6 {
    #[inline(always)]
    fn default() -> Crit0r6 {
        Crit0r6(0)
    }
}
#[doc = "Redundant copy of CRIT0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit0r7(pub u32);
impl Crit0r7 {
    #[inline(always)]
    pub const fn crit0_r7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit0_r7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit0r7 {
    #[inline(always)]
    fn default() -> Crit0r7 {
        Crit0r7(0)
    }
}
#[doc = "Page 1 critical boot flags (RBIT-8)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1(pub u32);
impl Crit1 {
    #[doc = "Enable boot signature enforcement, and permanently disable the RISC-V cores."]
    #[inline(always)]
    pub const fn secure_boot_enable(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable boot signature enforcement, and permanently disable the RISC-V cores."]
    #[inline(always)]
    pub fn set_secure_boot_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Disable Secure debug access"]
    #[inline(always)]
    pub const fn secure_debug_disable(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Disable Secure debug access"]
    #[inline(always)]
    pub fn set_secure_debug_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Disable all debug access"]
    #[inline(always)]
    pub const fn debug_disable(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Disable all debug access"]
    #[inline(always)]
    pub fn set_debug_disable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Set the default boot architecture, 0=ARM 1=RISC-V. Ignored if ARM_DISABLE, RISCV_DISABLE or SECURE_BOOT_ENABLE is set."]
    #[inline(always)]
    pub const fn boot_arch(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Set the default boot architecture, 0=ARM 1=RISC-V. Ignored if ARM_DISABLE, RISCV_DISABLE or SECURE_BOOT_ENABLE is set."]
    #[inline(always)]
    pub fn set_boot_arch(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "Arm the glitch detectors to reset the system if an abnormal clock/power event is observed."]
    #[inline(always)]
    pub const fn glitch_detector_enable(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Arm the glitch detectors to reset the system if an abnormal clock/power event is observed."]
    #[inline(always)]
    pub fn set_glitch_detector_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Increase the sensitivity of the glitch detectors from their default."]
    #[inline(always)]
    pub const fn glitch_detector_sens(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Increase the sensitivity of the glitch detectors from their default."]
    #[inline(always)]
    pub fn set_glitch_detector_sens(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
}
impl Default for Crit1 {
    #[inline(always)]
    fn default() -> Crit1 {
        Crit1(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r1(pub u32);
impl Crit1r1 {
    #[inline(always)]
    pub const fn crit1_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r1 {
    #[inline(always)]
    fn default() -> Crit1r1 {
        Crit1r1(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r2(pub u32);
impl Crit1r2 {
    #[inline(always)]
    pub const fn crit1_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r2 {
    #[inline(always)]
    fn default() -> Crit1r2 {
        Crit1r2(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r3(pub u32);
impl Crit1r3 {
    #[inline(always)]
    pub const fn crit1_r3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r3 {
    #[inline(always)]
    fn default() -> Crit1r3 {
        Crit1r3(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r4(pub u32);
impl Crit1r4 {
    #[inline(always)]
    pub const fn crit1_r4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r4 {
    #[inline(always)]
    fn default() -> Crit1r4 {
        Crit1r4(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r5(pub u32);
impl Crit1r5 {
    #[inline(always)]
    pub const fn crit1_r5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r5 {
    #[inline(always)]
    fn default() -> Crit1r5 {
        Crit1r5(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r6(pub u32);
impl Crit1r6 {
    #[inline(always)]
    pub const fn crit1_r6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r6 {
    #[inline(always)]
    fn default() -> Crit1r6 {
        Crit1r6(0)
    }
}
#[doc = "Redundant copy of CRIT1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Crit1r7(pub u32);
impl Crit1r7 {
    #[inline(always)]
    pub const fn crit1_r7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_crit1_r7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Crit1r7 {
    #[inline(always)]
    fn default() -> Crit1r7 {
        Crit1r7(0)
    }
}
#[doc = "Default boot version thermometer counter, bits 23:0 (RBIT-3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0(pub u32);
impl DefaultBootVersion0 {
    #[inline(always)]
    pub const fn default_boot_version0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0 {
        DefaultBootVersion0(0)
    }
}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0r1(pub u32);
impl DefaultBootVersion0r1 {
    #[inline(always)]
    pub const fn default_boot_version0_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version0_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0r1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0r1 {
        DefaultBootVersion0r1(0)
    }
}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion0r2(pub u32);
impl DefaultBootVersion0r2 {
    #[inline(always)]
    pub const fn default_boot_version0_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version0_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion0r2 {
    #[inline(always)]
    fn default() -> DefaultBootVersion0r2 {
        DefaultBootVersion0r2(0)
    }
}
#[doc = "Default boot version thermometer counter, bits 47:24 (RBIT-3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1(pub u32);
impl DefaultBootVersion1 {
    #[inline(always)]
    pub const fn default_boot_version1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1 {
        DefaultBootVersion1(0)
    }
}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1r1(pub u32);
impl DefaultBootVersion1r1 {
    #[inline(always)]
    pub const fn default_boot_version1_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version1_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1r1 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1r1 {
        DefaultBootVersion1r1(0)
    }
}
#[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct DefaultBootVersion1r2(pub u32);
impl DefaultBootVersion1r2 {
    #[inline(always)]
    pub const fn default_boot_version1_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_default_boot_version1_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for DefaultBootVersion1r2 {
    #[inline(always)]
    fn default() -> DefaultBootVersion1r2 {
        DefaultBootVersion1r2(0)
    }
}
#[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashDevinfo(pub u32);
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
        self.0 = (self.0 & !(0x3f << 0usize)) | (((val as u32) & 0x3f) << 0usize);
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
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
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
        self.0 = (self.0 & !(0x0f << 8usize)) | (((val.to_bits() as u32) & 0x0f) << 8usize);
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub const fn cs1_size(&self) -> super::vals::Cs1size {
        let val = (self.0 >> 12usize) & 0x0fff;
        super::vals::Cs1size::from_bits(val as u16)
    }
    #[doc = "The size of the flash/PSRAM device on chip select 1 (addressable at 0x11000000 through 0x11ffffff). A value of zero is decoded as a size of zero (no device). Nonzero values are decoded as 4kiB << CS1_SIZE. For example, four megabytes is encoded with a CS1_SIZE value of 10, and 16 megabytes is encoded with a CS1_SIZE value of 12. When BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is not set, a default of zero is used."]
    #[inline(always)]
    pub fn set_cs1_size(&mut self, val: super::vals::Cs1size) {
        self.0 = (self.0 & !(0x0fff << 12usize)) | (((val.to_bits() as u32) & 0x0fff) << 12usize);
    }
}
impl Default for FlashDevinfo {
    #[inline(always)]
    fn default() -> FlashDevinfo {
        FlashDevinfo(0)
    }
}
#[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct FlashPartitionSlotSize(pub u32);
impl FlashPartitionSlotSize {
    #[inline(always)]
    pub const fn flash_partition_slot_size(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_flash_partition_slot_size(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for FlashPartitionSlotSize {
    #[inline(always)]
    fn default() -> FlashPartitionSlotSize {
        FlashPartitionSlotSize(0)
    }
}
#[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc0(pub u32);
impl InfoCrc0 {
    #[inline(always)]
    pub const fn info_crc0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_info_crc0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for InfoCrc0 {
    #[inline(always)]
    fn default() -> InfoCrc0 {
        InfoCrc0(0)
    }
}
#[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct InfoCrc1(pub u32);
impl InfoCrc1 {
    #[inline(always)]
    pub const fn info_crc1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_info_crc1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for InfoCrc1 {
    #[inline(always)]
    fn default() -> InfoCrc1 {
        InfoCrc1(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key10(pub u32);
impl Key10 {
    #[inline(always)]
    pub const fn key1_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key10 {
    #[inline(always)]
    fn default() -> Key10 {
        Key10(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key11(pub u32);
impl Key11 {
    #[inline(always)]
    pub const fn key1_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key11 {
    #[inline(always)]
    fn default() -> Key11 {
        Key11(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key12(pub u32);
impl Key12 {
    #[inline(always)]
    pub const fn key1_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key12 {
    #[inline(always)]
    fn default() -> Key12 {
        Key12(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key13(pub u32);
impl Key13 {
    #[inline(always)]
    pub const fn key1_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key13 {
    #[inline(always)]
    fn default() -> Key13 {
        Key13(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key14(pub u32);
impl Key14 {
    #[inline(always)]
    pub const fn key1_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key14 {
    #[inline(always)]
    fn default() -> Key14 {
        Key14(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key15(pub u32);
impl Key15 {
    #[inline(always)]
    pub const fn key1_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key15 {
    #[inline(always)]
    fn default() -> Key15 {
        Key15(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key16(pub u32);
impl Key16 {
    #[inline(always)]
    pub const fn key1_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key16 {
    #[inline(always)]
    fn default() -> Key16 {
        Key16(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 1 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key17(pub u32);
impl Key17 {
    #[inline(always)]
    pub const fn key1_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key1_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key17 {
    #[inline(always)]
    fn default() -> Key17 {
        Key17(0)
    }
}
#[doc = "Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key1valid(pub u32);
impl Key1valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key1valid {
    #[inline(always)]
    fn default() -> Key1valid {
        Key1valid(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key20(pub u32);
impl Key20 {
    #[inline(always)]
    pub const fn key2_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key20 {
    #[inline(always)]
    fn default() -> Key20 {
        Key20(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key21(pub u32);
impl Key21 {
    #[inline(always)]
    pub const fn key2_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key21 {
    #[inline(always)]
    fn default() -> Key21 {
        Key21(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key22(pub u32);
impl Key22 {
    #[inline(always)]
    pub const fn key2_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key22 {
    #[inline(always)]
    fn default() -> Key22 {
        Key22(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key23(pub u32);
impl Key23 {
    #[inline(always)]
    pub const fn key2_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key23 {
    #[inline(always)]
    fn default() -> Key23 {
        Key23(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key24(pub u32);
impl Key24 {
    #[inline(always)]
    pub const fn key2_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key24 {
    #[inline(always)]
    fn default() -> Key24 {
        Key24(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key25(pub u32);
impl Key25 {
    #[inline(always)]
    pub const fn key2_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key25 {
    #[inline(always)]
    fn default() -> Key25 {
        Key25(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key26(pub u32);
impl Key26 {
    #[inline(always)]
    pub const fn key2_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key26 {
    #[inline(always)]
    fn default() -> Key26 {
        Key26(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 2 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key27(pub u32);
impl Key27 {
    #[inline(always)]
    pub const fn key2_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key2_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key27 {
    #[inline(always)]
    fn default() -> Key27 {
        Key27(0)
    }
}
#[doc = "Valid flag for key 2. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key2valid(pub u32);
impl Key2valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key2valid {
    #[inline(always)]
    fn default() -> Key2valid {
        Key2valid(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key30(pub u32);
impl Key30 {
    #[inline(always)]
    pub const fn key3_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key30 {
    #[inline(always)]
    fn default() -> Key30 {
        Key30(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key31(pub u32);
impl Key31 {
    #[inline(always)]
    pub const fn key3_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key31 {
    #[inline(always)]
    fn default() -> Key31 {
        Key31(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key32(pub u32);
impl Key32 {
    #[inline(always)]
    pub const fn key3_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key32 {
    #[inline(always)]
    fn default() -> Key32 {
        Key32(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key33(pub u32);
impl Key33 {
    #[inline(always)]
    pub const fn key3_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key33 {
    #[inline(always)]
    fn default() -> Key33 {
        Key33(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key34(pub u32);
impl Key34 {
    #[inline(always)]
    pub const fn key3_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key34 {
    #[inline(always)]
    fn default() -> Key34 {
        Key34(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key35(pub u32);
impl Key35 {
    #[inline(always)]
    pub const fn key3_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key35 {
    #[inline(always)]
    fn default() -> Key35 {
        Key35(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key36(pub u32);
impl Key36 {
    #[inline(always)]
    pub const fn key3_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key36 {
    #[inline(always)]
    fn default() -> Key36 {
        Key36(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 3 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key37(pub u32);
impl Key37 {
    #[inline(always)]
    pub const fn key3_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key3_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key37 {
    #[inline(always)]
    fn default() -> Key37 {
        Key37(0)
    }
}
#[doc = "Valid flag for key 3. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key3valid(pub u32);
impl Key3valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key3valid {
    #[inline(always)]
    fn default() -> Key3valid {
        Key3valid(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key40(pub u32);
impl Key40 {
    #[inline(always)]
    pub const fn key4_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key40 {
    #[inline(always)]
    fn default() -> Key40 {
        Key40(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key41(pub u32);
impl Key41 {
    #[inline(always)]
    pub const fn key4_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key41 {
    #[inline(always)]
    fn default() -> Key41 {
        Key41(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key42(pub u32);
impl Key42 {
    #[inline(always)]
    pub const fn key4_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key42 {
    #[inline(always)]
    fn default() -> Key42 {
        Key42(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key43(pub u32);
impl Key43 {
    #[inline(always)]
    pub const fn key4_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key43 {
    #[inline(always)]
    fn default() -> Key43 {
        Key43(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key44(pub u32);
impl Key44 {
    #[inline(always)]
    pub const fn key4_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key44 {
    #[inline(always)]
    fn default() -> Key44 {
        Key44(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key45(pub u32);
impl Key45 {
    #[inline(always)]
    pub const fn key4_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key45 {
    #[inline(always)]
    fn default() -> Key45 {
        Key45(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key46(pub u32);
impl Key46 {
    #[inline(always)]
    pub const fn key4_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key46 {
    #[inline(always)]
    fn default() -> Key46 {
        Key46(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 4 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key47(pub u32);
impl Key47 {
    #[inline(always)]
    pub const fn key4_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key4_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key47 {
    #[inline(always)]
    fn default() -> Key47 {
        Key47(0)
    }
}
#[doc = "Valid flag for key 4. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key4valid(pub u32);
impl Key4valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key4valid {
    #[inline(always)]
    fn default() -> Key4valid {
        Key4valid(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key50(pub u32);
impl Key50 {
    #[inline(always)]
    pub const fn key5_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key50 {
    #[inline(always)]
    fn default() -> Key50 {
        Key50(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key51(pub u32);
impl Key51 {
    #[inline(always)]
    pub const fn key5_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key51 {
    #[inline(always)]
    fn default() -> Key51 {
        Key51(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key52(pub u32);
impl Key52 {
    #[inline(always)]
    pub const fn key5_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key52 {
    #[inline(always)]
    fn default() -> Key52 {
        Key52(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key53(pub u32);
impl Key53 {
    #[inline(always)]
    pub const fn key5_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key53 {
    #[inline(always)]
    fn default() -> Key53 {
        Key53(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key54(pub u32);
impl Key54 {
    #[inline(always)]
    pub const fn key5_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key54 {
    #[inline(always)]
    fn default() -> Key54 {
        Key54(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key55(pub u32);
impl Key55 {
    #[inline(always)]
    pub const fn key5_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key55 {
    #[inline(always)]
    fn default() -> Key55 {
        Key55(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key56(pub u32);
impl Key56 {
    #[inline(always)]
    pub const fn key5_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key56 {
    #[inline(always)]
    fn default() -> Key56 {
        Key56(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 5 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key57(pub u32);
impl Key57 {
    #[inline(always)]
    pub const fn key5_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key5_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key57 {
    #[inline(always)]
    fn default() -> Key57 {
        Key57(0)
    }
}
#[doc = "Valid flag for key 5. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key5valid(pub u32);
impl Key5valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key5valid {
    #[inline(always)]
    fn default() -> Key5valid {
        Key5valid(0)
    }
}
#[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key60(pub u32);
impl Key60 {
    #[inline(always)]
    pub const fn key6_0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key60 {
    #[inline(always)]
    fn default() -> Key60 {
        Key60(0)
    }
}
#[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key61(pub u32);
impl Key61 {
    #[inline(always)]
    pub const fn key6_1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key61 {
    #[inline(always)]
    fn default() -> Key61 {
        Key61(0)
    }
}
#[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key62(pub u32);
impl Key62 {
    #[inline(always)]
    pub const fn key6_2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key62 {
    #[inline(always)]
    fn default() -> Key62 {
        Key62(0)
    }
}
#[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key63(pub u32);
impl Key63 {
    #[inline(always)]
    pub const fn key6_3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key63 {
    #[inline(always)]
    fn default() -> Key63 {
        Key63(0)
    }
}
#[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key64(pub u32);
impl Key64 {
    #[inline(always)]
    pub const fn key6_4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key64 {
    #[inline(always)]
    fn default() -> Key64 {
        Key64(0)
    }
}
#[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key65(pub u32);
impl Key65 {
    #[inline(always)]
    pub const fn key6_5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key65 {
    #[inline(always)]
    fn default() -> Key65 {
        Key65(0)
    }
}
#[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key66(pub u32);
impl Key66 {
    #[inline(always)]
    pub const fn key6_6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key66 {
    #[inline(always)]
    fn default() -> Key66 {
        Key66(0)
    }
}
#[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key67(pub u32);
impl Key67 {
    #[inline(always)]
    pub const fn key6_7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_key6_7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Key67 {
    #[inline(always)]
    fn default() -> Key67 {
        Key67(0)
    }
}
#[doc = "Valid flag for key 6. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key6valid(pub u32);
impl Key6valid {
    #[inline(always)]
    pub const fn valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r1(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub const fn valid_r2(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Redundant copy of VALID, with 3-way majority vote"]
    #[inline(always)]
    pub fn set_valid_r2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
}
impl Default for Key6valid {
    #[inline(always)]
    fn default() -> Key6valid {
        Key6valid(0)
    }
}
#[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct LposcCalib(pub u32);
impl LposcCalib {
    #[inline(always)]
    pub const fn lposc_calib(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_lposc_calib(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for LposcCalib {
    #[inline(always)]
    fn default() -> LposcCalib {
        LposcCalib(0)
    }
}
#[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct NumGpios(pub u32);
impl NumGpios {
    #[inline(always)]
    pub const fn num_gpios(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_num_gpios(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for NumGpios {
    #[inline(always)]
    fn default() -> NumGpios {
        NumGpios(0)
    }
}
#[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst0(pub u32);
impl OtpbootDst0 {
    #[inline(always)]
    pub const fn otpboot_dst0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_otpboot_dst0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootDst0 {
    #[inline(always)]
    fn default() -> OtpbootDst0 {
        OtpbootDst0(0)
    }
}
#[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootDst1(pub u32);
impl OtpbootDst1 {
    #[inline(always)]
    pub const fn otpboot_dst1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_otpboot_dst1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootDst1 {
    #[inline(always)]
    fn default() -> OtpbootDst1 {
        OtpbootDst1(0)
    }
}
#[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootLen(pub u32);
impl OtpbootLen {
    #[inline(always)]
    pub const fn otpboot_len(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_otpboot_len(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootLen {
    #[inline(always)]
    fn default() -> OtpbootLen {
        OtpbootLen(0)
    }
}
#[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct OtpbootSrc(pub u32);
impl OtpbootSrc {
    #[inline(always)]
    pub const fn otpboot_src(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_otpboot_src(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for OtpbootSrc {
    #[inline(always)]
    fn default() -> OtpbootSrc {
        OtpbootSrc(0)
    }
}
#[doc = "Lock configuration LSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page0lock0(pub u32);
impl Page0lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page0lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page0lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page0lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page0lock0 {
    #[inline(always)]
    fn default() -> Page0lock0 {
        Page0lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 0 (rows 0x0 through 0x3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page0lock1(pub u32);
impl Page0lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page0lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page0lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page0lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page0lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page0lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page0lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page0lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page0lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page0lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page0lock1 {
    #[inline(always)]
    fn default() -> Page0lock1 {
        Page0lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page10lock0(pub u32);
impl Page10lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page10lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page10lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page10lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page10lock0 {
    #[inline(always)]
    fn default() -> Page10lock0 {
        Page10lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 10 (rows 0x280 through 0x2bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page10lock1(pub u32);
impl Page10lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page10lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page10lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page10lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page10lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page10lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page10lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page10lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page10lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page10lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page10lock1 {
    #[inline(always)]
    fn default() -> Page10lock1 {
        Page10lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page11lock0(pub u32);
impl Page11lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page11lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page11lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page11lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page11lock0 {
    #[inline(always)]
    fn default() -> Page11lock0 {
        Page11lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 11 (rows 0x2c0 through 0x2ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page11lock1(pub u32);
impl Page11lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page11lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page11lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page11lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page11lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page11lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page11lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page11lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page11lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page11lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page11lock1 {
    #[inline(always)]
    fn default() -> Page11lock1 {
        Page11lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page12lock0(pub u32);
impl Page12lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page12lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page12lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page12lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page12lock0 {
    #[inline(always)]
    fn default() -> Page12lock0 {
        Page12lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 12 (rows 0x300 through 0x33f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page12lock1(pub u32);
impl Page12lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page12lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page12lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page12lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page12lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page12lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page12lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page12lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page12lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page12lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page12lock1 {
    #[inline(always)]
    fn default() -> Page12lock1 {
        Page12lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page13lock0(pub u32);
impl Page13lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page13lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page13lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page13lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page13lock0 {
    #[inline(always)]
    fn default() -> Page13lock0 {
        Page13lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 13 (rows 0x340 through 0x37f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page13lock1(pub u32);
impl Page13lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page13lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page13lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page13lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page13lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page13lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page13lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page13lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page13lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page13lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page13lock1 {
    #[inline(always)]
    fn default() -> Page13lock1 {
        Page13lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page14lock0(pub u32);
impl Page14lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page14lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page14lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page14lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page14lock0 {
    #[inline(always)]
    fn default() -> Page14lock0 {
        Page14lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 14 (rows 0x380 through 0x3bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page14lock1(pub u32);
impl Page14lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page14lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page14lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page14lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page14lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page14lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page14lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page14lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page14lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page14lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page14lock1 {
    #[inline(always)]
    fn default() -> Page14lock1 {
        Page14lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page15lock0(pub u32);
impl Page15lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page15lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page15lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page15lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page15lock0 {
    #[inline(always)]
    fn default() -> Page15lock0 {
        Page15lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 15 (rows 0x3c0 through 0x3ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page15lock1(pub u32);
impl Page15lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page15lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page15lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page15lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page15lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page15lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page15lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page15lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page15lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page15lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page15lock1 {
    #[inline(always)]
    fn default() -> Page15lock1 {
        Page15lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page16lock0(pub u32);
impl Page16lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page16lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page16lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page16lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page16lock0 {
    #[inline(always)]
    fn default() -> Page16lock0 {
        Page16lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 16 (rows 0x400 through 0x43f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page16lock1(pub u32);
impl Page16lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page16lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page16lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page16lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page16lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page16lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page16lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page16lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page16lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page16lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page16lock1 {
    #[inline(always)]
    fn default() -> Page16lock1 {
        Page16lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page17lock0(pub u32);
impl Page17lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page17lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page17lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page17lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page17lock0 {
    #[inline(always)]
    fn default() -> Page17lock0 {
        Page17lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 17 (rows 0x440 through 0x47f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page17lock1(pub u32);
impl Page17lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page17lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page17lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page17lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page17lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page17lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page17lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page17lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page17lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page17lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page17lock1 {
    #[inline(always)]
    fn default() -> Page17lock1 {
        Page17lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page18lock0(pub u32);
impl Page18lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page18lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page18lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page18lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page18lock0 {
    #[inline(always)]
    fn default() -> Page18lock0 {
        Page18lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 18 (rows 0x480 through 0x4bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page18lock1(pub u32);
impl Page18lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page18lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page18lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page18lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page18lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page18lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page18lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page18lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page18lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page18lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page18lock1 {
    #[inline(always)]
    fn default() -> Page18lock1 {
        Page18lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page19lock0(pub u32);
impl Page19lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page19lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page19lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page19lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page19lock0 {
    #[inline(always)]
    fn default() -> Page19lock0 {
        Page19lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 19 (rows 0x4c0 through 0x4ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page19lock1(pub u32);
impl Page19lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page19lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page19lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page19lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page19lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page19lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page19lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page19lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page19lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page19lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page19lock1 {
    #[inline(always)]
    fn default() -> Page19lock1 {
        Page19lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page1lock0(pub u32);
impl Page1lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page1lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page1lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page1lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page1lock0 {
    #[inline(always)]
    fn default() -> Page1lock0 {
        Page1lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 1 (rows 0x40 through 0x7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page1lock1(pub u32);
impl Page1lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page1lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page1lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page1lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page1lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page1lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page1lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page1lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page1lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page1lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page1lock1 {
    #[inline(always)]
    fn default() -> Page1lock1 {
        Page1lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page20lock0(pub u32);
impl Page20lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page20lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page20lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page20lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page20lock0 {
    #[inline(always)]
    fn default() -> Page20lock0 {
        Page20lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 20 (rows 0x500 through 0x53f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page20lock1(pub u32);
impl Page20lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page20lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page20lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page20lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page20lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page20lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page20lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page20lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page20lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page20lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page20lock1 {
    #[inline(always)]
    fn default() -> Page20lock1 {
        Page20lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page21lock0(pub u32);
impl Page21lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page21lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page21lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page21lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page21lock0 {
    #[inline(always)]
    fn default() -> Page21lock0 {
        Page21lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 21 (rows 0x540 through 0x57f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page21lock1(pub u32);
impl Page21lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page21lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page21lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page21lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page21lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page21lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page21lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page21lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page21lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page21lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page21lock1 {
    #[inline(always)]
    fn default() -> Page21lock1 {
        Page21lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page22lock0(pub u32);
impl Page22lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page22lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page22lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page22lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page22lock0 {
    #[inline(always)]
    fn default() -> Page22lock0 {
        Page22lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 22 (rows 0x580 through 0x5bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page22lock1(pub u32);
impl Page22lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page22lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page22lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page22lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page22lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page22lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page22lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page22lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page22lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page22lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page22lock1 {
    #[inline(always)]
    fn default() -> Page22lock1 {
        Page22lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page23lock0(pub u32);
impl Page23lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page23lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page23lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page23lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page23lock0 {
    #[inline(always)]
    fn default() -> Page23lock0 {
        Page23lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 23 (rows 0x5c0 through 0x5ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page23lock1(pub u32);
impl Page23lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page23lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page23lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page23lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page23lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page23lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page23lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page23lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page23lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page23lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page23lock1 {
    #[inline(always)]
    fn default() -> Page23lock1 {
        Page23lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page24lock0(pub u32);
impl Page24lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page24lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page24lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page24lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page24lock0 {
    #[inline(always)]
    fn default() -> Page24lock0 {
        Page24lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 24 (rows 0x600 through 0x63f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page24lock1(pub u32);
impl Page24lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page24lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page24lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page24lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page24lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page24lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page24lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page24lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page24lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page24lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page24lock1 {
    #[inline(always)]
    fn default() -> Page24lock1 {
        Page24lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page25lock0(pub u32);
impl Page25lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page25lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page25lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page25lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page25lock0 {
    #[inline(always)]
    fn default() -> Page25lock0 {
        Page25lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 25 (rows 0x640 through 0x67f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page25lock1(pub u32);
impl Page25lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page25lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page25lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page25lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page25lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page25lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page25lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page25lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page25lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page25lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page25lock1 {
    #[inline(always)]
    fn default() -> Page25lock1 {
        Page25lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page26lock0(pub u32);
impl Page26lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page26lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page26lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page26lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page26lock0 {
    #[inline(always)]
    fn default() -> Page26lock0 {
        Page26lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 26 (rows 0x680 through 0x6bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page26lock1(pub u32);
impl Page26lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page26lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page26lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page26lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page26lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page26lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page26lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page26lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page26lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page26lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page26lock1 {
    #[inline(always)]
    fn default() -> Page26lock1 {
        Page26lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page27lock0(pub u32);
impl Page27lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page27lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page27lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page27lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page27lock0 {
    #[inline(always)]
    fn default() -> Page27lock0 {
        Page27lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 27 (rows 0x6c0 through 0x6ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page27lock1(pub u32);
impl Page27lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page27lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page27lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page27lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page27lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page27lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page27lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page27lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page27lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page27lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page27lock1 {
    #[inline(always)]
    fn default() -> Page27lock1 {
        Page27lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page28lock0(pub u32);
impl Page28lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page28lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page28lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page28lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page28lock0 {
    #[inline(always)]
    fn default() -> Page28lock0 {
        Page28lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 28 (rows 0x700 through 0x73f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page28lock1(pub u32);
impl Page28lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page28lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page28lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page28lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page28lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page28lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page28lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page28lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page28lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page28lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page28lock1 {
    #[inline(always)]
    fn default() -> Page28lock1 {
        Page28lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page29lock0(pub u32);
impl Page29lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page29lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page29lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page29lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page29lock0 {
    #[inline(always)]
    fn default() -> Page29lock0 {
        Page29lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 29 (rows 0x740 through 0x77f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page29lock1(pub u32);
impl Page29lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page29lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page29lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page29lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page29lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page29lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page29lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page29lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page29lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page29lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page29lock1 {
    #[inline(always)]
    fn default() -> Page29lock1 {
        Page29lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page2lock0(pub u32);
impl Page2lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page2lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page2lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page2lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page2lock0 {
    #[inline(always)]
    fn default() -> Page2lock0 {
        Page2lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 2 (rows 0x80 through 0xbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page2lock1(pub u32);
impl Page2lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page2lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page2lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page2lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page2lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page2lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page2lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page2lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page2lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page2lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page2lock1 {
    #[inline(always)]
    fn default() -> Page2lock1 {
        Page2lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page30lock0(pub u32);
impl Page30lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page30lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page30lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page30lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page30lock0 {
    #[inline(always)]
    fn default() -> Page30lock0 {
        Page30lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 30 (rows 0x780 through 0x7bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page30lock1(pub u32);
impl Page30lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page30lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page30lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page30lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page30lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page30lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page30lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page30lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page30lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page30lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page30lock1 {
    #[inline(always)]
    fn default() -> Page30lock1 {
        Page30lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page31lock0(pub u32);
impl Page31lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page31lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page31lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page31lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page31lock0 {
    #[inline(always)]
    fn default() -> Page31lock0 {
        Page31lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 31 (rows 0x7c0 through 0x7ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page31lock1(pub u32);
impl Page31lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page31lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page31lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page31lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page31lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page31lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page31lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page31lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page31lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page31lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page31lock1 {
    #[inline(always)]
    fn default() -> Page31lock1 {
        Page31lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page32lock0(pub u32);
impl Page32lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page32lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page32lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page32lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page32lock0 {
    #[inline(always)]
    fn default() -> Page32lock0 {
        Page32lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 32 (rows 0x800 through 0x83f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page32lock1(pub u32);
impl Page32lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page32lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page32lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page32lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page32lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page32lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page32lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page32lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page32lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page32lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page32lock1 {
    #[inline(always)]
    fn default() -> Page32lock1 {
        Page32lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page33lock0(pub u32);
impl Page33lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page33lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page33lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page33lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page33lock0 {
    #[inline(always)]
    fn default() -> Page33lock0 {
        Page33lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 33 (rows 0x840 through 0x87f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page33lock1(pub u32);
impl Page33lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page33lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page33lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page33lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page33lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page33lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page33lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page33lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page33lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page33lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page33lock1 {
    #[inline(always)]
    fn default() -> Page33lock1 {
        Page33lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page34lock0(pub u32);
impl Page34lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page34lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page34lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page34lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page34lock0 {
    #[inline(always)]
    fn default() -> Page34lock0 {
        Page34lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 34 (rows 0x880 through 0x8bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page34lock1(pub u32);
impl Page34lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page34lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page34lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page34lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page34lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page34lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page34lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page34lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page34lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page34lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page34lock1 {
    #[inline(always)]
    fn default() -> Page34lock1 {
        Page34lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page35lock0(pub u32);
impl Page35lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page35lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page35lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page35lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page35lock0 {
    #[inline(always)]
    fn default() -> Page35lock0 {
        Page35lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 35 (rows 0x8c0 through 0x8ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page35lock1(pub u32);
impl Page35lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page35lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page35lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page35lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page35lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page35lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page35lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page35lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page35lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page35lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page35lock1 {
    #[inline(always)]
    fn default() -> Page35lock1 {
        Page35lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page36lock0(pub u32);
impl Page36lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page36lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page36lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page36lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page36lock0 {
    #[inline(always)]
    fn default() -> Page36lock0 {
        Page36lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 36 (rows 0x900 through 0x93f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page36lock1(pub u32);
impl Page36lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page36lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page36lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page36lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page36lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page36lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page36lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page36lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page36lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page36lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page36lock1 {
    #[inline(always)]
    fn default() -> Page36lock1 {
        Page36lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page37lock0(pub u32);
impl Page37lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page37lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page37lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page37lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page37lock0 {
    #[inline(always)]
    fn default() -> Page37lock0 {
        Page37lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 37 (rows 0x940 through 0x97f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page37lock1(pub u32);
impl Page37lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page37lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page37lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page37lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page37lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page37lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page37lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page37lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page37lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page37lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page37lock1 {
    #[inline(always)]
    fn default() -> Page37lock1 {
        Page37lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page38lock0(pub u32);
impl Page38lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page38lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page38lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page38lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page38lock0 {
    #[inline(always)]
    fn default() -> Page38lock0 {
        Page38lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 38 (rows 0x980 through 0x9bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page38lock1(pub u32);
impl Page38lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page38lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page38lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page38lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page38lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page38lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page38lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page38lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page38lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page38lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page38lock1 {
    #[inline(always)]
    fn default() -> Page38lock1 {
        Page38lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page39lock0(pub u32);
impl Page39lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page39lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page39lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page39lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page39lock0 {
    #[inline(always)]
    fn default() -> Page39lock0 {
        Page39lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 39 (rows 0x9c0 through 0x9ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page39lock1(pub u32);
impl Page39lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page39lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page39lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page39lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page39lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page39lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page39lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page39lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page39lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page39lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page39lock1 {
    #[inline(always)]
    fn default() -> Page39lock1 {
        Page39lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page3lock0(pub u32);
impl Page3lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page3lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page3lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page3lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page3lock0 {
    #[inline(always)]
    fn default() -> Page3lock0 {
        Page3lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 3 (rows 0xc0 through 0xff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page3lock1(pub u32);
impl Page3lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page3lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page3lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page3lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page3lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page3lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page3lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page3lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page3lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page3lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page3lock1 {
    #[inline(always)]
    fn default() -> Page3lock1 {
        Page3lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page40lock0(pub u32);
impl Page40lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page40lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page40lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page40lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page40lock0 {
    #[inline(always)]
    fn default() -> Page40lock0 {
        Page40lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 40 (rows 0xa00 through 0xa3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page40lock1(pub u32);
impl Page40lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page40lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page40lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page40lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page40lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page40lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page40lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page40lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page40lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page40lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page40lock1 {
    #[inline(always)]
    fn default() -> Page40lock1 {
        Page40lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page41lock0(pub u32);
impl Page41lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page41lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page41lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page41lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page41lock0 {
    #[inline(always)]
    fn default() -> Page41lock0 {
        Page41lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 41 (rows 0xa40 through 0xa7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page41lock1(pub u32);
impl Page41lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page41lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page41lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page41lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page41lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page41lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page41lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page41lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page41lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page41lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page41lock1 {
    #[inline(always)]
    fn default() -> Page41lock1 {
        Page41lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page42lock0(pub u32);
impl Page42lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page42lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page42lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page42lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page42lock0 {
    #[inline(always)]
    fn default() -> Page42lock0 {
        Page42lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 42 (rows 0xa80 through 0xabf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page42lock1(pub u32);
impl Page42lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page42lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page42lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page42lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page42lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page42lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page42lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page42lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page42lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page42lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page42lock1 {
    #[inline(always)]
    fn default() -> Page42lock1 {
        Page42lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page43lock0(pub u32);
impl Page43lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page43lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page43lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page43lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page43lock0 {
    #[inline(always)]
    fn default() -> Page43lock0 {
        Page43lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 43 (rows 0xac0 through 0xaff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page43lock1(pub u32);
impl Page43lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page43lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page43lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page43lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page43lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page43lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page43lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page43lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page43lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page43lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page43lock1 {
    #[inline(always)]
    fn default() -> Page43lock1 {
        Page43lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page44lock0(pub u32);
impl Page44lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page44lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page44lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page44lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page44lock0 {
    #[inline(always)]
    fn default() -> Page44lock0 {
        Page44lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 44 (rows 0xb00 through 0xb3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page44lock1(pub u32);
impl Page44lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page44lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page44lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page44lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page44lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page44lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page44lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page44lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page44lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page44lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page44lock1 {
    #[inline(always)]
    fn default() -> Page44lock1 {
        Page44lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page45lock0(pub u32);
impl Page45lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page45lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page45lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page45lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page45lock0 {
    #[inline(always)]
    fn default() -> Page45lock0 {
        Page45lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 45 (rows 0xb40 through 0xb7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page45lock1(pub u32);
impl Page45lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page45lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page45lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page45lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page45lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page45lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page45lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page45lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page45lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page45lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page45lock1 {
    #[inline(always)]
    fn default() -> Page45lock1 {
        Page45lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page46lock0(pub u32);
impl Page46lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page46lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page46lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page46lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page46lock0 {
    #[inline(always)]
    fn default() -> Page46lock0 {
        Page46lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 46 (rows 0xb80 through 0xbbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page46lock1(pub u32);
impl Page46lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page46lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page46lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page46lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page46lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page46lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page46lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page46lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page46lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page46lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page46lock1 {
    #[inline(always)]
    fn default() -> Page46lock1 {
        Page46lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page47lock0(pub u32);
impl Page47lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page47lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page47lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page47lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page47lock0 {
    #[inline(always)]
    fn default() -> Page47lock0 {
        Page47lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 47 (rows 0xbc0 through 0xbff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page47lock1(pub u32);
impl Page47lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page47lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page47lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page47lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page47lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page47lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page47lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page47lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page47lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page47lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page47lock1 {
    #[inline(always)]
    fn default() -> Page47lock1 {
        Page47lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page48lock0(pub u32);
impl Page48lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page48lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page48lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page48lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page48lock0 {
    #[inline(always)]
    fn default() -> Page48lock0 {
        Page48lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 48 (rows 0xc00 through 0xc3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page48lock1(pub u32);
impl Page48lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page48lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page48lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page48lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page48lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page48lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page48lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page48lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page48lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page48lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page48lock1 {
    #[inline(always)]
    fn default() -> Page48lock1 {
        Page48lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page49lock0(pub u32);
impl Page49lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page49lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page49lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page49lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page49lock0 {
    #[inline(always)]
    fn default() -> Page49lock0 {
        Page49lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 49 (rows 0xc40 through 0xc7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page49lock1(pub u32);
impl Page49lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page49lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page49lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page49lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page49lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page49lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page49lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page49lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page49lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page49lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page49lock1 {
    #[inline(always)]
    fn default() -> Page49lock1 {
        Page49lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page4lock0(pub u32);
impl Page4lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page4lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page4lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page4lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page4lock0 {
    #[inline(always)]
    fn default() -> Page4lock0 {
        Page4lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 4 (rows 0x100 through 0x13f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page4lock1(pub u32);
impl Page4lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page4lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page4lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page4lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page4lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page4lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page4lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page4lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page4lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page4lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page4lock1 {
    #[inline(always)]
    fn default() -> Page4lock1 {
        Page4lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page50lock0(pub u32);
impl Page50lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page50lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page50lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page50lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page50lock0 {
    #[inline(always)]
    fn default() -> Page50lock0 {
        Page50lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 50 (rows 0xc80 through 0xcbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page50lock1(pub u32);
impl Page50lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page50lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page50lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page50lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page50lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page50lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page50lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page50lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page50lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page50lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page50lock1 {
    #[inline(always)]
    fn default() -> Page50lock1 {
        Page50lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page51lock0(pub u32);
impl Page51lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page51lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page51lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page51lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page51lock0 {
    #[inline(always)]
    fn default() -> Page51lock0 {
        Page51lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 51 (rows 0xcc0 through 0xcff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page51lock1(pub u32);
impl Page51lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page51lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page51lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page51lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page51lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page51lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page51lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page51lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page51lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page51lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page51lock1 {
    #[inline(always)]
    fn default() -> Page51lock1 {
        Page51lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page52lock0(pub u32);
impl Page52lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page52lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page52lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page52lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page52lock0 {
    #[inline(always)]
    fn default() -> Page52lock0 {
        Page52lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 52 (rows 0xd00 through 0xd3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page52lock1(pub u32);
impl Page52lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page52lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page52lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page52lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page52lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page52lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page52lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page52lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page52lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page52lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page52lock1 {
    #[inline(always)]
    fn default() -> Page52lock1 {
        Page52lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page53lock0(pub u32);
impl Page53lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page53lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page53lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page53lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page53lock0 {
    #[inline(always)]
    fn default() -> Page53lock0 {
        Page53lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 53 (rows 0xd40 through 0xd7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page53lock1(pub u32);
impl Page53lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page53lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page53lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page53lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page53lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page53lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page53lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page53lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page53lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page53lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page53lock1 {
    #[inline(always)]
    fn default() -> Page53lock1 {
        Page53lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page54lock0(pub u32);
impl Page54lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page54lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page54lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page54lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page54lock0 {
    #[inline(always)]
    fn default() -> Page54lock0 {
        Page54lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 54 (rows 0xd80 through 0xdbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page54lock1(pub u32);
impl Page54lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page54lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page54lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page54lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page54lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page54lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page54lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page54lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page54lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page54lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page54lock1 {
    #[inline(always)]
    fn default() -> Page54lock1 {
        Page54lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page55lock0(pub u32);
impl Page55lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page55lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page55lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page55lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page55lock0 {
    #[inline(always)]
    fn default() -> Page55lock0 {
        Page55lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 55 (rows 0xdc0 through 0xdff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page55lock1(pub u32);
impl Page55lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page55lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page55lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page55lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page55lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page55lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page55lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page55lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page55lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page55lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page55lock1 {
    #[inline(always)]
    fn default() -> Page55lock1 {
        Page55lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page56lock0(pub u32);
impl Page56lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page56lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page56lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page56lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page56lock0 {
    #[inline(always)]
    fn default() -> Page56lock0 {
        Page56lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 56 (rows 0xe00 through 0xe3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page56lock1(pub u32);
impl Page56lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page56lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page56lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page56lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page56lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page56lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page56lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page56lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page56lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page56lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page56lock1 {
    #[inline(always)]
    fn default() -> Page56lock1 {
        Page56lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page57lock0(pub u32);
impl Page57lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page57lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page57lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page57lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page57lock0 {
    #[inline(always)]
    fn default() -> Page57lock0 {
        Page57lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 57 (rows 0xe40 through 0xe7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page57lock1(pub u32);
impl Page57lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page57lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page57lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page57lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page57lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page57lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page57lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page57lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page57lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page57lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page57lock1 {
    #[inline(always)]
    fn default() -> Page57lock1 {
        Page57lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page58lock0(pub u32);
impl Page58lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page58lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page58lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page58lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page58lock0 {
    #[inline(always)]
    fn default() -> Page58lock0 {
        Page58lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 58 (rows 0xe80 through 0xebf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page58lock1(pub u32);
impl Page58lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page58lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page58lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page58lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page58lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page58lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page58lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page58lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page58lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page58lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page58lock1 {
    #[inline(always)]
    fn default() -> Page58lock1 {
        Page58lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page59lock0(pub u32);
impl Page59lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page59lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page59lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page59lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page59lock0 {
    #[inline(always)]
    fn default() -> Page59lock0 {
        Page59lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 59 (rows 0xec0 through 0xeff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page59lock1(pub u32);
impl Page59lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page59lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page59lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page59lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page59lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page59lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page59lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page59lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page59lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page59lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page59lock1 {
    #[inline(always)]
    fn default() -> Page59lock1 {
        Page59lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page5lock0(pub u32);
impl Page5lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page5lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page5lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page5lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page5lock0 {
    #[inline(always)]
    fn default() -> Page5lock0 {
        Page5lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 5 (rows 0x140 through 0x17f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page5lock1(pub u32);
impl Page5lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page5lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page5lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page5lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page5lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page5lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page5lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page5lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page5lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page5lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page5lock1 {
    #[inline(always)]
    fn default() -> Page5lock1 {
        Page5lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page60lock0(pub u32);
impl Page60lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page60lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page60lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page60lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page60lock0 {
    #[inline(always)]
    fn default() -> Page60lock0 {
        Page60lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 60 (rows 0xf00 through 0xf3f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page60lock1(pub u32);
impl Page60lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page60lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page60lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page60lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page60lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page60lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page60lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page60lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page60lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page60lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page60lock1 {
    #[inline(always)]
    fn default() -> Page60lock1 {
        Page60lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page61lock0(pub u32);
impl Page61lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page61lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page61lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page61lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page61lock0 {
    #[inline(always)]
    fn default() -> Page61lock0 {
        Page61lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 61 (rows 0xf40 through 0xf7f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page61lock1(pub u32);
impl Page61lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page61lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page61lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page61lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page61lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page61lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page61lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page61lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page61lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page61lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page61lock1 {
    #[inline(always)]
    fn default() -> Page61lock1 {
        Page61lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page62lock0(pub u32);
impl Page62lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page62lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page62lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page62lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page62lock0 {
    #[inline(always)]
    fn default() -> Page62lock0 {
        Page62lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 62 (rows 0xf80 through 0xfbf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page62lock1(pub u32);
impl Page62lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page62lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page62lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page62lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page62lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page62lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page62lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page62lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page62lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page62lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page62lock1 {
    #[inline(always)]
    fn default() -> Page62lock1 {
        Page62lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page63lock0(pub u32);
impl Page63lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page63lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page63lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page63lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Decommission for RMA of a suspected faulty device. This re-enables the factory test JTAG interface, and makes pages 3 through 61 of the OTP permanently inaccessible."]
    #[inline(always)]
    pub const fn rma(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Decommission for RMA of a suspected faulty device. This re-enables the factory test JTAG interface, and makes pages 3 through 61 of the OTP permanently inaccessible."]
    #[inline(always)]
    pub fn set_rma(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page63lock0 {
    #[inline(always)]
    fn default() -> Page63lock0 {
        Page63lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page63lock1(pub u32);
impl Page63lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page63lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page63lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page63lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page63lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page63lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page63lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page63lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page63lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page63lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page63lock1 {
    #[inline(always)]
    fn default() -> Page63lock1 {
        Page63lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page6lock0(pub u32);
impl Page6lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page6lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page6lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page6lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page6lock0 {
    #[inline(always)]
    fn default() -> Page6lock0 {
        Page6lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 6 (rows 0x180 through 0x1bf). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page6lock1(pub u32);
impl Page6lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page6lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page6lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page6lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page6lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page6lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page6lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page6lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page6lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page6lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page6lock1 {
    #[inline(always)]
    fn default() -> Page6lock1 {
        Page6lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page7lock0(pub u32);
impl Page7lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page7lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page7lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page7lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page7lock0 {
    #[inline(always)]
    fn default() -> Page7lock0 {
        Page7lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 7 (rows 0x1c0 through 0x1ff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page7lock1(pub u32);
impl Page7lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page7lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page7lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page7lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page7lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page7lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page7lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page7lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page7lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page7lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page7lock1 {
    #[inline(always)]
    fn default() -> Page7lock1 {
        Page7lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page8lock0(pub u32);
impl Page8lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page8lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page8lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page8lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page8lock0 {
    #[inline(always)]
    fn default() -> Page8lock0 {
        Page8lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 8 (rows 0x200 through 0x23f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page8lock1(pub u32);
impl Page8lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page8lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page8lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page8lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page8lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page8lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page8lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page8lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page8lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page8lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page8lock1 {
    #[inline(always)]
    fn default() -> Page8lock1 {
        Page8lock1(0)
    }
}
#[doc = "Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page9lock0(pub u32);
impl Page9lock0 {
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_w(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant write access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_w(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 0usize)) | (((val as u32) & 0x07) << 0usize);
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub const fn key_r(&self) -> u8 {
        let val = (self.0 >> 3usize) & 0x07;
        val as u8
    }
    #[doc = "Index 1-6 of a hardware key which must be entered to grant read access, or 0 if no such key is required."]
    #[inline(always)]
    pub fn set_key_r(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 3usize)) | (((val as u32) & 0x07) << 3usize);
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub const fn no_key_state(&self) -> super::vals::Page9lock0noKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::Page9lock0noKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::Page9lock0noKeyState) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val.to_bits() as u32) & 0x01) << 6usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page9lock0 {
    #[inline(always)]
    fn default() -> Page9lock0 {
        Page9lock0(0)
    }
}
#[doc = "Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Page9lock1(pub u32);
impl Page9lock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::Page9lock1lockS {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::Page9lock1lockS::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::Page9lock1lockS) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::Page9lock1lockNs {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::Page9lock1lockNs::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::Page9lock1lockNs) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::Page9lock1lockBl {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Page9lock1lockBl::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::Page9lock1lockBl) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r1(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r1(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub const fn r2(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0xff;
        val as u8
    }
    #[doc = "Redundant copy of bits 7:0"]
    #[inline(always)]
    pub fn set_r2(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 16usize)) | (((val as u32) & 0xff) << 16usize);
    }
}
impl Default for Page9lock1 {
    #[inline(always)]
    fn default() -> Page9lock1 {
        Page9lock1(0)
    }
}
#[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid0(pub u32);
impl Randid0 {
    #[inline(always)]
    pub const fn randid0(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid0(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid0 {
    #[inline(always)]
    fn default() -> Randid0 {
        Randid0(0)
    }
}
#[doc = "Bits 31:16 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid1(pub u32);
impl Randid1 {
    #[inline(always)]
    pub const fn randid1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid1 {
    #[inline(always)]
    fn default() -> Randid1 {
        Randid1(0)
    }
}
#[doc = "Bits 47:32 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid2(pub u32);
impl Randid2 {
    #[inline(always)]
    pub const fn randid2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid2 {
    #[inline(always)]
    fn default() -> Randid2 {
        Randid2(0)
    }
}
#[doc = "Bits 63:48 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid3(pub u32);
impl Randid3 {
    #[inline(always)]
    pub const fn randid3(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid3(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid3 {
    #[inline(always)]
    fn default() -> Randid3 {
        Randid3(0)
    }
}
#[doc = "Bits 79:64 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid4(pub u32);
impl Randid4 {
    #[inline(always)]
    pub const fn randid4(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid4(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid4 {
    #[inline(always)]
    fn default() -> Randid4 {
        Randid4(0)
    }
}
#[doc = "Bits 95:80 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid5(pub u32);
impl Randid5 {
    #[inline(always)]
    pub const fn randid5(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid5(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid5 {
    #[inline(always)]
    fn default() -> Randid5 {
        Randid5(0)
    }
}
#[doc = "Bits 111:96 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid6(pub u32);
impl Randid6 {
    #[inline(always)]
    pub const fn randid6(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid6(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid6 {
    #[inline(always)]
    fn default() -> Randid6 {
        Randid6(0)
    }
}
#[doc = "Bits 127:112 of private per-device random number (ECC)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Randid7(pub u32);
impl Randid7 {
    #[inline(always)]
    pub const fn randid7(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_randid7(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for Randid7 {
    #[inline(always)]
    fn default() -> Randid7 {
        Randid7(0)
    }
}
#[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct RoscCalib(pub u32);
impl RoscCalib {
    #[inline(always)]
    pub const fn rosc_calib(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_rosc_calib(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for RoscCalib {
    #[inline(always)]
    fn default() -> RoscCalib {
        RoscCalib(0)
    }
}
#[doc = "USB boot specific feature flags (RBIT-3)"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbBootFlags(pub u32);
impl UsbBootFlags {
    #[doc = "valid flag for USB_DEVICE_VID_VALUE entry of the USB_WHITE_LABEL struct (index 0)"]
    #[inline(always)]
    pub const fn wl_usb_device_vid_value_valid(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_VID_VALUE entry of the USB_WHITE_LABEL struct (index 0)"]
    #[inline(always)]
    pub fn set_wl_usb_device_vid_value_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "valid flag for USB_DEVICE_PID_VALUE entry of the USB_WHITE_LABEL struct (index 1)"]
    #[inline(always)]
    pub const fn wl_usb_device_pid_value_valid(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_PID_VALUE entry of the USB_WHITE_LABEL struct (index 1)"]
    #[inline(always)]
    pub fn set_wl_usb_device_pid_value_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "valid flag for USB_DEVICE_BCD_DEVICEVALUE entry of the USB_WHITE_LABEL struct (index 2)"]
    #[inline(always)]
    pub const fn wl_usb_device_serial_number_value_valid(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_BCD_DEVICEVALUE entry of the USB_WHITE_LABEL struct (index 2)"]
    #[inline(always)]
    pub fn set_wl_usb_device_serial_number_value_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "valid flag for USB_DEVICE_LANG_ID_VALUE entry of the USB_WHITE_LABEL struct (index 3)"]
    #[inline(always)]
    pub const fn wl_usb_device_lang_id_value_valid(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_LANG_ID_VALUE entry of the USB_WHITE_LABEL struct (index 3)"]
    #[inline(always)]
    pub fn set_wl_usb_device_lang_id_value_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[doc = "valid flag for USB_DEVICE_MANUFACTURER_STRDEF entry of the USB_WHITE_LABEL struct (index 4)"]
    #[inline(always)]
    pub const fn wl_usb_device_manufacturer_strdef_valid(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_MANUFACTURER_STRDEF entry of the USB_WHITE_LABEL struct (index 4)"]
    #[inline(always)]
    pub fn set_wl_usb_device_manufacturer_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "valid flag for USB_DEVICE_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 5)"]
    #[inline(always)]
    pub const fn wl_usb_device_product_strdef_valid(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 5)"]
    #[inline(always)]
    pub fn set_wl_usb_device_product_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[doc = "valid flag for USB_DEVICE_SERIAL_NUMBER_STRDEF entry of the USB_WHITE_LABEL struct (index 6)"]
    #[inline(always)]
    pub const fn wl_usb_device_serial_number_strdef_valid(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_DEVICE_SERIAL_NUMBER_STRDEF entry of the USB_WHITE_LABEL struct (index 6)"]
    #[inline(always)]
    pub fn set_wl_usb_device_serial_number_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "valid flag for USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES entry of the USB_WHITE_LABEL struct (index 7)"]
    #[inline(always)]
    pub const fn wl_usb_config_attributes_max_power_values_valid(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for USB_CONFIG_ATTRIBUTES_MAX_POWER_VALUES entry of the USB_WHITE_LABEL struct (index 7)"]
    #[inline(always)]
    pub fn set_wl_usb_config_attributes_max_power_values_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
    #[doc = "valid flag for VOLUME_LABEL_STRDEF entry of the USB_WHITE_LABEL struct (index 8)"]
    #[inline(always)]
    pub const fn wl_volume_label_strdef_valid(&self) -> bool {
        let val = (self.0 >> 8usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for VOLUME_LABEL_STRDEF entry of the USB_WHITE_LABEL struct (index 8)"]
    #[inline(always)]
    pub fn set_wl_volume_label_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8usize)) | (((val as u32) & 0x01) << 8usize);
    }
    #[doc = "valid flag for SCSI_INQUIRY_VENDOR_STRDEF entry of the USB_WHITE_LABEL struct (index 9)"]
    #[inline(always)]
    pub const fn wl_scsi_inquiry_vendor_strdef_valid(&self) -> bool {
        let val = (self.0 >> 9usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for SCSI_INQUIRY_VENDOR_STRDEF entry of the USB_WHITE_LABEL struct (index 9)"]
    #[inline(always)]
    pub fn set_wl_scsi_inquiry_vendor_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9usize)) | (((val as u32) & 0x01) << 9usize);
    }
    #[doc = "valid flag for SCSI_INQUIRY_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 10)"]
    #[inline(always)]
    pub const fn wl_scsi_inquiry_product_strdef_valid(&self) -> bool {
        let val = (self.0 >> 10usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for SCSI_INQUIRY_PRODUCT_STRDEF entry of the USB_WHITE_LABEL struct (index 10)"]
    #[inline(always)]
    pub fn set_wl_scsi_inquiry_product_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10usize)) | (((val as u32) & 0x01) << 10usize);
    }
    #[doc = "valid flag for SCSI_INQUIRY_VERSION_STRDEF entry of the USB_WHITE_LABEL struct (index 11)"]
    #[inline(always)]
    pub const fn wl_scsi_inquiry_version_strdef_valid(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for SCSI_INQUIRY_VERSION_STRDEF entry of the USB_WHITE_LABEL struct (index 11)"]
    #[inline(always)]
    pub fn set_wl_scsi_inquiry_version_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "valid flag for INDEX_HTM_REDIRECT_URL_STRDEF entry of the USB_WHITE_LABEL struct (index 12)"]
    #[inline(always)]
    pub const fn wl_index_htm_redirect_url_strdef_valid(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for INDEX_HTM_REDIRECT_URL_STRDEF entry of the USB_WHITE_LABEL struct (index 12)"]
    #[inline(always)]
    pub fn set_wl_index_htm_redirect_url_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "valid flag for INDEX_HTM_REDIRECT_NAME_STRDEF entry of the USB_WHITE_LABEL struct (index 13)"]
    #[inline(always)]
    pub const fn wl_index_htm_redirect_name_strdef_valid(&self) -> bool {
        let val = (self.0 >> 13usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for INDEX_HTM_REDIRECT_NAME_STRDEF entry of the USB_WHITE_LABEL struct (index 13)"]
    #[inline(always)]
    pub fn set_wl_index_htm_redirect_name_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13usize)) | (((val as u32) & 0x01) << 13usize);
    }
    #[doc = "valid flag for INFO_UF2_TXT_MODEL_STRDEF entry of the USB_WHITE_LABEL struct (index 14)"]
    #[inline(always)]
    pub const fn wl_info_uf2_txt_model_strdef_valid(&self) -> bool {
        let val = (self.0 >> 14usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for INFO_UF2_TXT_MODEL_STRDEF entry of the USB_WHITE_LABEL struct (index 14)"]
    #[inline(always)]
    pub fn set_wl_info_uf2_txt_model_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14usize)) | (((val as u32) & 0x01) << 14usize);
    }
    #[doc = "valid flag for the USB_WHITE_LABEL_ADDR field"]
    #[inline(always)]
    pub const fn wl_info_uf2_txt_board_id_strdef_valid(&self) -> bool {
        let val = (self.0 >> 15usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for the USB_WHITE_LABEL_ADDR field"]
    #[inline(always)]
    pub fn set_wl_info_uf2_txt_board_id_strdef_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15usize)) | (((val as u32) & 0x01) << 15usize);
    }
    #[doc = "valid flag for INFO_UF2_TXT_BOARD_ID_STRDEF entry of the USB_WHITE_LABEL struct (index 15)"]
    #[inline(always)]
    pub const fn white_label_addr_valid(&self) -> bool {
        let val = (self.0 >> 22usize) & 0x01;
        val != 0
    }
    #[doc = "valid flag for INFO_UF2_TXT_BOARD_ID_STRDEF entry of the USB_WHITE_LABEL struct (index 15)"]
    #[inline(always)]
    pub fn set_white_label_addr_valid(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22usize)) | (((val as u32) & 0x01) << 22usize);
    }
    #[doc = "Swap DM/DP during USB boot, to support board layouts with mirrored USB routing (deliberate or accidental)."]
    #[inline(always)]
    pub const fn dp_dm_swap(&self) -> bool {
        let val = (self.0 >> 23usize) & 0x01;
        val != 0
    }
    #[doc = "Swap DM/DP during USB boot, to support board layouts with mirrored USB routing (deliberate or accidental)."]
    #[inline(always)]
    pub fn set_dp_dm_swap(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23usize)) | (((val as u32) & 0x01) << 23usize);
    }
}
impl Default for UsbBootFlags {
    #[inline(always)]
    fn default() -> UsbBootFlags {
        UsbBootFlags(0)
    }
}
#[doc = "Redundant copy of USB_BOOT_FLAGS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbBootFlagsR1(pub u32);
impl UsbBootFlagsR1 {
    #[inline(always)]
    pub const fn usb_boot_flags_r1(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_usb_boot_flags_r1(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for UsbBootFlagsR1 {
    #[inline(always)]
    fn default() -> UsbBootFlagsR1 {
        UsbBootFlagsR1(0)
    }
}
#[doc = "Redundant copy of USB_BOOT_FLAGS"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbBootFlagsR2(pub u32);
impl UsbBootFlagsR2 {
    #[inline(always)]
    pub const fn usb_boot_flags_r2(&self) -> u32 {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        val as u32
    }
    #[inline(always)]
    pub fn set_usb_boot_flags_r2(&mut self, val: u32) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize)) | (((val as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for UsbBootFlagsR2 {
    #[inline(always)]
    fn default() -> UsbBootFlagsR2 {
        UsbBootFlagsR2(0)
    }
}
#[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct UsbWhiteLabelAddr(pub u32);
impl UsbWhiteLabelAddr {
    #[inline(always)]
    pub const fn usb_white_label_addr(&self) -> super::vals::UsbWhiteLabelAddr {
        let val = (self.0 >> 0usize) & 0x00ff_ffff;
        super::vals::UsbWhiteLabelAddr::from_bits(val as u32)
    }
    #[inline(always)]
    pub fn set_usb_white_label_addr(&mut self, val: super::vals::UsbWhiteLabelAddr) {
        self.0 = (self.0 & !(0x00ff_ffff << 0usize))
            | (((val.to_bits() as u32) & 0x00ff_ffff) << 0usize);
    }
}
impl Default for UsbWhiteLabelAddr {
    #[inline(always)]
    fn default() -> UsbWhiteLabelAddr {
        UsbWhiteLabelAddr(0)
    }
}
