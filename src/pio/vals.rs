#[repr(u8)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum SmExecctrlStatusSel {
    #[doc = "All-ones if TX FIFO level < N, otherwise all-zeroes"]
    TXLEVEL = 0,
    #[doc = "All-ones if RX FIFO level < N, otherwise all-zeroes"]
    RXLEVEL = 0x01,
}
impl SmExecctrlStatusSel {
    #[inline(always)]
    pub const fn from_bits(val: u8) -> SmExecctrlStatusSel {
        unsafe { core::mem::transmute(val & 0x01) }
    }
    #[inline(always)]
    pub const fn to_bits(self) -> u8 {
        unsafe { core::mem::transmute(self) }
    }
}
impl From<u8> for SmExecctrlStatusSel {
    #[inline(always)]
    fn from(val: u8) -> SmExecctrlStatusSel {
        SmExecctrlStatusSel::from_bits(val)
    }
}
impl From<SmExecctrlStatusSel> for u8 {
    #[inline(always)]
    fn from(val: SmExecctrlStatusSel) -> u8 {
        SmExecctrlStatusSel::to_bits(val)
    }
}
