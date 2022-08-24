#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Inte(pub u32);
impl Inte {
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Inte {
    #[inline(always)]
    fn default() -> Inte {
        Inte(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5ctr(pub u32);
impl Ch5ctr {
    #[inline(always)]
    pub const fn ch5_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch5_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch5ctr {
    #[inline(always)]
    fn default() -> Ch5ctr {
        Ch5ctr(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6cc(pub u32);
impl Ch6cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch6cc {
    #[inline(always)]
    fn default() -> Ch6cc {
        Ch6cc(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6top(pub u32);
impl Ch6top {
    #[inline(always)]
    pub const fn ch6_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch6_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch6top {
    #[inline(always)]
    fn default() -> Ch6top {
        Ch6top(0)
    }
}
#[doc = "Interrupt status after masking & forcing"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ints(pub u32);
impl Ints {
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ints {
    #[inline(always)]
    fn default() -> Ints {
        Ints(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5cc(pub u32);
impl Ch5cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch5cc {
    #[inline(always)]
    fn default() -> Ch5cc {
        Ch5cc(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1div(pub u32);
impl Ch1div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch1div {
    #[inline(always)]
    fn default() -> Ch1div {
        Ch1div(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0cc(pub u32);
impl Ch0cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch0cc {
    #[inline(always)]
    fn default() -> Ch0cc {
        Ch0cc(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4div(pub u32);
impl Ch4div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch4div {
    #[inline(always)]
    fn default() -> Ch4div {
        Ch4div(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7ctr(pub u32);
impl Ch7ctr {
    #[inline(always)]
    pub const fn ch7_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch7_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch7ctr {
    #[inline(always)]
    fn default() -> Ch7ctr {
        Ch7ctr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2ctr(pub u32);
impl Ch2ctr {
    #[inline(always)]
    pub const fn ch2_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch2_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2ctr {
    #[inline(always)]
    fn default() -> Ch2ctr {
        Ch2ctr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3top(pub u32);
impl Ch3top {
    #[inline(always)]
    pub const fn ch3_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch3_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3top {
    #[inline(always)]
    fn default() -> Ch3top {
        Ch3top(0)
    }
}
#[doc = "This register aliases the CSR_EN bits for all channels. Writing to this register allows multiple channels to be enabled or disabled simultaneously, so they can run in perfect sync. For each channel, there is only one physical EN register bit, which can be accessed through here or CHx_CSR."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct En(pub u32);
impl En {
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for En {
    #[inline(always)]
    fn default() -> En {
        En(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1csr(pub u32);
impl Ch1csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch1csr {
    #[inline(always)]
    fn default() -> Ch1csr {
        Ch1csr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4ctr(pub u32);
impl Ch4ctr {
    #[inline(always)]
    pub const fn ch4_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch4_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch4ctr {
    #[inline(always)]
    fn default() -> Ch4ctr {
        Ch4ctr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6csr(pub u32);
impl Ch6csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch6csr {
    #[inline(always)]
    fn default() -> Ch6csr {
        Ch6csr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7csr(pub u32);
impl Ch7csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch7csr {
    #[inline(always)]
    fn default() -> Ch7csr {
        Ch7csr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3ctr(pub u32);
impl Ch3ctr {
    #[inline(always)]
    pub const fn ch3_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch3_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch3ctr {
    #[inline(always)]
    fn default() -> Ch3ctr {
        Ch3ctr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0csr(pub u32);
impl Ch0csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch0csr {
    #[inline(always)]
    fn default() -> Ch0csr {
        Ch0csr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0top(pub u32);
impl Ch0top {
    #[inline(always)]
    pub const fn ch0_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch0_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0top {
    #[inline(always)]
    fn default() -> Ch0top {
        Ch0top(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0div(pub u32);
impl Ch0div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch0div {
    #[inline(always)]
    fn default() -> Ch0div {
        Ch0div(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4cc(pub u32);
impl Ch4cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch4cc {
    #[inline(always)]
    fn default() -> Ch4cc {
        Ch4cc(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5div(pub u32);
impl Ch5div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch5div {
    #[inline(always)]
    fn default() -> Ch5div {
        Ch5div(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4top(pub u32);
impl Ch4top {
    #[inline(always)]
    pub const fn ch4_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch4_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch4top {
    #[inline(always)]
    fn default() -> Ch4top {
        Ch4top(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3cc(pub u32);
impl Ch3cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch3cc {
    #[inline(always)]
    fn default() -> Ch3cc {
        Ch3cc(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5csr(pub u32);
impl Ch5csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch5csr {
    #[inline(always)]
    fn default() -> Ch5csr {
        Ch5csr(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3div(pub u32);
impl Ch3div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch3div {
    #[inline(always)]
    fn default() -> Ch3div {
        Ch3div(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2div(pub u32);
impl Ch2div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch2div {
    #[inline(always)]
    fn default() -> Ch2div {
        Ch2div(0)
    }
}
#[doc = "Interrupt Force"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intf(pub u32);
impl Intf {
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intf {
    #[inline(always)]
    fn default() -> Intf {
        Intf(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch5top(pub u32);
impl Ch5top {
    #[inline(always)]
    pub const fn ch5_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch5_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch5top {
    #[inline(always)]
    fn default() -> Ch5top {
        Ch5top(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2cc(pub u32);
impl Ch2cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch2cc {
    #[inline(always)]
    fn default() -> Ch2cc {
        Ch2cc(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7cc(pub u32);
impl Ch7cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch7cc {
    #[inline(always)]
    fn default() -> Ch7cc {
        Ch7cc(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch0ctr(pub u32);
impl Ch0ctr {
    #[inline(always)]
    pub const fn ch0_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch0_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch0ctr {
    #[inline(always)]
    fn default() -> Ch0ctr {
        Ch0ctr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2csr(pub u32);
impl Ch2csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch2csr {
    #[inline(always)]
    fn default() -> Ch2csr {
        Ch2csr(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch4csr(pub u32);
impl Ch4csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch4csr {
    #[inline(always)]
    fn default() -> Ch4csr {
        Ch4csr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch2top(pub u32);
impl Ch2top {
    #[inline(always)]
    pub const fn ch2_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch2_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch2top {
    #[inline(always)]
    fn default() -> Ch2top {
        Ch2top(0)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch3csr(pub u32);
impl Ch3csr {
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PWM channel."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub const fn ph_correct(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "1: Enable phase-correct modulation. 0: Trailing-edge"]
    #[inline(always)]
    pub fn set_ph_correct(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub const fn a_inv(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output A"]
    #[inline(always)]
    pub fn set_a_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub const fn b_inv(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[doc = "Invert output B"]
    #[inline(always)]
    pub fn set_b_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn divmode(&self) -> super::vals::Divmode {
        let val = (self.0 >> 4usize) & 0x03;
        super::vals::Divmode(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.0 as u32) & 0x03) << 4usize);
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub const fn ph_ret(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[doc = "Retard the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running."]
    #[inline(always)]
    pub fn set_ph_ret(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub const fn ph_adv(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[doc = "Advance the phase of the counter by 1 count, while it is running. Self-clearing. Write a 1, and poll until low. Counter must be running at less than full speed (div_int + div_frac / 16 > 1)"]
    #[inline(always)]
    pub fn set_ph_adv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Ch3csr {
    #[inline(always)]
    fn default() -> Ch3csr {
        Ch3csr(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7div(pub u32);
impl Ch7div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch7div {
    #[inline(always)]
    fn default() -> Ch7div {
        Ch7div(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Intr(pub u32);
impl Intr {
    #[inline(always)]
    pub const fn ch0(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[inline(always)]
    pub const fn ch1(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[inline(always)]
    pub const fn ch2(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch2(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
    #[inline(always)]
    pub const fn ch3(&self) -> bool {
        let val = (self.0 >> 3usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch3(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3usize)) | (((val as u32) & 0x01) << 3usize);
    }
    #[inline(always)]
    pub const fn ch4(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch4(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[inline(always)]
    pub const fn ch5(&self) -> bool {
        let val = (self.0 >> 5usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch5(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5usize)) | (((val as u32) & 0x01) << 5usize);
    }
    #[inline(always)]
    pub const fn ch6(&self) -> bool {
        let val = (self.0 >> 6usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch6(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6usize)) | (((val as u32) & 0x01) << 6usize);
    }
    #[inline(always)]
    pub const fn ch7(&self) -> bool {
        let val = (self.0 >> 7usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_ch7(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7usize)) | (((val as u32) & 0x01) << 7usize);
    }
}
impl Default for Intr {
    #[inline(always)]
    fn default() -> Intr {
        Intr(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6ctr(pub u32);
impl Ch6ctr {
    #[inline(always)]
    pub const fn ch6_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch6_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch6ctr {
    #[inline(always)]
    fn default() -> Ch6ctr {
        Ch6ctr(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch7top(pub u32);
impl Ch7top {
    #[inline(always)]
    pub const fn ch7_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch7_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch7top {
    #[inline(always)]
    fn default() -> Ch7top {
        Ch7top(0)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch6div(pub u32);
impl Ch6div {
    #[inline(always)]
    pub const fn frac(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_frac(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
    #[inline(always)]
    pub const fn int(&self) -> u8 {
        let val = (self.0 >> 4usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_int(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 4usize)) | (((val as u32) & 0xff) << 4usize);
    }
}
impl Default for Ch6div {
    #[inline(always)]
    fn default() -> Ch6div {
        Ch6div(0)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1top(pub u32);
impl Ch1top {
    #[inline(always)]
    pub const fn ch1_top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch1_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1top {
    #[inline(always)]
    fn default() -> Ch1top {
        Ch1top(0)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1ctr(pub u32);
impl Ch1ctr {
    #[inline(always)]
    pub const fn ch1_ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ch1_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for Ch1ctr {
    #[inline(always)]
    fn default() -> Ch1ctr {
        Ch1ctr(0)
    }
}
#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Ch1cc(pub u32);
impl Ch1cc {
    #[inline(always)]
    pub const fn a(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_a(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn b(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_b(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for Ch1cc {
    #[inline(always)]
    fn default() -> Ch1cc {
        Ch1cc(0)
    }
}
