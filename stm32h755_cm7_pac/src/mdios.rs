#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mdios_cr: MdiosCr,
    mdios_wrfr: MdiosWrfr,
    mdios_cwrfr: MdiosCwrfr,
    mdios_rdfr: MdiosRdfr,
    mdios_crdfr: MdiosCrdfr,
    mdios_sr: MdiosSr,
    mdios_clrfr: MdiosClrfr,
    mdios_dinr0: MdiosDinr0,
    mdios_dinr1: MdiosDinr1,
    mdios_dinr2: MdiosDinr2,
    mdios_dinr3: MdiosDinr3,
    mdios_dinr4: MdiosDinr4,
    mdios_dinr5: MdiosDinr5,
    mdios_dinr6: MdiosDinr6,
    mdios_dinr7: MdiosDinr7,
    mdios_dinr8: MdiosDinr8,
    mdios_dinr9: MdiosDinr9,
    mdios_dinr10: MdiosDinr10,
    mdios_dinr11: MdiosDinr11,
    mdios_dinr12: MdiosDinr12,
    mdios_dinr13: MdiosDinr13,
    mdios_dinr14: MdiosDinr14,
    mdios_dinr15: MdiosDinr15,
    mdios_dinr16: MdiosDinr16,
    mdios_dinr17: MdiosDinr17,
    mdios_dinr18: MdiosDinr18,
    mdios_dinr19: MdiosDinr19,
    mdios_dinr20: MdiosDinr20,
    mdios_dinr21: MdiosDinr21,
    mdios_dinr22: MdiosDinr22,
    mdios_dinr23: MdiosDinr23,
    mdios_dinr24: MdiosDinr24,
    mdios_dinr25: MdiosDinr25,
    mdios_dinr26: MdiosDinr26,
    mdios_dinr27: MdiosDinr27,
    mdios_dinr28: MdiosDinr28,
    mdios_dinr29: MdiosDinr29,
    mdios_dinr30: MdiosDinr30,
    mdios_dinr31: MdiosDinr31,
    mdios_doutr0: MdiosDoutr0,
    mdios_doutr1: MdiosDoutr1,
    mdios_doutr2: MdiosDoutr2,
    mdios_doutr3: MdiosDoutr3,
    mdios_doutr4: MdiosDoutr4,
    mdios_doutr5: MdiosDoutr5,
    mdios_doutr6: MdiosDoutr6,
    mdios_doutr7: MdiosDoutr7,
    mdios_doutr8: MdiosDoutr8,
    mdios_doutr9: MdiosDoutr9,
    mdios_doutr10: MdiosDoutr10,
    mdios_doutr11: MdiosDoutr11,
    mdios_doutr12: MdiosDoutr12,
    mdios_doutr13: MdiosDoutr13,
    mdios_doutr14: MdiosDoutr14,
    mdios_doutr15: MdiosDoutr15,
    mdios_doutr16: MdiosDoutr16,
    mdios_doutr17: MdiosDoutr17,
    mdios_doutr18: MdiosDoutr18,
    mdios_doutr19: MdiosDoutr19,
    mdios_doutr20: MdiosDoutr20,
    mdios_doutr21: MdiosDoutr21,
    mdios_doutr22: MdiosDoutr22,
    mdios_doutr23: MdiosDoutr23,
    mdios_doutr24: MdiosDoutr24,
    mdios_doutr25: MdiosDoutr25,
    mdios_doutr26: MdiosDoutr26,
    mdios_doutr27: MdiosDoutr27,
    mdios_doutr28: MdiosDoutr28,
    mdios_doutr29: MdiosDoutr29,
    mdios_doutr30: MdiosDoutr30,
    mdios_doutr31: MdiosDoutr31,
}
impl RegisterBlock {
    #[doc = "0x00 - MDIOS configuration register"]
    #[inline(always)]
    pub const fn mdios_cr(&self) -> &MdiosCr {
        &self.mdios_cr
    }
    #[doc = "0x04 - MDIOS write flag register"]
    #[inline(always)]
    pub const fn mdios_wrfr(&self) -> &MdiosWrfr {
        &self.mdios_wrfr
    }
    #[doc = "0x08 - MDIOS clear write flag register"]
    #[inline(always)]
    pub const fn mdios_cwrfr(&self) -> &MdiosCwrfr {
        &self.mdios_cwrfr
    }
    #[doc = "0x0c - MDIOS read flag register"]
    #[inline(always)]
    pub const fn mdios_rdfr(&self) -> &MdiosRdfr {
        &self.mdios_rdfr
    }
    #[doc = "0x10 - MDIOS clear read flag register"]
    #[inline(always)]
    pub const fn mdios_crdfr(&self) -> &MdiosCrdfr {
        &self.mdios_crdfr
    }
    #[doc = "0x14 - MDIOS status register"]
    #[inline(always)]
    pub const fn mdios_sr(&self) -> &MdiosSr {
        &self.mdios_sr
    }
    #[doc = "0x18 - MDIOS clear flag register"]
    #[inline(always)]
    pub const fn mdios_clrfr(&self) -> &MdiosClrfr {
        &self.mdios_clrfr
    }
    #[doc = "0x1c - MDIOS input data register 0"]
    #[inline(always)]
    pub const fn mdios_dinr0(&self) -> &MdiosDinr0 {
        &self.mdios_dinr0
    }
    #[doc = "0x20 - MDIOS input data register 1"]
    #[inline(always)]
    pub const fn mdios_dinr1(&self) -> &MdiosDinr1 {
        &self.mdios_dinr1
    }
    #[doc = "0x24 - MDIOS input data register 2"]
    #[inline(always)]
    pub const fn mdios_dinr2(&self) -> &MdiosDinr2 {
        &self.mdios_dinr2
    }
    #[doc = "0x28 - MDIOS input data register 3"]
    #[inline(always)]
    pub const fn mdios_dinr3(&self) -> &MdiosDinr3 {
        &self.mdios_dinr3
    }
    #[doc = "0x2c - MDIOS input data register 4"]
    #[inline(always)]
    pub const fn mdios_dinr4(&self) -> &MdiosDinr4 {
        &self.mdios_dinr4
    }
    #[doc = "0x30 - MDIOS input data register 5"]
    #[inline(always)]
    pub const fn mdios_dinr5(&self) -> &MdiosDinr5 {
        &self.mdios_dinr5
    }
    #[doc = "0x34 - MDIOS input data register 6"]
    #[inline(always)]
    pub const fn mdios_dinr6(&self) -> &MdiosDinr6 {
        &self.mdios_dinr6
    }
    #[doc = "0x38 - MDIOS input data register 7"]
    #[inline(always)]
    pub const fn mdios_dinr7(&self) -> &MdiosDinr7 {
        &self.mdios_dinr7
    }
    #[doc = "0x3c - MDIOS input data register 8"]
    #[inline(always)]
    pub const fn mdios_dinr8(&self) -> &MdiosDinr8 {
        &self.mdios_dinr8
    }
    #[doc = "0x40 - MDIOS input data register 9"]
    #[inline(always)]
    pub const fn mdios_dinr9(&self) -> &MdiosDinr9 {
        &self.mdios_dinr9
    }
    #[doc = "0x44 - MDIOS input data register 10"]
    #[inline(always)]
    pub const fn mdios_dinr10(&self) -> &MdiosDinr10 {
        &self.mdios_dinr10
    }
    #[doc = "0x48 - MDIOS input data register 11"]
    #[inline(always)]
    pub const fn mdios_dinr11(&self) -> &MdiosDinr11 {
        &self.mdios_dinr11
    }
    #[doc = "0x4c - MDIOS input data register 12"]
    #[inline(always)]
    pub const fn mdios_dinr12(&self) -> &MdiosDinr12 {
        &self.mdios_dinr12
    }
    #[doc = "0x50 - MDIOS input data register 13"]
    #[inline(always)]
    pub const fn mdios_dinr13(&self) -> &MdiosDinr13 {
        &self.mdios_dinr13
    }
    #[doc = "0x54 - MDIOS input data register 14"]
    #[inline(always)]
    pub const fn mdios_dinr14(&self) -> &MdiosDinr14 {
        &self.mdios_dinr14
    }
    #[doc = "0x58 - MDIOS input data register 15"]
    #[inline(always)]
    pub const fn mdios_dinr15(&self) -> &MdiosDinr15 {
        &self.mdios_dinr15
    }
    #[doc = "0x5c - MDIOS input data register 16"]
    #[inline(always)]
    pub const fn mdios_dinr16(&self) -> &MdiosDinr16 {
        &self.mdios_dinr16
    }
    #[doc = "0x60 - MDIOS input data register 17"]
    #[inline(always)]
    pub const fn mdios_dinr17(&self) -> &MdiosDinr17 {
        &self.mdios_dinr17
    }
    #[doc = "0x64 - MDIOS input data register 18"]
    #[inline(always)]
    pub const fn mdios_dinr18(&self) -> &MdiosDinr18 {
        &self.mdios_dinr18
    }
    #[doc = "0x68 - MDIOS input data register 19"]
    #[inline(always)]
    pub const fn mdios_dinr19(&self) -> &MdiosDinr19 {
        &self.mdios_dinr19
    }
    #[doc = "0x6c - MDIOS input data register 20"]
    #[inline(always)]
    pub const fn mdios_dinr20(&self) -> &MdiosDinr20 {
        &self.mdios_dinr20
    }
    #[doc = "0x70 - MDIOS input data register 21"]
    #[inline(always)]
    pub const fn mdios_dinr21(&self) -> &MdiosDinr21 {
        &self.mdios_dinr21
    }
    #[doc = "0x74 - MDIOS input data register 22"]
    #[inline(always)]
    pub const fn mdios_dinr22(&self) -> &MdiosDinr22 {
        &self.mdios_dinr22
    }
    #[doc = "0x78 - MDIOS input data register 23"]
    #[inline(always)]
    pub const fn mdios_dinr23(&self) -> &MdiosDinr23 {
        &self.mdios_dinr23
    }
    #[doc = "0x7c - MDIOS input data register 24"]
    #[inline(always)]
    pub const fn mdios_dinr24(&self) -> &MdiosDinr24 {
        &self.mdios_dinr24
    }
    #[doc = "0x80 - MDIOS input data register 25"]
    #[inline(always)]
    pub const fn mdios_dinr25(&self) -> &MdiosDinr25 {
        &self.mdios_dinr25
    }
    #[doc = "0x84 - MDIOS input data register 26"]
    #[inline(always)]
    pub const fn mdios_dinr26(&self) -> &MdiosDinr26 {
        &self.mdios_dinr26
    }
    #[doc = "0x88 - MDIOS input data register 27"]
    #[inline(always)]
    pub const fn mdios_dinr27(&self) -> &MdiosDinr27 {
        &self.mdios_dinr27
    }
    #[doc = "0x8c - MDIOS input data register 28"]
    #[inline(always)]
    pub const fn mdios_dinr28(&self) -> &MdiosDinr28 {
        &self.mdios_dinr28
    }
    #[doc = "0x90 - MDIOS input data register 29"]
    #[inline(always)]
    pub const fn mdios_dinr29(&self) -> &MdiosDinr29 {
        &self.mdios_dinr29
    }
    #[doc = "0x94 - MDIOS input data register 30"]
    #[inline(always)]
    pub const fn mdios_dinr30(&self) -> &MdiosDinr30 {
        &self.mdios_dinr30
    }
    #[doc = "0x98 - MDIOS input data register 31"]
    #[inline(always)]
    pub const fn mdios_dinr31(&self) -> &MdiosDinr31 {
        &self.mdios_dinr31
    }
    #[doc = "0x9c - MDIOS output data register 0"]
    #[inline(always)]
    pub const fn mdios_doutr0(&self) -> &MdiosDoutr0 {
        &self.mdios_doutr0
    }
    #[doc = "0xa0 - MDIOS output data register 1"]
    #[inline(always)]
    pub const fn mdios_doutr1(&self) -> &MdiosDoutr1 {
        &self.mdios_doutr1
    }
    #[doc = "0xa4 - MDIOS output data register 2"]
    #[inline(always)]
    pub const fn mdios_doutr2(&self) -> &MdiosDoutr2 {
        &self.mdios_doutr2
    }
    #[doc = "0xa8 - MDIOS output data register 3"]
    #[inline(always)]
    pub const fn mdios_doutr3(&self) -> &MdiosDoutr3 {
        &self.mdios_doutr3
    }
    #[doc = "0xac - MDIOS output data register 4"]
    #[inline(always)]
    pub const fn mdios_doutr4(&self) -> &MdiosDoutr4 {
        &self.mdios_doutr4
    }
    #[doc = "0xb0 - MDIOS output data register 5"]
    #[inline(always)]
    pub const fn mdios_doutr5(&self) -> &MdiosDoutr5 {
        &self.mdios_doutr5
    }
    #[doc = "0xb4 - MDIOS output data register 6"]
    #[inline(always)]
    pub const fn mdios_doutr6(&self) -> &MdiosDoutr6 {
        &self.mdios_doutr6
    }
    #[doc = "0xb8 - MDIOS output data register 7"]
    #[inline(always)]
    pub const fn mdios_doutr7(&self) -> &MdiosDoutr7 {
        &self.mdios_doutr7
    }
    #[doc = "0xbc - MDIOS output data register 8"]
    #[inline(always)]
    pub const fn mdios_doutr8(&self) -> &MdiosDoutr8 {
        &self.mdios_doutr8
    }
    #[doc = "0xc0 - MDIOS output data register 9"]
    #[inline(always)]
    pub const fn mdios_doutr9(&self) -> &MdiosDoutr9 {
        &self.mdios_doutr9
    }
    #[doc = "0xc4 - MDIOS output data register 10"]
    #[inline(always)]
    pub const fn mdios_doutr10(&self) -> &MdiosDoutr10 {
        &self.mdios_doutr10
    }
    #[doc = "0xc8 - MDIOS output data register 11"]
    #[inline(always)]
    pub const fn mdios_doutr11(&self) -> &MdiosDoutr11 {
        &self.mdios_doutr11
    }
    #[doc = "0xcc - MDIOS output data register 12"]
    #[inline(always)]
    pub const fn mdios_doutr12(&self) -> &MdiosDoutr12 {
        &self.mdios_doutr12
    }
    #[doc = "0xd0 - MDIOS output data register 13"]
    #[inline(always)]
    pub const fn mdios_doutr13(&self) -> &MdiosDoutr13 {
        &self.mdios_doutr13
    }
    #[doc = "0xd4 - MDIOS output data register 14"]
    #[inline(always)]
    pub const fn mdios_doutr14(&self) -> &MdiosDoutr14 {
        &self.mdios_doutr14
    }
    #[doc = "0xd8 - MDIOS output data register 15"]
    #[inline(always)]
    pub const fn mdios_doutr15(&self) -> &MdiosDoutr15 {
        &self.mdios_doutr15
    }
    #[doc = "0xdc - MDIOS output data register 16"]
    #[inline(always)]
    pub const fn mdios_doutr16(&self) -> &MdiosDoutr16 {
        &self.mdios_doutr16
    }
    #[doc = "0xe0 - MDIOS output data register 17"]
    #[inline(always)]
    pub const fn mdios_doutr17(&self) -> &MdiosDoutr17 {
        &self.mdios_doutr17
    }
    #[doc = "0xe4 - MDIOS output data register 18"]
    #[inline(always)]
    pub const fn mdios_doutr18(&self) -> &MdiosDoutr18 {
        &self.mdios_doutr18
    }
    #[doc = "0xe8 - MDIOS output data register 19"]
    #[inline(always)]
    pub const fn mdios_doutr19(&self) -> &MdiosDoutr19 {
        &self.mdios_doutr19
    }
    #[doc = "0xec - MDIOS output data register 20"]
    #[inline(always)]
    pub const fn mdios_doutr20(&self) -> &MdiosDoutr20 {
        &self.mdios_doutr20
    }
    #[doc = "0xf0 - MDIOS output data register 21"]
    #[inline(always)]
    pub const fn mdios_doutr21(&self) -> &MdiosDoutr21 {
        &self.mdios_doutr21
    }
    #[doc = "0xf4 - MDIOS output data register 22"]
    #[inline(always)]
    pub const fn mdios_doutr22(&self) -> &MdiosDoutr22 {
        &self.mdios_doutr22
    }
    #[doc = "0xf8 - MDIOS output data register 23"]
    #[inline(always)]
    pub const fn mdios_doutr23(&self) -> &MdiosDoutr23 {
        &self.mdios_doutr23
    }
    #[doc = "0xfc - MDIOS output data register 24"]
    #[inline(always)]
    pub const fn mdios_doutr24(&self) -> &MdiosDoutr24 {
        &self.mdios_doutr24
    }
    #[doc = "0x100 - MDIOS output data register 25"]
    #[inline(always)]
    pub const fn mdios_doutr25(&self) -> &MdiosDoutr25 {
        &self.mdios_doutr25
    }
    #[doc = "0x104 - MDIOS output data register 26"]
    #[inline(always)]
    pub const fn mdios_doutr26(&self) -> &MdiosDoutr26 {
        &self.mdios_doutr26
    }
    #[doc = "0x108 - MDIOS output data register 27"]
    #[inline(always)]
    pub const fn mdios_doutr27(&self) -> &MdiosDoutr27 {
        &self.mdios_doutr27
    }
    #[doc = "0x10c - MDIOS output data register 28"]
    #[inline(always)]
    pub const fn mdios_doutr28(&self) -> &MdiosDoutr28 {
        &self.mdios_doutr28
    }
    #[doc = "0x110 - MDIOS output data register 29"]
    #[inline(always)]
    pub const fn mdios_doutr29(&self) -> &MdiosDoutr29 {
        &self.mdios_doutr29
    }
    #[doc = "0x114 - MDIOS output data register 30"]
    #[inline(always)]
    pub const fn mdios_doutr30(&self) -> &MdiosDoutr30 {
        &self.mdios_doutr30
    }
    #[doc = "0x118 - MDIOS output data register 31"]
    #[inline(always)]
    pub const fn mdios_doutr31(&self) -> &MdiosDoutr31 {
        &self.mdios_doutr31
    }
}
#[doc = "MDIOS_CR (rw) register accessor: MDIOS configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_cr`] module"]
#[doc(alias = "MDIOS_CR")]
pub type MdiosCr = crate::Reg<mdios_cr::MdiosCrSpec>;
#[doc = "MDIOS configuration register"]
pub mod mdios_cr;
#[doc = "MDIOS_WRFR (r) register accessor: MDIOS write flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_wrfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_wrfr`] module"]
#[doc(alias = "MDIOS_WRFR")]
pub type MdiosWrfr = crate::Reg<mdios_wrfr::MdiosWrfrSpec>;
#[doc = "MDIOS write flag register"]
pub mod mdios_wrfr;
#[doc = "MDIOS_CWRFR (rw) register accessor: MDIOS clear write flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_cwrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_cwrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_cwrfr`] module"]
#[doc(alias = "MDIOS_CWRFR")]
pub type MdiosCwrfr = crate::Reg<mdios_cwrfr::MdiosCwrfrSpec>;
#[doc = "MDIOS clear write flag register"]
pub mod mdios_cwrfr;
#[doc = "MDIOS_RDFR (r) register accessor: MDIOS read flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_rdfr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_rdfr`] module"]
#[doc(alias = "MDIOS_RDFR")]
pub type MdiosRdfr = crate::Reg<mdios_rdfr::MdiosRdfrSpec>;
#[doc = "MDIOS read flag register"]
pub mod mdios_rdfr;
#[doc = "MDIOS_CRDFR (rw) register accessor: MDIOS clear read flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_crdfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_crdfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_crdfr`] module"]
#[doc(alias = "MDIOS_CRDFR")]
pub type MdiosCrdfr = crate::Reg<mdios_crdfr::MdiosCrdfrSpec>;
#[doc = "MDIOS clear read flag register"]
pub mod mdios_crdfr;
#[doc = "MDIOS_SR (r) register accessor: MDIOS status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_sr`] module"]
#[doc(alias = "MDIOS_SR")]
pub type MdiosSr = crate::Reg<mdios_sr::MdiosSrSpec>;
#[doc = "MDIOS status register"]
pub mod mdios_sr;
#[doc = "MDIOS_CLRFR (rw) register accessor: MDIOS clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_clrfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_clrfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_clrfr`] module"]
#[doc(alias = "MDIOS_CLRFR")]
pub type MdiosClrfr = crate::Reg<mdios_clrfr::MdiosClrfrSpec>;
#[doc = "MDIOS clear flag register"]
pub mod mdios_clrfr;
#[doc = "MDIOS_DINR0 (r) register accessor: MDIOS input data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr0`] module"]
#[doc(alias = "MDIOS_DINR0")]
pub type MdiosDinr0 = crate::Reg<mdios_dinr0::MdiosDinr0Spec>;
#[doc = "MDIOS input data register 0"]
pub mod mdios_dinr0;
#[doc = "MDIOS_DINR1 (r) register accessor: MDIOS input data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr1`] module"]
#[doc(alias = "MDIOS_DINR1")]
pub type MdiosDinr1 = crate::Reg<mdios_dinr1::MdiosDinr1Spec>;
#[doc = "MDIOS input data register 1"]
pub mod mdios_dinr1;
#[doc = "MDIOS_DINR2 (r) register accessor: MDIOS input data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr2`] module"]
#[doc(alias = "MDIOS_DINR2")]
pub type MdiosDinr2 = crate::Reg<mdios_dinr2::MdiosDinr2Spec>;
#[doc = "MDIOS input data register 2"]
pub mod mdios_dinr2;
#[doc = "MDIOS_DINR3 (r) register accessor: MDIOS input data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr3`] module"]
#[doc(alias = "MDIOS_DINR3")]
pub type MdiosDinr3 = crate::Reg<mdios_dinr3::MdiosDinr3Spec>;
#[doc = "MDIOS input data register 3"]
pub mod mdios_dinr3;
#[doc = "MDIOS_DINR4 (r) register accessor: MDIOS input data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr4`] module"]
#[doc(alias = "MDIOS_DINR4")]
pub type MdiosDinr4 = crate::Reg<mdios_dinr4::MdiosDinr4Spec>;
#[doc = "MDIOS input data register 4"]
pub mod mdios_dinr4;
#[doc = "MDIOS_DINR5 (r) register accessor: MDIOS input data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr5`] module"]
#[doc(alias = "MDIOS_DINR5")]
pub type MdiosDinr5 = crate::Reg<mdios_dinr5::MdiosDinr5Spec>;
#[doc = "MDIOS input data register 5"]
pub mod mdios_dinr5;
#[doc = "MDIOS_DINR6 (r) register accessor: MDIOS input data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr6`] module"]
#[doc(alias = "MDIOS_DINR6")]
pub type MdiosDinr6 = crate::Reg<mdios_dinr6::MdiosDinr6Spec>;
#[doc = "MDIOS input data register 6"]
pub mod mdios_dinr6;
#[doc = "MDIOS_DINR7 (r) register accessor: MDIOS input data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr7`] module"]
#[doc(alias = "MDIOS_DINR7")]
pub type MdiosDinr7 = crate::Reg<mdios_dinr7::MdiosDinr7Spec>;
#[doc = "MDIOS input data register 7"]
pub mod mdios_dinr7;
#[doc = "MDIOS_DINR8 (r) register accessor: MDIOS input data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr8`] module"]
#[doc(alias = "MDIOS_DINR8")]
pub type MdiosDinr8 = crate::Reg<mdios_dinr8::MdiosDinr8Spec>;
#[doc = "MDIOS input data register 8"]
pub mod mdios_dinr8;
#[doc = "MDIOS_DINR9 (r) register accessor: MDIOS input data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr9`] module"]
#[doc(alias = "MDIOS_DINR9")]
pub type MdiosDinr9 = crate::Reg<mdios_dinr9::MdiosDinr9Spec>;
#[doc = "MDIOS input data register 9"]
pub mod mdios_dinr9;
#[doc = "MDIOS_DINR10 (r) register accessor: MDIOS input data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr10`] module"]
#[doc(alias = "MDIOS_DINR10")]
pub type MdiosDinr10 = crate::Reg<mdios_dinr10::MdiosDinr10Spec>;
#[doc = "MDIOS input data register 10"]
pub mod mdios_dinr10;
#[doc = "MDIOS_DINR11 (r) register accessor: MDIOS input data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr11`] module"]
#[doc(alias = "MDIOS_DINR11")]
pub type MdiosDinr11 = crate::Reg<mdios_dinr11::MdiosDinr11Spec>;
#[doc = "MDIOS input data register 11"]
pub mod mdios_dinr11;
#[doc = "MDIOS_DINR12 (r) register accessor: MDIOS input data register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr12`] module"]
#[doc(alias = "MDIOS_DINR12")]
pub type MdiosDinr12 = crate::Reg<mdios_dinr12::MdiosDinr12Spec>;
#[doc = "MDIOS input data register 12"]
pub mod mdios_dinr12;
#[doc = "MDIOS_DINR13 (r) register accessor: MDIOS input data register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr13`] module"]
#[doc(alias = "MDIOS_DINR13")]
pub type MdiosDinr13 = crate::Reg<mdios_dinr13::MdiosDinr13Spec>;
#[doc = "MDIOS input data register 13"]
pub mod mdios_dinr13;
#[doc = "MDIOS_DINR14 (r) register accessor: MDIOS input data register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr14`] module"]
#[doc(alias = "MDIOS_DINR14")]
pub type MdiosDinr14 = crate::Reg<mdios_dinr14::MdiosDinr14Spec>;
#[doc = "MDIOS input data register 14"]
pub mod mdios_dinr14;
#[doc = "MDIOS_DINR15 (r) register accessor: MDIOS input data register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr15`] module"]
#[doc(alias = "MDIOS_DINR15")]
pub type MdiosDinr15 = crate::Reg<mdios_dinr15::MdiosDinr15Spec>;
#[doc = "MDIOS input data register 15"]
pub mod mdios_dinr15;
#[doc = "MDIOS_DINR16 (r) register accessor: MDIOS input data register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr16`] module"]
#[doc(alias = "MDIOS_DINR16")]
pub type MdiosDinr16 = crate::Reg<mdios_dinr16::MdiosDinr16Spec>;
#[doc = "MDIOS input data register 16"]
pub mod mdios_dinr16;
#[doc = "MDIOS_DINR17 (r) register accessor: MDIOS input data register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr17`] module"]
#[doc(alias = "MDIOS_DINR17")]
pub type MdiosDinr17 = crate::Reg<mdios_dinr17::MdiosDinr17Spec>;
#[doc = "MDIOS input data register 17"]
pub mod mdios_dinr17;
#[doc = "MDIOS_DINR18 (r) register accessor: MDIOS input data register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr18`] module"]
#[doc(alias = "MDIOS_DINR18")]
pub type MdiosDinr18 = crate::Reg<mdios_dinr18::MdiosDinr18Spec>;
#[doc = "MDIOS input data register 18"]
pub mod mdios_dinr18;
#[doc = "MDIOS_DINR19 (r) register accessor: MDIOS input data register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr19`] module"]
#[doc(alias = "MDIOS_DINR19")]
pub type MdiosDinr19 = crate::Reg<mdios_dinr19::MdiosDinr19Spec>;
#[doc = "MDIOS input data register 19"]
pub mod mdios_dinr19;
#[doc = "MDIOS_DINR20 (r) register accessor: MDIOS input data register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr20`] module"]
#[doc(alias = "MDIOS_DINR20")]
pub type MdiosDinr20 = crate::Reg<mdios_dinr20::MdiosDinr20Spec>;
#[doc = "MDIOS input data register 20"]
pub mod mdios_dinr20;
#[doc = "MDIOS_DINR21 (r) register accessor: MDIOS input data register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr21`] module"]
#[doc(alias = "MDIOS_DINR21")]
pub type MdiosDinr21 = crate::Reg<mdios_dinr21::MdiosDinr21Spec>;
#[doc = "MDIOS input data register 21"]
pub mod mdios_dinr21;
#[doc = "MDIOS_DINR22 (r) register accessor: MDIOS input data register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr22`] module"]
#[doc(alias = "MDIOS_DINR22")]
pub type MdiosDinr22 = crate::Reg<mdios_dinr22::MdiosDinr22Spec>;
#[doc = "MDIOS input data register 22"]
pub mod mdios_dinr22;
#[doc = "MDIOS_DINR23 (r) register accessor: MDIOS input data register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr23`] module"]
#[doc(alias = "MDIOS_DINR23")]
pub type MdiosDinr23 = crate::Reg<mdios_dinr23::MdiosDinr23Spec>;
#[doc = "MDIOS input data register 23"]
pub mod mdios_dinr23;
#[doc = "MDIOS_DINR24 (r) register accessor: MDIOS input data register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr24`] module"]
#[doc(alias = "MDIOS_DINR24")]
pub type MdiosDinr24 = crate::Reg<mdios_dinr24::MdiosDinr24Spec>;
#[doc = "MDIOS input data register 24"]
pub mod mdios_dinr24;
#[doc = "MDIOS_DINR25 (r) register accessor: MDIOS input data register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr25`] module"]
#[doc(alias = "MDIOS_DINR25")]
pub type MdiosDinr25 = crate::Reg<mdios_dinr25::MdiosDinr25Spec>;
#[doc = "MDIOS input data register 25"]
pub mod mdios_dinr25;
#[doc = "MDIOS_DINR26 (r) register accessor: MDIOS input data register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr26`] module"]
#[doc(alias = "MDIOS_DINR26")]
pub type MdiosDinr26 = crate::Reg<mdios_dinr26::MdiosDinr26Spec>;
#[doc = "MDIOS input data register 26"]
pub mod mdios_dinr26;
#[doc = "MDIOS_DINR27 (r) register accessor: MDIOS input data register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr27`] module"]
#[doc(alias = "MDIOS_DINR27")]
pub type MdiosDinr27 = crate::Reg<mdios_dinr27::MdiosDinr27Spec>;
#[doc = "MDIOS input data register 27"]
pub mod mdios_dinr27;
#[doc = "MDIOS_DINR28 (r) register accessor: MDIOS input data register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr28`] module"]
#[doc(alias = "MDIOS_DINR28")]
pub type MdiosDinr28 = crate::Reg<mdios_dinr28::MdiosDinr28Spec>;
#[doc = "MDIOS input data register 28"]
pub mod mdios_dinr28;
#[doc = "MDIOS_DINR29 (r) register accessor: MDIOS input data register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr29`] module"]
#[doc(alias = "MDIOS_DINR29")]
pub type MdiosDinr29 = crate::Reg<mdios_dinr29::MdiosDinr29Spec>;
#[doc = "MDIOS input data register 29"]
pub mod mdios_dinr29;
#[doc = "MDIOS_DINR30 (r) register accessor: MDIOS input data register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr30`] module"]
#[doc(alias = "MDIOS_DINR30")]
pub type MdiosDinr30 = crate::Reg<mdios_dinr30::MdiosDinr30Spec>;
#[doc = "MDIOS input data register 30"]
pub mod mdios_dinr30;
#[doc = "MDIOS_DINR31 (r) register accessor: MDIOS input data register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_dinr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_dinr31`] module"]
#[doc(alias = "MDIOS_DINR31")]
pub type MdiosDinr31 = crate::Reg<mdios_dinr31::MdiosDinr31Spec>;
#[doc = "MDIOS input data register 31"]
pub mod mdios_dinr31;
#[doc = "MDIOS_DOUTR0 (rw) register accessor: MDIOS output data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr0`] module"]
#[doc(alias = "MDIOS_DOUTR0")]
pub type MdiosDoutr0 = crate::Reg<mdios_doutr0::MdiosDoutr0Spec>;
#[doc = "MDIOS output data register 0"]
pub mod mdios_doutr0;
#[doc = "MDIOS_DOUTR1 (rw) register accessor: MDIOS output data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr1`] module"]
#[doc(alias = "MDIOS_DOUTR1")]
pub type MdiosDoutr1 = crate::Reg<mdios_doutr1::MdiosDoutr1Spec>;
#[doc = "MDIOS output data register 1"]
pub mod mdios_doutr1;
#[doc = "MDIOS_DOUTR2 (rw) register accessor: MDIOS output data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr2`] module"]
#[doc(alias = "MDIOS_DOUTR2")]
pub type MdiosDoutr2 = crate::Reg<mdios_doutr2::MdiosDoutr2Spec>;
#[doc = "MDIOS output data register 2"]
pub mod mdios_doutr2;
#[doc = "MDIOS_DOUTR3 (rw) register accessor: MDIOS output data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr3`] module"]
#[doc(alias = "MDIOS_DOUTR3")]
pub type MdiosDoutr3 = crate::Reg<mdios_doutr3::MdiosDoutr3Spec>;
#[doc = "MDIOS output data register 3"]
pub mod mdios_doutr3;
#[doc = "MDIOS_DOUTR4 (rw) register accessor: MDIOS output data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr4`] module"]
#[doc(alias = "MDIOS_DOUTR4")]
pub type MdiosDoutr4 = crate::Reg<mdios_doutr4::MdiosDoutr4Spec>;
#[doc = "MDIOS output data register 4"]
pub mod mdios_doutr4;
#[doc = "MDIOS_DOUTR5 (rw) register accessor: MDIOS output data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr5`] module"]
#[doc(alias = "MDIOS_DOUTR5")]
pub type MdiosDoutr5 = crate::Reg<mdios_doutr5::MdiosDoutr5Spec>;
#[doc = "MDIOS output data register 5"]
pub mod mdios_doutr5;
#[doc = "MDIOS_DOUTR6 (rw) register accessor: MDIOS output data register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr6`] module"]
#[doc(alias = "MDIOS_DOUTR6")]
pub type MdiosDoutr6 = crate::Reg<mdios_doutr6::MdiosDoutr6Spec>;
#[doc = "MDIOS output data register 6"]
pub mod mdios_doutr6;
#[doc = "MDIOS_DOUTR7 (rw) register accessor: MDIOS output data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr7`] module"]
#[doc(alias = "MDIOS_DOUTR7")]
pub type MdiosDoutr7 = crate::Reg<mdios_doutr7::MdiosDoutr7Spec>;
#[doc = "MDIOS output data register 7"]
pub mod mdios_doutr7;
#[doc = "MDIOS_DOUTR8 (rw) register accessor: MDIOS output data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr8`] module"]
#[doc(alias = "MDIOS_DOUTR8")]
pub type MdiosDoutr8 = crate::Reg<mdios_doutr8::MdiosDoutr8Spec>;
#[doc = "MDIOS output data register 8"]
pub mod mdios_doutr8;
#[doc = "MDIOS_DOUTR9 (rw) register accessor: MDIOS output data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr9`] module"]
#[doc(alias = "MDIOS_DOUTR9")]
pub type MdiosDoutr9 = crate::Reg<mdios_doutr9::MdiosDoutr9Spec>;
#[doc = "MDIOS output data register 9"]
pub mod mdios_doutr9;
#[doc = "MDIOS_DOUTR10 (rw) register accessor: MDIOS output data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr10`] module"]
#[doc(alias = "MDIOS_DOUTR10")]
pub type MdiosDoutr10 = crate::Reg<mdios_doutr10::MdiosDoutr10Spec>;
#[doc = "MDIOS output data register 10"]
pub mod mdios_doutr10;
#[doc = "MDIOS_DOUTR11 (rw) register accessor: MDIOS output data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr11`] module"]
#[doc(alias = "MDIOS_DOUTR11")]
pub type MdiosDoutr11 = crate::Reg<mdios_doutr11::MdiosDoutr11Spec>;
#[doc = "MDIOS output data register 11"]
pub mod mdios_doutr11;
#[doc = "MDIOS_DOUTR12 (rw) register accessor: MDIOS output data register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr12`] module"]
#[doc(alias = "MDIOS_DOUTR12")]
pub type MdiosDoutr12 = crate::Reg<mdios_doutr12::MdiosDoutr12Spec>;
#[doc = "MDIOS output data register 12"]
pub mod mdios_doutr12;
#[doc = "MDIOS_DOUTR13 (rw) register accessor: MDIOS output data register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr13`] module"]
#[doc(alias = "MDIOS_DOUTR13")]
pub type MdiosDoutr13 = crate::Reg<mdios_doutr13::MdiosDoutr13Spec>;
#[doc = "MDIOS output data register 13"]
pub mod mdios_doutr13;
#[doc = "MDIOS_DOUTR14 (rw) register accessor: MDIOS output data register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr14`] module"]
#[doc(alias = "MDIOS_DOUTR14")]
pub type MdiosDoutr14 = crate::Reg<mdios_doutr14::MdiosDoutr14Spec>;
#[doc = "MDIOS output data register 14"]
pub mod mdios_doutr14;
#[doc = "MDIOS_DOUTR15 (rw) register accessor: MDIOS output data register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr15`] module"]
#[doc(alias = "MDIOS_DOUTR15")]
pub type MdiosDoutr15 = crate::Reg<mdios_doutr15::MdiosDoutr15Spec>;
#[doc = "MDIOS output data register 15"]
pub mod mdios_doutr15;
#[doc = "MDIOS_DOUTR16 (rw) register accessor: MDIOS output data register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr16`] module"]
#[doc(alias = "MDIOS_DOUTR16")]
pub type MdiosDoutr16 = crate::Reg<mdios_doutr16::MdiosDoutr16Spec>;
#[doc = "MDIOS output data register 16"]
pub mod mdios_doutr16;
#[doc = "MDIOS_DOUTR17 (rw) register accessor: MDIOS output data register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr17`] module"]
#[doc(alias = "MDIOS_DOUTR17")]
pub type MdiosDoutr17 = crate::Reg<mdios_doutr17::MdiosDoutr17Spec>;
#[doc = "MDIOS output data register 17"]
pub mod mdios_doutr17;
#[doc = "MDIOS_DOUTR18 (rw) register accessor: MDIOS output data register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr18`] module"]
#[doc(alias = "MDIOS_DOUTR18")]
pub type MdiosDoutr18 = crate::Reg<mdios_doutr18::MdiosDoutr18Spec>;
#[doc = "MDIOS output data register 18"]
pub mod mdios_doutr18;
#[doc = "MDIOS_DOUTR19 (rw) register accessor: MDIOS output data register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr19`] module"]
#[doc(alias = "MDIOS_DOUTR19")]
pub type MdiosDoutr19 = crate::Reg<mdios_doutr19::MdiosDoutr19Spec>;
#[doc = "MDIOS output data register 19"]
pub mod mdios_doutr19;
#[doc = "MDIOS_DOUTR20 (rw) register accessor: MDIOS output data register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr20`] module"]
#[doc(alias = "MDIOS_DOUTR20")]
pub type MdiosDoutr20 = crate::Reg<mdios_doutr20::MdiosDoutr20Spec>;
#[doc = "MDIOS output data register 20"]
pub mod mdios_doutr20;
#[doc = "MDIOS_DOUTR21 (rw) register accessor: MDIOS output data register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr21`] module"]
#[doc(alias = "MDIOS_DOUTR21")]
pub type MdiosDoutr21 = crate::Reg<mdios_doutr21::MdiosDoutr21Spec>;
#[doc = "MDIOS output data register 21"]
pub mod mdios_doutr21;
#[doc = "MDIOS_DOUTR22 (rw) register accessor: MDIOS output data register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr22`] module"]
#[doc(alias = "MDIOS_DOUTR22")]
pub type MdiosDoutr22 = crate::Reg<mdios_doutr22::MdiosDoutr22Spec>;
#[doc = "MDIOS output data register 22"]
pub mod mdios_doutr22;
#[doc = "MDIOS_DOUTR23 (rw) register accessor: MDIOS output data register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr23`] module"]
#[doc(alias = "MDIOS_DOUTR23")]
pub type MdiosDoutr23 = crate::Reg<mdios_doutr23::MdiosDoutr23Spec>;
#[doc = "MDIOS output data register 23"]
pub mod mdios_doutr23;
#[doc = "MDIOS_DOUTR24 (rw) register accessor: MDIOS output data register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr24`] module"]
#[doc(alias = "MDIOS_DOUTR24")]
pub type MdiosDoutr24 = crate::Reg<mdios_doutr24::MdiosDoutr24Spec>;
#[doc = "MDIOS output data register 24"]
pub mod mdios_doutr24;
#[doc = "MDIOS_DOUTR25 (rw) register accessor: MDIOS output data register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr25`] module"]
#[doc(alias = "MDIOS_DOUTR25")]
pub type MdiosDoutr25 = crate::Reg<mdios_doutr25::MdiosDoutr25Spec>;
#[doc = "MDIOS output data register 25"]
pub mod mdios_doutr25;
#[doc = "MDIOS_DOUTR26 (rw) register accessor: MDIOS output data register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr26`] module"]
#[doc(alias = "MDIOS_DOUTR26")]
pub type MdiosDoutr26 = crate::Reg<mdios_doutr26::MdiosDoutr26Spec>;
#[doc = "MDIOS output data register 26"]
pub mod mdios_doutr26;
#[doc = "MDIOS_DOUTR27 (rw) register accessor: MDIOS output data register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr27`] module"]
#[doc(alias = "MDIOS_DOUTR27")]
pub type MdiosDoutr27 = crate::Reg<mdios_doutr27::MdiosDoutr27Spec>;
#[doc = "MDIOS output data register 27"]
pub mod mdios_doutr27;
#[doc = "MDIOS_DOUTR28 (rw) register accessor: MDIOS output data register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr28`] module"]
#[doc(alias = "MDIOS_DOUTR28")]
pub type MdiosDoutr28 = crate::Reg<mdios_doutr28::MdiosDoutr28Spec>;
#[doc = "MDIOS output data register 28"]
pub mod mdios_doutr28;
#[doc = "MDIOS_DOUTR29 (rw) register accessor: MDIOS output data register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr29`] module"]
#[doc(alias = "MDIOS_DOUTR29")]
pub type MdiosDoutr29 = crate::Reg<mdios_doutr29::MdiosDoutr29Spec>;
#[doc = "MDIOS output data register 29"]
pub mod mdios_doutr29;
#[doc = "MDIOS_DOUTR30 (rw) register accessor: MDIOS output data register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr30`] module"]
#[doc(alias = "MDIOS_DOUTR30")]
pub type MdiosDoutr30 = crate::Reg<mdios_doutr30::MdiosDoutr30Spec>;
#[doc = "MDIOS output data register 30"]
pub mod mdios_doutr30;
#[doc = "MDIOS_DOUTR31 (rw) register accessor: MDIOS output data register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mdios_doutr31`] module"]
#[doc(alias = "MDIOS_DOUTR31")]
pub type MdiosDoutr31 = crate::Reg<mdios_doutr31::MdiosDoutr31Spec>;
#[doc = "MDIOS output data register 31"]
pub mod mdios_doutr31;
