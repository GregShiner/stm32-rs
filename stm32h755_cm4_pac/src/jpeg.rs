#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    confr0: Confr0,
    confr1: Confr1,
    confr2: Confr2,
    confr3: Confr3,
    confrn1: Confrn1,
    confrn2: Confrn2,
    confrn3: Confrn3,
    confrn4: Confrn4,
    _reserved8: [u8; 0x10],
    cr: Cr,
    sr: Sr,
    cfr: Cfr,
    _reserved11: [u8; 0x04],
    dir: Dir,
    dor: Dor,
}
impl RegisterBlock {
    #[doc = "0x00 - JPEG codec control register"]
    #[inline(always)]
    pub const fn confr0(&self) -> &Confr0 {
        &self.confr0
    }
    #[doc = "0x04 - JPEG codec configuration register 1"]
    #[inline(always)]
    pub const fn confr1(&self) -> &Confr1 {
        &self.confr1
    }
    #[doc = "0x08 - JPEG codec configuration register 2"]
    #[inline(always)]
    pub const fn confr2(&self) -> &Confr2 {
        &self.confr2
    }
    #[doc = "0x0c - JPEG codec configuration register 3"]
    #[inline(always)]
    pub const fn confr3(&self) -> &Confr3 {
        &self.confr3
    }
    #[doc = "0x10 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn1(&self) -> &Confrn1 {
        &self.confrn1
    }
    #[doc = "0x14 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn2(&self) -> &Confrn2 {
        &self.confrn2
    }
    #[doc = "0x18 - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn3(&self) -> &Confrn3 {
        &self.confrn3
    }
    #[doc = "0x1c - JPEG codec configuration register 4-7"]
    #[inline(always)]
    pub const fn confrn4(&self) -> &Confrn4 {
        &self.confrn4
    }
    #[doc = "0x30 - JPEG control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x34 - JPEG status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x38 - JPEG clear flag register"]
    #[inline(always)]
    pub const fn cfr(&self) -> &Cfr {
        &self.cfr
    }
    #[doc = "0x40 - JPEG data input register"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x44 - JPEG data output register"]
    #[inline(always)]
    pub const fn dor(&self) -> &Dor {
        &self.dor
    }
}
#[doc = "CONFR0 (w) register accessor: JPEG codec control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr0`] module"]
#[doc(alias = "CONFR0")]
pub type Confr0 = crate::Reg<confr0::Confr0Spec>;
#[doc = "JPEG codec control register"]
pub mod confr0;
#[doc = "CONFR1 (rw) register accessor: JPEG codec configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`confr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr1`] module"]
#[doc(alias = "CONFR1")]
pub type Confr1 = crate::Reg<confr1::Confr1Spec>;
#[doc = "JPEG codec configuration register 1"]
pub mod confr1;
#[doc = "CONFR2 (rw) register accessor: JPEG codec configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`confr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr2`] module"]
#[doc(alias = "CONFR2")]
pub type Confr2 = crate::Reg<confr2::Confr2Spec>;
#[doc = "JPEG codec configuration register 2"]
pub mod confr2;
#[doc = "CONFR3 (rw) register accessor: JPEG codec configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`confr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confr3`] module"]
#[doc(alias = "CONFR3")]
pub type Confr3 = crate::Reg<confr3::Confr3Spec>;
#[doc = "JPEG codec configuration register 3"]
pub mod confr3;
#[doc = "CONFRN1 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`confrn1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn1`] module"]
#[doc(alias = "CONFRN1")]
pub type Confrn1 = crate::Reg<confrn1::Confrn1Spec>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn1;
#[doc = "CONFRN2 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`confrn2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn2`] module"]
#[doc(alias = "CONFRN2")]
pub type Confrn2 = crate::Reg<confrn2::Confrn2Spec>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn2;
#[doc = "CONFRN3 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`confrn3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn3`] module"]
#[doc(alias = "CONFRN3")]
pub type Confrn3 = crate::Reg<confrn3::Confrn3Spec>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn3;
#[doc = "CONFRN4 (rw) register accessor: JPEG codec configuration register 4-7\n\nYou can [`read`](crate::Reg::read) this register and get [`confrn4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confrn4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@confrn4`] module"]
#[doc(alias = "CONFRN4")]
pub type Confrn4 = crate::Reg<confrn4::Confrn4Spec>;
#[doc = "JPEG codec configuration register 4-7"]
pub mod confrn4;
#[doc = "CR (rw) register accessor: JPEG control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "JPEG control register"]
pub mod cr;
#[doc = "SR (r) register accessor: JPEG status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "JPEG status register"]
pub mod sr;
#[doc = "CFR (rw) register accessor: JPEG clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfr`] module"]
#[doc(alias = "CFR")]
pub type Cfr = crate::Reg<cfr::CfrSpec>;
#[doc = "JPEG clear flag register"]
pub mod cfr;
#[doc = "DIR (w) register accessor: JPEG data input register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dir::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "JPEG data input register"]
pub mod dir;
#[doc = "DOR (r) register accessor: JPEG data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor`] module"]
#[doc(alias = "DOR")]
pub type Dor = crate::Reg<dor::DorSpec>;
#[doc = "JPEG data output register"]
pub mod dor;
