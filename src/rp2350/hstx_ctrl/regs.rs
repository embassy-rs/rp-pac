#[doc = "Data control register for output bit 0"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit0(pub u32);
impl Bit0 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit0 {
    #[inline(always)]
    fn default() -> Bit0 {
        Bit0(0)
    }
}
#[doc = "Data control register for output bit 1"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit1(pub u32);
impl Bit1 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit1 {
    #[inline(always)]
    fn default() -> Bit1 {
        Bit1(0)
    }
}
#[doc = "Data control register for output bit 2"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit2(pub u32);
impl Bit2 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit2 {
    #[inline(always)]
    fn default() -> Bit2 {
        Bit2(0)
    }
}
#[doc = "Data control register for output bit 3"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit3(pub u32);
impl Bit3 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit3 {
    #[inline(always)]
    fn default() -> Bit3 {
        Bit3(0)
    }
}
#[doc = "Data control register for output bit 4"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit4(pub u32);
impl Bit4 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit4 {
    #[inline(always)]
    fn default() -> Bit4 {
        Bit4(0)
    }
}
#[doc = "Data control register for output bit 5"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit5(pub u32);
impl Bit5 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit5 {
    #[inline(always)]
    fn default() -> Bit5 {
        Bit5(0)
    }
}
#[doc = "Data control register for output bit 6"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit6(pub u32);
impl Bit6 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit6 {
    #[inline(always)]
    fn default() -> Bit6 {
        Bit6(0)
    }
}
#[doc = "Data control register for output bit 7"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Bit7(pub u32);
impl Bit7 {
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_p(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the first half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_p(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub const fn sel_n(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Shift register data bit select for the second half of the HSTX clock cycle"]
    #[inline(always)]
    pub fn set_sel_n(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub const fn inv(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Invert this data output (logical NOT)"]
    #[inline(always)]
    pub fn set_inv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub const fn clk(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Connect this output to the generated clock, rather than the data shift register. SEL_P and SEL_N are ignored if this bit is set, but INV can still be set to generate an antiphase clock."]
    #[inline(always)]
    pub fn set_clk(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
}
impl Default for Bit7 {
    #[inline(always)]
    fn default() -> Bit7 {
        Bit7(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Csr(pub u32);
impl Csr {
    #[doc = "When EN is 1, the HSTX will shift out data as it appears in the FIFO. As long as there is data, the HSTX shift register will shift once per clock cycle, and the frequency of popping from the FIFO is determined by the ratio of SHIFT and SHIFT_THRESH. When EN is 0, the FIFO is not popped. The shift counter and clock generator are also reset to their initial state for as long as EN is low. Note the initial phase of the clock generator can be configured by the CLKPHASE field. Once the HSTX is enabled again, and data is pushed to the FIFO, the generated clock's first rising edge will be one half-period after the first data is launched."]
    #[inline(always)]
    pub const fn en(&self) -> bool {
        let val = (self.0 >> 0usize) & 0x01;
        val != 0
    }
    #[doc = "When EN is 1, the HSTX will shift out data as it appears in the FIFO. As long as there is data, the HSTX shift register will shift once per clock cycle, and the frequency of popping from the FIFO is determined by the ratio of SHIFT and SHIFT_THRESH. When EN is 0, the FIFO is not popped. The shift counter and clock generator are also reset to their initial state for as long as EN is low. Note the initial phase of the clock generator can be configured by the CLKPHASE field. Once the HSTX is enabled again, and data is pushed to the FIFO, the generated clock's first rising edge will be one half-period after the first data is launched."]
    #[inline(always)]
    pub fn set_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0usize)) | (((val as u32) & 0x01) << 0usize);
    }
    #[doc = "Enable the command expander. When 0, raw FIFO data is passed directly to the output shift register. When 1, the command expander can perform simple operations such as run length decoding on data between the FIFO and the shift register. Do not change CXPD_EN whilst EN is set. It's safe to set CXPD_EN simultaneously with setting EN."]
    #[inline(always)]
    pub const fn expand_en(&self) -> bool {
        let val = (self.0 >> 1usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the command expander. When 0, raw FIFO data is passed directly to the output shift register. When 1, the command expander can perform simple operations such as run length decoding on data between the FIFO and the shift register. Do not change CXPD_EN whilst EN is set. It's safe to set CXPD_EN simultaneously with setting EN."]
    #[inline(always)]
    pub fn set_expand_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1usize)) | (((val as u32) & 0x01) << 1usize);
    }
    #[doc = "Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* from the system clock (not just from some other clock source of the same frequency) for this synchronous interface to function correctly. When COUPLED_MODE is set, BITx_SEL_P and SEL_N indices 24 through 31 will select bits from the 8-bit PIO-to-HSTX path, rather than shifter bits. Indices of 0 through 23 will still index the shift register as normal. The PIO outputs connected to the PIO-to-HSTX bus are those same outputs that would appear on the HSTX-capable pins if those pins' FUNCSELs were set to PIO instead of HSTX. For example, if HSTX is on GPIOs 12 through 19, then PIO outputs 12 through 19 are connected to the HSTX when coupled mode is engaged."]
    #[inline(always)]
    pub const fn coupled_mode(&self) -> bool {
        let val = (self.0 >> 4usize) & 0x01;
        val != 0
    }
    #[doc = "Enable the PIO-to-HSTX 1:1 connection. The HSTX must be clocked *directly* from the system clock (not just from some other clock source of the same frequency) for this synchronous interface to function correctly. When COUPLED_MODE is set, BITx_SEL_P and SEL_N indices 24 through 31 will select bits from the 8-bit PIO-to-HSTX path, rather than shifter bits. Indices of 0 through 23 will still index the shift register as normal. The PIO outputs connected to the PIO-to-HSTX bus are those same outputs that would appear on the HSTX-capable pins if those pins' FUNCSELs were set to PIO instead of HSTX. For example, if HSTX is on GPIOs 12 through 19, then PIO outputs 12 through 19 are connected to the HSTX when coupled mode is engaged."]
    #[inline(always)]
    pub fn set_coupled_mode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4usize)) | (((val as u32) & 0x01) << 4usize);
    }
    #[doc = "Select which PIO to use for coupled mode operation."]
    #[inline(always)]
    pub const fn coupled_sel(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x03;
        val as u8
    }
    #[doc = "Select which PIO to use for coupled mode operation."]
    #[inline(always)]
    pub fn set_coupled_sel(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 5usize)) | (((val as u32) & 0x03) << 5usize);
    }
    #[doc = "How many bits to right-rotate the shift register by each cycle. The use of a rotate rather than a shift allows left shifts to be emulated, by subtracting the left-shift amount from 32. It also allows data to be repeated, when the product of SHIFT and N_SHIFTS is greater than 32."]
    #[inline(always)]
    pub const fn shift(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "How many bits to right-rotate the shift register by each cycle. The use of a rotate rather than a shift allows left shifts to be emulated, by subtracting the left-shift amount from 32. It also allows data to be repeated, when the product of SHIFT and N_SHIFTS is greater than 32."]
    #[inline(always)]
    pub fn set_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Number of times to shift the shift register before refilling it from the FIFO. (A count of how many times it has been shifted, *not* the total shift distance.) A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub const fn n_shifts(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of times to shift the shift register before refilling it from the FIFO. (A count of how many times it has been shifted, *not* the total shift distance.) A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub fn set_n_shifts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Set the initial phase of the generated clock. A CLKPHASE of 0 means the clock is initially low, and the first rising edge occurs after one half period of the generated clock (i.e. CLKDIV/2 cycles of clk_hstx). Incrementing CLKPHASE by 1 will advance the initial clock phase by one half clk_hstx period. For example, if CLKDIV=2 and CLKPHASE=1: * The clock will be initially low * The first rising edge will be 0.5 clk_hstx cycles after asserting first data * The first falling edge will be 1.5 clk_hstx cycles after asserting first data This configuration would be suitable for serialising at a bit rate of clk_hstx with a centre-aligned DDR clock. When the HSTX is halted by clearing CSR_EN, the clock generator will return to its initial phase as configured by the CLKPHASE field. Note CLKPHASE must be strictly less than double the value of CLKDIV (one full period), else its operation is undefined."]
    #[inline(always)]
    pub const fn clkphase(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x0f;
        val as u8
    }
    #[doc = "Set the initial phase of the generated clock. A CLKPHASE of 0 means the clock is initially low, and the first rising edge occurs after one half period of the generated clock (i.e. CLKDIV/2 cycles of clk_hstx). Incrementing CLKPHASE by 1 will advance the initial clock phase by one half clk_hstx period. For example, if CLKDIV=2 and CLKPHASE=1: * The clock will be initially low * The first rising edge will be 0.5 clk_hstx cycles after asserting first data * The first falling edge will be 1.5 clk_hstx cycles after asserting first data This configuration would be suitable for serialising at a bit rate of clk_hstx with a centre-aligned DDR clock. When the HSTX is halted by clearing CSR_EN, the clock generator will return to its initial phase as configured by the CLKPHASE field. Note CLKPHASE must be strictly less than double the value of CLKDIV (one full period), else its operation is undefined."]
    #[inline(always)]
    pub fn set_clkphase(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 24usize)) | (((val as u32) & 0x0f) << 24usize);
    }
    #[doc = "Clock period of the generated clock, measured in HSTX clock cycles. Can be odd or even. The generated clock advances only on cycles where the shift register shifts. For example, a clkdiv of 5 would generate a complete output clock period for every 5 HSTX clocks (or every 10 half-clocks). A CLKDIV value of 0 is mapped to a period of 16 HSTX clock cycles."]
    #[inline(always)]
    pub const fn clkdiv(&self) -> u8 {
        let val = (self.0 >> 28usize) & 0x0f;
        val as u8
    }
    #[doc = "Clock period of the generated clock, measured in HSTX clock cycles. Can be odd or even. The generated clock advances only on cycles where the shift register shifts. For example, a clkdiv of 5 would generate a complete output clock period for every 5 HSTX clocks (or every 10 half-clocks). A CLKDIV value of 0 is mapped to a period of 16 HSTX clock cycles."]
    #[inline(always)]
    pub fn set_clkdiv(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 28usize)) | (((val as u32) & 0x0f) << 28usize);
    }
}
impl Default for Csr {
    #[inline(always)]
    fn default() -> Csr {
        Csr(0)
    }
}
#[doc = "Configure the optional shifter inside the command expander"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExpandShift(pub u32);
impl ExpandShift {
    #[doc = "How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
    #[inline(always)]
    pub const fn raw_shift(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is a raw data command."]
    #[inline(always)]
    pub fn set_raw_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub const fn raw_n_shifts(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of times to consume from the shift register before refilling it from the FIFO, when the current command is a raw data command. A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub fn set_raw_n_shifts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
    #[inline(always)]
    pub const fn enc_shift(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "How many bits to right-rotate the shift register by each time data is pushed to the output shifter, when the current command is an encoded data command (e.g. TMDS)."]
    #[inline(always)]
    pub fn set_enc_shift(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub const fn enc_n_shifts(&self) -> u8 {
        let val = (self.0 >> 24usize) & 0x1f;
        val as u8
    }
    #[doc = "Number of times to consume from the shift register before refilling it from the FIFO, when the current command is an encoded data command (e.g. TMDS). A register value of 0 means shift 32 times."]
    #[inline(always)]
    pub fn set_enc_n_shifts(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 24usize)) | (((val as u32) & 0x1f) << 24usize);
    }
}
impl Default for ExpandShift {
    #[inline(always)]
    fn default() -> ExpandShift {
        ExpandShift(0)
    }
}
#[doc = "Configure the optional TMDS encoder inside the command expander"]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct ExpandTmds(pub u32);
impl ExpandTmds {
    #[doc = "Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
    #[inline(always)]
    pub const fn l0_rot(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to the current shifter data before the lane 0 TMDS encoder."]
    #[inline(always)]
    pub fn set_l0_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0usize)) | (((val as u32) & 0x1f) << 0usize);
    }
    #[doc = "Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub const fn l0_nbits(&self) -> u8 {
        let val = (self.0 >> 5usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid data bits for the lane 0 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn set_l0_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 5usize)) | (((val as u32) & 0x07) << 5usize);
    }
    #[doc = "Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
    #[inline(always)]
    pub const fn l1_rot(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to the current shifter data before the lane 1 TMDS encoder."]
    #[inline(always)]
    pub fn set_l1_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8usize)) | (((val as u32) & 0x1f) << 8usize);
    }
    #[doc = "Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub const fn l1_nbits(&self) -> u8 {
        let val = (self.0 >> 13usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid data bits for the lane 1 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn set_l1_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 13usize)) | (((val as u32) & 0x07) << 13usize);
    }
    #[doc = "Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
    #[inline(always)]
    pub const fn l2_rot(&self) -> u8 {
        let val = (self.0 >> 16usize) & 0x1f;
        val as u8
    }
    #[doc = "Right-rotate applied to the current shifter data before the lane 2 TMDS encoder."]
    #[inline(always)]
    pub fn set_l2_rot(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 16usize)) | (((val as u32) & 0x1f) << 16usize);
    }
    #[doc = "Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub const fn l2_nbits(&self) -> u8 {
        let val = (self.0 >> 21usize) & 0x07;
        val as u8
    }
    #[doc = "Number of valid data bits for the lane 2 TMDS encoder, starting from bit 7 of the rotated data. Field values of 0 -> 7 encode counts of 1 -> 8 bits."]
    #[inline(always)]
    pub fn set_l2_nbits(&mut self, val: u8) {
        self.0 = (self.0 & !(0x07 << 21usize)) | (((val as u32) & 0x07) << 21usize);
    }
}
impl Default for ExpandTmds {
    #[inline(always)]
    fn default() -> ExpandTmds {
        ExpandTmds(0)
    }
}
