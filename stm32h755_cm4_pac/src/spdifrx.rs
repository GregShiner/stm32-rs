#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    imr: Imr,
    sr: Sr,
    ifcr: Ifcr,
    _reserved_4_dr_: [u8; 0x04],
    csr: Csr,
    dir: Dir,
    _reserved7: [u8; 0x03d8],
    verr: Verr,
    idr: Idr,
    sidr: Sidr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - Interrupt mask register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x08 - Status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - Interrupt Flag Clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_10(&self) -> &Dr10 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_01(&self) -> &Dr01 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x10 - Data input register"]
    #[inline(always)]
    pub const fn dr_00(&self) -> &Dr00 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    #[doc = "0x14 - Channel Status register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x18 - Debug Information register"]
    #[inline(always)]
    pub const fn dir(&self) -> &Dir {
        &self.dir
    }
    #[doc = "0x3f4 - SPDIFRX version register"]
    #[inline(always)]
    pub const fn verr(&self) -> &Verr {
        &self.verr
    }
    #[doc = "0x3f8 - SPDIFRX identification register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x3fc - SPDIFRX size identification register"]
    #[inline(always)]
    pub const fn sidr(&self) -> &Sidr {
        &self.sidr
    }
}
#[doc = "CR (rw) register accessor: Control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "Control register"]
pub mod cr;
#[doc = "IMR (rw) register accessor: Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`] module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt mask register"]
pub mod imr;
#[doc = "SR (r) register accessor: Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Status register"]
pub mod sr;
#[doc = "IFCR (w) register accessor: Interrupt Flag Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "Interrupt Flag Clear register"]
pub mod ifcr;
#[doc = "DR_00 (r) register accessor: Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_00::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_00`] module"]
#[doc(alias = "DR_00")]
pub type Dr00 = crate::Reg<dr_00::Dr00Spec>;
#[doc = "Data input register"]
pub mod dr_00;
#[doc = "CSR (r) register accessor: Channel Status register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Channel Status register"]
pub mod csr;
#[doc = "DIR (r) register accessor: Debug Information register\n\nYou can [`read`](crate::Reg::read) this register and get [`dir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dir`] module"]
#[doc(alias = "DIR")]
pub type Dir = crate::Reg<dir::DirSpec>;
#[doc = "Debug Information register"]
pub mod dir;
#[doc = "VERR (r) register accessor: SPDIFRX version register\n\nYou can [`read`](crate::Reg::read) this register and get [`verr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@verr`] module"]
#[doc(alias = "VERR")]
pub type Verr = crate::Reg<verr::VerrSpec>;
#[doc = "SPDIFRX version register"]
pub mod verr;
#[doc = "IDR (r) register accessor: SPDIFRX identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`idr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`] module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "SPDIFRX identification register"]
pub mod idr;
#[doc = "SIDR (r) register accessor: SPDIFRX size identification register\n\nYou can [`read`](crate::Reg::read) this register and get [`sidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sidr`] module"]
#[doc(alias = "SIDR")]
pub type Sidr = crate::Reg<sidr::SidrSpec>;
#[doc = "SPDIFRX size identification register"]
pub mod sidr;
#[doc = "DR_01 (r) register accessor: Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_01::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_01`] module"]
#[doc(alias = "DR_01")]
pub type Dr01 = crate::Reg<dr_01::Dr01Spec>;
#[doc = "Data input register"]
pub mod dr_01;
#[doc = "DR_10 (r) register accessor: Data input register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr_10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr_10`] module"]
#[doc(alias = "DR_10")]
pub type Dr10 = crate::Reg<dr_10::Dr10Spec>;
#[doc = "Data input register"]
pub mod dr_10;
