#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    pmcr: Pmcr,
    exticr1: Exticr1,
    exticr2: Exticr2,
    exticr3: Exticr3,
    exticr4: Exticr4,
    cfgr: Cfgr,
    _reserved6: [u8; 0x04],
    cccsr: Cccsr,
    ccvr: Ccvr,
    cccr: Cccr,
    pwrcr: Pwrcr,
    _reserved10: [u8; 0xf4],
    pkgr: Pkgr,
    _reserved11: [u8; 0x01d8],
    ur0: Ur0,
    ur1: Ur1,
    ur2: Ur2,
    ur3: Ur3,
    ur4: Ur4,
    ur5: Ur5,
    ur6: Ur6,
    ur7: Ur7,
    ur8: Ur8,
    ur9: Ur9,
    ur10: Ur10,
    ur11: Ur11,
    ur12: Ur12,
    ur13: Ur13,
    ur14: Ur14,
    ur15: Ur15,
    ur16: Ur16,
    ur17: Ur17,
}
impl RegisterBlock {
    #[doc = "0x04 - peripheral mode configuration register"]
    #[inline(always)]
    pub const fn pmcr(&self) -> &Pmcr {
        &self.pmcr
    }
    #[doc = "0x08 - external interrupt configuration register 1"]
    #[inline(always)]
    pub const fn exticr1(&self) -> &Exticr1 {
        &self.exticr1
    }
    #[doc = "0x0c - external interrupt configuration register 2"]
    #[inline(always)]
    pub const fn exticr2(&self) -> &Exticr2 {
        &self.exticr2
    }
    #[doc = "0x10 - external interrupt configuration register 3"]
    #[inline(always)]
    pub const fn exticr3(&self) -> &Exticr3 {
        &self.exticr3
    }
    #[doc = "0x14 - external interrupt configuration register 4"]
    #[inline(always)]
    pub const fn exticr4(&self) -> &Exticr4 {
        &self.exticr4
    }
    #[doc = "0x18 - configuration register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x20 - compensation cell control/status register"]
    #[inline(always)]
    pub const fn cccsr(&self) -> &Cccsr {
        &self.cccsr
    }
    #[doc = "0x24 - SYSCFG compensation cell value register"]
    #[inline(always)]
    pub const fn ccvr(&self) -> &Ccvr {
        &self.ccvr
    }
    #[doc = "0x28 - SYSCFG compensation cell code register"]
    #[inline(always)]
    pub const fn cccr(&self) -> &Cccr {
        &self.cccr
    }
    #[doc = "0x2c - SYSCFG power control register"]
    #[inline(always)]
    pub const fn pwrcr(&self) -> &Pwrcr {
        &self.pwrcr
    }
    #[doc = "0x124 - SYSCFG package register"]
    #[inline(always)]
    pub const fn pkgr(&self) -> &Pkgr {
        &self.pkgr
    }
    #[doc = "0x300 - SYSCFG user register 0"]
    #[inline(always)]
    pub const fn ur0(&self) -> &Ur0 {
        &self.ur0
    }
    #[doc = "0x304 - SYSCFG user register 1"]
    #[inline(always)]
    pub const fn ur1(&self) -> &Ur1 {
        &self.ur1
    }
    #[doc = "0x308 - SYSCFG user register 2"]
    #[inline(always)]
    pub const fn ur2(&self) -> &Ur2 {
        &self.ur2
    }
    #[doc = "0x30c - SYSCFG user register 3"]
    #[inline(always)]
    pub const fn ur3(&self) -> &Ur3 {
        &self.ur3
    }
    #[doc = "0x310 - SYSCFG user register 4"]
    #[inline(always)]
    pub const fn ur4(&self) -> &Ur4 {
        &self.ur4
    }
    #[doc = "0x314 - SYSCFG user register 5"]
    #[inline(always)]
    pub const fn ur5(&self) -> &Ur5 {
        &self.ur5
    }
    #[doc = "0x318 - SYSCFG user register 6"]
    #[inline(always)]
    pub const fn ur6(&self) -> &Ur6 {
        &self.ur6
    }
    #[doc = "0x31c - SYSCFG user register 7"]
    #[inline(always)]
    pub const fn ur7(&self) -> &Ur7 {
        &self.ur7
    }
    #[doc = "0x320 - SYSCFG user register 8"]
    #[inline(always)]
    pub const fn ur8(&self) -> &Ur8 {
        &self.ur8
    }
    #[doc = "0x324 - SYSCFG user register 9"]
    #[inline(always)]
    pub const fn ur9(&self) -> &Ur9 {
        &self.ur9
    }
    #[doc = "0x328 - SYSCFG user register 10"]
    #[inline(always)]
    pub const fn ur10(&self) -> &Ur10 {
        &self.ur10
    }
    #[doc = "0x32c - SYSCFG user register 11"]
    #[inline(always)]
    pub const fn ur11(&self) -> &Ur11 {
        &self.ur11
    }
    #[doc = "0x330 - SYSCFG user register 12"]
    #[inline(always)]
    pub const fn ur12(&self) -> &Ur12 {
        &self.ur12
    }
    #[doc = "0x334 - SYSCFG user register 13"]
    #[inline(always)]
    pub const fn ur13(&self) -> &Ur13 {
        &self.ur13
    }
    #[doc = "0x338 - SYSCFG user register 14"]
    #[inline(always)]
    pub const fn ur14(&self) -> &Ur14 {
        &self.ur14
    }
    #[doc = "0x33c - SYSCFG user register 15"]
    #[inline(always)]
    pub const fn ur15(&self) -> &Ur15 {
        &self.ur15
    }
    #[doc = "0x340 - SYSCFG user register 16"]
    #[inline(always)]
    pub const fn ur16(&self) -> &Ur16 {
        &self.ur16
    }
    #[doc = "0x344 - SYSCFG user register 17"]
    #[inline(always)]
    pub const fn ur17(&self) -> &Ur17 {
        &self.ur17
    }
}
#[doc = "PMCR (rw) register accessor: peripheral mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pmcr`] module"]
#[doc(alias = "PMCR")]
pub type Pmcr = crate::Reg<pmcr::PmcrSpec>;
#[doc = "peripheral mode configuration register"]
pub mod pmcr;
#[doc = "EXTICR1 (rw) register accessor: external interrupt configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr1`] module"]
#[doc(alias = "EXTICR1")]
pub type Exticr1 = crate::Reg<exticr1::Exticr1Spec>;
#[doc = "external interrupt configuration register 1"]
pub mod exticr1;
#[doc = "EXTICR2 (rw) register accessor: external interrupt configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr2`] module"]
#[doc(alias = "EXTICR2")]
pub type Exticr2 = crate::Reg<exticr2::Exticr2Spec>;
#[doc = "external interrupt configuration register 2"]
pub mod exticr2;
#[doc = "EXTICR3 (rw) register accessor: external interrupt configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr3`] module"]
#[doc(alias = "EXTICR3")]
pub type Exticr3 = crate::Reg<exticr3::Exticr3Spec>;
#[doc = "external interrupt configuration register 3"]
pub mod exticr3;
#[doc = "EXTICR4 (rw) register accessor: external interrupt configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`exticr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`exticr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@exticr4`] module"]
#[doc(alias = "EXTICR4")]
pub type Exticr4 = crate::Reg<exticr4::Exticr4Spec>;
#[doc = "external interrupt configuration register 4"]
pub mod exticr4;
#[doc = "CFGR (rw) register accessor: configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "configuration register"]
pub mod cfgr;
#[doc = "CCCSR (rw) register accessor: compensation cell control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccsr`] module"]
#[doc(alias = "CCCSR")]
pub type Cccsr = crate::Reg<cccsr::CccsrSpec>;
#[doc = "compensation cell control/status register"]
pub mod cccsr;
#[doc = "CCVR (r) register accessor: SYSCFG compensation cell value register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccvr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccvr`] module"]
#[doc(alias = "CCVR")]
pub type Ccvr = crate::Reg<ccvr::CcvrSpec>;
#[doc = "SYSCFG compensation cell value register"]
pub mod ccvr;
#[doc = "CCCR (rw) register accessor: SYSCFG compensation cell code register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cccr`] module"]
#[doc(alias = "CCCR")]
pub type Cccr = crate::Reg<cccr::CccrSpec>;
#[doc = "SYSCFG compensation cell code register"]
pub mod cccr;
#[doc = "PWRCR (rw) register accessor: SYSCFG power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`pwrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pwrcr`] module"]
#[doc(alias = "PWRCR")]
pub type Pwrcr = crate::Reg<pwrcr::PwrcrSpec>;
#[doc = "SYSCFG power control register"]
pub mod pwrcr;
#[doc = "PKGR (r) register accessor: SYSCFG package register\n\nYou can [`read`](crate::Reg::read) this register and get [`pkgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pkgr`] module"]
#[doc(alias = "PKGR")]
pub type Pkgr = crate::Reg<pkgr::PkgrSpec>;
#[doc = "SYSCFG package register"]
pub mod pkgr;
#[doc = "UR0 (r) register accessor: SYSCFG user register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`ur0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur0`] module"]
#[doc(alias = "UR0")]
pub type Ur0 = crate::Reg<ur0::Ur0Spec>;
#[doc = "SYSCFG user register 0"]
pub mod ur0;
#[doc = "UR1 (rw) register accessor: SYSCFG user register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ur1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur1`] module"]
#[doc(alias = "UR1")]
pub type Ur1 = crate::Reg<ur1::Ur1Spec>;
#[doc = "SYSCFG user register 1"]
pub mod ur1;
#[doc = "UR2 (rw) register accessor: SYSCFG user register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ur2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur2`] module"]
#[doc(alias = "UR2")]
pub type Ur2 = crate::Reg<ur2::Ur2Spec>;
#[doc = "SYSCFG user register 2"]
pub mod ur2;
#[doc = "UR3 (rw) register accessor: SYSCFG user register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ur3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur3`] module"]
#[doc(alias = "UR3")]
pub type Ur3 = crate::Reg<ur3::Ur3Spec>;
#[doc = "SYSCFG user register 3"]
pub mod ur3;
#[doc = "UR4 (rw) register accessor: SYSCFG user register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`ur4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur4`] module"]
#[doc(alias = "UR4")]
pub type Ur4 = crate::Reg<ur4::Ur4Spec>;
#[doc = "SYSCFG user register 4"]
pub mod ur4;
#[doc = "UR5 (r) register accessor: SYSCFG user register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`ur5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur5`] module"]
#[doc(alias = "UR5")]
pub type Ur5 = crate::Reg<ur5::Ur5Spec>;
#[doc = "SYSCFG user register 5"]
pub mod ur5;
#[doc = "UR6 (r) register accessor: SYSCFG user register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ur6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur6`] module"]
#[doc(alias = "UR6")]
pub type Ur6 = crate::Reg<ur6::Ur6Spec>;
#[doc = "SYSCFG user register 6"]
pub mod ur6;
#[doc = "UR7 (r) register accessor: SYSCFG user register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`ur7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur7`] module"]
#[doc(alias = "UR7")]
pub type Ur7 = crate::Reg<ur7::Ur7Spec>;
#[doc = "SYSCFG user register 7"]
pub mod ur7;
#[doc = "UR8 (r) register accessor: SYSCFG user register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`ur8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur8`] module"]
#[doc(alias = "UR8")]
pub type Ur8 = crate::Reg<ur8::Ur8Spec>;
#[doc = "SYSCFG user register 8"]
pub mod ur8;
#[doc = "UR9 (r) register accessor: SYSCFG user register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`ur9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur9`] module"]
#[doc(alias = "UR9")]
pub type Ur9 = crate::Reg<ur9::Ur9Spec>;
#[doc = "SYSCFG user register 9"]
pub mod ur9;
#[doc = "UR10 (r) register accessor: SYSCFG user register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ur10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur10`] module"]
#[doc(alias = "UR10")]
pub type Ur10 = crate::Reg<ur10::Ur10Spec>;
#[doc = "SYSCFG user register 10"]
pub mod ur10;
#[doc = "UR11 (r) register accessor: SYSCFG user register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`ur11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur11`] module"]
#[doc(alias = "UR11")]
pub type Ur11 = crate::Reg<ur11::Ur11Spec>;
#[doc = "SYSCFG user register 11"]
pub mod ur11;
#[doc = "UR12 (r) register accessor: SYSCFG user register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`ur12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur12`] module"]
#[doc(alias = "UR12")]
pub type Ur12 = crate::Reg<ur12::Ur12Spec>;
#[doc = "SYSCFG user register 12"]
pub mod ur12;
#[doc = "UR13 (r) register accessor: SYSCFG user register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`ur13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur13`] module"]
#[doc(alias = "UR13")]
pub type Ur13 = crate::Reg<ur13::Ur13Spec>;
#[doc = "SYSCFG user register 13"]
pub mod ur13;
#[doc = "UR14 (rw) register accessor: SYSCFG user register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`ur14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur14`] module"]
#[doc(alias = "UR14")]
pub type Ur14 = crate::Reg<ur14::Ur14Spec>;
#[doc = "SYSCFG user register 14"]
pub mod ur14;
#[doc = "UR15 (rw) register accessor: SYSCFG user register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`ur15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur15`] module"]
#[doc(alias = "UR15")]
pub type Ur15 = crate::Reg<ur15::Ur15Spec>;
#[doc = "SYSCFG user register 15"]
pub mod ur15;
#[doc = "UR16 (r) register accessor: SYSCFG user register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`ur16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur16`] module"]
#[doc(alias = "UR16")]
pub type Ur16 = crate::Reg<ur16::Ur16Spec>;
#[doc = "SYSCFG user register 16"]
pub mod ur16;
#[doc = "UR17 (r) register accessor: SYSCFG user register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`ur17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ur17`] module"]
#[doc(alias = "UR17")]
pub type Ur17 = crate::Reg<ur17::Ur17Spec>;
#[doc = "SYSCFG user register 17"]
pub mod ur17;
