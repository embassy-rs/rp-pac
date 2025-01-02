#[doc = "Indicates the armed/disarmed status of each alarm. A write to the corresponding ALARMx register arms the alarm. Alarms automatically disarm upon firing, but writing ones here will disarm immediately without waiting to fire."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Armed(pub u32);
impl Armed {
    #[inline(always)]
    pub const fn armed(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x0f;
        val as u8
    }
    #[inline(always)]
    pub fn set_armed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 0usize)) | (((val as u32) & 0x0f) << 0usize);
    }
}
impl Default for Armed {
    #[inline(always)]
    fn default() -> Armed {
        Armed(0)
    }
}
impl core::fmt::Debug for Armed {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Armed")
            .field("armed", &self.armed())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Armed {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Armed {
            armed: u8,
        }
        let proxy = Armed {
            armed: self.armed(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set bits high to enable pause when the corresponding debug ports are active"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Dbgpause(pub u32);
impl Dbgpause {
    #[doc = "Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub const fn dbg0(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 0 is in debug mode"]
    #[inline(always)]
    pub fn set_dbg0(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub const fn dbg1(&self) -> bool {
        let val = (self.0 >> 2usize) & 0x01;
        val != 0
    }
    #[doc = "Pause when processor 1 is in debug mode"]
    #[inline(always)]
    pub fn set_dbg1(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2usize)) | (((val as u32) & 0x01) << 2usize);
    }
}
impl Default for Dbgpause {
    #[inline(always)]
    fn default() -> Dbgpause {
        Dbgpause(0)
    }
}
impl core::fmt::Debug for Dbgpause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Dbgpause")
            .field("dbg0", &self.dbg0())
            .field("dbg1", &self.dbg1())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Dbgpause {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Dbgpause {
            dbg0: bool,
            dbg1: bool,
        }
        let proxy = Dbgpause {
            dbg0: self.dbg0(),
            dbg1: self.dbg1(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Interrupt Enable"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Int(pub u32);
impl Int {
    #[inline(always)]
    pub const fn alarm(&self, n: usize) -> bool {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_alarm(&mut self, n: usize, val: bool) {
        assert!(n < 4usize);
        let offs = 0usize + n * 1usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
}
impl Default for Int {
    #[inline(always)]
    fn default() -> Int {
        Int(0)
    }
}
impl core::fmt::Debug for Int {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Int")
            .field(
                "alarm",
                &[
                    self.alarm(0usize),
                    self.alarm(1usize),
                    self.alarm(2usize),
                    self.alarm(3usize),
                ],
            )
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Int {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Int {
            alarm: [bool; 4usize],
        }
        let proxy = Int {
            alarm: [
                self.alarm(0usize),
                self.alarm(1usize),
                self.alarm(2usize),
                self.alarm(3usize),
            ],
        };
        defmt::write!(f, "{}", proxy)
    }
}
#[doc = "Set high to pause the timer"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pause(pub u32);
impl Pause {
    #[inline(always)]
    pub const fn pause(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[inline(always)]
    pub fn set_pause(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
}
impl Default for Pause {
    #[inline(always)]
    fn default() -> Pause {
        Pause(0)
    }
}
impl core::fmt::Debug for Pause {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Pause")
            .field("pause", &self.pause())
            .finish()
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Pause {
    fn format(&self, f: defmt::Formatter) {
        #[derive(defmt :: Format)]
        struct Pause {
            pause: bool,
        }
        let proxy = Pause {
            pause: self.pause(),
        };
        defmt::write!(f, "{}", proxy)
    }
}
