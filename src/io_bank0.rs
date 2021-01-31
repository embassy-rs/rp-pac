use crate::generic::*;
#[derive(Copy, Clone)]
pub struct IoBank0(*mut u8);
unsafe impl Send for IoBank0 {}
unsafe impl Sync for IoBank0 {}
impl IoBank0 {
    pub const fn from_ptr(ptr: *mut u8) -> Self {
        Self(ptr)
    }
    #[doc = "GPIO status"]
    pub fn gpio0_status(self) -> Reg<fields::Gpio0Status, RW> {
        unsafe { Reg::new(self.0.add(0usize), fields::Gpio0Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio0_ctrl(self) -> Reg<fields::Gpio0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(4usize), fields::Gpio0Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio1_status(self) -> Reg<fields::Gpio1Status, RW> {
        unsafe { Reg::new(self.0.add(8usize), fields::Gpio1Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio1_ctrl(self) -> Reg<fields::Gpio1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(12usize), fields::Gpio1Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio2_status(self) -> Reg<fields::Gpio2Status, RW> {
        unsafe { Reg::new(self.0.add(16usize), fields::Gpio2Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio2_ctrl(self) -> Reg<fields::Gpio2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(20usize), fields::Gpio2Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio3_status(self) -> Reg<fields::Gpio3Status, RW> {
        unsafe { Reg::new(self.0.add(24usize), fields::Gpio3Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio3_ctrl(self) -> Reg<fields::Gpio3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(28usize), fields::Gpio3Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio4_status(self) -> Reg<fields::Gpio4Status, RW> {
        unsafe { Reg::new(self.0.add(32usize), fields::Gpio4Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio4_ctrl(self) -> Reg<fields::Gpio4Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize), fields::Gpio4Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio5_status(self) -> Reg<fields::Gpio5Status, RW> {
        unsafe { Reg::new(self.0.add(40usize), fields::Gpio5Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio5_ctrl(self) -> Reg<fields::Gpio5Ctrl, RW> {
        unsafe { Reg::new(self.0.add(44usize), fields::Gpio5Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio6_status(self) -> Reg<fields::Gpio6Status, RW> {
        unsafe { Reg::new(self.0.add(48usize), fields::Gpio6Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio6_ctrl(self) -> Reg<fields::Gpio6Ctrl, RW> {
        unsafe { Reg::new(self.0.add(52usize), fields::Gpio6Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio7_status(self) -> Reg<fields::Gpio7Status, RW> {
        unsafe { Reg::new(self.0.add(56usize), fields::Gpio7Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio7_ctrl(self) -> Reg<fields::Gpio7Ctrl, RW> {
        unsafe { Reg::new(self.0.add(60usize), fields::Gpio7Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio8_status(self) -> Reg<fields::Gpio8Status, RW> {
        unsafe { Reg::new(self.0.add(64usize), fields::Gpio8Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio8_ctrl(self) -> Reg<fields::Gpio8Ctrl, RW> {
        unsafe { Reg::new(self.0.add(68usize), fields::Gpio8Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio9_status(self) -> Reg<fields::Gpio9Status, RW> {
        unsafe { Reg::new(self.0.add(72usize), fields::Gpio9Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio9_ctrl(self) -> Reg<fields::Gpio9Ctrl, RW> {
        unsafe { Reg::new(self.0.add(76usize), fields::Gpio9Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio10_status(self) -> Reg<fields::Gpio10Status, RW> {
        unsafe { Reg::new(self.0.add(80usize), fields::Gpio10Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio10_ctrl(self) -> Reg<fields::Gpio10Ctrl, RW> {
        unsafe { Reg::new(self.0.add(84usize), fields::Gpio10Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio11_status(self) -> Reg<fields::Gpio11Status, RW> {
        unsafe { Reg::new(self.0.add(88usize), fields::Gpio11Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio11_ctrl(self) -> Reg<fields::Gpio11Ctrl, RW> {
        unsafe { Reg::new(self.0.add(92usize), fields::Gpio11Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio12_status(self) -> Reg<fields::Gpio12Status, RW> {
        unsafe { Reg::new(self.0.add(96usize), fields::Gpio12Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio12_ctrl(self) -> Reg<fields::Gpio12Ctrl, RW> {
        unsafe { Reg::new(self.0.add(100usize), fields::Gpio12Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio13_status(self) -> Reg<fields::Gpio13Status, RW> {
        unsafe { Reg::new(self.0.add(104usize), fields::Gpio13Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio13_ctrl(self) -> Reg<fields::Gpio13Ctrl, RW> {
        unsafe { Reg::new(self.0.add(108usize), fields::Gpio13Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio14_status(self) -> Reg<fields::Gpio14Status, RW> {
        unsafe { Reg::new(self.0.add(112usize), fields::Gpio14Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio14_ctrl(self) -> Reg<fields::Gpio14Ctrl, RW> {
        unsafe { Reg::new(self.0.add(116usize), fields::Gpio14Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio15_status(self) -> Reg<fields::Gpio15Status, RW> {
        unsafe { Reg::new(self.0.add(120usize), fields::Gpio15Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio15_ctrl(self) -> Reg<fields::Gpio15Ctrl, RW> {
        unsafe { Reg::new(self.0.add(124usize), fields::Gpio15Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio16_status(self) -> Reg<fields::Gpio16Status, RW> {
        unsafe { Reg::new(self.0.add(128usize), fields::Gpio16Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio16_ctrl(self) -> Reg<fields::Gpio16Ctrl, RW> {
        unsafe { Reg::new(self.0.add(132usize), fields::Gpio16Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio17_status(self) -> Reg<fields::Gpio17Status, RW> {
        unsafe { Reg::new(self.0.add(136usize), fields::Gpio17Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio17_ctrl(self) -> Reg<fields::Gpio17Ctrl, RW> {
        unsafe { Reg::new(self.0.add(140usize), fields::Gpio17Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio18_status(self) -> Reg<fields::Gpio18Status, RW> {
        unsafe { Reg::new(self.0.add(144usize), fields::Gpio18Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio18_ctrl(self) -> Reg<fields::Gpio18Ctrl, RW> {
        unsafe { Reg::new(self.0.add(148usize), fields::Gpio18Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio19_status(self) -> Reg<fields::Gpio19Status, RW> {
        unsafe { Reg::new(self.0.add(152usize), fields::Gpio19Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio19_ctrl(self) -> Reg<fields::Gpio19Ctrl, RW> {
        unsafe { Reg::new(self.0.add(156usize), fields::Gpio19Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio20_status(self) -> Reg<fields::Gpio20Status, RW> {
        unsafe { Reg::new(self.0.add(160usize), fields::Gpio20Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio20_ctrl(self) -> Reg<fields::Gpio20Ctrl, RW> {
        unsafe { Reg::new(self.0.add(164usize), fields::Gpio20Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio21_status(self) -> Reg<fields::Gpio21Status, RW> {
        unsafe { Reg::new(self.0.add(168usize), fields::Gpio21Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio21_ctrl(self) -> Reg<fields::Gpio21Ctrl, RW> {
        unsafe { Reg::new(self.0.add(172usize), fields::Gpio21Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio22_status(self) -> Reg<fields::Gpio22Status, RW> {
        unsafe { Reg::new(self.0.add(176usize), fields::Gpio22Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio22_ctrl(self) -> Reg<fields::Gpio22Ctrl, RW> {
        unsafe { Reg::new(self.0.add(180usize), fields::Gpio22Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio23_status(self) -> Reg<fields::Gpio23Status, RW> {
        unsafe { Reg::new(self.0.add(184usize), fields::Gpio23Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio23_ctrl(self) -> Reg<fields::Gpio23Ctrl, RW> {
        unsafe { Reg::new(self.0.add(188usize), fields::Gpio23Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio24_status(self) -> Reg<fields::Gpio24Status, RW> {
        unsafe { Reg::new(self.0.add(192usize), fields::Gpio24Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio24_ctrl(self) -> Reg<fields::Gpio24Ctrl, RW> {
        unsafe { Reg::new(self.0.add(196usize), fields::Gpio24Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio25_status(self) -> Reg<fields::Gpio25Status, RW> {
        unsafe { Reg::new(self.0.add(200usize), fields::Gpio25Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio25_ctrl(self) -> Reg<fields::Gpio25Ctrl, RW> {
        unsafe { Reg::new(self.0.add(204usize), fields::Gpio25Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio26_status(self) -> Reg<fields::Gpio26Status, RW> {
        unsafe { Reg::new(self.0.add(208usize), fields::Gpio26Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio26_ctrl(self) -> Reg<fields::Gpio26Ctrl, RW> {
        unsafe { Reg::new(self.0.add(212usize), fields::Gpio26Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio27_status(self) -> Reg<fields::Gpio27Status, RW> {
        unsafe { Reg::new(self.0.add(216usize), fields::Gpio27Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio27_ctrl(self) -> Reg<fields::Gpio27Ctrl, RW> {
        unsafe { Reg::new(self.0.add(220usize), fields::Gpio27Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio28_status(self) -> Reg<fields::Gpio28Status, RW> {
        unsafe { Reg::new(self.0.add(224usize), fields::Gpio28Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio28_ctrl(self) -> Reg<fields::Gpio28Ctrl, RW> {
        unsafe { Reg::new(self.0.add(228usize), fields::Gpio28Ctrl::from_bits(31)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio29_status(self) -> Reg<fields::Gpio29Status, RW> {
        unsafe { Reg::new(self.0.add(232usize), fields::Gpio29Status::from_bits(0)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio29_ctrl(self) -> Reg<fields::Gpio29Ctrl, RW> {
        unsafe { Reg::new(self.0.add(236usize), fields::Gpio29Ctrl::from_bits(31)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr0(self) -> Reg<fields::Intr0, RW> {
        unsafe { Reg::new(self.0.add(240usize), fields::Intr0::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr1(self) -> Reg<fields::Intr1, RW> {
        unsafe { Reg::new(self.0.add(244usize), fields::Intr1::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr2(self) -> Reg<fields::Intr2, RW> {
        unsafe { Reg::new(self.0.add(248usize), fields::Intr2::from_bits(0)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr3(self) -> Reg<fields::Intr3, RW> {
        unsafe { Reg::new(self.0.add(252usize), fields::Intr3::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte0(self) -> Reg<fields::Proc0Inte0, RW> {
        unsafe { Reg::new(self.0.add(256usize), fields::Proc0Inte0::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte1(self) -> Reg<fields::Proc0Inte1, RW> {
        unsafe { Reg::new(self.0.add(260usize), fields::Proc0Inte1::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte2(self) -> Reg<fields::Proc0Inte2, RW> {
        unsafe { Reg::new(self.0.add(264usize), fields::Proc0Inte2::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte3(self) -> Reg<fields::Proc0Inte3, RW> {
        unsafe { Reg::new(self.0.add(268usize), fields::Proc0Inte3::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf0(self) -> Reg<fields::Proc0Intf0, RW> {
        unsafe { Reg::new(self.0.add(272usize), fields::Proc0Intf0::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf1(self) -> Reg<fields::Proc0Intf1, RW> {
        unsafe { Reg::new(self.0.add(276usize), fields::Proc0Intf1::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf2(self) -> Reg<fields::Proc0Intf2, RW> {
        unsafe { Reg::new(self.0.add(280usize), fields::Proc0Intf2::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf3(self) -> Reg<fields::Proc0Intf3, RW> {
        unsafe { Reg::new(self.0.add(284usize), fields::Proc0Intf3::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints0(self) -> Reg<fields::Proc0Ints0, RW> {
        unsafe { Reg::new(self.0.add(288usize), fields::Proc0Ints0::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints1(self) -> Reg<fields::Proc0Ints1, RW> {
        unsafe { Reg::new(self.0.add(292usize), fields::Proc0Ints1::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints2(self) -> Reg<fields::Proc0Ints2, RW> {
        unsafe { Reg::new(self.0.add(296usize), fields::Proc0Ints2::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints3(self) -> Reg<fields::Proc0Ints3, RW> {
        unsafe { Reg::new(self.0.add(300usize), fields::Proc0Ints3::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte0(self) -> Reg<fields::Proc1Inte0, RW> {
        unsafe { Reg::new(self.0.add(304usize), fields::Proc1Inte0::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte1(self) -> Reg<fields::Proc1Inte1, RW> {
        unsafe { Reg::new(self.0.add(308usize), fields::Proc1Inte1::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte2(self) -> Reg<fields::Proc1Inte2, RW> {
        unsafe { Reg::new(self.0.add(312usize), fields::Proc1Inte2::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte3(self) -> Reg<fields::Proc1Inte3, RW> {
        unsafe { Reg::new(self.0.add(316usize), fields::Proc1Inte3::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf0(self) -> Reg<fields::Proc1Intf0, RW> {
        unsafe { Reg::new(self.0.add(320usize), fields::Proc1Intf0::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf1(self) -> Reg<fields::Proc1Intf1, RW> {
        unsafe { Reg::new(self.0.add(324usize), fields::Proc1Intf1::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf2(self) -> Reg<fields::Proc1Intf2, RW> {
        unsafe { Reg::new(self.0.add(328usize), fields::Proc1Intf2::from_bits(0)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf3(self) -> Reg<fields::Proc1Intf3, RW> {
        unsafe { Reg::new(self.0.add(332usize), fields::Proc1Intf3::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints0(self) -> Reg<fields::Proc1Ints0, RW> {
        unsafe { Reg::new(self.0.add(336usize), fields::Proc1Ints0::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints1(self) -> Reg<fields::Proc1Ints1, RW> {
        unsafe { Reg::new(self.0.add(340usize), fields::Proc1Ints1::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints2(self) -> Reg<fields::Proc1Ints2, RW> {
        unsafe { Reg::new(self.0.add(344usize), fields::Proc1Ints2::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints3(self) -> Reg<fields::Proc1Ints3, RW> {
        unsafe { Reg::new(self.0.add(348usize), fields::Proc1Ints3::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte0(self) -> Reg<fields::DormantWakeInte0, RW> {
        unsafe { Reg::new(self.0.add(352usize), fields::DormantWakeInte0::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte1(self) -> Reg<fields::DormantWakeInte1, RW> {
        unsafe { Reg::new(self.0.add(356usize), fields::DormantWakeInte1::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte2(self) -> Reg<fields::DormantWakeInte2, RW> {
        unsafe { Reg::new(self.0.add(360usize), fields::DormantWakeInte2::from_bits(0)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte3(self) -> Reg<fields::DormantWakeInte3, RW> {
        unsafe { Reg::new(self.0.add(364usize), fields::DormantWakeInte3::from_bits(0)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf0(self) -> Reg<fields::DormantWakeIntf0, RW> {
        unsafe { Reg::new(self.0.add(368usize), fields::DormantWakeIntf0::from_bits(0)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf1(self) -> Reg<fields::DormantWakeIntf1, RW> {
        unsafe { Reg::new(self.0.add(372usize), fields::DormantWakeIntf1::from_bits(0)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf2(self) -> Reg<fields::DormantWakeIntf2, RW> {
        unsafe { Reg::new(self.0.add(376usize), fields::DormantWakeIntf2::from_bits(0)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf3(self) -> Reg<fields::DormantWakeIntf3, RW> {
        unsafe { Reg::new(self.0.add(380usize), fields::DormantWakeIntf3::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints0(self) -> Reg<fields::DormantWakeInts0, RW> {
        unsafe { Reg::new(self.0.add(384usize), fields::DormantWakeInts0::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints1(self) -> Reg<fields::DormantWakeInts1, RW> {
        unsafe { Reg::new(self.0.add(388usize), fields::DormantWakeInts1::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints2(self) -> Reg<fields::DormantWakeInts2, RW> {
        unsafe { Reg::new(self.0.add(392usize), fields::DormantWakeInts2::from_bits(0)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints3(self) -> Reg<fields::DormantWakeInts3, RW> {
        unsafe { Reg::new(self.0.add(396usize), fields::DormantWakeInts3::from_bits(0)) }
    }
}
pub mod fields;
pub mod values;
