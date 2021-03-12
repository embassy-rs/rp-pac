use crate::generic::*;
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints(u32);
impl Ints {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Ints {
        Ints(val)
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub const fn fifo(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub fn set_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ints {
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "Result of most recent ADC conversion"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Result(u32);
impl Result {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Result {
        Result(val)
    }
    pub const fn result(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x0fff;
        val as u16
    }
    pub fn set_result(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val as u32) & 0x0fff) << 0u32);
    }
}
impl Default for Result {
    fn default() -> Result {
        Result(0)
    }
}
#[doc = "ADC Control and Status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Cs(u32);
impl Cs {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Cs {
        Cs(val)
    }
    #[doc = "Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion. The first channel to be sampled will be the one currently indicated by AINSEL. AINSEL will be updated after each conversion with the newly-selected channel."]
    pub const fn rrobin(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x1f;
        val as u8
    }
    #[doc = "Round-robin sampling. 1 bit per channel. Set all bits to 0 to disable. Otherwise, the ADC will cycle through each enabled channel in a round-robin fashion. The first channel to be sampled will be the one currently indicated by AINSEL. AINSEL will be updated after each conversion with the newly-selected channel."]
    pub fn set_rrobin(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16u32)) | (((val as u32) & 0x1f) << 16u32);
    }
    #[doc = "Select analog mux input. Updated automatically in round-robin mode."]
    pub const fn ainsel(&self) -> u8 {
        let val = (self.0 >> 12u32) & 0x07;
        val as u8
    }
    #[doc = "Select analog mux input. Updated automatically in round-robin mode."]
    pub fn set_ainsel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 12u32)) | (((val as u32) & 0x07) << 12u32);
    }
    #[doc = "Some past ADC conversion encountered an error. Write 1 to clear."]
    pub const fn err_sticky(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Some past ADC conversion encountered an error. Write 1 to clear."]
    pub fn set_err_sticky(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "The most recent ADC conversion encountered an error; result is undefined or noisy."]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "The most recent ADC conversion encountered an error; result is undefined or noisy."]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed. 0 whilst conversion in progress."]
    pub const fn ready(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "1 if the ADC is ready to start a new conversion. Implies any previous conversion has completed. 0 whilst conversion in progress."]
    pub fn set_ready(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    pub const fn start_many(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Continuously perform conversions whilst this bit is 1. A new conversion will start immediately after the previous finishes."]
    pub fn set_start_many(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    pub const fn start_once(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Start a single conversion. Self-clearing. Ignored if start_many is asserted."]
    pub fn set_start_once(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Power on temperature sensor. 1 - enabled. 0 - disabled."]
    pub const fn ts_en(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Power on temperature sensor. 1 - enabled. 0 - disabled."]
    pub fn set_ts_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Power on ADC and enable its clock. 1 - enabled. 0 - disabled."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Power on ADC and enable its clock. 1 - enabled. 0 - disabled."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Cs {
    fn default() -> Cs {
        Cs(0)
    }
}
#[doc = "FIFO control and status"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fcs(u32);
impl Fcs {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Fcs {
        Fcs(val)
    }
    #[doc = "DREQ/IRQ asserted when level >= threshold"]
    pub const fn thresh(&self) -> u8 {
        let val = (self.0 >> 24u32) & 0x0f;
        val as u8
    }
    #[doc = "DREQ/IRQ asserted when level >= threshold"]
    pub fn set_thresh(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24u32)) | (((val as u32) & 0x0f) << 24u32);
    }
    #[doc = "The number of conversion results currently waiting in the FIFO"]
    pub const fn level(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x0f;
        val as u8
    }
    #[doc = "The number of conversion results currently waiting in the FIFO"]
    pub fn set_level(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16u32)) | (((val as u32) & 0x0f) << 16u32);
    }
    #[doc = "1 if the FIFO has been overflowed. Write 1 to clear."]
    pub const fn over(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "1 if the FIFO has been overflowed. Write 1 to clear."]
    pub fn set_over(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "1 if the FIFO has been underflowed. Write 1 to clear."]
    pub const fn under(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "1 if the FIFO has been underflowed. Write 1 to clear."]
    pub fn set_under(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn full(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_full(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn empty(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_empty(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "If 1: assert DMA requests when FIFO contains data"]
    pub const fn dreq_en(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "If 1: assert DMA requests when FIFO contains data"]
    pub fn set_dreq_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "If 1: conversion error bit appears in the FIFO alongside the result"]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "If 1: conversion error bit appears in the FIFO alongside the result"]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    pub const fn shift(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "If 1: FIFO results are right-shifted to be one byte in size. Enables DMA to byte buffers."]
    pub fn set_shift(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "If 1: write result to the FIFO after each conversion."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "If 1: write result to the FIFO after each conversion."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Fcs {
    fn default() -> Fcs {
        Fcs(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte(u32);
impl Inte {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Inte {
        Inte(val)
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub const fn fifo(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub fn set_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Inte {
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Conversion result FIFO"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Fifo(u32);
impl Fifo {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Fifo {
        Fifo(val)
    }
    #[doc = "1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    pub const fn err(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "1 if this particular sample experienced a conversion error. Remains in the same location if the sample is shifted."]
    pub fn set_err(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn val(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x0fff;
        val as u16
    }
    pub fn set_val(&mut self, val: u16) {
        self.0 = (self.0 & !(0x0fff << 0u32)) | (((val as u32) & 0x0fff) << 0u32);
    }
}
impl Default for Fifo {
    fn default() -> Fifo {
        Fifo(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(u32);
impl Intr {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intr {
        Intr(val)
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub const fn fifo(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub fn set_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Clock divider. If non-zero, CS_START_MANY will start conversions at regular intervals rather than back-to-back. The divider is reset when either of these fields are written. Total period is 1 + INT + FRAC / 256"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Div(u32);
impl Div {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Div {
        Div(val)
    }
    #[doc = "Integer part of clock divisor."]
    pub const fn int(&self) -> u16 {
        let val = (self.0 >> 8u32) & 0xffff;
        val as u16
    }
    #[doc = "Integer part of clock divisor."]
    pub fn set_int(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 8u32)) | (((val as u32) & 0xffff) << 8u32);
    }
    #[doc = "Fractional part of clock divisor. First-order delta-sigma."]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0xff;
        val as u8
    }
    #[doc = "Fractional part of clock divisor. First-order delta-sigma."]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0u32)) | (((val as u32) & 0xff) << 0u32);
    }
}
impl Default for Div {
    fn default() -> Div {
        Div(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf(u32);
impl Intf {
    pub const fn to_bits(&self) -> u32 {
        self.0
    }
    pub const fn from_bits(val: u32) -> Intf {
        Intf(val)
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub const fn fifo(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Triggered when the sample FIFO reaches a certain level. This level can be programmed via the FCS_THRESH field."]
    pub fn set_fifo(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intf {
    fn default() -> Intf {
        Intf(0)
    }
}
