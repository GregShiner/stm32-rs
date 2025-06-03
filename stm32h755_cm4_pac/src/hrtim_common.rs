#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    cr2: Cr2,
    isr: Isr,
    icr: Icr,
    ier: Ier,
    oenr: Oenr,
    disr: Disr,
    odsr: Odsr,
    bmcr: Bmcr,
    bmtrg: Bmtrg,
    bmcmpr6: Bmcmpr6,
    bmper: Bmper,
    eecr1: Eecr1,
    eecr2: Eecr2,
    eecr3: Eecr3,
    adc1r: Adc1r,
    adc2r: Adc2r,
    adc3r: Adc3r,
    adc4r: Adc4r,
    dllcr: Dllcr,
    fltinr1: Fltinr1,
    fltinr2: Fltinr2,
    bdmupdr: Bdmupdr,
    bdtx_upr: BdtxUpr,
    bdmadr: Bdmadr,
}
impl RegisterBlock {
    #[doc = "0x00 - Control Register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - Control Register 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x08 - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x0c - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Output Enable Register"]
    #[inline(always)]
    pub const fn oenr(&self) -> &Oenr {
        &self.oenr
    }
    #[doc = "0x18 - DISR"]
    #[inline(always)]
    pub const fn disr(&self) -> &Disr {
        &self.disr
    }
    #[doc = "0x1c - Output Disable Status Register"]
    #[inline(always)]
    pub const fn odsr(&self) -> &Odsr {
        &self.odsr
    }
    #[doc = "0x20 - Burst Mode Control Register"]
    #[inline(always)]
    pub const fn bmcr(&self) -> &Bmcr {
        &self.bmcr
    }
    #[doc = "0x24 - BMTRG"]
    #[inline(always)]
    pub const fn bmtrg(&self) -> &Bmtrg {
        &self.bmtrg
    }
    #[doc = "0x28 - BMCMPR6"]
    #[inline(always)]
    pub const fn bmcmpr6(&self) -> &Bmcmpr6 {
        &self.bmcmpr6
    }
    #[doc = "0x2c - Burst Mode Period Register"]
    #[inline(always)]
    pub const fn bmper(&self) -> &Bmper {
        &self.bmper
    }
    #[doc = "0x30 - Timer External Event Control Register 1"]
    #[inline(always)]
    pub const fn eecr1(&self) -> &Eecr1 {
        &self.eecr1
    }
    #[doc = "0x34 - Timer External Event Control Register 2"]
    #[inline(always)]
    pub const fn eecr2(&self) -> &Eecr2 {
        &self.eecr2
    }
    #[doc = "0x38 - Timer External Event Control Register 3"]
    #[inline(always)]
    pub const fn eecr3(&self) -> &Eecr3 {
        &self.eecr3
    }
    #[doc = "0x3c - ADC Trigger 1 Register"]
    #[inline(always)]
    pub const fn adc1r(&self) -> &Adc1r {
        &self.adc1r
    }
    #[doc = "0x40 - ADC Trigger 2 Register"]
    #[inline(always)]
    pub const fn adc2r(&self) -> &Adc2r {
        &self.adc2r
    }
    #[doc = "0x44 - ADC Trigger 3 Register"]
    #[inline(always)]
    pub const fn adc3r(&self) -> &Adc3r {
        &self.adc3r
    }
    #[doc = "0x48 - ADC Trigger 4 Register"]
    #[inline(always)]
    pub const fn adc4r(&self) -> &Adc4r {
        &self.adc4r
    }
    #[doc = "0x4c - DLL Control Register"]
    #[inline(always)]
    pub const fn dllcr(&self) -> &Dllcr {
        &self.dllcr
    }
    #[doc = "0x50 - HRTIM Fault Input Register 1"]
    #[inline(always)]
    pub const fn fltinr1(&self) -> &Fltinr1 {
        &self.fltinr1
    }
    #[doc = "0x54 - HRTIM Fault Input Register 2"]
    #[inline(always)]
    pub const fn fltinr2(&self) -> &Fltinr2 {
        &self.fltinr2
    }
    #[doc = "0x58 - BDMUPDR"]
    #[inline(always)]
    pub const fn bdmupdr(&self) -> &Bdmupdr {
        &self.bdmupdr
    }
    #[doc = "0x5c - Burst DMA Timerx update Register"]
    #[inline(always)]
    pub const fn bdtx_upr(&self) -> &BdtxUpr {
        &self.bdtx_upr
    }
    #[doc = "0x60 - Burst DMA Data Register"]
    #[inline(always)]
    pub const fn bdmadr(&self) -> &Bdmadr {
        &self.bdmadr
    }
}
#[doc = "CR1 (rw) register accessor: Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "Control Register 1"]
pub mod cr1;
#[doc = "CR2 (rw) register accessor: Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "Control Register 2"]
pub mod cr2;
#[doc = "ISR (rw) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (rw) register accessor: Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`] module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "IER (rw) register accessor: Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "OENR (w) register accessor: Output Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oenr`] module"]
#[doc(alias = "OENR")]
pub type Oenr = crate::Reg<oenr::OenrSpec>;
#[doc = "Output Enable Register"]
pub mod oenr;
#[doc = "DISR (rw) register accessor: DISR\n\nYou can [`read`](crate::Reg::read) this register and get [`disr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@disr`] module"]
#[doc(alias = "DISR")]
pub type Disr = crate::Reg<disr::DisrSpec>;
#[doc = "DISR"]
pub mod disr;
#[doc = "ODSR (r) register accessor: Output Disable Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`odsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@odsr`] module"]
#[doc(alias = "ODSR")]
pub type Odsr = crate::Reg<odsr::OdsrSpec>;
#[doc = "Output Disable Status Register"]
pub mod odsr;
#[doc = "BMCR (rw) register accessor: Burst Mode Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcr`] module"]
#[doc(alias = "BMCR")]
pub type Bmcr = crate::Reg<bmcr::BmcrSpec>;
#[doc = "Burst Mode Control Register"]
pub mod bmcr;
#[doc = "BMTRG (rw) register accessor: BMTRG\n\nYou can [`read`](crate::Reg::read) this register and get [`bmtrg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmtrg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmtrg`] module"]
#[doc(alias = "BMTRG")]
pub type Bmtrg = crate::Reg<bmtrg::BmtrgSpec>;
#[doc = "BMTRG"]
pub mod bmtrg;
#[doc = "BMCMPR6 (rw) register accessor: BMCMPR6\n\nYou can [`read`](crate::Reg::read) this register and get [`bmcmpr6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmcmpr6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmcmpr6`] module"]
#[doc(alias = "BMCMPR6")]
pub type Bmcmpr6 = crate::Reg<bmcmpr6::Bmcmpr6Spec>;
#[doc = "BMCMPR6"]
pub mod bmcmpr6;
#[doc = "BMPER (rw) register accessor: Burst Mode Period Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bmper::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bmper::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bmper`] module"]
#[doc(alias = "BMPER")]
pub type Bmper = crate::Reg<bmper::BmperSpec>;
#[doc = "Burst Mode Period Register"]
pub mod bmper;
#[doc = "EECR1 (rw) register accessor: Timer External Event Control Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr1`] module"]
#[doc(alias = "EECR1")]
pub type Eecr1 = crate::Reg<eecr1::Eecr1Spec>;
#[doc = "Timer External Event Control Register 1"]
pub mod eecr1;
#[doc = "EECR2 (rw) register accessor: Timer External Event Control Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr2`] module"]
#[doc(alias = "EECR2")]
pub type Eecr2 = crate::Reg<eecr2::Eecr2Spec>;
#[doc = "Timer External Event Control Register 2"]
pub mod eecr2;
#[doc = "EECR3 (rw) register accessor: Timer External Event Control Register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`eecr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eecr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@eecr3`] module"]
#[doc(alias = "EECR3")]
pub type Eecr3 = crate::Reg<eecr3::Eecr3Spec>;
#[doc = "Timer External Event Control Register 3"]
pub mod eecr3;
#[doc = "ADC1R (rw) register accessor: ADC Trigger 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc1r`] module"]
#[doc(alias = "ADC1R")]
pub type Adc1r = crate::Reg<adc1r::Adc1rSpec>;
#[doc = "ADC Trigger 1 Register"]
pub mod adc1r;
#[doc = "ADC2R (rw) register accessor: ADC Trigger 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc2r`] module"]
#[doc(alias = "ADC2R")]
pub type Adc2r = crate::Reg<adc2r::Adc2rSpec>;
#[doc = "ADC Trigger 2 Register"]
pub mod adc2r;
#[doc = "ADC3R (rw) register accessor: ADC Trigger 3 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc3r`] module"]
#[doc(alias = "ADC3R")]
pub type Adc3r = crate::Reg<adc3r::Adc3rSpec>;
#[doc = "ADC Trigger 3 Register"]
pub mod adc3r;
#[doc = "ADC4R (rw) register accessor: ADC Trigger 4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`adc4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adc4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@adc4r`] module"]
#[doc(alias = "ADC4R")]
pub type Adc4r = crate::Reg<adc4r::Adc4rSpec>;
#[doc = "ADC Trigger 4 Register"]
pub mod adc4r;
#[doc = "DLLCR (rw) register accessor: DLL Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dllcr`] module"]
#[doc(alias = "DLLCR")]
pub type Dllcr = crate::Reg<dllcr::DllcrSpec>;
#[doc = "DLL Control Register"]
pub mod dllcr;
#[doc = "FLTINR1 (rw) register accessor: HRTIM Fault Input Register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`fltinr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr1`] module"]
#[doc(alias = "FLTINR1")]
pub type Fltinr1 = crate::Reg<fltinr1::Fltinr1Spec>;
#[doc = "HRTIM Fault Input Register 1"]
pub mod fltinr1;
#[doc = "FLTINR2 (rw) register accessor: HRTIM Fault Input Register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`fltinr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fltinr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fltinr2`] module"]
#[doc(alias = "FLTINR2")]
pub type Fltinr2 = crate::Reg<fltinr2::Fltinr2Spec>;
#[doc = "HRTIM Fault Input Register 2"]
pub mod fltinr2;
#[doc = "BDMUPDR (rw) register accessor: BDMUPDR\n\nYou can [`read`](crate::Reg::read) this register and get [`bdmupdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmupdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmupdr`] module"]
#[doc(alias = "BDMUPDR")]
pub type Bdmupdr = crate::Reg<bdmupdr::BdmupdrSpec>;
#[doc = "BDMUPDR"]
pub mod bdmupdr;
#[doc = "BDTxUPR (rw) register accessor: Burst DMA Timerx update Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdtx_upr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdtx_upr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdtx_upr`] module"]
#[doc(alias = "BDTxUPR")]
pub type BdtxUpr = crate::Reg<bdtx_upr::BdtxUprSpec>;
#[doc = "Burst DMA Timerx update Register"]
pub mod bdtx_upr;
#[doc = "BDMADR (rw) register accessor: Burst DMA Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdmadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdmadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdmadr`] module"]
#[doc(alias = "BDMADR")]
pub type Bdmadr = crate::Reg<bdmadr::BdmadrSpec>;
#[doc = "Burst DMA Data Register"]
pub mod bdmadr;
