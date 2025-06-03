#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    opamp1_csr: Opamp1Csr,
    opamp1_otr: Opamp1Otr,
    opamp1_hsotr: Opamp1Hsotr,
    _reserved3: [u8; 0x04],
    opamp2_csr: Opamp2Csr,
    opamp2_otr: Opamp2Otr,
    opamp2_hsotr: Opamp2Hsotr,
}
impl RegisterBlock {
    #[doc = "0x00 - OPAMP1 control/status register"]
    #[inline(always)]
    pub const fn opamp1_csr(&self) -> &Opamp1Csr {
        &self.opamp1_csr
    }
    #[doc = "0x04 - OPAMP1 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp1_otr(&self) -> &Opamp1Otr {
        &self.opamp1_otr
    }
    #[doc = "0x08 - OPAMP1 offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp1_hsotr(&self) -> &Opamp1Hsotr {
        &self.opamp1_hsotr
    }
    #[doc = "0x10 - OPAMP2 control/status register"]
    #[inline(always)]
    pub const fn opamp2_csr(&self) -> &Opamp2Csr {
        &self.opamp2_csr
    }
    #[doc = "0x14 - OPAMP2 offset trimming register in normal mode"]
    #[inline(always)]
    pub const fn opamp2_otr(&self) -> &Opamp2Otr {
        &self.opamp2_otr
    }
    #[doc = "0x18 - OPAMP2 offset trimming register in low-power mode"]
    #[inline(always)]
    pub const fn opamp2_hsotr(&self) -> &Opamp2Hsotr {
        &self.opamp2_hsotr
    }
}
#[doc = "OPAMP1_CSR (rw) register accessor: OPAMP1 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_csr`] module"]
#[doc(alias = "OPAMP1_CSR")]
pub type Opamp1Csr = crate::Reg<opamp1_csr::Opamp1CsrSpec>;
#[doc = "OPAMP1 control/status register"]
pub mod opamp1_csr;
#[doc = "OPAMP1_OTR (rw) register accessor: OPAMP1 offset trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_otr`] module"]
#[doc(alias = "OPAMP1_OTR")]
pub type Opamp1Otr = crate::Reg<opamp1_otr::Opamp1OtrSpec>;
#[doc = "OPAMP1 offset trimming register in normal mode"]
pub mod opamp1_otr;
#[doc = "OPAMP1_HSOTR (rw) register accessor: OPAMP1 offset trimming register in low-power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp1_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp1_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp1_hsotr`] module"]
#[doc(alias = "OPAMP1_HSOTR")]
pub type Opamp1Hsotr = crate::Reg<opamp1_hsotr::Opamp1HsotrSpec>;
#[doc = "OPAMP1 offset trimming register in low-power mode"]
pub mod opamp1_hsotr;
#[doc = "OPAMP2_CSR (rw) register accessor: OPAMP2 control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_csr`] module"]
#[doc(alias = "OPAMP2_CSR")]
pub type Opamp2Csr = crate::Reg<opamp2_csr::Opamp2CsrSpec>;
#[doc = "OPAMP2 control/status register"]
pub mod opamp2_csr;
#[doc = "OPAMP2_OTR (rw) register accessor: OPAMP2 offset trimming register in normal mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_otr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_otr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_otr`] module"]
#[doc(alias = "OPAMP2_OTR")]
pub type Opamp2Otr = crate::Reg<opamp2_otr::Opamp2OtrSpec>;
#[doc = "OPAMP2 offset trimming register in normal mode"]
pub mod opamp2_otr;
#[doc = "OPAMP2_HSOTR (rw) register accessor: OPAMP2 offset trimming register in low-power mode\n\nYou can [`read`](crate::Reg::read) this register and get [`opamp2_hsotr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opamp2_hsotr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opamp2_hsotr`] module"]
#[doc(alias = "OPAMP2_HSOTR")]
pub type Opamp2Hsotr = crate::Reg<opamp2_hsotr::Opamp2HsotrSpec>;
#[doc = "OPAMP2 offset trimming register in low-power mode"]
pub mod opamp2_hsotr;
