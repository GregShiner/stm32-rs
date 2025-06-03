#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    sr: Sr,
    icfr: Icfr,
    or: Or,
    cfgr1: Cfgr1,
    cfgr2: Cfgr2,
}
impl RegisterBlock {
    #[doc = "0x00 - Comparator status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x04 - Comparator interrupt clear flag register"]
    #[inline(always)]
    pub const fn icfr(&self) -> &Icfr {
        &self.icfr
    }
    #[doc = "0x08 - Comparator option register"]
    #[inline(always)]
    pub const fn or(&self) -> &Or {
        &self.or
    }
    #[doc = "0x0c - Comparator configuration register 1"]
    #[inline(always)]
    pub const fn cfgr1(&self) -> &Cfgr1 {
        &self.cfgr1
    }
    #[doc = "0x10 - Comparator configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
}
#[doc = "SR (r) register accessor: Comparator status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Comparator status register"]
pub mod sr;
#[doc = "ICFR (w) register accessor: Comparator interrupt clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icfr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icfr`] module"]
#[doc(alias = "ICFR")]
pub type Icfr = crate::Reg<icfr::IcfrSpec>;
#[doc = "Comparator interrupt clear flag register"]
pub mod icfr;
#[doc = "OR (rw) register accessor: Comparator option register\n\nYou can [`read`](crate::Reg::read) this register and get [`or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@or`] module"]
#[doc(alias = "OR")]
pub type Or = crate::Reg<or::OrSpec>;
#[doc = "Comparator option register"]
pub mod or;
#[doc = "CFGR1 (rw) register accessor: Comparator configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr1`] module"]
#[doc(alias = "CFGR1")]
pub type Cfgr1 = crate::Reg<cfgr1::Cfgr1Spec>;
#[doc = "Comparator configuration register 1"]
pub mod cfgr1;
#[doc = "CFGR2 (rw) register accessor: Comparator configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "Comparator configuration register 2"]
pub mod cfgr2;
