use crate::generic::*;
#[doc = "This register allows for direct control of the USB phy. Use in conjunction with usbphy_direct_override register to enable each override bit."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbphyDirect(pub u32);
impl UsbphyDirect {
    #[doc = "DM over voltage"]
    pub const fn dm_ovv(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    #[doc = "DM over voltage"]
    pub fn set_dm_ovv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    #[doc = "DP over voltage"]
    pub const fn dp_ovv(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    #[doc = "DP over voltage"]
    pub fn set_dp_ovv(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    #[doc = "DM overcurrent"]
    pub const fn dm_ovcn(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    #[doc = "DM overcurrent"]
    pub fn set_dm_ovcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    #[doc = "DP overcurrent"]
    pub const fn dp_ovcn(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "DP overcurrent"]
    pub fn set_dp_ovcn(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "DPM pin state"]
    pub const fn rx_dm(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "DPM pin state"]
    pub fn set_rx_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "DPP pin state"]
    pub const fn rx_dp(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "DPP pin state"]
    pub fn set_rx_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "Differential RX"]
    pub const fn rx_dd(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Differential RX"]
    pub fn set_rx_dd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "TX_DIFFMODE=0: Single ended mode TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    pub const fn tx_diffmode(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "TX_DIFFMODE=0: Single ended mode TX_DIFFMODE=1: Differential drive mode (TX_DM, TX_DM_OE ignored)"]
    pub fn set_tx_diffmode(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "TX_FSSLEW=0: Low speed slew rate TX_FSSLEW=1: Full speed slew rate"]
    pub const fn tx_fsslew(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    #[doc = "TX_FSSLEW=0: Low speed slew rate TX_FSSLEW=1: Full speed slew rate"]
    pub fn set_tx_fsslew(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    #[doc = "TX power down override (if override enable is set). 1 = powered down."]
    pub const fn tx_pd(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "TX power down override (if override enable is set). 1 = powered down."]
    pub fn set_tx_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "RX power down override (if override enable is set). 1 = powered down."]
    pub const fn rx_pd(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "RX power down override (if override enable is set). 1 = powered down."]
    pub fn set_rx_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Output data. TX_DIFFMODE=1, Ignored TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    pub const fn tx_dm(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Output data. TX_DIFFMODE=1, Ignored TX_DIFFMODE=0, Drives DPM only. TX_DM_OE=1 to enable drive. DPM=TX_DM"]
    pub fn set_tx_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    pub const fn tx_dp(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Output data. If TX_DIFFMODE=1, Drives DPP/DPM diff pair. TX_DP_OE=1 to enable drive. DPP=TX_DP, DPM=~TX_DP If TX_DIFFMODE=0, Drives DPP only. TX_DP_OE=1 to enable drive. DPP=TX_DP"]
    pub fn set_tx_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, Ignored. If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    pub const fn tx_dm_oe(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, Ignored. If TX_DIFFMODE=0, OE for DPM only. 0 - DPM in Hi-Z state; 1 - DPM driving"]
    pub fn set_tx_dm_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    pub const fn tx_dp_oe(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Output enable. If TX_DIFFMODE=1, OE for DPP/DPM diff pair. 0 - DPP/DPM in Hi-Z state; 1 - DPP/DPM driving If TX_DIFFMODE=0, OE for DPP only. 0 - DPP in Hi-Z state; 1 - DPP driving"]
    pub fn set_tx_dp_oe(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "DM pull down enable"]
    pub const fn dm_pulldn_en(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "DM pull down enable"]
    pub fn set_dm_pulldn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "DM pull up enable"]
    pub const fn dm_pullup_en(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "DM pull up enable"]
    pub fn set_dm_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    pub const fn dm_pullup_hisel(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the second DM pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    pub fn set_dm_pullup_hisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "DP pull down enable"]
    pub const fn dp_pulldn_en(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "DP pull down enable"]
    pub fn set_dp_pulldn_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "DP pull up enable"]
    pub const fn dp_pullup_en(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "DP pull up enable"]
    pub fn set_dp_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    pub const fn dp_pullup_hisel(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable the second DP pull up resistor. 0 - Pull = Rpu2; 1 - Pull = Rpu1 + Rpu2"]
    pub fn set_dp_pullup_hisel(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for UsbphyDirect {
    fn default() -> UsbphyDirect {
        UsbphyDirect(0)
    }
}
#[doc = "Buffer status register. A bit set here indicates that a buffer has completed on the endpoint (if the buffer interrupt is enabled). It is possible for 2 buffers to be completed, so clearing the buffer status bit may instantly re set it on the next clock cycle."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BuffStatus(pub u32);
impl BuffStatus {
    pub const fn ep15_out(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn ep15_in(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn ep14_out(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn ep14_in(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn ep13_out(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn ep13_in(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn ep12_out(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn ep12_in(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn ep11_out(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn ep11_in(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn ep10_out(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn ep10_in(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn ep9_out(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn ep9_in(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn ep8_out(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn ep8_in(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn ep7_out(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn ep7_in(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn ep6_out(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn ep6_in(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn ep5_out(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn ep5_in(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn ep4_out(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn ep4_in(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn ep3_out(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ep3_in(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ep2_out(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ep2_in(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ep1_out(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ep1_in(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for BuffStatus {
    fn default() -> BuffStatus {
        BuffStatus(0)
    }
}
#[doc = "Interrupt endpoint 8. Only valid for HOST mode."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AddrEndpX(pub u32);
impl AddrEndpX {
    #[doc = "Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    pub const fn intep_preamble(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "Interrupt EP requires preamble (is a low speed device on a full speed hub)"]
    pub fn set_intep_preamble(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "Direction of the interrupt endpoint. In=0, Out=1"]
    pub const fn intep_dir(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Direction of the interrupt endpoint. In=0, Out=1"]
    pub fn set_intep_dir(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Endpoint number of the interrupt endpoint"]
    pub const fn endpoint(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x0f;
        val as u8
    }
    #[doc = "Endpoint number of the interrupt endpoint"]
    pub fn set_endpoint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16u32)) | (((val as u32) & 0x0f) << 16u32);
    }
    #[doc = "Device address"]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x7f;
        val as u8
    }
    #[doc = "Device address"]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0u32)) | (((val as u32) & 0x7f) << 0u32);
    }
}
impl Default for AddrEndpX {
    fn default() -> AddrEndpX {
        AddrEndpX(0)
    }
}
#[doc = "Read the last SOF (Start of Frame) frame number seen. In device mode the last SOF received from the host. In host mode the last SOF sent by the host."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SofRd(pub u32);
impl SofRd {
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x07ff;
        val as u16
    }
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0u32)) | (((val as u32) & 0x07ff) << 0u32);
    }
}
impl Default for SofRd {
    fn default() -> SofRd {
        SofRd(0)
    }
}
#[doc = "Where to connect the USB controller. Should be to_phy by default."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbMuxing(pub u32);
impl UsbMuxing {
    pub const fn softcon(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_softcon(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn to_digital_pad(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_to_digital_pad(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn to_extphy(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_to_extphy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn to_phy(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_to_phy(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for UsbMuxing {
    fn default() -> UsbMuxing {
        UsbMuxing(0)
    }
}
#[doc = "Used by the host controller. Sets the wait time in microseconds before trying again if the device replies with a NAK."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct NakPoll(pub u32);
impl NakPoll {
    #[doc = "NAK polling interval for a full speed device"]
    pub const fn delay_fs(&self) -> u16 {
        let val = (self.0 >> 16u32) & 0x03ff;
        val as u16
    }
    #[doc = "NAK polling interval for a full speed device"]
    pub fn set_delay_fs(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 16u32)) | (((val as u32) & 0x03ff) << 16u32);
    }
    #[doc = "NAK polling interval for a low speed device"]
    pub const fn delay_ls(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x03ff;
        val as u16
    }
    #[doc = "NAK polling interval for a low speed device"]
    pub fn set_delay_ls(&mut self, val: u16) {
        self.0 = (self.0 & !(0x03ff << 0u32)) | (((val as u32) & 0x03ff) << 0u32);
    }
}
impl Default for NakPoll {
    fn default() -> NakPoll {
        NakPoll(0)
    }
}
#[doc = "Device: this bit must be set in conjunction with the `STALL` bit in the buffer control register to send a STALL on EP0. The device controller clears these bits when a SETUP packet is received because the USB spec requires that a STALL condition is cleared when a SETUP packet is received."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EpStallArm(pub u32);
impl EpStallArm {
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for EpStallArm {
    fn default() -> EpStallArm {
        EpStallArm(0)
    }
}
#[doc = "Raw Interrupts"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct Int(pub u32);
impl Int {
    #[doc = "Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    pub const fn ep_stall_nak(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in EP_STATUS_STALL_NAK is set. Clear by clearing all bits in EP_STATUS_STALL_NAK."]
    pub fn set_ep_stall_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    pub const fn abort_done(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in ABORT_DONE is set. Clear by clearing all bits in ABORT_DONE."]
    pub fn set_abort_done(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    pub const fn dev_sof(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "Set every time the device receives a SOF (Start of Frame) packet. Cleared by reading SOF_RD"]
    pub fn set_dev_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "Device. Source: SIE_STATUS.SETUP_REC"]
    pub const fn setup_req(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Device. Source: SIE_STATUS.SETUP_REC"]
    pub fn set_setup_req(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    pub const fn dev_resume_from_host(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "Set when the device receives a resume from the host. Cleared by writing to SIE_STATUS.RESUME"]
    pub fn set_dev_resume_from_host(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    pub const fn dev_suspend(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    #[doc = "Set when the device suspend state changes. Cleared by writing to SIE_STATUS.SUSPENDED"]
    pub fn set_dev_suspend(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    #[doc = "Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    pub const fn dev_conn_dis(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "Set when the device connection state changes. Cleared by writing to SIE_STATUS.CONNECTED"]
    pub fn set_dev_conn_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "Source: SIE_STATUS.BUS_RESET"]
    pub const fn bus_reset(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.BUS_RESET"]
    pub fn set_bus_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Source: SIE_STATUS.VBUS_DETECT"]
    pub const fn vbus_detect(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.VBUS_DETECT"]
    pub fn set_vbus_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Source: SIE_STATUS.STALL_REC"]
    pub const fn stall(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.STALL_REC"]
    pub fn set_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Source: SIE_STATUS.CRC_ERROR"]
    pub const fn error_crc(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.CRC_ERROR"]
    pub fn set_error_crc(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "Source: SIE_STATUS.BIT_STUFF_ERROR"]
    pub const fn error_bit_stuff(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.BIT_STUFF_ERROR"]
    pub fn set_error_bit_stuff(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Source: SIE_STATUS.RX_OVERFLOW"]
    pub const fn error_rx_overflow(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.RX_OVERFLOW"]
    pub fn set_error_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    #[doc = "Source: SIE_STATUS.RX_TIMEOUT"]
    pub const fn error_rx_timeout(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.RX_TIMEOUT"]
    pub fn set_error_rx_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Source: SIE_STATUS.DATA_SEQ_ERROR"]
    pub const fn error_data_seq(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    #[doc = "Source: SIE_STATUS.DATA_SEQ_ERROR"]
    pub fn set_error_data_seq(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    #[doc = "Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    pub const fn buff_status(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Raised when any bit in BUFF_STATUS is set. Clear by clearing all bits in BUFF_STATUS."]
    pub fn set_buff_status(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    pub const fn trans_complete(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Raised every time SIE_STATUS.TRANS_COMPLETE is set. Clear by writing to this bit."]
    pub fn set_trans_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    pub const fn host_sof(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Host: raised every time the host sends a SOF (Start of Frame). Cleared by reading SOF_RD"]
    pub fn set_host_sof(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    pub const fn host_resume(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Host: raised when a device wakes up the host. Cleared by writing to SIE_STATUS.RESUME"]
    pub fn set_host_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    pub const fn host_conn_dis(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Host: raised when a device is connected or disconnected (i.e. when SIE_STATUS.SPEED changes). Cleared by writing to SIE_STATUS.SPEED"]
    pub fn set_host_conn_dis(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for Int {
    fn default() -> Int {
        Int(0)
    }
}
#[doc = "Which of the double buffers should be handled. Only valid if using an interrupt per buffer (i.e. not per 2 buffers). Not valid for host interrupt endpoint polling because they are only single buffered."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct BuffCpuShouldHandle(pub u32);
impl BuffCpuShouldHandle {
    pub const fn ep15_out(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn ep15_in(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn ep14_out(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn ep14_in(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn ep13_out(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn ep13_in(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn ep12_out(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn ep12_in(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn ep11_out(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn ep11_in(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn ep10_out(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn ep10_in(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn ep9_out(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn ep9_in(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn ep8_out(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn ep8_in(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn ep7_out(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn ep7_in(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn ep6_out(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn ep6_in(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn ep5_out(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn ep5_in(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn ep4_out(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn ep4_in(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn ep3_out(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ep3_in(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ep2_out(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ep2_in(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ep1_out(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ep1_in(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for BuffCpuShouldHandle {
    fn default() -> BuffCpuShouldHandle {
        BuffCpuShouldHandle(0)
    }
}
#[doc = "Used to adjust trim values of USB phy pull down resistors."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbphyTrim(pub u32);
impl UsbphyTrim {
    #[doc = "Value to drive to USB PHY DM pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    pub const fn dm_pulldn_trim(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x1f;
        val as u8
    }
    #[doc = "Value to drive to USB PHY DM pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    pub fn set_dm_pulldn_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 8u32)) | (((val as u32) & 0x1f) << 8u32);
    }
    #[doc = "Value to drive to USB PHY DP pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    pub const fn dp_pulldn_trim(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x1f;
        val as u8
    }
    #[doc = "Value to drive to USB PHY DP pulldown resistor trim control Experimental data suggests that the reset value will work, but this register allows adjustment if required"]
    pub fn set_dp_pulldn_trim(&mut self, val: u8) {
        self.0 = (self.0 & !(0x1f << 0u32)) | (((val as u32) & 0x1f) << 0u32);
    }
}
impl Default for UsbphyTrim {
    fn default() -> UsbphyTrim {
        UsbphyTrim(0)
    }
}
#[doc = "Device address and endpoint control"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct AddrEndp(pub u32);
impl AddrEndp {
    #[doc = "Device endpoint to send data to. Only valid for HOST mode."]
    pub const fn endpoint(&self) -> u8 {
        let val = (self.0 >> 16u32) & 0x0f;
        val as u8
    }
    #[doc = "Device endpoint to send data to. Only valid for HOST mode."]
    pub fn set_endpoint(&mut self, val: u8) {
        self.0 = (self.0 & !(0x0f << 16u32)) | (((val as u32) & 0x0f) << 16u32);
    }
    #[doc = "In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    pub const fn address(&self) -> u8 {
        let val = (self.0 >> 0u32) & 0x7f;
        val as u8
    }
    #[doc = "In device mode, the address that the device should respond to. Set in response to a SET_ADDR setup packet from the host. In host mode set to the address of the device to communicate with."]
    pub fn set_address(&mut self, val: u8) {
        self.0 = (self.0 & !(0x7f << 0u32)) | (((val as u32) & 0x7f) << 0u32);
    }
}
impl Default for AddrEndp {
    fn default() -> AddrEndp {
        AddrEndp(0)
    }
}
#[doc = "Override enable for each control in usbphy_direct"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbphyDirectOverride(pub u32);
impl UsbphyDirectOverride {
    pub const fn tx_diffmode_override_en(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_tx_diffmode_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn dm_pullup_override_en(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_dm_pullup_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn tx_fsslew_override_en(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_tx_fsslew_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn tx_pd_override_en(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_tx_pd_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn rx_pd_override_en(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_rx_pd_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn tx_dm_override_en(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_tx_dm_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn tx_dp_override_en(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_tx_dp_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn tx_dm_oe_override_en(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_tx_dm_oe_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn tx_dp_oe_override_en(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_tx_dp_oe_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn dm_pulldn_en_override_en(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_dm_pulldn_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn dp_pulldn_en_override_en(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_dp_pulldn_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn dp_pullup_en_override_en(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_dp_pullup_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn dm_pullup_hisel_override_en(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_dm_pullup_hisel_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn dp_pullup_hisel_override_en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_dp_pullup_hisel_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for UsbphyDirectOverride {
    fn default() -> UsbphyDirectOverride {
        UsbphyDirectOverride(0)
    }
}
#[doc = "SIE control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SieCtrl(pub u32);
impl SieCtrl {
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    pub const fn ep0_int_stall(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a STALL"]
    pub fn set_ep0_int_stall(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "Device: EP0 single buffered = 0, double buffered = 1"]
    pub const fn ep0_double_buf(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "Device: EP0 single buffered = 0, double buffered = 1"]
    pub fn set_ep0_double_buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    pub const fn ep0_int_1buf(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every buffer completed on EP0"]
    pub fn set_ep0_int_1buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    pub const fn ep0_int_2buf(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in BUFF_STATUS for every 2 buffers completed on EP0"]
    pub fn set_ep0_int_2buf(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    pub const fn ep0_int_nak(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Set bit in EP_STATUS_STALL_NAK when EP0 sends a NAK"]
    pub fn set_ep0_int_nak(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    #[doc = "Direct bus drive enable"]
    pub const fn direct_en(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "Direct bus drive enable"]
    pub fn set_direct_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "Direct control of DP"]
    pub const fn direct_dp(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Direct control of DP"]
    pub fn set_direct_dp(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "Direct control of DM"]
    pub const fn direct_dm(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "Direct control of DM"]
    pub fn set_direct_dm(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Power down bus transceiver"]
    pub const fn transceiver_pd(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "Power down bus transceiver"]
    pub fn set_transceiver_pd(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "Device: Pull-up strength (0=1K2, 1=2k3)"]
    pub const fn rpu_opt(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Pull-up strength (0=1K2, 1=2k3)"]
    pub fn set_rpu_opt(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "Device: Enable pull up resistor"]
    pub const fn pullup_en(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Enable pull up resistor"]
    pub fn set_pullup_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Host: Enable pull down resistors"]
    pub const fn pulldown_en(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable pull down resistors"]
    pub fn set_pulldown_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    #[doc = "Host: Reset bus"]
    pub const fn reset_bus(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Reset bus"]
    pub fn set_reset_bus(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    #[doc = "Device: Remote wakeup. Device can initiate its own resume after suspend."]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Remote wakeup. Device can initiate its own resume after suspend."]
    pub fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    #[doc = "Host: Enable VBUS"]
    pub const fn vbus_en(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable VBUS"]
    pub fn set_vbus_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "Host: Enable keep alive packet (for low speed bus)"]
    pub const fn keep_alive_en(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable keep alive packet (for low speed bus)"]
    pub fn set_keep_alive_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Host: Enable SOF generation (for full speed bus)"]
    pub const fn sof_en(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Enable SOF generation (for full speed bus)"]
    pub fn set_sof_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    #[doc = "Host: Delay packet(s) until after SOF"]
    pub const fn sof_sync(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Delay packet(s) until after SOF"]
    pub fn set_sof_sync(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    #[doc = "Host: Preable enable for LS device on FS hub"]
    pub const fn preamble_en(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Preable enable for LS device on FS hub"]
    pub fn set_preamble_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    #[doc = "Host: Stop transaction"]
    pub const fn stop_trans(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Stop transaction"]
    pub fn set_stop_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "Host: Receive transaction (IN to host)"]
    pub const fn receive_data(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Receive transaction (IN to host)"]
    pub fn set_receive_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    #[doc = "Host: Send transaction (OUT from host)"]
    pub const fn send_data(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Send transaction (OUT from host)"]
    pub fn set_send_data(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    #[doc = "Host: Send Setup packet"]
    pub const fn send_setup(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Send Setup packet"]
    pub fn set_send_setup(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Host: Start transaction"]
    pub const fn start_trans(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Start transaction"]
    pub fn set_start_trans(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SieCtrl {
    fn default() -> SieCtrl {
        SieCtrl(0)
    }
}
#[doc = "Device only: Can be set to ignore the buffer control register for this endpoint in case you would like to revoke a buffer. A NAK will be sent for every access to the endpoint until this bit is cleared. A corresponding bit in `EP_ABORT_DONE` is set when it is safe to modify the buffer control register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EpAbort(pub u32);
impl EpAbort {
    pub const fn ep15_out(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn ep15_in(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn ep14_out(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn ep14_in(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn ep13_out(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn ep13_in(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn ep12_out(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn ep12_in(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn ep11_out(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn ep11_in(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn ep10_out(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn ep10_in(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn ep9_out(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn ep9_in(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn ep8_out(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn ep8_in(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn ep7_out(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn ep7_in(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn ep6_out(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn ep6_in(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn ep5_out(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn ep5_in(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn ep4_out(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn ep4_in(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn ep3_out(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ep3_in(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ep2_out(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ep2_in(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ep1_out(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ep1_in(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for EpAbort {
    fn default() -> EpAbort {
        EpAbort(0)
    }
}
#[doc = "Device only: Used in conjunction with `EP_ABORT`. Set once an endpoint is idle so the programmer knows it is safe to modify the buffer control register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EpAbortDone(pub u32);
impl EpAbortDone {
    pub const fn ep15_out(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn ep15_in(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn ep14_out(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn ep14_in(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn ep13_out(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn ep13_in(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn ep12_out(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn ep12_in(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn ep11_out(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn ep11_in(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn ep10_out(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn ep10_in(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn ep9_out(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn ep9_in(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn ep8_out(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn ep8_in(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn ep7_out(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn ep7_in(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn ep6_out(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn ep6_in(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn ep5_out(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn ep5_in(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn ep4_out(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn ep4_in(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn ep3_out(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ep3_in(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ep2_out(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ep2_in(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ep1_out(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ep1_in(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for EpAbortDone {
    fn default() -> EpAbortDone {
        EpAbortDone(0)
    }
}
#[doc = "interrupt endpoint control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct IntEpCtrl(pub u32);
impl IntEpCtrl {
    #[doc = "Host: Enable interrupt endpoint 1 -> 15"]
    pub const fn int_ep_active(&self) -> u16 {
        let val = (self.0 >> 1u32) & 0x7fff;
        val as u16
    }
    #[doc = "Host: Enable interrupt endpoint 1 -> 15"]
    pub fn set_int_ep_active(&mut self, val: u16) {
        self.0 = (self.0 & !(0x7fff << 1u32)) | (((val as u32) & 0x7fff) << 1u32);
    }
}
impl Default for IntEpCtrl {
    fn default() -> IntEpCtrl {
        IntEpCtrl(0)
    }
}
#[doc = "Set the SOF (Start of Frame) frame number in the host controller. The SOF packet is sent every 1ms and the host will increment the frame number by 1 each time."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SofWr(pub u32);
impl SofWr {
    pub const fn count(&self) -> u16 {
        let val = (self.0 >> 0u32) & 0x07ff;
        val as u16
    }
    pub fn set_count(&mut self, val: u16) {
        self.0 = (self.0 & !(0x07ff << 0u32)) | (((val as u32) & 0x07ff) << 0u32);
    }
}
impl Default for SofWr {
    fn default() -> SofWr {
        SofWr(0)
    }
}
#[doc = "Main control register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct MainCtrl(pub u32);
impl MainCtrl {
    #[doc = "Reduced timings for simulation"]
    pub const fn sim_timing(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Reduced timings for simulation"]
    pub fn set_sim_timing(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "Device mode = 0, Host mode = 1"]
    pub const fn host_ndevice(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    #[doc = "Device mode = 0, Host mode = 1"]
    pub fn set_host_ndevice(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    #[doc = "Enable controller"]
    pub const fn controller_en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Enable controller"]
    pub fn set_controller_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for MainCtrl {
    fn default() -> MainCtrl {
        MainCtrl(0)
    }
}
#[doc = "SIE status register"]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct SieStatus(pub u32);
impl SieStatus {
    #[doc = "Data Sequence Error. The device can raise a sequence error in the following conditions: * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM The host can raise a data sequence error in the following conditions: * An IN packet from the device has the wrong data PID"]
    pub const fn data_seq_error(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    #[doc = "Data Sequence Error. The device can raise a sequence error in the following conditions: * A SETUP packet is received followed by a DATA1 packet (data phase should always be DATA0) * An OUT packet is received from the host but doesn't match the data pid in the buffer control register read from DPSRAM The host can raise a data sequence error in the following conditions: * An IN packet from the device has the wrong data PID"]
    pub fn set_data_seq_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    #[doc = "ACK received. Raised by both host and device."]
    pub const fn ack_rec(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    #[doc = "ACK received. Raised by both host and device."]
    pub fn set_ack_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    #[doc = "Host: STALL received"]
    pub const fn stall_rec(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    #[doc = "Host: STALL received"]
    pub fn set_stall_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    #[doc = "Host: NAK received"]
    pub const fn nak_rec(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    #[doc = "Host: NAK received"]
    pub fn set_nak_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    #[doc = "RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    pub const fn rx_timeout(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    #[doc = "RX timeout is raised by both the host and device if an ACK is not received in the maximum time specified by the USB spec."]
    pub fn set_rx_timeout(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    #[doc = "RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    pub const fn rx_overflow(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    #[doc = "RX overflow is raised by the Serial RX engine if the incoming data is too fast."]
    pub fn set_rx_overflow(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    #[doc = "Bit Stuff Error. Raised by the Serial RX engine."]
    pub const fn bit_stuff_error(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    #[doc = "Bit Stuff Error. Raised by the Serial RX engine."]
    pub fn set_bit_stuff_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    #[doc = "CRC Error. Raised by the Serial RX engine."]
    pub const fn crc_error(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    #[doc = "CRC Error. Raised by the Serial RX engine."]
    pub fn set_crc_error(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    #[doc = "Device: bus reset received"]
    pub const fn bus_reset(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    #[doc = "Device: bus reset received"]
    pub fn set_bus_reset(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    #[doc = "Transaction complete. Raised by device if: * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register Raised by host if: * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    pub const fn trans_complete(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    #[doc = "Transaction complete. Raised by device if: * An IN or OUT packet is sent with the `LAST_BUFF` bit set in the buffer control register Raised by host if: * A setup packet is sent when no data in or data out transaction follows * An IN packet is received and the `LAST_BUFF` bit is set in the buffer control register * An IN packet is received with zero length * An OUT packet is sent and the `LAST_BUFF` bit is set"]
    pub fn set_trans_complete(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    #[doc = "Device: Setup packet received"]
    pub const fn setup_rec(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    #[doc = "Device: Setup packet received"]
    pub fn set_setup_rec(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    #[doc = "Device: connected"]
    pub const fn connected(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    #[doc = "Device: connected"]
    pub fn set_connected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    #[doc = "Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    pub const fn resume(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    #[doc = "Host: Device has initiated a remote resume. Device: host has initiated a resume."]
    pub fn set_resume(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    #[doc = "VBUS over current detected"]
    pub const fn vbus_over_curr(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    #[doc = "VBUS over current detected"]
    pub fn set_vbus_over_curr(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    #[doc = "Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    pub const fn speed(&self) -> u8 {
        let val = (self.0 >> 8u32) & 0x03;
        val as u8
    }
    #[doc = "Host: device speed. Disconnected = 00, LS = 01, FS = 10"]
    pub fn set_speed(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 8u32)) | (((val as u32) & 0x03) << 8u32);
    }
    #[doc = "Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    pub const fn suspended(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    #[doc = "Bus in suspended state. Valid for device and host. Host and device will go into suspend if neither Keep Alive / SOF frames are enabled."]
    pub fn set_suspended(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    #[doc = "USB bus line state"]
    pub const fn line_state(&self) -> u8 {
        let val = (self.0 >> 2u32) & 0x03;
        val as u8
    }
    #[doc = "USB bus line state"]
    pub fn set_line_state(&mut self, val: u8) {
        self.0 = (self.0 & !(0x03 << 2u32)) | (((val as u32) & 0x03) << 2u32);
    }
    #[doc = "Device: VBUS Detected"]
    pub const fn vbus_detected(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    #[doc = "Device: VBUS Detected"]
    pub fn set_vbus_detected(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for SieStatus {
    fn default() -> SieStatus {
        SieStatus(0)
    }
}
#[doc = "Device: bits are set when the `IRQ_ON_NAK` or `IRQ_ON_STALL` bits are set. For EP0 this comes from `SIE_CTRL`. For all other endpoints it comes from the endpoint control register."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct EpStatusStallNak(pub u32);
impl EpStatusStallNak {
    pub const fn ep15_out(&self) -> bool {
        let val = (self.0 >> 31u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 31u32)) | (((val as u32) & 0x01) << 31u32);
    }
    pub const fn ep15_in(&self) -> bool {
        let val = (self.0 >> 30u32) & 0x01;
        val != 0
    }
    pub fn set_ep15_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 30u32)) | (((val as u32) & 0x01) << 30u32);
    }
    pub const fn ep14_out(&self) -> bool {
        let val = (self.0 >> 29u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 29u32)) | (((val as u32) & 0x01) << 29u32);
    }
    pub const fn ep14_in(&self) -> bool {
        let val = (self.0 >> 28u32) & 0x01;
        val != 0
    }
    pub fn set_ep14_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 28u32)) | (((val as u32) & 0x01) << 28u32);
    }
    pub const fn ep13_out(&self) -> bool {
        let val = (self.0 >> 27u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 27u32)) | (((val as u32) & 0x01) << 27u32);
    }
    pub const fn ep13_in(&self) -> bool {
        let val = (self.0 >> 26u32) & 0x01;
        val != 0
    }
    pub fn set_ep13_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 26u32)) | (((val as u32) & 0x01) << 26u32);
    }
    pub const fn ep12_out(&self) -> bool {
        let val = (self.0 >> 25u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 25u32)) | (((val as u32) & 0x01) << 25u32);
    }
    pub const fn ep12_in(&self) -> bool {
        let val = (self.0 >> 24u32) & 0x01;
        val != 0
    }
    pub fn set_ep12_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 24u32)) | (((val as u32) & 0x01) << 24u32);
    }
    pub const fn ep11_out(&self) -> bool {
        let val = (self.0 >> 23u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 23u32)) | (((val as u32) & 0x01) << 23u32);
    }
    pub const fn ep11_in(&self) -> bool {
        let val = (self.0 >> 22u32) & 0x01;
        val != 0
    }
    pub fn set_ep11_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 22u32)) | (((val as u32) & 0x01) << 22u32);
    }
    pub const fn ep10_out(&self) -> bool {
        let val = (self.0 >> 21u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 21u32)) | (((val as u32) & 0x01) << 21u32);
    }
    pub const fn ep10_in(&self) -> bool {
        let val = (self.0 >> 20u32) & 0x01;
        val != 0
    }
    pub fn set_ep10_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 20u32)) | (((val as u32) & 0x01) << 20u32);
    }
    pub const fn ep9_out(&self) -> bool {
        let val = (self.0 >> 19u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 19u32)) | (((val as u32) & 0x01) << 19u32);
    }
    pub const fn ep9_in(&self) -> bool {
        let val = (self.0 >> 18u32) & 0x01;
        val != 0
    }
    pub fn set_ep9_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 18u32)) | (((val as u32) & 0x01) << 18u32);
    }
    pub const fn ep8_out(&self) -> bool {
        let val = (self.0 >> 17u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 17u32)) | (((val as u32) & 0x01) << 17u32);
    }
    pub const fn ep8_in(&self) -> bool {
        let val = (self.0 >> 16u32) & 0x01;
        val != 0
    }
    pub fn set_ep8_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 16u32)) | (((val as u32) & 0x01) << 16u32);
    }
    pub const fn ep7_out(&self) -> bool {
        let val = (self.0 >> 15u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 15u32)) | (((val as u32) & 0x01) << 15u32);
    }
    pub const fn ep7_in(&self) -> bool {
        let val = (self.0 >> 14u32) & 0x01;
        val != 0
    }
    pub fn set_ep7_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 14u32)) | (((val as u32) & 0x01) << 14u32);
    }
    pub const fn ep6_out(&self) -> bool {
        let val = (self.0 >> 13u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 13u32)) | (((val as u32) & 0x01) << 13u32);
    }
    pub const fn ep6_in(&self) -> bool {
        let val = (self.0 >> 12u32) & 0x01;
        val != 0
    }
    pub fn set_ep6_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 12u32)) | (((val as u32) & 0x01) << 12u32);
    }
    pub const fn ep5_out(&self) -> bool {
        let val = (self.0 >> 11u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 11u32)) | (((val as u32) & 0x01) << 11u32);
    }
    pub const fn ep5_in(&self) -> bool {
        let val = (self.0 >> 10u32) & 0x01;
        val != 0
    }
    pub fn set_ep5_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 10u32)) | (((val as u32) & 0x01) << 10u32);
    }
    pub const fn ep4_out(&self) -> bool {
        let val = (self.0 >> 9u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 9u32)) | (((val as u32) & 0x01) << 9u32);
    }
    pub const fn ep4_in(&self) -> bool {
        let val = (self.0 >> 8u32) & 0x01;
        val != 0
    }
    pub fn set_ep4_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 8u32)) | (((val as u32) & 0x01) << 8u32);
    }
    pub const fn ep3_out(&self) -> bool {
        let val = (self.0 >> 7u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 7u32)) | (((val as u32) & 0x01) << 7u32);
    }
    pub const fn ep3_in(&self) -> bool {
        let val = (self.0 >> 6u32) & 0x01;
        val != 0
    }
    pub fn set_ep3_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 6u32)) | (((val as u32) & 0x01) << 6u32);
    }
    pub const fn ep2_out(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn ep2_in(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_ep2_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn ep1_out(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn ep1_in(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_ep1_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn ep0_out(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_out(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn ep0_in(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_ep0_in(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for EpStatusStallNak {
    fn default() -> EpStatusStallNak {
        EpStatusStallNak(0)
    }
}
#[doc = "Overrides for the power signals in the event that the VBUS signals are not hooked up to GPIO. Set the value of the override and then the override enable to switch over to the override value."]
#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct UsbPwr(pub u32);
impl UsbPwr {
    pub const fn overcurr_detect_en(&self) -> bool {
        let val = (self.0 >> 5u32) & 0x01;
        val != 0
    }
    pub fn set_overcurr_detect_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 5u32)) | (((val as u32) & 0x01) << 5u32);
    }
    pub const fn overcurr_detect(&self) -> bool {
        let val = (self.0 >> 4u32) & 0x01;
        val != 0
    }
    pub fn set_overcurr_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 4u32)) | (((val as u32) & 0x01) << 4u32);
    }
    pub const fn vbus_detect_override_en(&self) -> bool {
        let val = (self.0 >> 3u32) & 0x01;
        val != 0
    }
    pub fn set_vbus_detect_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 3u32)) | (((val as u32) & 0x01) << 3u32);
    }
    pub const fn vbus_detect(&self) -> bool {
        let val = (self.0 >> 2u32) & 0x01;
        val != 0
    }
    pub fn set_vbus_detect(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 2u32)) | (((val as u32) & 0x01) << 2u32);
    }
    pub const fn vbus_en_override_en(&self) -> bool {
        let val = (self.0 >> 1u32) & 0x01;
        val != 0
    }
    pub fn set_vbus_en_override_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 1u32)) | (((val as u32) & 0x01) << 1u32);
    }
    pub const fn vbus_en(&self) -> bool {
        let val = (self.0 >> 0u32) & 0x01;
        val != 0
    }
    pub fn set_vbus_en(&mut self, val: bool) {
        self.0 = (self.0 & !(0x01 << 0u32)) | (((val as u32) & 0x01) << 0u32);
    }
}
impl Default for UsbPwr {
    fn default() -> UsbPwr {
        UsbPwr(0)
    }
}
