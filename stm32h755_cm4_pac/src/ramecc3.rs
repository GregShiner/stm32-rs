#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    ier: Ier,
    _reserved1: [u8; 0x1c],
    m1cr: M1cr,
    m1sr: M1sr,
    m1far: M1far,
    m1fdrl: M1fdrl,
    m1fdrh: M1fdrh,
    m1fecr: M1fecr,
    _reserved7: [u8; 0x08],
    m2cr: M2cr,
    m2sr: M2sr,
    m2far: M2far,
    m2fdrl: M2fdrl,
    m2fdrh: M2fdrh,
    _reserved12: [u8; 0x04],
    m2fecr: M2fecr,
}
impl RegisterBlock {
    #[doc = "0x00 - RAMECC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x20 - RAMECC monitor x configuration register"]
    #[inline(always)]
    pub const fn m1cr(&self) -> &M1cr {
        &self.m1cr
    }
    #[doc = "0x24 - RAMECC monitor x status register"]
    #[inline(always)]
    pub const fn m1sr(&self) -> &M1sr {
        &self.m1sr
    }
    #[doc = "0x28 - RAMECC monitor x failing address register"]
    #[inline(always)]
    pub const fn m1far(&self) -> &M1far {
        &self.m1far
    }
    #[doc = "0x2c - RAMECC monitor x failing data low register"]
    #[inline(always)]
    pub const fn m1fdrl(&self) -> &M1fdrl {
        &self.m1fdrl
    }
    #[doc = "0x30 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m1fdrh(&self) -> &M1fdrh {
        &self.m1fdrh
    }
    #[doc = "0x34 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m1fecr(&self) -> &M1fecr {
        &self.m1fecr
    }
    #[doc = "0x40 - RAMECC monitor x configuration register"]
    #[inline(always)]
    pub const fn m2cr(&self) -> &M2cr {
        &self.m2cr
    }
    #[doc = "0x44 - RAMECC monitor x status register"]
    #[inline(always)]
    pub const fn m2sr(&self) -> &M2sr {
        &self.m2sr
    }
    #[doc = "0x48 - RAMECC monitor x failing address register"]
    #[inline(always)]
    pub const fn m2far(&self) -> &M2far {
        &self.m2far
    }
    #[doc = "0x4c - RAMECC monitor x failing data low register"]
    #[inline(always)]
    pub const fn m2fdrl(&self) -> &M2fdrl {
        &self.m2fdrl
    }
    #[doc = "0x50 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m2fdrh(&self) -> &M2fdrh {
        &self.m2fdrh
    }
    #[doc = "0x58 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m2fecr(&self) -> &M2fecr {
        &self.m2fecr
    }
}
#[doc = "IER (rw) register accessor: RAMECC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "RAMECC interrupt enable register"]
pub mod ier;
#[doc = "M1CR (rw) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1cr`] module"]
#[doc(alias = "M1CR")]
pub type M1cr = crate::Reg<m1cr::M1crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m1cr;
#[doc = "M2CR (rw) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2cr`] module"]
#[doc(alias = "M2CR")]
pub type M2cr = crate::Reg<m2cr::M2crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m2cr;
#[doc = "M1SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1sr`] module"]
#[doc(alias = "M1SR")]
pub type M1sr = crate::Reg<m1sr::M1srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m1sr;
#[doc = "M2SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2sr`] module"]
#[doc(alias = "M2SR")]
pub type M2sr = crate::Reg<m2sr::M2srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m2sr;
#[doc = "M1FAR (r) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1far::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1far`] module"]
#[doc(alias = "M1FAR")]
pub type M1far = crate::Reg<m1far::M1farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m1far;
#[doc = "M2FAR (r) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2far::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2far`] module"]
#[doc(alias = "M2FAR")]
pub type M2far = crate::Reg<m2far::M2farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m2far;
#[doc = "M1FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrl`] module"]
#[doc(alias = "M1FDRL")]
pub type M1fdrl = crate::Reg<m1fdrl::M1fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m1fdrl;
#[doc = "M2FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrl`] module"]
#[doc(alias = "M2FDRL")]
pub type M2fdrl = crate::Reg<m2fdrl::M2fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m2fdrl;
#[doc = "M1FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrh`] module"]
#[doc(alias = "M1FDRH")]
pub type M1fdrh = crate::Reg<m1fdrh::M1fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m1fdrh;
#[doc = "M2FDRH (rw) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrh`] module"]
#[doc(alias = "M2FDRH")]
pub type M2fdrh = crate::Reg<m2fdrh::M2fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m2fdrh;
#[doc = "M1FECR (rw) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fecr`] module"]
#[doc(alias = "M1FECR")]
pub type M1fecr = crate::Reg<m1fecr::M1fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m1fecr;
#[doc = "M2FECR (rw) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fecr`] module"]
#[doc(alias = "M2FECR")]
pub type M2fecr = crate::Reg<m2fecr::M2fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m2fecr;
