use crate::generic::*;
#[doc = "Execution/behavioural settings for state machine 3"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SmExecctrlStatusSel(u8);
impl SmExecctrlStatusSel {
    pub const fn to_bits(&self) -> u8 {
        self.0
    }
    pub const fn from_bits(val: u8) -> SmExecctrlStatusSel {
        SmExecctrlStatusSel(val)
    }
    #[doc = "All-ones if TX FIFO level < N, otherwise all-zeroes"]
    pub const TXLEVEL: Self = Self(0);
    #[doc = "All-ones if RX FIFO level < N, otherwise all-zeroes"]
    pub const RXLEVEL: Self = Self(0x01);
}
