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
    pub fn gpio0_status(self) -> Reg<regs::Gpio0Status, RW> {
        unsafe { Reg::new(self.0.add(0usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio0_ctrl(self) -> Reg<regs::Gpio0Ctrl, RW> {
        unsafe { Reg::new(self.0.add(4usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio1_status(self) -> Reg<regs::Gpio1Status, RW> {
        unsafe { Reg::new(self.0.add(8usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio1_ctrl(self) -> Reg<regs::Gpio1Ctrl, RW> {
        unsafe { Reg::new(self.0.add(12usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio2_status(self) -> Reg<regs::Gpio2Status, RW> {
        unsafe { Reg::new(self.0.add(16usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio2_ctrl(self) -> Reg<regs::Gpio2Ctrl, RW> {
        unsafe { Reg::new(self.0.add(20usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio3_status(self) -> Reg<regs::Gpio3Status, RW> {
        unsafe { Reg::new(self.0.add(24usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio3_ctrl(self) -> Reg<regs::Gpio3Ctrl, RW> {
        unsafe { Reg::new(self.0.add(28usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio4_status(self) -> Reg<regs::Gpio4Status, RW> {
        unsafe { Reg::new(self.0.add(32usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio4_ctrl(self) -> Reg<regs::Gpio4Ctrl, RW> {
        unsafe { Reg::new(self.0.add(36usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio5_status(self) -> Reg<regs::Gpio5Status, RW> {
        unsafe { Reg::new(self.0.add(40usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio5_ctrl(self) -> Reg<regs::Gpio5Ctrl, RW> {
        unsafe { Reg::new(self.0.add(44usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio6_status(self) -> Reg<regs::Gpio6Status, RW> {
        unsafe { Reg::new(self.0.add(48usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio6_ctrl(self) -> Reg<regs::Gpio6Ctrl, RW> {
        unsafe { Reg::new(self.0.add(52usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio7_status(self) -> Reg<regs::Gpio7Status, RW> {
        unsafe { Reg::new(self.0.add(56usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio7_ctrl(self) -> Reg<regs::Gpio7Ctrl, RW> {
        unsafe { Reg::new(self.0.add(60usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio8_status(self) -> Reg<regs::Gpio8Status, RW> {
        unsafe { Reg::new(self.0.add(64usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio8_ctrl(self) -> Reg<regs::Gpio8Ctrl, RW> {
        unsafe { Reg::new(self.0.add(68usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio9_status(self) -> Reg<regs::Gpio9Status, RW> {
        unsafe { Reg::new(self.0.add(72usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio9_ctrl(self) -> Reg<regs::Gpio9Ctrl, RW> {
        unsafe { Reg::new(self.0.add(76usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio10_status(self) -> Reg<regs::Gpio10Status, RW> {
        unsafe { Reg::new(self.0.add(80usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio10_ctrl(self) -> Reg<regs::Gpio10Ctrl, RW> {
        unsafe { Reg::new(self.0.add(84usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio11_status(self) -> Reg<regs::Gpio11Status, RW> {
        unsafe { Reg::new(self.0.add(88usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio11_ctrl(self) -> Reg<regs::Gpio11Ctrl, RW> {
        unsafe { Reg::new(self.0.add(92usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio12_status(self) -> Reg<regs::Gpio12Status, RW> {
        unsafe { Reg::new(self.0.add(96usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio12_ctrl(self) -> Reg<regs::Gpio12Ctrl, RW> {
        unsafe { Reg::new(self.0.add(100usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio13_status(self) -> Reg<regs::Gpio13Status, RW> {
        unsafe { Reg::new(self.0.add(104usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio13_ctrl(self) -> Reg<regs::Gpio13Ctrl, RW> {
        unsafe { Reg::new(self.0.add(108usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio14_status(self) -> Reg<regs::Gpio14Status, RW> {
        unsafe { Reg::new(self.0.add(112usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio14_ctrl(self) -> Reg<regs::Gpio14Ctrl, RW> {
        unsafe { Reg::new(self.0.add(116usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio15_status(self) -> Reg<regs::Gpio15Status, RW> {
        unsafe { Reg::new(self.0.add(120usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio15_ctrl(self) -> Reg<regs::Gpio15Ctrl, RW> {
        unsafe { Reg::new(self.0.add(124usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio16_status(self) -> Reg<regs::Gpio16Status, RW> {
        unsafe { Reg::new(self.0.add(128usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio16_ctrl(self) -> Reg<regs::Gpio16Ctrl, RW> {
        unsafe { Reg::new(self.0.add(132usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio17_status(self) -> Reg<regs::Gpio17Status, RW> {
        unsafe { Reg::new(self.0.add(136usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio17_ctrl(self) -> Reg<regs::Gpio17Ctrl, RW> {
        unsafe { Reg::new(self.0.add(140usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio18_status(self) -> Reg<regs::Gpio18Status, RW> {
        unsafe { Reg::new(self.0.add(144usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio18_ctrl(self) -> Reg<regs::Gpio18Ctrl, RW> {
        unsafe { Reg::new(self.0.add(148usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio19_status(self) -> Reg<regs::Gpio19Status, RW> {
        unsafe { Reg::new(self.0.add(152usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio19_ctrl(self) -> Reg<regs::Gpio19Ctrl, RW> {
        unsafe { Reg::new(self.0.add(156usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio20_status(self) -> Reg<regs::Gpio20Status, RW> {
        unsafe { Reg::new(self.0.add(160usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio20_ctrl(self) -> Reg<regs::Gpio20Ctrl, RW> {
        unsafe { Reg::new(self.0.add(164usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio21_status(self) -> Reg<regs::Gpio21Status, RW> {
        unsafe { Reg::new(self.0.add(168usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio21_ctrl(self) -> Reg<regs::Gpio21Ctrl, RW> {
        unsafe { Reg::new(self.0.add(172usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio22_status(self) -> Reg<regs::Gpio22Status, RW> {
        unsafe { Reg::new(self.0.add(176usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio22_ctrl(self) -> Reg<regs::Gpio22Ctrl, RW> {
        unsafe { Reg::new(self.0.add(180usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio23_status(self) -> Reg<regs::Gpio23Status, RW> {
        unsafe { Reg::new(self.0.add(184usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio23_ctrl(self) -> Reg<regs::Gpio23Ctrl, RW> {
        unsafe { Reg::new(self.0.add(188usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio24_status(self) -> Reg<regs::Gpio24Status, RW> {
        unsafe { Reg::new(self.0.add(192usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio24_ctrl(self) -> Reg<regs::Gpio24Ctrl, RW> {
        unsafe { Reg::new(self.0.add(196usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio25_status(self) -> Reg<regs::Gpio25Status, RW> {
        unsafe { Reg::new(self.0.add(200usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio25_ctrl(self) -> Reg<regs::Gpio25Ctrl, RW> {
        unsafe { Reg::new(self.0.add(204usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio26_status(self) -> Reg<regs::Gpio26Status, RW> {
        unsafe { Reg::new(self.0.add(208usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio26_ctrl(self) -> Reg<regs::Gpio26Ctrl, RW> {
        unsafe { Reg::new(self.0.add(212usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio27_status(self) -> Reg<regs::Gpio27Status, RW> {
        unsafe { Reg::new(self.0.add(216usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio27_ctrl(self) -> Reg<regs::Gpio27Ctrl, RW> {
        unsafe { Reg::new(self.0.add(220usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio28_status(self) -> Reg<regs::Gpio28Status, RW> {
        unsafe { Reg::new(self.0.add(224usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio28_ctrl(self) -> Reg<regs::Gpio28Ctrl, RW> {
        unsafe { Reg::new(self.0.add(228usize)) }
    }
    #[doc = "GPIO status"]
    pub fn gpio29_status(self) -> Reg<regs::Gpio29Status, RW> {
        unsafe { Reg::new(self.0.add(232usize)) }
    }
    #[doc = "GPIO control including function select and overrides."]
    pub fn gpio29_ctrl(self) -> Reg<regs::Gpio29Ctrl, RW> {
        unsafe { Reg::new(self.0.add(236usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr0(self) -> Reg<regs::Intr0, RW> {
        unsafe { Reg::new(self.0.add(240usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr1(self) -> Reg<regs::Intr1, RW> {
        unsafe { Reg::new(self.0.add(244usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr2(self) -> Reg<regs::Intr2, RW> {
        unsafe { Reg::new(self.0.add(248usize)) }
    }
    #[doc = "Raw Interrupts"]
    pub fn intr3(self) -> Reg<regs::Intr3, RW> {
        unsafe { Reg::new(self.0.add(252usize)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte0(self) -> Reg<regs::Proc0Inte0, RW> {
        unsafe { Reg::new(self.0.add(256usize)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte1(self) -> Reg<regs::Proc0Inte1, RW> {
        unsafe { Reg::new(self.0.add(260usize)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte2(self) -> Reg<regs::Proc0Inte2, RW> {
        unsafe { Reg::new(self.0.add(264usize)) }
    }
    #[doc = "Interrupt Enable for proc0"]
    pub fn proc0_inte3(self) -> Reg<regs::Proc0Inte3, RW> {
        unsafe { Reg::new(self.0.add(268usize)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf0(self) -> Reg<regs::Proc0Intf0, RW> {
        unsafe { Reg::new(self.0.add(272usize)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf1(self) -> Reg<regs::Proc0Intf1, RW> {
        unsafe { Reg::new(self.0.add(276usize)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf2(self) -> Reg<regs::Proc0Intf2, RW> {
        unsafe { Reg::new(self.0.add(280usize)) }
    }
    #[doc = "Interrupt Force for proc0"]
    pub fn proc0_intf3(self) -> Reg<regs::Proc0Intf3, RW> {
        unsafe { Reg::new(self.0.add(284usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints0(self) -> Reg<regs::Proc0Ints0, RW> {
        unsafe { Reg::new(self.0.add(288usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints1(self) -> Reg<regs::Proc0Ints1, RW> {
        unsafe { Reg::new(self.0.add(292usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints2(self) -> Reg<regs::Proc0Ints2, RW> {
        unsafe { Reg::new(self.0.add(296usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc0"]
    pub fn proc0_ints3(self) -> Reg<regs::Proc0Ints3, RW> {
        unsafe { Reg::new(self.0.add(300usize)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte0(self) -> Reg<regs::Proc1Inte0, RW> {
        unsafe { Reg::new(self.0.add(304usize)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte1(self) -> Reg<regs::Proc1Inte1, RW> {
        unsafe { Reg::new(self.0.add(308usize)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte2(self) -> Reg<regs::Proc1Inte2, RW> {
        unsafe { Reg::new(self.0.add(312usize)) }
    }
    #[doc = "Interrupt Enable for proc1"]
    pub fn proc1_inte3(self) -> Reg<regs::Proc1Inte3, RW> {
        unsafe { Reg::new(self.0.add(316usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf0(self) -> Reg<regs::Proc1Intf0, RW> {
        unsafe { Reg::new(self.0.add(320usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf1(self) -> Reg<regs::Proc1Intf1, RW> {
        unsafe { Reg::new(self.0.add(324usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf2(self) -> Reg<regs::Proc1Intf2, RW> {
        unsafe { Reg::new(self.0.add(328usize)) }
    }
    #[doc = "Interrupt Force for proc1"]
    pub fn proc1_intf3(self) -> Reg<regs::Proc1Intf3, RW> {
        unsafe { Reg::new(self.0.add(332usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints0(self) -> Reg<regs::Proc1Ints0, RW> {
        unsafe { Reg::new(self.0.add(336usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints1(self) -> Reg<regs::Proc1Ints1, RW> {
        unsafe { Reg::new(self.0.add(340usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints2(self) -> Reg<regs::Proc1Ints2, RW> {
        unsafe { Reg::new(self.0.add(344usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for proc1"]
    pub fn proc1_ints3(self) -> Reg<regs::Proc1Ints3, RW> {
        unsafe { Reg::new(self.0.add(348usize)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte0(self) -> Reg<regs::DormantWakeInte0, RW> {
        unsafe { Reg::new(self.0.add(352usize)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte1(self) -> Reg<regs::DormantWakeInte1, RW> {
        unsafe { Reg::new(self.0.add(356usize)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte2(self) -> Reg<regs::DormantWakeInte2, RW> {
        unsafe { Reg::new(self.0.add(360usize)) }
    }
    #[doc = "Interrupt Enable for dormant_wake"]
    pub fn dormant_wake_inte3(self) -> Reg<regs::DormantWakeInte3, RW> {
        unsafe { Reg::new(self.0.add(364usize)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf0(self) -> Reg<regs::DormantWakeIntf0, RW> {
        unsafe { Reg::new(self.0.add(368usize)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf1(self) -> Reg<regs::DormantWakeIntf1, RW> {
        unsafe { Reg::new(self.0.add(372usize)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf2(self) -> Reg<regs::DormantWakeIntf2, RW> {
        unsafe { Reg::new(self.0.add(376usize)) }
    }
    #[doc = "Interrupt Force for dormant_wake"]
    pub fn dormant_wake_intf3(self) -> Reg<regs::DormantWakeIntf3, RW> {
        unsafe { Reg::new(self.0.add(380usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints0(self) -> Reg<regs::DormantWakeInts0, RW> {
        unsafe { Reg::new(self.0.add(384usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints1(self) -> Reg<regs::DormantWakeInts1, RW> {
        unsafe { Reg::new(self.0.add(388usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints2(self) -> Reg<regs::DormantWakeInts2, RW> {
        unsafe { Reg::new(self.0.add(392usize)) }
    }
    #[doc = "Interrupt status after masking & forcing for dormant_wake"]
    pub fn dormant_wake_ints3(self) -> Reg<regs::DormantWakeInts3, RW> {
        unsafe { Reg::new(self.0.add(396usize)) }
    }
}
pub mod regs;
pub mod vals;
