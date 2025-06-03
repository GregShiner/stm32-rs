#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    idc: Idc,
    cr: Cr,
    _reserved2: [u8; 0x2c],
    apb3fz1: Apb3fz1,
    apb3fz2: Apb3fz2,
    apb1lfz1: Apb1lfz1,
    apb1lfz2: Apb1lfz2,
    _reserved6: [u8; 0x08],
    apb2fz1: Apb2fz1,
    apb2fz2: Apb2fz2,
    apb4fz1: Apb4fz1,
    apb4fz2: Apb4fz2,
}
impl RegisterBlock {
    #[doc = "0x00 - DBGMCU Identity Code Register"]
    #[inline(always)]
    pub const fn idc(&self) -> &Idc {
        &self.idc
    }
    #[doc = "0x04 - DBGMCU Configuration Register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x34 - DBGMCU APB3 peripheral freeze register CPU1"]
    #[inline(always)]
    pub const fn apb3fz1(&self) -> &Apb3fz1 {
        &self.apb3fz1
    }
    #[doc = "0x38 - DBGMCU APB3 peripheral freeze register CPU2"]
    #[inline(always)]
    pub const fn apb3fz2(&self) -> &Apb3fz2 {
        &self.apb3fz2
    }
    #[doc = "0x3c - DBGMCU APB1L peripheral freeze register"]
    #[inline(always)]
    pub const fn apb1lfz1(&self) -> &Apb1lfz1 {
        &self.apb1lfz1
    }
    #[doc = "0x40 - DBGMCU APB1L peripheral freeze register CPU2"]
    #[inline(always)]
    pub const fn apb1lfz2(&self) -> &Apb1lfz2 {
        &self.apb1lfz2
    }
    #[doc = "0x4c - DBGMCU APB2 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb2fz1(&self) -> &Apb2fz1 {
        &self.apb2fz1
    }
    #[doc = "0x50 - DBGMCU APB2 peripheral freeze register CPU2"]
    #[inline(always)]
    pub const fn apb2fz2(&self) -> &Apb2fz2 {
        &self.apb2fz2
    }
    #[doc = "0x54 - DBGMCU APB4 peripheral freeze register"]
    #[inline(always)]
    pub const fn apb4fz1(&self) -> &Apb4fz1 {
        &self.apb4fz1
    }
    #[doc = "0x58 - DBGMCU APB4 peripheral freeze register CPU2"]
    #[inline(always)]
    pub const fn apb4fz2(&self) -> &Apb4fz2 {
        &self.apb4fz2
    }
}
#[doc = "IDC (r) register accessor: DBGMCU Identity Code Register\n\nYou can [`read`](crate::Reg::read) this register and get [`idc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idc`] module"]
#[doc(alias = "IDC")]
pub type Idc = crate::Reg<idc::IdcSpec>;
#[doc = "DBGMCU Identity Code Register"]
pub mod idc;
#[doc = "CR (rw) register accessor: DBGMCU Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "DBGMCU Configuration Register"]
pub mod cr;
#[doc = "APB3FZ1 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU1\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3fz1`] module"]
#[doc(alias = "APB3FZ1")]
pub type Apb3fz1 = crate::Reg<apb3fz1::Apb3fz1Spec>;
#[doc = "DBGMCU APB3 peripheral freeze register CPU1"]
pub mod apb3fz1;
#[doc = "APB3FZ2 (rw) register accessor: DBGMCU APB3 peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3fz2`] module"]
#[doc(alias = "APB3FZ2")]
pub type Apb3fz2 = crate::Reg<apb3fz2::Apb3fz2Spec>;
#[doc = "DBGMCU APB3 peripheral freeze register CPU2"]
pub mod apb3fz2;
#[doc = "APB1LFZ1 (rw) register accessor: DBGMCU APB1L peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lfz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lfz1`] module"]
#[doc(alias = "APB1LFZ1")]
pub type Apb1lfz1 = crate::Reg<apb1lfz1::Apb1lfz1Spec>;
#[doc = "DBGMCU APB1L peripheral freeze register"]
pub mod apb1lfz1;
#[doc = "APB1LFZ2 (rw) register accessor: DBGMCU APB1L peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lfz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lfz2`] module"]
#[doc(alias = "APB1LFZ2")]
pub type Apb1lfz2 = crate::Reg<apb1lfz2::Apb1lfz2Spec>;
#[doc = "DBGMCU APB1L peripheral freeze register CPU2"]
pub mod apb1lfz2;
#[doc = "APB2FZ1 (rw) register accessor: DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fz1`] module"]
#[doc(alias = "APB2FZ1")]
pub type Apb2fz1 = crate::Reg<apb2fz1::Apb2fz1Spec>;
#[doc = "DBGMCU APB2 peripheral freeze register"]
pub mod apb2fz1;
#[doc = "APB2FZ2 (rw) register accessor: DBGMCU APB2 peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2fz2`] module"]
#[doc(alias = "APB2FZ2")]
pub type Apb2fz2 = crate::Reg<apb2fz2::Apb2fz2Spec>;
#[doc = "DBGMCU APB2 peripheral freeze register CPU2"]
pub mod apb2fz2;
#[doc = "APB4FZ1 (rw) register accessor: DBGMCU APB4 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4fz1`] module"]
#[doc(alias = "APB4FZ1")]
pub type Apb4fz1 = crate::Reg<apb4fz1::Apb4fz1Spec>;
#[doc = "DBGMCU APB4 peripheral freeze register"]
pub mod apb4fz1;
#[doc = "APB4FZ2 (rw) register accessor: DBGMCU APB4 peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4fz2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4fz2`] module"]
#[doc(alias = "APB4FZ2")]
pub type Apb4fz2 = crate::Reg<apb4fz2::Apb4fz2Spec>;
#[doc = "DBGMCU APB4 peripheral freeze register CPU2"]
pub mod apb4fz2;
