#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    otg_hs_pcgcr: OtgHsPcgcr,
}
impl RegisterBlock {
    #[doc = "0x00 - Power and clock gating control register"]
    #[inline(always)]
    pub const fn otg_hs_pcgcr(&self) -> &OtgHsPcgcr {
        &self.otg_hs_pcgcr
    }
}
#[doc = "OTG_HS_PCGCR (rw) register accessor: Power and clock gating control register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_pcgcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_pcgcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@otg_hs_pcgcr`] module"]
#[doc(alias = "OTG_HS_PCGCR")]
pub type OtgHsPcgcr = crate::Reg<otg_hs_pcgcr::OtgHsPcgcrSpec>;
#[doc = "Power and clock gating control register"]
pub mod otg_hs_pcgcr;
