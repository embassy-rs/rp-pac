#[doc = "Counter compare values"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCc(pub u32);
impl ChCc {
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
impl Default for ChCc {
    #[inline(always)]
    fn default() -> ChCc {
        ChCc(0)
    }
}
impl core::fmt::Debug for ChCc {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCc")
            .field("a", &self.a())
            .field("b", &self.b())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCc {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChCc {
            a: u16,
            b: u16,
        }
        let proxy = ChCc {
            a: self.a(),
            b: self.b(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Control and status register"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCsr(pub u32);
impl ChCsr {
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
        super::vals::Divmode::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_divmode(&mut self, val: super::vals::Divmode) {
        self.0 = (self.0 & !(0x03 << 4usize)) | (((val.to_bits() as u32) & 0x03) << 4usize);
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
impl Default for ChCsr {
    #[inline(always)]
    fn default() -> ChCsr {
        ChCsr(0)
    }
}
impl core::fmt::Debug for ChCsr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCsr")
            .field("en", &self.en())
            .field("ph_correct", &self.ph_correct())
            .field("a_inv", &self.a_inv())
            .field("b_inv", &self.b_inv())
            .field("divmode", &self.divmode())
            .field("ph_ret", &self.ph_ret())
            .field("ph_adv", &self.ph_adv())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCsr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChCsr {
            en: bool,
            ph_correct: bool,
            a_inv: bool,
            b_inv: bool,
            divmode: super::vals::Divmode,
            ph_ret: bool,
            ph_adv: bool,
        }
        let proxy = ChCsr {
            en: self.en(),
            ph_correct: self.ph_correct(),
            a_inv: self.a_inv(),
            b_inv: self.b_inv(),
            divmode: self.divmode(),
            ph_ret: self.ph_ret(),
            ph_adv: self.ph_adv(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Direct access to the PWM counter"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChCtr(pub u32);
impl ChCtr {
    #[inline(always)]
    pub const fn ctr(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_ctr(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChCtr {
    #[inline(always)]
    fn default() -> ChCtr {
        ChCtr(0)
    }
}
impl core::fmt::Debug for ChCtr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChCtr").field("ctr", &self.ctr()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChCtr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChCtr {
            ctr: u16,
        }
        let proxy = ChCtr { ctr: self.ctr() };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "INT and FRAC form a fixed-point fractional number. Counting rate is system clock frequency divided by this number. Fractional division uses simple 1st-order sigma-delta."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChDiv(pub u32);
impl ChDiv {
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
impl Default for ChDiv {
    #[inline(always)]
    fn default() -> ChDiv {
        ChDiv(0)
    }
}
impl core::fmt::Debug for ChDiv {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChDiv")
            .field("frac", &self.frac())
            .field("int", &self.int())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChDiv {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChDiv {
            frac: u8,
            int: u8,
        }
        let proxy = ChDiv {
            frac: self.frac(),
            int: self.int(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Counter wrap value"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ChTop(pub u32);
impl ChTop {
    #[inline(always)]
    pub const fn top(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_top(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
}
impl Default for ChTop {
    #[inline(always)]
    fn default() -> ChTop {
        ChTop(0)
    }
}
impl core::fmt::Debug for ChTop {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ChTop").field("top", &self.top()).finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for ChTop {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct ChTop {
            top: u16,
        }
        let proxy = ChTop { top: self.top() };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for En {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("En")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for En {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct En {
            ch0: bool,
            ch1: bool,
            ch2: bool,
            ch3: bool,
            ch4: bool,
            ch5: bool,
            ch6: bool,
            ch7: bool,
        }
        let proxy = En {
            ch0: self.ch0(),
            ch1: self.ch1(),
            ch2: self.ch2(),
            ch3: self.ch3(),
            ch4: self.ch4(),
            ch5: self.ch5(),
            ch6: self.ch6(),
            ch7: self.ch7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
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
impl core::fmt::Debug for Inte {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Inte")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Inte {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Inte {
            ch0: bool,
            ch1: bool,
            ch2: bool,
            ch3: bool,
            ch4: bool,
            ch5: bool,
            ch6: bool,
            ch7: bool,
        }
        let proxy = Inte {
            ch0: self.ch0(),
            ch1: self.ch1(),
            ch2: self.ch2(),
            ch3: self.ch3(),
            ch4: self.ch4(),
            ch5: self.ch5(),
            ch6: self.ch6(),
            ch7: self.ch7(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Intf {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intf")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intf {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intf {
            ch0: bool,
            ch1: bool,
            ch2: bool,
            ch3: bool,
            ch4: bool,
            ch5: bool,
            ch6: bool,
            ch7: bool,
        }
        let proxy = Intf {
            ch0: self.ch0(),
            ch1: self.ch1(),
            ch2: self.ch2(),
            ch3: self.ch3(),
            ch4: self.ch4(),
            ch5: self.ch5(),
            ch6: self.ch6(),
            ch7: self.ch7(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Intr {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Intr")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Intr {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Intr {
            ch0: bool,
            ch1: bool,
            ch2: bool,
            ch3: bool,
            ch4: bool,
            ch5: bool,
            ch6: bool,
            ch7: bool,
        }
        let proxy = Intr {
            ch0: self.ch0(),
            ch1: self.ch1(),
            ch2: self.ch2(),
            ch3: self.ch3(),
            ch4: self.ch4(),
            ch5: self.ch5(),
            ch6: self.ch6(),
            ch7: self.ch7(),
        };
        defmt::write!(f, "{}", proxy)
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
impl core::fmt::Debug for Ints {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Ints")
            .field("ch0", &self.ch0())
            .field("ch1", &self.ch1())
            .field("ch2", &self.ch2())
            .field("ch3", &self.ch3())
            .field("ch4", &self.ch4())
            .field("ch5", &self.ch5())
            .field("ch6", &self.ch6())
            .field("ch7", &self.ch7())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Ints {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Ints {
            ch0: bool,
            ch1: bool,
            ch2: bool,
            ch3: bool,
            ch4: bool,
            ch5: bool,
            ch6: bool,
            ch7: bool,
        }
        let proxy = Ints {
            ch0: self.ch0(),
            ch1: self.ch1(),
            ch2: self.ch2(),
            ch3: self.ch3(),
            ch4: self.ch4(),
            ch5: self.ch5(),
            ch6: self.ch6(),
            ch7: self.ch7(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
