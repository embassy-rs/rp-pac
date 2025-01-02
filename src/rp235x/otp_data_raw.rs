#[derive(Copy, Clone, Eq, PartialEq)]
pub struct BootKey {
    ptr: *mut u8,
}
unsafe impl Send for BootKey {}
unsafe impl Sync for BootKey {}
impl BootKey {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 15:0 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Bits 31:16 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 47:32 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Bits 63:48 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Bits 79:64 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 95:80 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Bits 111:96 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Bits 127:112 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Bits 143:128 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part8(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Bits 159:144 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part9(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Bits 175:160 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part10(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Bits 191:176 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part11(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Bits 207:192 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part12(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x30usize) as _) }
    }
    #[doc = "Bits 223:208 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part13(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x34usize) as _) }
    }
    #[doc = "Bits 239:224 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part14(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x38usize) as _) }
    }
    #[doc = "Bits 255:240 of SHA-256 hash of boot key 3 (ECC)"]
    #[inline(always)]
    pub const fn part15(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3cusize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Key {
    ptr: *mut u8,
}
unsafe impl Send for Key {}
unsafe impl Sync for Key {}
impl Key {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Bits 15:0 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Bits 31:16 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 47:32 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Bits 63:48 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Bits 79:64 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 95:80 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Bits 111:96 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Bits 127:112 of OTP access key 6 (ECC)"]
    #[inline(always)]
    pub const fn part7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
}
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
    pub const fn chipid0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Bits 31:16 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
    #[doc = "Bits 47:32 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x08usize) as _) }
    }
    #[doc = "Bits 63:48 of public device ID (ECC)"]
    #[inline(always)]
    pub const fn chipid3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0cusize) as _) }
    }
    #[doc = "Bits 15:0 of private per-device random number (ECC) The RANDID0..7 rows form a 128-bit random number generated during device test. This ID is not exposed through the USB PICOBOOT GET_INFO command or the ROM `get_sys_info()` API. However note that the USB PICOBOOT OTP access point can read the entirety of page 0, so this value is not meaningfully private unless the USB PICOBOOT interface is disabled via the DISABLE_BOOTSEL_USB_PICOBOOT_IFC flag in BOOT_FLAGS0."]
    #[inline(always)]
    pub const fn randid0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x10usize) as _) }
    }
    #[doc = "Bits 31:16 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x14usize) as _) }
    }
    #[doc = "Bits 47:32 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x18usize) as _) }
    }
    #[doc = "Bits 63:48 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x1cusize) as _) }
    }
    #[doc = "Bits 79:64 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x20usize) as _) }
    }
    #[doc = "Bits 95:80 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x24usize) as _) }
    }
    #[doc = "Bits 111:96 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x28usize) as _) }
    }
    #[doc = "Bits 127:112 of private per-device random number (ECC)"]
    #[inline(always)]
    pub const fn randid7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x2cusize) as _) }
    }
    #[doc = "Ring oscillator frequency in kHz, measured during manufacturing (ECC) This is measured at 1.1 V, at room temperature, with the ROSC configuration registers in their reset state."]
    #[inline(always)]
    pub const fn rosc_calib(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x40usize) as _) }
    }
    #[doc = "Low-power oscillator frequency in Hz, measured during manufacturing (ECC) This is measured at 1.1V, at room temperature, with the LPOSC trim register in its reset state."]
    #[inline(always)]
    pub const fn lposc_calib(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x44usize) as _) }
    }
    #[doc = "The number of main user GPIOs (bank 0). Should read 48 in the QFN80 package, and 30 in the QFN60 package. (ECC)"]
    #[inline(always)]
    pub const fn num_gpios(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x60usize) as _) }
    }
    #[doc = "Lower 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (polynomial 0x4c11db7, input reflected, output reflected, seed all-ones, final XOR all-ones) (ECC)"]
    #[inline(always)]
    pub const fn info_crc0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xd8usize) as _) }
    }
    #[doc = "Upper 16 bits of CRC32 of OTP addresses 0x00 through 0x6b (ECC)"]
    #[inline(always)]
    pub const fn info_crc1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xdcusize) as _) }
    }
    #[doc = "Page 0 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit0(self) -> crate::common::Reg<regs::Crit0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe0usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe4usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xe8usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xecusize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf0usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf4usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xf8usize) as _) }
    }
    #[doc = "Redundant copy of CRIT0"]
    #[inline(always)]
    pub const fn crit0_r7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0xfcusize) as _) }
    }
    #[doc = "Page 1 critical boot flags (RBIT-8)"]
    #[inline(always)]
    pub const fn crit1(self) -> crate::common::Reg<regs::Crit1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0100usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0104usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0108usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r3(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x010cusize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r4(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0110usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r5(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0114usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r6(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0118usize) as _) }
    }
    #[doc = "Redundant copy of CRIT1"]
    #[inline(always)]
    pub const fn crit1_r7(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x011cusize) as _) }
    }
    #[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags0(self) -> crate::common::Reg<regs::BootFlags0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0120usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0124usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS0"]
    #[inline(always)]
    pub const fn boot_flags0_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0128usize) as _) }
    }
    #[doc = "Disable/Enable boot paths/features in the RP2350 mask ROM. Disables always supersede enables. Enables are provided where there are other configurations in OTP that must be valid. (RBIT-3)"]
    #[inline(always)]
    pub const fn boot_flags1(self) -> crate::common::Reg<regs::BootFlags1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x012cusize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0130usize) as _) }
    }
    #[doc = "Redundant copy of BOOT_FLAGS1"]
    #[inline(always)]
    pub const fn boot_flags1_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0134usize) as _) }
    }
    #[doc = "Default boot version thermometer counter, bits 23:0 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0138usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x013cusize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION0"]
    #[inline(always)]
    pub const fn default_boot_version0_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0140usize) as _) }
    }
    #[doc = "Default boot version thermometer counter, bits 47:24 (RBIT-3)"]
    #[inline(always)]
    pub const fn default_boot_version1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0144usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0148usize) as _) }
    }
    #[doc = "Redundant copy of DEFAULT_BOOT_VERSION1"]
    #[inline(always)]
    pub const fn default_boot_version1_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x014cusize) as _) }
    }
    #[doc = "Stores information about external flash device(s). (ECC) Assumed to be valid if BOOT_FLAGS0_FLASH_DEVINFO_ENABLE is set."]
    #[inline(always)]
    pub const fn flash_devinfo(self) -> crate::common::Reg<regs::FlashDevinfo, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0150usize) as _) }
    }
    #[doc = "Gap between partition table slot 0 and slot 1 at the start of flash (the default size is 4096 bytes) (ECC) Enabled by the OVERRIDE_FLASH_PARTITION_SLOT_SIZE bit in BOOT_FLAGS, the size is 4096 * (value + 1)"]
    #[inline(always)]
    pub const fn flash_partition_slot_size(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0154usize) as _) }
    }
    #[doc = "Pin configuration for LED status, used by USB bootloader. (ECC) Must be valid if BOOT_FLAGS0_ENABLE_BOOTSEL_LED is set."]
    #[inline(always)]
    pub const fn bootsel_led_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselLedCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0158usize) as _) }
    }
    #[doc = "Optional PLL configuration for BOOTSEL mode. (ECC) This should be configured to produce an exact 48 MHz based on the crystal oscillator frequency. User mode software may also use this value to calculate the expected crystal frequency based on an assumed 48 MHz PLL output. If no configuration is given, the crystal is assumed to be 12 MHz. The PLL frequency can be calculated as: PLL out = (XOSC frequency / (REFDIV+1)) x FBDIV / (POSTDIV1 x POSTDIV2) Conversely the crystal frequency can be calculated as: XOSC frequency = 48 MHz x (REFDIV+1) x (POSTDIV1 x POSTDIV2) / FBDIV (Note the +1 on REFDIV is because the value stored in this OTP location is the actual divisor value minus one.) Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_XOSC_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_pll_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselPllCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x015cusize) as _) }
    }
    #[doc = "Non-default crystal oscillator configuration for the USB bootloader. (ECC) These values may also be used by user code configuring the crystal oscillator. Used if and only if ENABLE_BOOTSEL_NON_DEFAULT_PLL_XOSC_CFG is set in BOOT_FLAGS0. That bit should be set only after this row and BOOTSEL_PLL_CFG are both correctly programmed."]
    #[inline(always)]
    pub const fn bootsel_xosc_cfg(
        self,
    ) -> crate::common::Reg<regs::BootselXoscCfg, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0160usize) as _) }
    }
    #[doc = "USB boot specific feature flags (RBIT-3)"]
    #[inline(always)]
    pub const fn usb_boot_flags(self) -> crate::common::Reg<regs::UsbBootFlags, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0164usize) as _) }
    }
    #[doc = "Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0168usize) as _) }
    }
    #[doc = "Redundant copy of USB_BOOT_FLAGS"]
    #[inline(always)]
    pub const fn usb_boot_flags_r2(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x016cusize) as _) }
    }
    #[doc = "Row index of the USB_WHITE_LABEL structure within OTP (ECC) The table has 16 rows, each of which are also ECC and marked valid by the corresponding valid bit in USB_BOOT_FLAGS (ECC). The entries are either _VALUEs where the 16 bit value is used as is, or _STRDEFs which acts as a pointers to a string value. The value stored in a _STRDEF is two separate bytes: The low seven bits of the first (LSB) byte indicates the number of characters in the string, and the top bit of the first (LSB) byte if set to indicate that each character in the string is two bytes (Unicode) versus one byte if unset. The second (MSB) byte represents the location of the string data, and is encoded as the number of rows from this USB_WHITE_LABEL_ADDR; i.e. the row of the start of the string is USB_WHITE_LABEL_ADDR value + msb_byte. In each case, the corresponding valid bit enables replacing the default value for the corresponding item provided by the boot rom. Note that Unicode _STRDEFs are only supported for USB_DEVICE_PRODUCT_STRDEF, USB_DEVICE_SERIAL_NUMBER_STRDEF and USB_DEVICE_MANUFACTURER_STRDEF. Unicode values will be ignored if specified for other fields, and non-unicode values for these three items will be converted to Unicode characters by setting the upper 8 bits to zero. Note that if the USB_WHITE_LABEL structure or the corresponding strings are not readable by BOOTSEL mode based on OTP permissions, or if alignment requirements are not met, then the corresponding default values are used. The index values indicate where each field is located (row USB_WHITE_LABEL_ADDR value + index):"]
    #[inline(always)]
    pub const fn usb_white_label_addr(
        self,
    ) -> crate::common::Reg<regs::UsbWhiteLabelAddr, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0170usize) as _) }
    }
    #[doc = "OTP start row for the OTP boot image. (ECC) If OTP boot is enabled, the bootrom will load from this location into SRAM and then directly enter the loaded image. Note that the image must be signed if SECURE_BOOT_ENABLE is set. The image itself is assumed to be ECC-protected. This must be an even number. Equivalently, the OTP boot image must start at a word-aligned location in the ECC read data address window."]
    #[inline(always)]
    pub const fn otpboot_src(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0178usize) as _) }
    }
    #[doc = "Length in rows of the OTP boot image. (ECC) OTPBOOT_LEN must be even. The total image size must be a multiple of 4 bytes (32 bits)."]
    #[inline(always)]
    pub const fn otpboot_len(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x017cusize) as _) }
    }
    #[doc = "Bits 15:0 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst0(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0180usize) as _) }
    }
    #[doc = "Bits 31:16 of the OTP boot image load destination (and entry point). (ECC) This must be a location in main SRAM (main SRAM is addresses 0x20000000 through 0x20082000) and must be word-aligned."]
    #[inline(always)]
    pub const fn otpboot_dst1(self) -> crate::common::Reg<u32, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0184usize) as _) }
    }
    #[inline(always)]
    pub const fn bootkey(self, n: usize) -> BootKey {
        assert!(n < 4usize);
        unsafe { BootKey::from_ptr(self.ptr.add(0x0200usize + n * 64usize) as _) }
    }
    #[inline(always)]
    pub const fn key(self, n: usize) -> Key {
        assert!(n < 6usize);
        unsafe { Key::from_ptr(self.ptr.add(0x3d20usize + n * 32usize) as _) }
    }
    #[doc = "Valid flag for key 1. Once the valid flag is set, the key can no longer be read or written, and becomes a valid fixed key for protecting OTP pages."]
    #[inline(always)]
    pub const fn key_valid(
        self,
        n: usize,
    ) -> crate::common::Reg<regs::KeyValid, crate::common::RW> {
        assert!(n < 6usize);
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x3de4usize + n * 4usize) as _) }
    }
    #[inline(always)]
    pub const fn page_lock(self, n: usize) -> PageLock {
        assert!(n < 64usize);
        unsafe { PageLock::from_ptr(self.ptr.add(0x3e00usize + n * 8usize) as _) }
    }
}
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct PageLock {
    ptr: *mut u8,
}
unsafe impl Send for PageLock {}
unsafe impl Sync for PageLock {}
impl PageLock {
    #[inline(always)]
    pub const unsafe fn from_ptr(ptr: *mut ()) -> Self {
        Self { ptr: ptr as _ }
    }
    #[inline(always)]
    pub const fn as_ptr(&self) -> *mut () {
        self.ptr as _
    }
    #[doc = "Lock configuration LSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn lock0(self) -> crate::common::Reg<regs::PageLock0, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x0usize) as _) }
    }
    #[doc = "Lock configuration MSBs for page 9 (rows 0x240 through 0x27f). Locks are stored with 3-way majority vote encoding, so that bits can be set independently. This OTP location is always readable, and is write-protected by its own permissions."]
    #[inline(always)]
    pub const fn lock1(self) -> crate::common::Reg<regs::PageLock1, crate::common::RW> {
        unsafe { crate::common::Reg::from_ptr(self.ptr.add(0x04usize) as _) }
    }
}
pub mod regs;
pub mod vals;
