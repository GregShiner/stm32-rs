#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    dcr: Dcr,
    sr: Sr,
    fcr: Fcr,
    dlr: Dlr,
    ccr: Ccr,
    ar: Ar,
    abr: Abr,
    dr: Dr,
    psmkr: Psmkr,
    psmar: Psmar,
    pir: Pir,
    lptr: Lptr,
}
impl RegisterBlock {
    #[doc = "0x00 - QUADSPI control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - QUADSPI device configuration register"]
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    #[doc = "0x08 - QUADSPI status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x0c - QUADSPI flag clear register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x10 - QUADSPI data length register"]
    #[inline(always)]
    pub const fn dlr(&self) -> &Dlr {
        &self.dlr
    }
    #[doc = "0x14 - QUADSPI communication configuration register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x18 - QUADSPI address register"]
    #[inline(always)]
    pub const fn ar(&self) -> &Ar {
        &self.ar
    }
    #[doc = "0x1c - QUADSPI alternate bytes registers"]
    #[inline(always)]
    pub const fn abr(&self) -> &Abr {
        &self.abr
    }
    #[doc = "0x20 - QUADSPI data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x24 - QUADSPI polling status mask register"]
    #[inline(always)]
    pub const fn psmkr(&self) -> &Psmkr {
        &self.psmkr
    }
    #[doc = "0x28 - QUADSPI polling status match register"]
    #[inline(always)]
    pub const fn psmar(&self) -> &Psmar {
        &self.psmar
    }
    #[doc = "0x2c - QUADSPI polling interval register"]
    #[inline(always)]
    pub const fn pir(&self) -> &Pir {
        &self.pir
    }
    #[doc = "0x30 - QUADSPI low-power timeout register"]
    #[inline(always)]
    pub const fn lptr(&self) -> &Lptr {
        &self.lptr
    }
}
#[doc = "CR (rw) register accessor: QUADSPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "QUADSPI control register"]
pub mod cr;
#[doc = "DCR (rw) register accessor: QUADSPI device configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dcr`] module"]
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
#[doc = "QUADSPI device configuration register"]
pub mod dcr;
#[doc = "SR (r) register accessor: QUADSPI status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "QUADSPI status register"]
pub mod sr;
#[doc = "FCR (rw) register accessor: QUADSPI flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`] module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "QUADSPI flag clear register"]
pub mod fcr;
#[doc = "DLR (rw) register accessor: QUADSPI data length register\n\nYou can [`read`](crate::Reg::read) this register and get [`dlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dlr`] module"]
#[doc(alias = "DLR")]
pub type Dlr = crate::Reg<dlr::DlrSpec>;
#[doc = "QUADSPI data length register"]
pub mod dlr;
#[doc = "CCR (rw) register accessor: QUADSPI communication configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "QUADSPI communication configuration register"]
pub mod ccr;
#[doc = "AR (rw) register accessor: QUADSPI address register\n\nYou can [`read`](crate::Reg::read) this register and get [`ar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ar`] module"]
#[doc(alias = "AR")]
pub type Ar = crate::Reg<ar::ArSpec>;
#[doc = "QUADSPI address register"]
pub mod ar;
#[doc = "ABR (rw) register accessor: QUADSPI alternate bytes registers\n\nYou can [`read`](crate::Reg::read) this register and get [`abr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`abr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@abr`] module"]
#[doc(alias = "ABR")]
pub type Abr = crate::Reg<abr::AbrSpec>;
#[doc = "QUADSPI alternate bytes registers"]
pub mod abr;
#[doc = "DR (rw) register accessor: QUADSPI data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "QUADSPI data register"]
pub mod dr;
#[doc = "PSMKR (rw) register accessor: QUADSPI polling status mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmkr`] module"]
#[doc(alias = "PSMKR")]
pub type Psmkr = crate::Reg<psmkr::PsmkrSpec>;
#[doc = "QUADSPI polling status mask register"]
pub mod psmkr;
#[doc = "PSMAR (rw) register accessor: QUADSPI polling status match register\n\nYou can [`read`](crate::Reg::read) this register and get [`psmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@psmar`] module"]
#[doc(alias = "PSMAR")]
pub type Psmar = crate::Reg<psmar::PsmarSpec>;
#[doc = "QUADSPI polling status match register"]
pub mod psmar;
#[doc = "PIR (rw) register accessor: QUADSPI polling interval register\n\nYou can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pir`] module"]
#[doc(alias = "PIR")]
pub type Pir = crate::Reg<pir::PirSpec>;
#[doc = "QUADSPI polling interval register"]
pub mod pir;
#[doc = "LPTR (rw) register accessor: QUADSPI low-power timeout register\n\nYou can [`read`](crate::Reg::read) this register and get [`lptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lptr`] module"]
#[doc(alias = "LPTR")]
pub type Lptr = crate::Reg<lptr::LptrSpec>;
#[doc = "QUADSPI low-power timeout register"]
pub mod lptr;
