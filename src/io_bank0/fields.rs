use crate::generic::*;
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio28Status(u32);
impl Gpio28Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio28Status {
        Gpio28Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio23Status(u32);
impl Gpio23Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio23Status {
        Gpio23Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio15Status(u32);
impl Gpio15Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio15Status {
        Gpio15Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints3(u32);
impl Proc0Ints3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Ints3 {
        Proc0Ints3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf1(u32);
impl DormantWakeIntf1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeIntf1 {
        DormantWakeIntf1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts3(u32);
impl DormantWakeInts3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInts3 {
        DormantWakeInts3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints2(u32);
impl Proc0Ints2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Ints2 {
        Proc0Ints2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio2Ctrl(u32);
impl Gpio2Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio2Ctrl {
        Gpio2Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio2CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio2CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio2CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio7Ctrl(u32);
impl Gpio7Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio7Ctrl {
        Gpio7Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio7CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio7CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio7CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio22Status(u32);
impl Gpio22Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio22Status {
        Gpio22Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio21Status(u32);
impl Gpio21Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio21Status {
        Gpio21Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio22Ctrl(u32);
impl Gpio22Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio22Ctrl {
        Gpio22Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio22CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio22CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio22CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio29Ctrl(u32);
impl Gpio29Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio29Ctrl {
        Gpio29Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio29CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio29CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio29CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte3(u32);
impl Proc0Inte3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Inte3 {
        Proc0Inte3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte0(u32);
impl DormantWakeInte0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInte0 {
        DormantWakeInte0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio10Status(u32);
impl Gpio10Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio10Status {
        Gpio10Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf0(u32);
impl Proc1Intf0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Intf0 {
        Proc1Intf0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio16Status(u32);
impl Gpio16Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio16Status {
        Gpio16Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte1(u32);
impl DormantWakeInte1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInte1 {
        DormantWakeInte1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio26Ctrl(u32);
impl Gpio26Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio26Ctrl {
        Gpio26Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio26CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio26CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio26CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints2(u32);
impl Proc1Ints2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Ints2 {
        Proc1Ints2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte1(u32);
impl Proc1Inte1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Inte1 {
        Proc1Inte1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte2(u32);
impl Proc1Inte2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Inte2 {
        Proc1Inte2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte2(u32);
impl Proc0Inte2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Inte2 {
        Proc0Inte2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio18Ctrl(u32);
impl Gpio18Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio18Ctrl {
        Gpio18Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio18CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio18CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio18CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte3(u32);
impl DormantWakeInte3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInte3 {
        DormantWakeInte3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio8Status(u32);
impl Gpio8Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio8Status {
        Gpio8Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio24Ctrl(u32);
impl Gpio24Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio24Ctrl {
        Gpio24Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio24CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio24CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio24CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf3(u32);
impl Proc1Intf3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Intf3 {
        Proc1Intf3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio5Status(u32);
impl Gpio5Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio5Status {
        Gpio5Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio0Status(u32);
impl Gpio0Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio0Status {
        Gpio0Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio1Status(u32);
impl Gpio1Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio1Status {
        Gpio1Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints1(u32);
impl Proc1Ints1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Ints1 {
        Proc1Ints1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints0(u32);
impl Proc1Ints0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Ints0 {
        Proc1Ints0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr1(u32);
impl Intr1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr1 {
        Intr1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio12Ctrl(u32);
impl Gpio12Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio12Ctrl {
        Gpio12Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio12CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio12CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio12CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints1(u32);
impl Proc0Ints1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Ints1 {
        Proc0Ints1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio3Status(u32);
impl Gpio3Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio3Status {
        Gpio3Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio13Status(u32);
impl Gpio13Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio13Status {
        Gpio13Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio15Ctrl(u32);
impl Gpio15Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio15Ctrl {
        Gpio15Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio15CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio15CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio15CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio14Ctrl(u32);
impl Gpio14Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio14Ctrl {
        Gpio14Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio14CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio14CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio14CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio8Ctrl(u32);
impl Gpio8Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio8Ctrl {
        Gpio8Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio8CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio8CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio8CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Ints0(u32);
impl Proc0Ints0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Ints0 {
        Proc0Ints0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf0(u32);
impl DormantWakeIntf0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeIntf0 {
        DormantWakeIntf0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio19Status(u32);
impl Gpio19Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio19Status {
        Gpio19Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio24Status(u32);
impl Gpio24Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio24Status {
        Gpio24Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio5Ctrl(u32);
impl Gpio5Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio5Ctrl {
        Gpio5Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio5CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio5CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio5CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio23Ctrl(u32);
impl Gpio23Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio23Ctrl {
        Gpio23Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio23CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio23CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio23CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf2(u32);
impl DormantWakeIntf2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeIntf2 {
        DormantWakeIntf2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio9Ctrl(u32);
impl Gpio9Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio9Ctrl {
        Gpio9Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio9CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio9CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio9CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte0(u32);
impl Proc1Inte0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Inte0 {
        Proc1Inte0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio29Status(u32);
impl Gpio29Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio29Status {
        Gpio29Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio2Status(u32);
impl Gpio2Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio2Status {
        Gpio2Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio17Ctrl(u32);
impl Gpio17Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio17Ctrl {
        Gpio17Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio17CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio17CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio17CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr2(u32);
impl Intr2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr2 {
        Intr2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf2(u32);
impl Proc0Intf2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Intf2 {
        Proc0Intf2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio17Status(u32);
impl Gpio17Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio17Status {
        Gpio17Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio10Ctrl(u32);
impl Gpio10Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio10Ctrl {
        Gpio10Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio10CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio10CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio10CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr3(u32);
impl Intr3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr3 {
        Intr3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio20Ctrl(u32);
impl Gpio20Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio20Ctrl {
        Gpio20Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio20CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio20CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio20CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Inte3(u32);
impl Proc1Inte3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Inte3 {
        Proc1Inte3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio18Status(u32);
impl Gpio18Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio18Status {
        Gpio18Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts2(u32);
impl DormantWakeInts2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInts2 {
        DormantWakeInts2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Ints3(u32);
impl Proc1Ints3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Ints3 {
        Proc1Ints3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio27Ctrl(u32);
impl Gpio27Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio27Ctrl {
        Gpio27Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio27CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio27CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio27CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio21Ctrl(u32);
impl Gpio21Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio21Ctrl {
        Gpio21Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio21CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio21CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio21CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio4Status(u32);
impl Gpio4Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio4Status {
        Gpio4Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf0(u32);
impl Proc0Intf0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Intf0 {
        Proc0Intf0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio28Ctrl(u32);
impl Gpio28Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio28Ctrl {
        Gpio28Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio28CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio28CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio28CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Enable for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInte2(u32);
impl DormantWakeInte2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInte2 {
        DormantWakeInte2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio11Status(u32);
impl Gpio11Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio11Status {
        Gpio11Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte0(u32);
impl Proc0Inte0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Inte0 {
        Proc0Inte0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr0(u32);
impl Intr0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr0 {
        Intr0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio12Status(u32);
impl Gpio12Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio12Status {
        Gpio12Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio11Ctrl(u32);
impl Gpio11Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio11Ctrl {
        Gpio11Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio11CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio11CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio11CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf1(u32);
impl Proc1Intf1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Intf1 {
        Proc1Intf1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf1(u32);
impl Proc0Intf1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Intf1 {
        Proc0Intf1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio6Ctrl(u32);
impl Gpio6Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio6Ctrl {
        Gpio6Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio6CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio6CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio6CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Force for proc1"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc1Intf2(u32);
impl Proc1Intf2 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc1Intf2 {
        Proc1Intf2(val)
    }
    pub const fn gpio23_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio23_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio23_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio23_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio23_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio22_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio22_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio22_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio22_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio22_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio21_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio21_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio21_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio21_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio21_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio20_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio20_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio20_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio20_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio20_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio19_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio19_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio19_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio19_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio19_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio18_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio18_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio18_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio18_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio18_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio17_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio17_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio17_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio17_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio17_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio16_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio16_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio16_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio16_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio16_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio16Ctrl(u32);
impl Gpio16Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio16Ctrl {
        Gpio16Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio16CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio16CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio16CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio20Status(u32);
impl Gpio20Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio20Status {
        Gpio20Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio3Ctrl(u32);
impl Gpio3Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio3Ctrl {
        Gpio3Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio3CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio3CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio3CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio1Ctrl(u32);
impl Gpio1Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio1Ctrl {
        Gpio1Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio1CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio1CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio1CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio25Status(u32);
impl Gpio25Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio25Status {
        Gpio25Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio14Status(u32);
impl Gpio14Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio14Status {
        Gpio14Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio27Status(u32);
impl Gpio27Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio27Status {
        Gpio27Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio6Status(u32);
impl Gpio6Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio6Status {
        Gpio6Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio26Status(u32);
impl Gpio26Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio26Status {
        Gpio26Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt Force for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Intf3(u32);
impl Proc0Intf3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Intf3 {
        Proc0Intf3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts1(u32);
impl DormantWakeInts1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInts1 {
        DormantWakeInts1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio9Status(u32);
impl Gpio9Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio9Status {
        Gpio9Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "Interrupt status after masking & forcing for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeInts0(u32);
impl DormantWakeInts0 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeInts0 {
        DormantWakeInts0(val)
    }
    pub const fn gpio7_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio7_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio7_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio7_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio7_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio6_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio6_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio6_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio6_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio6_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio5_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio5_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio5_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio5_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio5_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio4_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio4_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio4_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio4_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio4_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio3_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio3_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio3_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio3_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio3_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio2_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio2_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio2_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio2_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio2_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio1_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio1_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio1_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio1_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio1_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio0_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio0_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio0_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio0_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio0_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "Interrupt Enable for proc0"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Proc0Inte1(u32);
impl Proc0Inte1 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Proc0Inte1 {
        Proc0Inte1(val)
    }
    pub const fn gpio15_edge_high(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn gpio15_edge_low(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn gpio15_level_high(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn gpio15_level_low(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_gpio15_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn gpio14_edge_high(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn gpio14_edge_low(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn gpio14_level_high(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn gpio14_level_low(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_gpio14_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn gpio13_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio13_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio13_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio13_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio13_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio12_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio12_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio12_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio12_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio12_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio11_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio11_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio11_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio11_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio11_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio10_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio10_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio10_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio10_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio10_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio9_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio9_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio9_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio9_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio9_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio8_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio8_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio8_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio8_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio8_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio7Status(u32);
impl Gpio7Status {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio7Status {
        Gpio7Status(val)
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub const fn irqtoproc(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt to processors, after override is applied"]
    pub fn set_irqtoproc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "interrupt from pad before override is applied"]
    pub const fn irqfrompad(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "interrupt from pad before override is applied"]
    pub fn set_irqfrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub const fn intoperi(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "input signal to peripheral, after override is applied"]
    pub fn set_intoperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "input signal from pad, before override is applied"]
    pub const fn infrompad(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "input signal from pad, before override is applied"]
    pub fn set_infrompad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "output enable to pad after register override is applied"]
    pub const fn oetopad(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "output enable to pad after register override is applied"]
    pub fn set_oetopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub const fn oefromperi(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "output enable from selected peripheral, before register override is applied"]
    pub fn set_oefromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "output signal to pad after register override is applied"]
    pub const fn outtopad(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "output signal to pad after register override is applied"]
    pub fn set_outtopad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub const fn outfromperi(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "output signal from selected peripheral, before register override is applied"]
    pub fn set_outfromperi(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio13Ctrl(u32);
impl Gpio13Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio13Ctrl {
        Gpio13Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio13CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio13CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio13CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio19Ctrl(u32);
impl Gpio19Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio19Ctrl {
        Gpio19Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio19CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio19CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio19CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "Interrupt Force for dormant_wake"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct DormantWakeIntf3(u32);
impl DormantWakeIntf3 {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> DormantWakeIntf3 {
        DormantWakeIntf3(val)
    }
    pub const fn gpio29_edge_high(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn gpio29_edge_low(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn gpio29_level_high(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn gpio29_level_low(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_gpio29_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn gpio28_edge_high(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn gpio28_edge_low(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn gpio28_level_high(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn gpio28_level_low(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_gpio28_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn gpio27_edge_high(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn gpio27_edge_low(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn gpio27_level_high(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn gpio27_level_low(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_gpio27_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn gpio26_edge_high(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn gpio26_edge_low(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn gpio26_level_high(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn gpio26_level_low(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_gpio26_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn gpio25_edge_high(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn gpio25_edge_low(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn gpio25_level_high(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn gpio25_level_low(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_gpio25_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn gpio24_edge_high(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn gpio24_edge_low(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_edge_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn gpio24_level_high(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_high(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn gpio24_level_low(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_gpio24_level_low(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio25Ctrl(u32);
impl Gpio25Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio25Ctrl {
        Gpio25Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio25CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio25CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio25CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio4Ctrl(u32);
impl Gpio4Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio4Ctrl {
        Gpio4Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio4CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio4CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio4CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
#[doc = "GPIO control including function select and overrides."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Gpio0Ctrl(u32);
impl Gpio0Ctrl {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Gpio0Ctrl {
        Gpio0Ctrl(val)
    }
    pub const fn irqover(&self) -> super::super::io::values::Irqover {
        let val = (self.0 >> 28u32) & 0x03;
        super::super::io::values::Irqover::from_bits(val as u8)
    }
    pub fn set_irqover(&mut self, val: super::super::io::values::Irqover) {
        self.0 = (self.0 & !(0x03 << 28u32)) | (((val.to_bits() as u32) & 0x03) << 28u32);
    }
    pub const fn inover(&self) -> super::super::io::values::Inover {
        let val = (self.0 >> 16u32) & 0x03;
        super::super::io::values::Inover::from_bits(val as u8)
    }
    pub fn set_inover(&mut self, val: super::super::io::values::Inover) {
        self.0 = (self.0 & !(0x03 << 16u32)) | (((val.to_bits() as u32) & 0x03) << 16u32);
    }
    pub const fn oeover(&self) -> super::super::io::values::Oeover {
        let val = (self.0 >> 12u32) & 0x03;
        super::super::io::values::Oeover::from_bits(val as u8)
    }
    pub fn set_oeover(&mut self, val: super::super::io::values::Oeover) {
        self.0 = (self.0 & !(0x03 << 12u32)) | (((val.to_bits() as u32) & 0x03) << 12u32);
    }
    pub const fn outover(&self) -> super::super::io::values::Outover {
        let val = (self.0 >> 8u32) & 0x03;
        super::super::io::values::Outover::from_bits(val as u8)
    }
    pub fn set_outover(&mut self, val: super::super::io::values::Outover) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val.to_bits() as u32) & 0x03) << 8u32);
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub const fn funcsel(&self) -> super::values::Gpio0CtrlFuncsel {
        let val = (self.0 >> 0u32) & 0x1f;
        super::values::Gpio0CtrlFuncsel::from_bits(val as u8)
    }
    #[doc = "0-31 -> selects pin function according to the gpio table 31 == NULL"]
    pub fn set_funcsel(&mut self, val: super::values::Gpio0CtrlFuncsel) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val.to_bits() as u32) & 0x1f) << 0u32);
    }
}
