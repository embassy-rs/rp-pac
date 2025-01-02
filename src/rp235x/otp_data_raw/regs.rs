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
impl core::fmt::Debug for BootFlags0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootFlags0")
            .field("disable_bootsel_exec2", &self.disable_bootsel_exec2())
            .field("enable_bootsel_led", &self.enable_bootsel_led())
            .field(
                "enable_bootsel_non_default_pll_xosc_cfg",
                &self.enable_bootsel_non_default_pll_xosc_cfg(),
            )
            .field("flash_io_voltage_1v8", &self.flash_io_voltage_1v8())
            .field("fast_sigcheck_rosc_div", &self.fast_sigcheck_rosc_div())
            .field("flash_devinfo_enable", &self.flash_devinfo_enable())
            .field(
                "override_flash_partition_slot_size",
                &self.override_flash_partition_slot_size(),
            )
            .field("single_flash_binary", &self.single_flash_binary())
            .field("disable_auto_switch_arch", &self.disable_auto_switch_arch())
            .field("secure_partition_table", &self.secure_partition_table())
            .field("hashed_partition_table", &self.hashed_partition_table())
            .field("rollback_required", &self.rollback_required())
            .field("disable_flash_boot", &self.disable_flash_boot())
            .field("disable_otp_boot", &self.disable_otp_boot())
            .field("enable_otp_boot", &self.enable_otp_boot())
            .field("disable_power_scratch", &self.disable_power_scratch())
            .field("disable_watchdog_scratch", &self.disable_watchdog_scratch())
            .field(
                "disable_bootsel_usb_msd_ifc",
                &self.disable_bootsel_usb_msd_ifc(),
            )
            .field(
                "disable_bootsel_usb_picoboot_ifc",
                &self.disable_bootsel_usb_picoboot_ifc(),
            )
            .field(
                "disable_bootsel_uart_boot",
                &self.disable_bootsel_uart_boot(),
            )
            .field(
                "disable_xip_access_on_sram_entry",
                &self.disable_xip_access_on_sram_entry(),
            )
            .field("disable_sram_window_boot", &self.disable_sram_window_boot())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootFlags0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BootFlags0 {
            disable_bootsel_exec2: bool,
            enable_bootsel_led: bool,
            enable_bootsel_non_default_pll_xosc_cfg: bool,
            flash_io_voltage_1v8: bool,
            fast_sigcheck_rosc_div: bool,
            flash_devinfo_enable: bool,
            override_flash_partition_slot_size: bool,
            single_flash_binary: bool,
            disable_auto_switch_arch: bool,
            secure_partition_table: bool,
            hashed_partition_table: bool,
            rollback_required: bool,
            disable_flash_boot: bool,
            disable_otp_boot: bool,
            enable_otp_boot: bool,
            disable_power_scratch: bool,
            disable_watchdog_scratch: bool,
            disable_bootsel_usb_msd_ifc: bool,
            disable_bootsel_usb_picoboot_ifc: bool,
            disable_bootsel_uart_boot: bool,
            disable_xip_access_on_sram_entry: bool,
            disable_sram_window_boot: bool,
        }
        let proxy = BootFlags0 {
            disable_bootsel_exec2: self.disable_bootsel_exec2(),
            enable_bootsel_led: self.enable_bootsel_led(),
            enable_bootsel_non_default_pll_xosc_cfg: self.enable_bootsel_non_default_pll_xosc_cfg(),
            flash_io_voltage_1v8: self.flash_io_voltage_1v8(),
            fast_sigcheck_rosc_div: self.fast_sigcheck_rosc_div(),
            flash_devinfo_enable: self.flash_devinfo_enable(),
            override_flash_partition_slot_size: self.override_flash_partition_slot_size(),
            single_flash_binary: self.single_flash_binary(),
            disable_auto_switch_arch: self.disable_auto_switch_arch(),
            secure_partition_table: self.secure_partition_table(),
            hashed_partition_table: self.hashed_partition_table(),
            rollback_required: self.rollback_required(),
            disable_flash_boot: self.disable_flash_boot(),
            disable_otp_boot: self.disable_otp_boot(),
            enable_otp_boot: self.enable_otp_boot(),
            disable_power_scratch: self.disable_power_scratch(),
            disable_watchdog_scratch: self.disable_watchdog_scratch(),
            disable_bootsel_usb_msd_ifc: self.disable_bootsel_usb_msd_ifc(),
            disable_bootsel_usb_picoboot_ifc: self.disable_bootsel_usb_picoboot_ifc(),
            disable_bootsel_uart_boot: self.disable_bootsel_uart_boot(),
            disable_xip_access_on_sram_entry: self.disable_xip_access_on_sram_entry(),
            disable_sram_window_boot: self.disable_sram_window_boot(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for BootFlags1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BootFlags1")
            .field("key_valid", &self.key_valid())
            .field("key_invalid", &self.key_invalid())
            .field("double_tap_delay", &self.double_tap_delay())
            .field("double_tap", &self.double_tap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for BootFlags1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct BootFlags1 {
            key_valid: u8,
            key_invalid: u8,
            double_tap_delay: u8,
            double_tap: bool,
        }
        let proxy = BootFlags1 {
            key_valid: self.key_valid(),
            key_invalid: self.key_invalid(),
            double_tap_delay: self.double_tap_delay(),
            double_tap: self.double_tap(),
        };
        defmt::write!(f, "{}", proxy)
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
            activelow: u16,
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
            refdiv: u16,
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
impl core::fmt::Debug for Crit0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crit0")
            .field("arm_disable", &self.arm_disable())
            .field("riscv_disable", &self.riscv_disable())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crit0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crit0 {
            arm_disable: bool,
            riscv_disable: bool,
        }
        let proxy = Crit0 {
            arm_disable: self.arm_disable(),
            riscv_disable: self.riscv_disable(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Crit1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Crit1")
            .field("secure_boot_enable", &self.secure_boot_enable())
            .field("secure_debug_disable", &self.secure_debug_disable())
            .field("debug_disable", &self.debug_disable())
            .field("boot_arch", &self.boot_arch())
            .field("glitch_detector_enable", &self.glitch_detector_enable())
            .field("glitch_detector_sens", &self.glitch_detector_sens())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Crit1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Crit1 {
            secure_boot_enable: bool,
            secure_debug_disable: bool,
            debug_disable: bool,
            boot_arch: bool,
            glitch_detector_enable: bool,
            glitch_detector_sens: u8,
        }
        let proxy = Crit1 {
            secure_boot_enable: self.secure_boot_enable(),
            secure_debug_disable: self.secure_debug_disable(),
            debug_disable: self.debug_disable(),
            boot_arch: self.boot_arch(),
            glitch_detector_enable: self.glitch_detector_enable(),
            glitch_detector_sens: self.glitch_detector_sens(),
        };
        defmt::write!(f, "{}", proxy)
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
#[doc = "Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct KeyValid(pub u32);
impl KeyValid {
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
impl Default for KeyValid {
    #[inline(always)]
    fn default() -> KeyValid {
        KeyValid(0)
    }
}
impl core::fmt::Debug for KeyValid {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KeyValid")
            .field("valid", &self.valid())
            .field("valid_r1", &self.valid_r1())
            .field("valid_r2", &self.valid_r2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for KeyValid {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct KeyValid {
            valid: bool,
            valid_r1: bool,
            valid_r2: bool,
        }
        let proxy = KeyValid {
            valid: self.valid(),
            valid_r1: self.valid_r1(),
            valid_r2: self.valid_r2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Lock configuration LSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PageLock0(pub u32);
impl PageLock0 {
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
    pub const fn no_key_state(&self) -> super::vals::PageLockNoKeyState {
        let val = (self.0 >> 6usize) & 0x01;
        super::vals::PageLockNoKeyState::from_bits(val as u8)
    }
    #[doc = "State when at least one key is registered for this page and no matching key has been entered."]
    #[inline(always)]
    pub fn set_no_key_state(&mut self, val: super::vals::PageLockNoKeyState) {
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
impl Default for PageLock0 {
    #[inline(always)]
    fn default() -> PageLock0 {
        PageLock0(0)
    }
}
impl core::fmt::Debug for PageLock0 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PageLock0")
            .field("key_w", &self.key_w())
            .field("key_r", &self.key_r())
            .field("no_key_state", &self.no_key_state())
            .field("rma", &self.rma())
            .field("r1", &self.r1())
            .field("r2", &self.r2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PageLock0 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PageLock0 {
            key_w: u8,
            key_r: u8,
            no_key_state: super::vals::PageLockNoKeyState,
            rma: bool,
            r1: u8,
            r2: u8,
        }
        let proxy = PageLock0 {
            key_w: self.key_w(),
            key_r: self.key_r(),
            no_key_state: self.no_key_state(),
            rma: self.rma(),
            r1: self.r1(),
            r2: self.r2(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Lock configuration MSBs for page 63 (rows 0xfc0 through 0xfff). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PageLock1(pub u32);
impl PageLock1 {
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_s(&self) -> super::vals::PageLock {
        let val = (self.0 >> 0usize) & 0x03;
        super::vals::PageLock::from_bits(val as u8)
    }
    #[doc = "Lock state for Secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_s(&mut self, val: super::vals::PageLock) {
        self.0 = (self.0 & !(0x03 << 0usize)) | (((val.to_bits() as u32) & 0x03) << 0usize);
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub const fn lock_ns(&self) -> super::vals::PageLock {
        let val = (self.0 >> 2usize) & 0x03;
        super::vals::PageLock::from_bits(val as u8)
    }
    #[doc = "Lock state for Non-secure accesses to this page. Thermometer-coded, so lock state can be advanced permanently from any state to any less-permissive state by programming OTP. Software can also advance the lock state temporarily (until next OTP reset) using the SW_LOCKx registers. Note that READ_WRITE and READ_ONLY are equivalent in hardware, as the SBPI programming interface is not accessible to Non-secure software. However, Secure software may check these bits to apply write permissions to a Non-secure OTP programming API."]
    #[inline(always)]
    pub fn set_lock_ns(&mut self, val: super::vals::PageLock) {
        self.0 = (self.0 & !(0x03 << 2usize)) | (((val.to_bits() as u32) & 0x03) << 2usize);
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub const fn lock_bl(&self) -> super::vals::PageLock {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::PageLock::from_bits(val as u8)
    }
    #[doc = "Dummy lock bits reserved for bootloaders (including the RP2350 USB bootloader) to store their own OTP access permissions. No hardware effect, and no corresponding SW_LOCKx registers."]
    #[inline(always)]
    pub fn set_lock_bl(&mut self, val: super::vals::PageLock) {
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
impl Default for PageLock1 {
    #[inline(always)]
    fn default() -> PageLock1 {
        PageLock1(0)
    }
}
impl core::fmt::Debug for PageLock1 {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PageLock1")
            .field("lock_s", &self.lock_s())
            .field("lock_ns", &self.lock_ns())
            .field("lock_bl", &self.lock_bl())
            .field("r1", &self.r1())
            .field("r2", &self.r2())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for PageLock1 {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct PageLock1 {
            lock_s: super::vals::PageLock,
            lock_ns: super::vals::PageLock,
            lock_bl: super::vals::PageLock,
            r1: u8,
            r2: u8,
        }
        let proxy = PageLock1 {
            lock_s: self.lock_s(),
            lock_ns: self.lock_ns(),
            lock_bl: self.lock_bl(),
            r1: self.r1(),
            r2: self.r2(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for UsbBootFlags {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UsbBootFlags")
            .field(
                "wl_usb_device_vid_value_valid",
                &self.wl_usb_device_vid_value_valid(),
            )
            .field(
                "wl_usb_device_pid_value_valid",
                &self.wl_usb_device_pid_value_valid(),
            )
            .field(
                "wl_usb_device_serial_number_value_valid",
                &self.wl_usb_device_serial_number_value_valid(),
            )
            .field(
                "wl_usb_device_lang_id_value_valid",
                &self.wl_usb_device_lang_id_value_valid(),
            )
            .field(
                "wl_usb_device_manufacturer_strdef_valid",
                &self.wl_usb_device_manufacturer_strdef_valid(),
            )
            .field(
                "wl_usb_device_product_strdef_valid",
                &self.wl_usb_device_product_strdef_valid(),
            )
            .field(
                "wl_usb_device_serial_number_strdef_valid",
                &self.wl_usb_device_serial_number_strdef_valid(),
            )
            .field(
                "wl_usb_config_attributes_max_power_values_valid",
                &self.wl_usb_config_attributes_max_power_values_valid(),
            )
            .field(
                "wl_volume_label_strdef_valid",
                &self.wl_volume_label_strdef_valid(),
            )
            .field(
                "wl_scsi_inquiry_vendor_strdef_valid",
                &self.wl_scsi_inquiry_vendor_strdef_valid(),
            )
            .field(
                "wl_scsi_inquiry_product_strdef_valid",
                &self.wl_scsi_inquiry_product_strdef_valid(),
            )
            .field(
                "wl_scsi_inquiry_version_strdef_valid",
                &self.wl_scsi_inquiry_version_strdef_valid(),
            )
            .field(
                "wl_index_htm_redirect_url_strdef_valid",
                &self.wl_index_htm_redirect_url_strdef_valid(),
            )
            .field(
                "wl_index_htm_redirect_name_strdef_valid",
                &self.wl_index_htm_redirect_name_strdef_valid(),
            )
            .field(
                "wl_info_uf2_txt_model_strdef_valid",
                &self.wl_info_uf2_txt_model_strdef_valid(),
            )
            .field(
                "wl_info_uf2_txt_board_id_strdef_valid",
                &self.wl_info_uf2_txt_board_id_strdef_valid(),
            )
            .field("white_label_addr_valid", &self.white_label_addr_valid())
            .field("dp_dm_swap", &self.dp_dm_swap())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for UsbBootFlags {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct UsbBootFlags {
            wl_usb_device_vid_value_valid: bool,
            wl_usb_device_pid_value_valid: bool,
            wl_usb_device_serial_number_value_valid: bool,
            wl_usb_device_lang_id_value_valid: bool,
            wl_usb_device_manufacturer_strdef_valid: bool,
            wl_usb_device_product_strdef_valid: bool,
            wl_usb_device_serial_number_strdef_valid: bool,
            wl_usb_config_attributes_max_power_values_valid: bool,
            wl_volume_label_strdef_valid: bool,
            wl_scsi_inquiry_vendor_strdef_valid: bool,
            wl_scsi_inquiry_product_strdef_valid: bool,
            wl_scsi_inquiry_version_strdef_valid: bool,
            wl_index_htm_redirect_url_strdef_valid: bool,
            wl_index_htm_redirect_name_strdef_valid: bool,
            wl_info_uf2_txt_model_strdef_valid: bool,
            wl_info_uf2_txt_board_id_strdef_valid: bool,
            white_label_addr_valid: bool,
            dp_dm_swap: bool,
        }
        let proxy = UsbBootFlags {
            wl_usb_device_vid_value_valid: self.wl_usb_device_vid_value_valid(),
            wl_usb_device_pid_value_valid: self.wl_usb_device_pid_value_valid(),
            wl_usb_device_serial_number_value_valid: self.wl_usb_device_serial_number_value_valid(),
            wl_usb_device_lang_id_value_valid: self.wl_usb_device_lang_id_value_valid(),
            wl_usb_device_manufacturer_strdef_valid: self.wl_usb_device_manufacturer_strdef_valid(),
            wl_usb_device_product_strdef_valid: self.wl_usb_device_product_strdef_valid(),
            wl_usb_device_serial_number_strdef_valid: self
                .wl_usb_device_serial_number_strdef_valid(),
            wl_usb_config_attributes_max_power_values_valid: self
                .wl_usb_config_attributes_max_power_values_valid(),
            wl_volume_label_strdef_valid: self.wl_volume_label_strdef_valid(),
            wl_scsi_inquiry_vendor_strdef_valid: self.wl_scsi_inquiry_vendor_strdef_valid(),
            wl_scsi_inquiry_product_strdef_valid: self.wl_scsi_inquiry_product_strdef_valid(),
            wl_scsi_inquiry_version_strdef_valid: self.wl_scsi_inquiry_version_strdef_valid(),
            wl_index_htm_redirect_url_strdef_valid: self.wl_index_htm_redirect_url_strdef_valid(),
            wl_index_htm_redirect_name_strdef_valid: self.wl_index_htm_redirect_name_strdef_valid(),
            wl_info_uf2_txt_model_strdef_valid: self.wl_info_uf2_txt_model_strdef_valid(),
            wl_info_uf2_txt_board_id_strdef_valid: self.wl_info_uf2_txt_board_id_strdef_valid(),
            white_label_addr_valid: self.white_label_addr_valid(),
            dp_dm_swap: self.dp_dm_swap(),
        };
        defmt::write!(f, "{}", proxy)
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
