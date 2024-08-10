#[doc = "Buffer control for both buffers of an endpoint. Fields ending in a _1 are for buffer 1. Fields ending in a _0 are for buffer 0. Buffer 1 controls are only valid if the endpoint is in double buffered mode."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpBufferControl(pub u32);
impl EpBufferControl {
    #[doc = "The length of the data in buffer 0."]
    #[inline(always)]
    pub const fn length(&self, n: usize) -> u16 {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        let val = (self.0 >> offs) & 0x03ff;
        val as u16
    }
    #[doc = "The length of the data in buffer 0."]
    #[inline(always)]
    pub fn set_length(&mut self, n: usize, val: u16) {
        assert!(n < 2usize);
        let offs = 0usize + n * 16usize;
        self.0 = (self.0 & !(0x03ff << offs)) | (((val as u32) & 0x03ff) << offs);
    }
    #[doc = "Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub const fn available(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 10usize + n * 16usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Buffer 0 is available. This bit is set to indicate the buffer can be used by the controller. The controller clears the available bit when writing the status back."]
    #[inline(always)]
    pub fn set_available(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 10usize + n * 16usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 11usize) & 0x01;
        val != 0
    }
    #[doc = "Reply with a stall (valid for both buffers)."]
    #[inline(always)]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11usize)) | (((val as u32) & 0x01) << 11usize);
    }
    #[doc = "Reset the buffer selector to buffer 0."]
    #[inline(always)]
    pub const fn reset(&self) -> bool {
        let val = (self.0 >> 12usize) & 0x01;
        val != 0
    }
    #[doc = "Reset the buffer selector to buffer 0."]
    #[inline(always)]
    pub fn set_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12usize)) | (((val as u32) & 0x01) << 12usize);
    }
    #[doc = "The data pid of buffer 0."]
    #[inline(always)]
    pub const fn pid(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 13usize + n * 16usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "The data pid of buffer 0."]
    #[inline(always)]
    pub fn set_pid(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 13usize + n * 16usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    pub const fn last(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 14usize + n * 16usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Buffer 0 is the last buffer of the transfer."]
    #[inline(always)]
    pub fn set_last(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 14usize + n * 16usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub const fn full(&self, n: usize) -> bool {
        assert!(n < 2usize);
        let offs = 15usize + n * 16usize;
        let val = (self.0 >> offs) & 0x01;
        val != 0
    }
    #[doc = "Buffer 0 is full. For an IN transfer (TX to the host) the bit is set to indicate the data is valid. For an OUT transfer (RX from the host) this bit should be left as a 0. The host will set it when it has filled the buffer with data."]
    #[inline(always)]
    pub fn set_full(&mut self, n: usize, val: bool) {
        assert!(n < 2usize);
        let offs = 15usize + n * 16usize;
        self.0 = (self.0 & !(0x01 << offs)) | (((val as u32) & 0x01) << offs);
    }
    #[doc = "The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    pub const fn double_buffer_iso_offset(
        &self,
    ) -> super::vals::EpBufferControlDoubleBufferIsoOffset {
        let val = (self.0 >> 27usize) & 0x03;
        super::vals::EpBufferControlDoubleBufferIsoOffset::from_bits(val as u8)
    }
    #[doc = "The number of bytes buffer 1 is offset from buffer 0 in Isochronous mode. Only valid in double buffered mode for an Isochronous endpoint. For a non Isochronous endpoint the offset is always 64 bytes."]
    #[inline(always)]
    pub fn set_double_buffer_iso_offset(
        &mut self,
        val: super::vals::EpBufferControlDoubleBufferIsoOffset,
    ) {
        self.0 = (self.0 & !(0x03 << 27usize)) | (((val.to_bits() as u32) & 0x03) << 27usize);
    }
}
impl Default for EpBufferControl {
    #[inline(always)]
    fn default() -> EpBufferControl {
        EpBufferControl(0)
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct EpControl(pub u32);
impl EpControl {
    #[doc = "64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub const fn buffer_address(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[doc = "64 byte aligned buffer address for this EP (bits 0-5 are ignored). Relative to the start of the DPRAM."]
    #[inline(always)]
    pub fn set_buffer_address(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[doc = "Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub const fn interrupt_on_nak(&self) -> bool {
        let val = (self.0 >> 16usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger an interrupt if a NAK is sent. Intended for debug only."]
    #[inline(always)]
    pub fn set_interrupt_on_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16usize)) | (((val as u32) & 0x01) << 16usize);
    }
    #[doc = "Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub const fn interrupt_on_stall(&self) -> bool {
        let val = (self.0 >> 17usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger an interrupt if a STALL is sent. Intended for debug only."]
    #[inline(always)]
    pub fn set_interrupt_on_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17usize)) | (((val as u32) & 0x01) << 17usize);
    }
    #[inline(always)]
    pub const fn endpoint_type(&self) -> super::vals::EpControlEndpointType {
        let val = (self.0 >> 26usize) & 0x03;
        super::vals::EpControlEndpointType::from_bits(val as u8)
    }
    #[inline(always)]
    pub fn set_endpoint_type(&mut self, val: super::vals::EpControlEndpointType) {
        self.0 = (self.0 & !(0x03 << 26usize)) | (((val.to_bits() as u32) & 0x03) << 26usize);
    }
    #[doc = "Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub const fn interrupt_per_double_buff(&self) -> bool {
        let val = (self.0 >> 28usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger an interrupt each time both buffers are done. Only valid in double buffered mode."]
    #[inline(always)]
    pub fn set_interrupt_per_double_buff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28usize)) | (((val as u32) & 0x01) << 28usize);
    }
    #[doc = "Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub const fn interrupt_per_buff(&self) -> bool {
        let val = (self.0 >> 29usize) & 0x01;
        val != 0
    }
    #[doc = "Trigger an interrupt each time a buffer is done."]
    #[inline(always)]
    pub fn set_interrupt_per_buff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29usize)) | (((val as u32) & 0x01) << 29usize);
    }
    #[doc = "This endpoint is double buffered."]
    #[inline(always)]
    pub const fn double_buffered(&self) -> bool {
        let val = (self.0 >> 30usize) & 0x01;
        val != 0
    }
    #[doc = "This endpoint is double buffered."]
    #[inline(always)]
    pub fn set_double_buffered(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30usize)) | (((val as u32) & 0x01) << 30usize);
    }
    #[doc = "Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub const fn enable(&self) -> bool {
        let val = (self.0 >> 31usize) & 0x01;
        val != 0
    }
    #[doc = "Enable this endpoint. The device will not reply to any packets for this endpoint if this bit is not set."]
    #[inline(always)]
    pub fn set_enable(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31usize)) | (((val as u32) & 0x01) << 31usize);
    }
}
impl Default for EpControl {
    #[inline(always)]
    fn default() -> EpControl {
        EpControl(0)
    }
}
#[doc = "Bytes 4-7 of the setup packet from the host."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetupPacketHigh(pub u32);
impl SetupPacketHigh {
    #[inline(always)]
    pub const fn windex(&self) -> u16 {
        let val = (self.0 >> 0usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_windex(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 0usize)) | (((val as u32) & 0xffff) << 0usize);
    }
    #[inline(always)]
    pub const fn wlength(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_wlength(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SetupPacketHigh {
    #[inline(always)]
    fn default() -> SetupPacketHigh {
        SetupPacketHigh(0)
    }
}
#[doc = "Bytes 0-3 of the SETUP packet from the host."]
#[repr(transparent)]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct SetupPacketLow(pub u32);
impl SetupPacketLow {
    #[inline(always)]
    pub const fn bmrequesttype(&self) -> u8 {
        let val = (self.0 >> 0usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_bmrequesttype(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 0usize)) | (((val as u32) & 0xff) << 0usize);
    }
    #[inline(always)]
    pub const fn brequest(&self) -> u8 {
        let val = (self.0 >> 8usize) & 0xff;
        val as u8
    }
    #[inline(always)]
    pub fn set_brequest(&mut self, val: u8) {
        self.0 = (self.0 & !(0xff << 8usize)) | (((val as u32) & 0xff) << 8usize);
    }
    #[inline(always)]
    pub const fn wvalue(&self) -> u16 {
        let val = (self.0 >> 16usize) & 0xffff;
        val as u16
    }
    #[inline(always)]
    pub fn set_wvalue(&mut self, val: u16) {
        self.0 = (self.0 & !(0xffff << 16usize)) | (((val as u32) & 0xffff) << 16usize);
    }
}
impl Default for SetupPacketLow {
    #[inline(always)]
    fn default() -> SetupPacketLow {
        SetupPacketLow(0)
    }
}
