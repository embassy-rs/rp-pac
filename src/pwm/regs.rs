use crate::generic::*;
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4Top(pub u32);
impl Ch4Top {
    pub const fn ch4_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch4_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch4Top {
    fn default() -> Ch4Top {
        Ch4Top(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0Ctr(pub u32);
impl Ch0Ctr {
    pub const fn ch0_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch0_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch0Ctr {
    fn default() -> Ch0Ctr {
        Ch0Ctr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2Csr(pub u32);
impl Ch2Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch2Csr {
    fn default() -> Ch2Csr {
        Ch2Csr(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ints(pub u32);
impl Ints {
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ints {
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5Div(pub u32);
impl Ch5Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch5Div {
    fn default() -> Ch5Div {
        Ch5Div(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4Div(pub u32);
impl Ch4Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch4Div {
    fn default() -> Ch4Div {
        Ch4Div(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3Ctr(pub u32);
impl Ch3Ctr {
    pub const fn ch3_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch3_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch3Ctr {
    fn default() -> Ch3Ctr {
        Ch3Ctr(0)
    }
}
#[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct En(pub u32);
impl En {
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for En {
    fn default() -> En {
        En(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6Div(pub u32);
impl Ch6Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch6Div {
    fn default() -> Ch6Div {
        Ch6Div(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7Cc(pub u32);
impl Ch7Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch7Cc {
    fn default() -> Ch7Cc {
        Ch7Cc(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1Cc(pub u32);
impl Ch1Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch1Cc {
    fn default() -> Ch1Cc {
        Ch1Cc(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2Div(pub u32);
impl Ch2Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch2Div {
    fn default() -> Ch2Div {
        Ch2Div(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6Cc(pub u32);
impl Ch6Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch6Cc {
    fn default() -> Ch6Cc {
        Ch6Cc(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5Cc(pub u32);
impl Ch5Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch5Cc {
    fn default() -> Ch5Cc {
        Ch5Cc(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4Csr(pub u32);
impl Ch4Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch4Csr {
    fn default() -> Ch4Csr {
        Ch4Csr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1Csr(pub u32);
impl Ch1Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch1Csr {
    fn default() -> Ch1Csr {
        Ch1Csr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4Ctr(pub u32);
impl Ch4Ctr {
    pub const fn ch4_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch4_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch4Ctr {
    fn default() -> Ch4Ctr {
        Ch4Ctr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3Top(pub u32);
impl Ch3Top {
    pub const fn ch3_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch3_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch3Top {
    fn default() -> Ch3Top {
        Ch3Top(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5Ctr(pub u32);
impl Ch5Ctr {
    pub const fn ch5_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch5_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch5Ctr {
    fn default() -> Ch5Ctr {
        Ch5Ctr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6Csr(pub u32);
impl Ch6Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch6Csr {
    fn default() -> Ch6Csr {
        Ch6Csr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2Top(pub u32);
impl Ch2Top {
    pub const fn ch2_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch2_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch2Top {
    fn default() -> Ch2Top {
        Ch2Top(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7Csr(pub u32);
impl Ch7Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch7Csr {
    fn default() -> Ch7Csr {
        Ch7Csr(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3Cc(pub u32);
impl Ch3Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch3Cc {
    fn default() -> Ch3Cc {
        Ch3Cc(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7Top(pub u32);
impl Ch7Top {
    pub const fn ch7_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch7_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch7Top {
    fn default() -> Ch7Top {
        Ch7Top(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3Div(pub u32);
impl Ch3Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch3Div {
    fn default() -> Ch3Div {
        Ch3Div(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch4Cc(pub u32);
impl Ch4Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch4Cc {
    fn default() -> Ch4Cc {
        Ch4Cc(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0Csr(pub u32);
impl Ch0Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch0Csr {
    fn default() -> Ch0Csr {
        Ch0Csr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch3Csr(pub u32);
impl Ch3Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch3Csr {
    fn default() -> Ch3Csr {
        Ch3Csr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7Ctr(pub u32);
impl Ch7Ctr {
    pub const fn ch7_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch7_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch7Ctr {
    fn default() -> Ch7Ctr {
        Ch7Ctr(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch7Div(pub u32);
impl Ch7Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch7Div {
    fn default() -> Ch7Div {
        Ch7Div(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1Ctr(pub u32);
impl Ch1Ctr {
    pub const fn ch1_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch1_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch1Ctr {
    fn default() -> Ch1Ctr {
        Ch1Ctr(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0Cc(pub u32);
impl Ch0Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch0Cc {
    fn default() -> Ch0Cc {
        Ch0Cc(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1Top(pub u32);
impl Ch1Top {
    pub const fn ch1_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch1_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch1Top {
    fn default() -> Ch1Top {
        Ch1Top(0)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Inte(pub u32);
impl Inte {
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Inte {
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intr(pub u32);
impl Intr {
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intr {
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Intf(pub u32);
impl Intf {
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Intf {
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2Ctr(pub u32);
impl Ch2Ctr {
    pub const fn ch2_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch2_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch2Ctr {
    fn default() -> Ch2Ctr {
        Ch2Ctr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6Ctr(pub u32);
impl Ch6Ctr {
    pub const fn ch6_ctr(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch6_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch6Ctr {
    fn default() -> Ch6Ctr {
        Ch6Ctr(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch2Cc(pub u32);
impl Ch2Cc {
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0xffff;
        val as u16
    }
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16u32)) | (((val as u32) & 0xffff) << 16u32);
    }
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch2Cc {
    fn default() -> Ch2Cc {
        Ch2Cc(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0Div(pub u32);
impl Ch0Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch0Div {
    fn default() -> Ch0Div {
        Ch0Div(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch1Div(pub u32);
impl Ch1Div {
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4u32) & 0xff;
        val as u8
    }
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4u32)) | (((val as u32) & 0xff) << 4u32);
    }
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x0f;
        val as u8
    }
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0u32)) | (((val as u32) & 0x0f) << 0u32);
    }
}
impl Default for Ch1Div {
    fn default() -> Ch1Div {
        Ch1Div(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch0Top(pub u32);
impl Ch0Top {
    pub const fn ch0_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch0_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch0Top {
    fn default() -> Ch0Top {
        Ch0Top(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5Csr(pub u32);
impl Ch5Csr {
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4u32) & 0x03;
        super::vals::Divmode(val as u8)
    }
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4u32)) | (((val.0 as u32) & 0x03) << 4u32);
    }
    #[doc = "Invert output B"]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Invert output A"]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the PWM channel."]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Ch5Csr {
    fn default() -> Ch5Csr {
        Ch5Csr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch6Top(pub u32);
impl Ch6Top {
    pub const fn ch6_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch6_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch6Top {
    fn default() -> Ch6Top {
        Ch6Top(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Ch5Top(pub u32);
impl Ch5Top {
    pub const fn ch5_top(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0xffff;
        val as u16
    }
    pub fn set_ch5_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0u32)) | (((val as u32) & 0xffff) << 0u32);
    }
}
impl Default for Ch5Top {
    fn default() -> Ch5Top {
        Ch5Top(0)
    }
}
