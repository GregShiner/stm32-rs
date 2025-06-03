#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    acr: Acr,
    keyr1: Keyr1,
    optkeyr: Optkeyr,
    cr1: Cr1,
    sr1: Sr1,
    ccr1: Ccr1,
    optcr: Optcr,
    optsr_cur: OptsrCur,
    optsr_prg: OptsrPrg,
    optccr: Optccr,
    prar_cur1: PrarCur1,
    _reserved_11_prar_prg: [u8; 0x04],
    scar_cur1: ScarCur1,
    scar_prg1: ScarPrg1,
    wpsn_cur1r: WpsnCur1r,
    wpsn_prg1r: WpsnPrg1r,
    boot_curr: BootCurr,
    boot_prgr: BootPrgr,
    _reserved18: [u8; 0x08],
    crccr1: Crccr1,
    crcsadd1r: Crcsadd1r,
    crceadd1r: Crceadd1r,
    crcdatar: Crcdatar,
    ecc_fa1r: EccFa1r,
    _reserved23: [u8; 0x9c],
    acr_: Acr_,
    keyr2: Keyr2,
    optkeyr_: Optkeyr_,
    cr2: Cr2,
    sr2: Sr2,
    ccr2: Ccr2,
    optcr_: Optcr_,
    optsr_cur_: OptsrCur_,
    optsr_prg_: OptsrPrg_,
    optccr_: Optccr_,
    prar_cur2: PrarCur2,
    _reserved34: [u8; 0x04],
    scar_cur2: ScarCur2,
    scar_prg2: ScarPrg2,
    wpsn_cur2r: WpsnCur2r,
    wpsn_prg2r: WpsnPrg2r,
    _reserved38: [u8; 0x10],
    crccr2: Crccr2,
    crcsadd2r: Crcsadd2r,
    crceadd2r: Crceadd2r,
    _reserved41: [u8; 0x04],
    ecc_fa2r: EccFa2r,
}
impl RegisterBlock {
    #[doc = "0x00 - Access control register"]
    #[inline(always)]
    pub const fn acr(&self) -> &Acr {
        &self.acr
    }
    #[doc = "0x04 - FLASH key register for bank 1"]
    #[inline(always)]
    pub const fn keyr1(&self) -> &Keyr1 {
        &self.keyr1
    }
    #[doc = "0x08 - FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr(&self) -> &Optkeyr {
        &self.optkeyr
    }
    #[doc = "0x0c - FLASH control register for bank 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x10 - FLASH status register for bank 1"]
    #[inline(always)]
    pub const fn sr1(&self) -> &Sr1 {
        &self.sr1
    }
    #[doc = "0x14 - FLASH clear control register for bank 1"]
    #[inline(always)]
    pub const fn ccr1(&self) -> &Ccr1 {
        &self.ccr1
    }
    #[doc = "0x18 - FLASH option control register"]
    #[inline(always)]
    pub const fn optcr(&self) -> &Optcr {
        &self.optcr
    }
    #[doc = "0x1c - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur(&self) -> &OptsrCur {
        &self.optsr_cur
    }
    #[doc = "0x20 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg(&self) -> &OptsrPrg {
        &self.optsr_prg
    }
    #[doc = "0x24 - FLASH option clear control register"]
    #[inline(always)]
    pub const fn optccr(&self) -> &Optccr {
        &self.optccr
    }
    #[doc = "0x28 - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_cur1(&self) -> &PrarCur1 {
        &self.prar_cur1
    }
    #[doc = "0x2c - FLASH protection address for bank 2"]
    #[inline(always)]
    pub const fn prar_prg2(&self) -> &PrarPrg2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x2c - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_prg1(&self) -> &PrarPrg1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(44).cast() }
    }
    #[doc = "0x30 - FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_cur1(&self) -> &ScarCur1 {
        &self.scar_cur1
    }
    #[doc = "0x34 - FLASH secure address for bank 1"]
    #[inline(always)]
    pub const fn scar_prg1(&self) -> &ScarPrg1 {
        &self.scar_prg1
    }
    #[doc = "0x38 - FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_cur1r(&self) -> &WpsnCur1r {
        &self.wpsn_cur1r
    }
    #[doc = "0x3c - FLASH write sector protection for bank 1"]
    #[inline(always)]
    pub const fn wpsn_prg1r(&self) -> &WpsnPrg1r {
        &self.wpsn_prg1r
    }
    #[doc = "0x40 - FLASH register with boot address"]
    #[inline(always)]
    pub const fn boot_curr(&self) -> &BootCurr {
        &self.boot_curr
    }
    #[doc = "0x44 - FLASH register with boot address"]
    #[inline(always)]
    pub const fn boot_prgr(&self) -> &BootPrgr {
        &self.boot_prgr
    }
    #[doc = "0x50 - FLASH CRC control register for bank 1"]
    #[inline(always)]
    pub const fn crccr1(&self) -> &Crccr1 {
        &self.crccr1
    }
    #[doc = "0x54 - FLASH CRC start address register for bank 1"]
    #[inline(always)]
    pub const fn crcsadd1r(&self) -> &Crcsadd1r {
        &self.crcsadd1r
    }
    #[doc = "0x58 - FLASH CRC end address register for bank 1"]
    #[inline(always)]
    pub const fn crceadd1r(&self) -> &Crceadd1r {
        &self.crceadd1r
    }
    #[doc = "0x5c - FLASH CRC data register"]
    #[inline(always)]
    pub const fn crcdatar(&self) -> &Crcdatar {
        &self.crcdatar
    }
    #[doc = "0x60 - FLASH ECC fail address for bank 1"]
    #[inline(always)]
    pub const fn ecc_fa1r(&self) -> &EccFa1r {
        &self.ecc_fa1r
    }
    #[doc = "0x100 - Access control register"]
    #[inline(always)]
    pub const fn acr_(&self) -> &Acr_ {
        &self.acr_
    }
    #[doc = "0x104 - FLASH key register for bank 2"]
    #[inline(always)]
    pub const fn keyr2(&self) -> &Keyr2 {
        &self.keyr2
    }
    #[doc = "0x108 - FLASH option key register"]
    #[inline(always)]
    pub const fn optkeyr_(&self) -> &Optkeyr_ {
        &self.optkeyr_
    }
    #[doc = "0x10c - FLASH control register for bank 2"]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x110 - FLASH status register for bank 2"]
    #[inline(always)]
    pub const fn sr2(&self) -> &Sr2 {
        &self.sr2
    }
    #[doc = "0x114 - FLASH clear control register for bank 2"]
    #[inline(always)]
    pub const fn ccr2(&self) -> &Ccr2 {
        &self.ccr2
    }
    #[doc = "0x118 - FLASH option control register"]
    #[inline(always)]
    pub const fn optcr_(&self) -> &Optcr_ {
        &self.optcr_
    }
    #[doc = "0x11c - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_cur_(&self) -> &OptsrCur_ {
        &self.optsr_cur_
    }
    #[doc = "0x120 - FLASH option status register"]
    #[inline(always)]
    pub const fn optsr_prg_(&self) -> &OptsrPrg_ {
        &self.optsr_prg_
    }
    #[doc = "0x124 - FLASH option clear control register"]
    #[inline(always)]
    pub const fn optccr_(&self) -> &Optccr_ {
        &self.optccr_
    }
    #[doc = "0x128 - FLASH protection address for bank 1"]
    #[inline(always)]
    pub const fn prar_cur2(&self) -> &PrarCur2 {
        &self.prar_cur2
    }
    #[doc = "0x130 - FLASH secure address for bank 2"]
    #[inline(always)]
    pub const fn scar_cur2(&self) -> &ScarCur2 {
        &self.scar_cur2
    }
    #[doc = "0x134 - FLASH secure address for bank 2"]
    #[inline(always)]
    pub const fn scar_prg2(&self) -> &ScarPrg2 {
        &self.scar_prg2
    }
    #[doc = "0x138 - FLASH write sector protection for bank 2"]
    #[inline(always)]
    pub const fn wpsn_cur2r(&self) -> &WpsnCur2r {
        &self.wpsn_cur2r
    }
    #[doc = "0x13c - FLASH write sector protection for bank 2"]
    #[inline(always)]
    pub const fn wpsn_prg2r(&self) -> &WpsnPrg2r {
        &self.wpsn_prg2r
    }
    #[doc = "0x150 - FLASH CRC control register for bank 1"]
    #[inline(always)]
    pub const fn crccr2(&self) -> &Crccr2 {
        &self.crccr2
    }
    #[doc = "0x154 - FLASH CRC start address register for bank 2"]
    #[inline(always)]
    pub const fn crcsadd2r(&self) -> &Crcsadd2r {
        &self.crcsadd2r
    }
    #[doc = "0x158 - FLASH CRC end address register for bank 2"]
    #[inline(always)]
    pub const fn crceadd2r(&self) -> &Crceadd2r {
        &self.crceadd2r
    }
    #[doc = "0x160 - FLASH ECC fail address for bank 2"]
    #[inline(always)]
    pub const fn ecc_fa2r(&self) -> &EccFa2r {
        &self.ecc_fa2r
    }
}
#[doc = "ACR (rw) register accessor: Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr`] module"]
#[doc(alias = "ACR")]
pub type Acr = crate::Reg<acr::AcrSpec>;
#[doc = "Access control register"]
pub mod acr;
#[doc = "ACR_ (rw) register accessor: Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@acr_`] module"]
#[doc(alias = "ACR_")]
pub type Acr_ = crate::Reg<acr_::Acr_Spec>;
#[doc = "Access control register"]
pub mod acr_;
#[doc = "KEYR1 (rw) register accessor: FLASH key register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`keyr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr1`] module"]
#[doc(alias = "KEYR1")]
pub type Keyr1 = crate::Reg<keyr1::Keyr1Spec>;
#[doc = "FLASH key register for bank 1"]
pub mod keyr1;
#[doc = "OPTKEYR (rw) register accessor: FLASH option key register\n\nYou can [`read`](crate::Reg::read) this register and get [`optkeyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr`] module"]
#[doc(alias = "OPTKEYR")]
pub type Optkeyr = crate::Reg<optkeyr::OptkeyrSpec>;
#[doc = "FLASH option key register"]
pub mod optkeyr;
#[doc = "OPTKEYR_ (rw) register accessor: FLASH option key register\n\nYou can [`read`](crate::Reg::read) this register and get [`optkeyr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optkeyr_`] module"]
#[doc(alias = "OPTKEYR_")]
pub type Optkeyr_ = crate::Reg<optkeyr_::Optkeyr_Spec>;
#[doc = "FLASH option key register"]
pub mod optkeyr_;
#[doc = "CR1 (rw) register accessor: FLASH control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "FLASH control register for bank 1"]
pub mod cr1;
#[doc = "SR1 (rw) register accessor: FLASH status register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr1`] module"]
#[doc(alias = "SR1")]
pub type Sr1 = crate::Reg<sr1::Sr1Spec>;
#[doc = "FLASH status register for bank 1"]
pub mod sr1;
#[doc = "CCR1 (rw) register accessor: FLASH clear control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr1`] module"]
#[doc(alias = "CCR1")]
pub type Ccr1 = crate::Reg<ccr1::Ccr1Spec>;
#[doc = "FLASH clear control register for bank 1"]
pub mod ccr1;
#[doc = "OPTCR (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::Reg::read) this register and get [`optcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr`] module"]
#[doc(alias = "OPTCR")]
pub type Optcr = crate::Reg<optcr::OptcrSpec>;
#[doc = "FLASH option control register"]
pub mod optcr;
#[doc = "OPTCR_ (rw) register accessor: FLASH option control register\n\nYou can [`read`](crate::Reg::read) this register and get [`optcr_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optcr_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optcr_`] module"]
#[doc(alias = "OPTCR_")]
pub type Optcr_ = crate::Reg<optcr_::Optcr_Spec>;
#[doc = "FLASH option control register"]
pub mod optcr_;
#[doc = "OPTSR_CUR_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_cur_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur_`] module"]
#[doc(alias = "OPTSR_CUR_")]
pub type OptsrCur_ = crate::Reg<optsr_cur_::OptsrCur_Spec>;
#[doc = "FLASH option status register"]
pub mod optsr_cur_;
#[doc = "OPTSR_CUR (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_cur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_cur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_cur`] module"]
#[doc(alias = "OPTSR_CUR")]
pub type OptsrCur = crate::Reg<optsr_cur::OptsrCurSpec>;
#[doc = "FLASH option status register"]
pub mod optsr_cur;
#[doc = "OPTSR_PRG (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_prg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg`] module"]
#[doc(alias = "OPTSR_PRG")]
pub type OptsrPrg = crate::Reg<optsr_prg::OptsrPrgSpec>;
#[doc = "FLASH option status register"]
pub mod optsr_prg;
#[doc = "OPTSR_PRG_ (rw) register accessor: FLASH option status register\n\nYou can [`read`](crate::Reg::read) this register and get [`optsr_prg_::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optsr_prg_::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optsr_prg_`] module"]
#[doc(alias = "OPTSR_PRG_")]
pub type OptsrPrg_ = crate::Reg<optsr_prg_::OptsrPrg_Spec>;
#[doc = "FLASH option status register"]
pub mod optsr_prg_;
#[doc = "OPTCCR_ (w) register accessor: FLASH option clear control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr_::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optccr_`] module"]
#[doc(alias = "OPTCCR_")]
pub type Optccr_ = crate::Reg<optccr_::Optccr_Spec>;
#[doc = "FLASH option clear control register"]
pub mod optccr_;
#[doc = "OPTCCR (w) register accessor: FLASH option clear control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@optccr`] module"]
#[doc(alias = "OPTCCR")]
pub type Optccr = crate::Reg<optccr::OptccrSpec>;
#[doc = "FLASH option clear control register"]
pub mod optccr;
#[doc = "PRAR_CUR1 (r) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_cur1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_cur1`] module"]
#[doc(alias = "PRAR_CUR1")]
pub type PrarCur1 = crate::Reg<prar_cur1::PrarCur1Spec>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur1;
#[doc = "PRAR_PRG1 (rw) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_prg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_prg1`] module"]
#[doc(alias = "PRAR_PRG1")]
pub type PrarPrg1 = crate::Reg<prar_prg1::PrarPrg1Spec>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_prg1;
#[doc = "SCAR_CUR1 (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_cur1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_cur1`] module"]
#[doc(alias = "SCAR_CUR1")]
pub type ScarCur1 = crate::Reg<scar_cur1::ScarCur1Spec>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_cur1;
#[doc = "SCAR_PRG1 (rw) register accessor: FLASH secure address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_prg1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_prg1`] module"]
#[doc(alias = "SCAR_PRG1")]
pub type ScarPrg1 = crate::Reg<scar_prg1::ScarPrg1Spec>;
#[doc = "FLASH secure address for bank 1"]
pub mod scar_prg1;
#[doc = "WPSN_CUR1R (r) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_cur1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_cur1r`] module"]
#[doc(alias = "WPSN_CUR1R")]
pub type WpsnCur1r = crate::Reg<wpsn_cur1r::WpsnCur1rSpec>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_cur1r;
#[doc = "WPSN_PRG1R (rw) register accessor: FLASH write sector protection for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_prg1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_prg1r`] module"]
#[doc(alias = "WPSN_PRG1R")]
pub type WpsnPrg1r = crate::Reg<wpsn_prg1r::WpsnPrg1rSpec>;
#[doc = "FLASH write sector protection for bank 1"]
pub mod wpsn_prg1r;
#[doc = "BOOT_CURR (r) register accessor: FLASH register with boot address\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_curr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_curr`] module"]
#[doc(alias = "BOOT_CURR")]
pub type BootCurr = crate::Reg<boot_curr::BootCurrSpec>;
#[doc = "FLASH register with boot address"]
pub mod boot_curr;
#[doc = "BOOT_PRGR (r) register accessor: FLASH register with boot address\n\nYou can [`read`](crate::Reg::read) this register and get [`boot_prgr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@boot_prgr`] module"]
#[doc(alias = "BOOT_PRGR")]
pub type BootPrgr = crate::Reg<boot_prgr::BootPrgrSpec>;
#[doc = "FLASH register with boot address"]
pub mod boot_prgr;
#[doc = "CRCCR1 (rw) register accessor: FLASH CRC control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr1`] module"]
#[doc(alias = "CRCCR1")]
pub type Crccr1 = crate::Reg<crccr1::Crccr1Spec>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr1;
#[doc = "CRCSADD1R (rw) register accessor: FLASH CRC start address register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsadd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsadd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsadd1r`] module"]
#[doc(alias = "CRCSADD1R")]
pub type Crcsadd1r = crate::Reg<crcsadd1r::Crcsadd1rSpec>;
#[doc = "FLASH CRC start address register for bank 1"]
pub mod crcsadd1r;
#[doc = "CRCEADD1R (rw) register accessor: FLASH CRC end address register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crceadd1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceadd1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crceadd1r`] module"]
#[doc(alias = "CRCEADD1R")]
pub type Crceadd1r = crate::Reg<crceadd1r::Crceadd1rSpec>;
#[doc = "FLASH CRC end address register for bank 1"]
pub mod crceadd1r;
#[doc = "CRCDATAR (rw) register accessor: FLASH CRC data register\n\nYou can [`read`](crate::Reg::read) this register and get [`crcdatar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdatar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcdatar`] module"]
#[doc(alias = "CRCDATAR")]
pub type Crcdatar = crate::Reg<crcdatar::CrcdatarSpec>;
#[doc = "FLASH CRC data register"]
pub mod crcdatar;
#[doc = "ECC_FA1R (r) register accessor: FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_fa1r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_fa1r`] module"]
#[doc(alias = "ECC_FA1R")]
pub type EccFa1r = crate::Reg<ecc_fa1r::EccFa1rSpec>;
#[doc = "FLASH ECC fail address for bank 1"]
pub mod ecc_fa1r;
#[doc = "KEYR2 (r) register accessor: FLASH key register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`keyr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@keyr2`] module"]
#[doc(alias = "KEYR2")]
pub type Keyr2 = crate::Reg<keyr2::Keyr2Spec>;
#[doc = "FLASH key register for bank 2"]
pub mod keyr2;
#[doc = "CR2 (rw) register accessor: FLASH control register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "FLASH control register for bank 2"]
pub mod cr2;
#[doc = "SR2 (rw) register accessor: FLASH status register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr2`] module"]
#[doc(alias = "SR2")]
pub type Sr2 = crate::Reg<sr2::Sr2Spec>;
#[doc = "FLASH status register for bank 2"]
pub mod sr2;
#[doc = "CCR2 (rw) register accessor: FLASH clear control register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr2`] module"]
#[doc(alias = "CCR2")]
pub type Ccr2 = crate::Reg<ccr2::Ccr2Spec>;
#[doc = "FLASH clear control register for bank 2"]
pub mod ccr2;
#[doc = "PRAR_CUR2 (r) register accessor: FLASH protection address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_cur2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_cur2`] module"]
#[doc(alias = "PRAR_CUR2")]
pub type PrarCur2 = crate::Reg<prar_cur2::PrarCur2Spec>;
#[doc = "FLASH protection address for bank 1"]
pub mod prar_cur2;
#[doc = "PRAR_PRG2 (rw) register accessor: FLASH protection address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`prar_prg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prar_prg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@prar_prg2`] module"]
#[doc(alias = "PRAR_PRG2")]
pub type PrarPrg2 = crate::Reg<prar_prg2::PrarPrg2Spec>;
#[doc = "FLASH protection address for bank 2"]
pub mod prar_prg2;
#[doc = "SCAR_CUR2 (rw) register accessor: FLASH secure address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_cur2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_cur2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_cur2`] module"]
#[doc(alias = "SCAR_CUR2")]
pub type ScarCur2 = crate::Reg<scar_cur2::ScarCur2Spec>;
#[doc = "FLASH secure address for bank 2"]
pub mod scar_cur2;
#[doc = "SCAR_PRG2 (rw) register accessor: FLASH secure address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`scar_prg2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scar_prg2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@scar_prg2`] module"]
#[doc(alias = "SCAR_PRG2")]
pub type ScarPrg2 = crate::Reg<scar_prg2::ScarPrg2Spec>;
#[doc = "FLASH secure address for bank 2"]
pub mod scar_prg2;
#[doc = "WPSN_CUR2R (r) register accessor: FLASH write sector protection for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_cur2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_cur2r`] module"]
#[doc(alias = "WPSN_CUR2R")]
pub type WpsnCur2r = crate::Reg<wpsn_cur2r::WpsnCur2rSpec>;
#[doc = "FLASH write sector protection for bank 2"]
pub mod wpsn_cur2r;
#[doc = "WPSN_PRG2R (rw) register accessor: FLASH write sector protection for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`wpsn_prg2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpsn_prg2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpsn_prg2r`] module"]
#[doc(alias = "WPSN_PRG2R")]
pub type WpsnPrg2r = crate::Reg<wpsn_prg2r::WpsnPrg2rSpec>;
#[doc = "FLASH write sector protection for bank 2"]
pub mod wpsn_prg2r;
#[doc = "CRCCR2 (rw) register accessor: FLASH CRC control register for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`crccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crccr2`] module"]
#[doc(alias = "CRCCR2")]
pub type Crccr2 = crate::Reg<crccr2::Crccr2Spec>;
#[doc = "FLASH CRC control register for bank 1"]
pub mod crccr2;
#[doc = "CRCSADD2R (rw) register accessor: FLASH CRC start address register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`crcsadd2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcsadd2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crcsadd2r`] module"]
#[doc(alias = "CRCSADD2R")]
pub type Crcsadd2r = crate::Reg<crcsadd2r::Crcsadd2rSpec>;
#[doc = "FLASH CRC start address register for bank 2"]
pub mod crcsadd2r;
#[doc = "CRCEADD2R (rw) register accessor: FLASH CRC end address register for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`crceadd2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crceadd2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crceadd2r`] module"]
#[doc(alias = "CRCEADD2R")]
pub type Crceadd2r = crate::Reg<crceadd2r::Crceadd2rSpec>;
#[doc = "FLASH CRC end address register for bank 2"]
pub mod crceadd2r;
#[doc = "ECC_FA2R (r) register accessor: FLASH ECC fail address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_fa2r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ecc_fa2r`] module"]
#[doc(alias = "ECC_FA2R")]
pub type EccFa2r = crate::Reg<ecc_fa2r::EccFa2rSpec>;
#[doc = "FLASH ECC fail address for bank 2"]
pub mod ecc_fa2r;
