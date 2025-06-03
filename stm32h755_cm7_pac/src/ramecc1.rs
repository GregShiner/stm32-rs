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
    _reserved13: [u8; 0x04],
    m3cr: M3cr,
    m3sr: M3sr,
    m3far: M3far,
    m3fdrl: M3fdrl,
    m3fdrh: M3fdrh,
    _reserved18: [u8; 0x08],
    m3fecr: M3fecr,
    m4cr: M4cr,
    m4sr: M4sr,
    m4far: M4far,
    m4fdrl: M4fdrl,
    _reserved_23_m: [u8; 0x04],
    _reserved24: [u8; 0x0c],
    m5cr: M5cr,
    m5sr: M5sr,
    m5far: M5far,
    m5fdrl: M5fdrl,
    m5fdrh: M5fdrh,
    m5fecr: M5fecr,
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
    #[doc = "0x60 - RAMECC monitor x configuration register"]
    #[inline(always)]
    pub const fn m3cr(&self) -> &M3cr {
        &self.m3cr
    }
    #[doc = "0x64 - RAMECC monitor x status register"]
    #[inline(always)]
    pub const fn m3sr(&self) -> &M3sr {
        &self.m3sr
    }
    #[doc = "0x68 - RAMECC monitor x failing address register"]
    #[inline(always)]
    pub const fn m3far(&self) -> &M3far {
        &self.m3far
    }
    #[doc = "0x6c - RAMECC monitor x failing data low register"]
    #[inline(always)]
    pub const fn m3fdrl(&self) -> &M3fdrl {
        &self.m3fdrl
    }
    #[doc = "0x70 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m3fdrh(&self) -> &M3fdrh {
        &self.m3fdrh
    }
    #[doc = "0x7c - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m3fecr(&self) -> &M3fecr {
        &self.m3fecr
    }
    #[doc = "0x80 - RAMECC monitor x configuration register"]
    #[inline(always)]
    pub const fn m4cr(&self) -> &M4cr {
        &self.m4cr
    }
    #[doc = "0x84 - RAMECC monitor x status register"]
    #[inline(always)]
    pub const fn m4sr(&self) -> &M4sr {
        &self.m4sr
    }
    #[doc = "0x88 - RAMECC monitor x failing address register"]
    #[inline(always)]
    pub const fn m4far(&self) -> &M4far {
        &self.m4far
    }
    #[doc = "0x8c - RAMECC monitor x failing data low register"]
    #[inline(always)]
    pub const fn m4fdrl(&self) -> &M4fdrl {
        &self.m4fdrl
    }
    #[doc = "0x90 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m4fecr(&self) -> &M4fecr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0x90 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m4fdrh(&self) -> &M4fdrh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).cast() }
    }
    #[doc = "0xa0 - RAMECC monitor x configuration register"]
    #[inline(always)]
    pub const fn m5cr(&self) -> &M5cr {
        &self.m5cr
    }
    #[doc = "0xa4 - RAMECC monitor x status register"]
    #[inline(always)]
    pub const fn m5sr(&self) -> &M5sr {
        &self.m5sr
    }
    #[doc = "0xa8 - RAMECC monitor x failing address register"]
    #[inline(always)]
    pub const fn m5far(&self) -> &M5far {
        &self.m5far
    }
    #[doc = "0xac - RAMECC monitor x failing data low register"]
    #[inline(always)]
    pub const fn m5fdrl(&self) -> &M5fdrl {
        &self.m5fdrl
    }
    #[doc = "0xb0 - RAMECC monitor x failing data high register"]
    #[inline(always)]
    pub const fn m5fdrh(&self) -> &M5fdrh {
        &self.m5fdrh
    }
    #[doc = "0xb4 - RAMECC monitor x failing ECC error code register"]
    #[inline(always)]
    pub const fn m5fecr(&self) -> &M5fecr {
        &self.m5fecr
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
#[doc = "M1SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1sr`] module"]
#[doc(alias = "M1SR")]
pub type M1sr = crate::Reg<m1sr::M1srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m1sr;
#[doc = "M1FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1far`] module"]
#[doc(alias = "M1FAR")]
pub type M1far = crate::Reg<m1far::M1farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m1far;
#[doc = "M1FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrl`] module"]
#[doc(alias = "M1FDRL")]
pub type M1fdrl = crate::Reg<m1fdrl::M1fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m1fdrl;
#[doc = "M1FDRH (rw) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fdrh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fdrh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fdrh`] module"]
#[doc(alias = "M1FDRH")]
pub type M1fdrh = crate::Reg<m1fdrh::M1fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m1fdrh;
#[doc = "M1FECR (rw) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m1fecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m1fecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m1fecr`] module"]
#[doc(alias = "M1FECR")]
pub type M1fecr = crate::Reg<m1fecr::M1fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m1fecr;
#[doc = "M2CR (rw) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2cr`] module"]
#[doc(alias = "M2CR")]
pub type M2cr = crate::Reg<m2cr::M2crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m2cr;
#[doc = "M2SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2sr`] module"]
#[doc(alias = "M2SR")]
pub type M2sr = crate::Reg<m2sr::M2srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m2sr;
#[doc = "M2FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2far`] module"]
#[doc(alias = "M2FAR")]
pub type M2far = crate::Reg<m2far::M2farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m2far;
#[doc = "M2FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrl`] module"]
#[doc(alias = "M2FDRL")]
pub type M2fdrl = crate::Reg<m2fdrl::M2fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m2fdrl;
#[doc = "M2FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fdrh`] module"]
#[doc(alias = "M2FDRH")]
pub type M2fdrh = crate::Reg<m2fdrh::M2fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m2fdrh;
#[doc = "M2FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2fecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m2fecr`] module"]
#[doc(alias = "M2FECR")]
pub type M2fecr = crate::Reg<m2fecr::M2fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m2fecr;
#[doc = "M3CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3cr`] module"]
#[doc(alias = "M3CR")]
pub type M3cr = crate::Reg<m3cr::M3crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m3cr;
#[doc = "M3SR (r) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3sr`] module"]
#[doc(alias = "M3SR")]
pub type M3sr = crate::Reg<m3sr::M3srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m3sr;
#[doc = "M3FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m3far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3far`] module"]
#[doc(alias = "M3FAR")]
pub type M3far = crate::Reg<m3far::M3farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m3far;
#[doc = "M3FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3fdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fdrl`] module"]
#[doc(alias = "M3FDRL")]
pub type M3fdrl = crate::Reg<m3fdrl::M3fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m3fdrl;
#[doc = "M3FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fdrh`] module"]
#[doc(alias = "M3FDRH")]
pub type M3fdrh = crate::Reg<m3fdrh::M3fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m3fdrh;
#[doc = "M3FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m3fecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m3fecr`] module"]
#[doc(alias = "M3FECR")]
pub type M3fecr = crate::Reg<m3fecr::M3fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m3fecr;
#[doc = "M4CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4cr`] module"]
#[doc(alias = "M4CR")]
pub type M4cr = crate::Reg<m4cr::M4crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m4cr;
#[doc = "M4SR (r) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4sr`] module"]
#[doc(alias = "M4SR")]
pub type M4sr = crate::Reg<m4sr::M4srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m4sr;
#[doc = "M4FAR (r) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4far::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4far`] module"]
#[doc(alias = "M4FAR")]
pub type M4far = crate::Reg<m4far::M4farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m4far;
#[doc = "M4FDRL (rw) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fdrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m4fdrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fdrl`] module"]
#[doc(alias = "M4FDRL")]
pub type M4fdrl = crate::Reg<m4fdrl::M4fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m4fdrl;
#[doc = "M4FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fdrh`] module"]
#[doc(alias = "M4FDRH")]
pub type M4fdrh = crate::Reg<m4fdrh::M4fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m4fdrh;
#[doc = "M4FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m4fecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m4fecr`] module"]
#[doc(alias = "M4FECR")]
pub type M4fecr = crate::Reg<m4fecr::M4fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m4fecr;
#[doc = "M5CR (r) register accessor: RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5cr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5cr`] module"]
#[doc(alias = "M5CR")]
pub type M5cr = crate::Reg<m5cr::M5crSpec>;
#[doc = "RAMECC monitor x configuration register"]
pub mod m5cr;
#[doc = "M5SR (rw) register accessor: RAMECC monitor x status register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5sr`] module"]
#[doc(alias = "M5SR")]
pub type M5sr = crate::Reg<m5sr::M5srSpec>;
#[doc = "RAMECC monitor x status register"]
pub mod m5sr;
#[doc = "M5FAR (rw) register accessor: RAMECC monitor x failing address register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5far::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m5far::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5far`] module"]
#[doc(alias = "M5FAR")]
pub type M5far = crate::Reg<m5far::M5farSpec>;
#[doc = "RAMECC monitor x failing address register"]
pub mod m5far;
#[doc = "M5FDRL (r) register accessor: RAMECC monitor x failing data low register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5fdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fdrl`] module"]
#[doc(alias = "M5FDRL")]
pub type M5fdrl = crate::Reg<m5fdrl::M5fdrlSpec>;
#[doc = "RAMECC monitor x failing data low register"]
pub mod m5fdrl;
#[doc = "M5FDRH (r) register accessor: RAMECC monitor x failing data high register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5fdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fdrh`] module"]
#[doc(alias = "M5FDRH")]
pub type M5fdrh = crate::Reg<m5fdrh::M5fdrhSpec>;
#[doc = "RAMECC monitor x failing data high register"]
pub mod m5fdrh;
#[doc = "M5FECR (r) register accessor: RAMECC monitor x failing ECC error code register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5fecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@m5fecr`] module"]
#[doc(alias = "M5FECR")]
pub type M5fecr = crate::Reg<m5fecr::M5fecrSpec>;
#[doc = "RAMECC monitor x failing ECC error code register"]
pub mod m5fecr;
