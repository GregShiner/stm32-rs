#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtc_tr: RtcTr,
    rtc_dr: RtcDr,
    rtc_cr: RtcCr,
    rtc_isr: RtcIsr,
    rtc_prer: RtcPrer,
    rtc_wutr: RtcWutr,
    _reserved6: [u8; 0x04],
    rtc_alrmar: RtcAlrmar,
    rtc_alrmbr: RtcAlrmbr,
    rtc_wpr: RtcWpr,
    rtc_ssr: RtcSsr,
    rtc_shiftr: RtcShiftr,
    rtc_tstr: RtcTstr,
    rtc_tsdr: RtcTsdr,
    rtc_tsssr: RtcTsssr,
    rtc_calr: RtcCalr,
    rtc_tampcr: RtcTampcr,
    rtc_alrmassr: RtcAlrmassr,
    rtc_alrmbssr: RtcAlrmbssr,
    rtc_or: RtcOr,
    rtc_bkp0r: RtcBkp0r,
    rtc_bkp1r: RtcBkp1r,
    rtc_bkp2r: RtcBkp2r,
    rtc_bkp3r: RtcBkp3r,
    rtc_bkp4r: RtcBkp4r,
    rtc_bkp5r: RtcBkp5r,
    rtc_bkp6r: RtcBkp6r,
    rtc_bkp7r: RtcBkp7r,
    rtc_bkp8r: RtcBkp8r,
    rtc_bkp9r: RtcBkp9r,
    rtc_bkp10r: RtcBkp10r,
    rtc_bkp11r: RtcBkp11r,
    rtc_bkp12r: RtcBkp12r,
    rtc_bkp13r: RtcBkp13r,
    rtc_bkp14r: RtcBkp14r,
    rtc_bkp15r: RtcBkp15r,
    rtc_bkp16r: RtcBkp16r,
    rtc_bkp17r: RtcBkp17r,
    rtc_bkp18r: RtcBkp18r,
    rtc_bkp19r: RtcBkp19r,
    rtc_bkp20r: RtcBkp20r,
    rtc_bkp21r: RtcBkp21r,
    rtc_bkp22r: RtcBkp22r,
    rtc_bkp23r: RtcBkp23r,
    rtc_bkp24r: RtcBkp24r,
    rtc_bkp25r: RtcBkp25r,
    rtc_bkp26r: RtcBkp26r,
    rtc_bkp27r: RtcBkp27r,
    rtc_bkp28r: RtcBkp28r,
    rtc_bkp29r: RtcBkp29r,
    rtc_bkp30r: RtcBkp30r,
    rtc_bkp31r: RtcBkp31r,
}
impl RegisterBlock {
    #[doc = "0x00 - The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_tr(&self) -> &RtcTr {
        &self.rtc_tr
    }
    #[doc = "0x04 - The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_dr(&self) -> &RtcDr {
        &self.rtc_dr
    }
    #[doc = "0x08 - RTC control register"]
    #[inline(always)]
    pub const fn rtc_cr(&self) -> &RtcCr {
        &self.rtc_cr
    }
    #[doc = "0x0c - This register is write protected (except for RTC_ISR\\[13:8\\] bits). The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_isr(&self) -> &RtcIsr {
        &self.rtc_isr
    }
    #[doc = "0x10 - This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_prer(&self) -> &RtcPrer {
        &self.rtc_prer
    }
    #[doc = "0x14 - This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_wutr(&self) -> &RtcWutr {
        &self.rtc_wutr
    }
    #[doc = "0x1c - This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_alrmar(&self) -> &RtcAlrmar {
        &self.rtc_alrmar
    }
    #[doc = "0x20 - This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_alrmbr(&self) -> &RtcAlrmbr {
        &self.rtc_alrmbr
    }
    #[doc = "0x24 - RTC write protection register"]
    #[inline(always)]
    pub const fn rtc_wpr(&self) -> &RtcWpr {
        &self.rtc_wpr
    }
    #[doc = "0x28 - RTC sub second register"]
    #[inline(always)]
    pub const fn rtc_ssr(&self) -> &RtcSsr {
        &self.rtc_ssr
    }
    #[doc = "0x2c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_shiftr(&self) -> &RtcShiftr {
        &self.rtc_shiftr
    }
    #[doc = "0x30 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    #[inline(always)]
    pub const fn rtc_tstr(&self) -> &RtcTstr {
        &self.rtc_tstr
    }
    #[doc = "0x34 - The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
    #[inline(always)]
    pub const fn rtc_tsdr(&self) -> &RtcTsdr {
        &self.rtc_tsdr
    }
    #[doc = "0x38 - The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
    #[inline(always)]
    pub const fn rtc_tsssr(&self) -> &RtcTsssr {
        &self.rtc_tsssr
    }
    #[doc = "0x3c - This register is write protected. The write access procedure is described in RTC register write protection on page9."]
    #[inline(always)]
    pub const fn rtc_calr(&self) -> &RtcCalr {
        &self.rtc_calr
    }
    #[doc = "0x40 - RTC tamper and alternate function configuration register"]
    #[inline(always)]
    pub const fn rtc_tampcr(&self) -> &RtcTampcr {
        &self.rtc_tampcr
    }
    #[doc = "0x44 - This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
    #[inline(always)]
    pub const fn rtc_alrmassr(&self) -> &RtcAlrmassr {
        &self.rtc_alrmassr
    }
    #[doc = "0x48 - This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
    #[inline(always)]
    pub const fn rtc_alrmbssr(&self) -> &RtcAlrmbssr {
        &self.rtc_alrmbssr
    }
    #[doc = "0x4c - RTC option register"]
    #[inline(always)]
    pub const fn rtc_or(&self) -> &RtcOr {
        &self.rtc_or
    }
    #[doc = "0x50 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp0r(&self) -> &RtcBkp0r {
        &self.rtc_bkp0r
    }
    #[doc = "0x54 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp1r(&self) -> &RtcBkp1r {
        &self.rtc_bkp1r
    }
    #[doc = "0x58 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp2r(&self) -> &RtcBkp2r {
        &self.rtc_bkp2r
    }
    #[doc = "0x5c - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp3r(&self) -> &RtcBkp3r {
        &self.rtc_bkp3r
    }
    #[doc = "0x60 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp4r(&self) -> &RtcBkp4r {
        &self.rtc_bkp4r
    }
    #[doc = "0x64 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp5r(&self) -> &RtcBkp5r {
        &self.rtc_bkp5r
    }
    #[doc = "0x68 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp6r(&self) -> &RtcBkp6r {
        &self.rtc_bkp6r
    }
    #[doc = "0x6c - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp7r(&self) -> &RtcBkp7r {
        &self.rtc_bkp7r
    }
    #[doc = "0x70 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp8r(&self) -> &RtcBkp8r {
        &self.rtc_bkp8r
    }
    #[doc = "0x74 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp9r(&self) -> &RtcBkp9r {
        &self.rtc_bkp9r
    }
    #[doc = "0x78 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp10r(&self) -> &RtcBkp10r {
        &self.rtc_bkp10r
    }
    #[doc = "0x7c - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp11r(&self) -> &RtcBkp11r {
        &self.rtc_bkp11r
    }
    #[doc = "0x80 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp12r(&self) -> &RtcBkp12r {
        &self.rtc_bkp12r
    }
    #[doc = "0x84 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp13r(&self) -> &RtcBkp13r {
        &self.rtc_bkp13r
    }
    #[doc = "0x88 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp14r(&self) -> &RtcBkp14r {
        &self.rtc_bkp14r
    }
    #[doc = "0x8c - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp15r(&self) -> &RtcBkp15r {
        &self.rtc_bkp15r
    }
    #[doc = "0x90 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp16r(&self) -> &RtcBkp16r {
        &self.rtc_bkp16r
    }
    #[doc = "0x94 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp17r(&self) -> &RtcBkp17r {
        &self.rtc_bkp17r
    }
    #[doc = "0x98 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp18r(&self) -> &RtcBkp18r {
        &self.rtc_bkp18r
    }
    #[doc = "0x9c - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp19r(&self) -> &RtcBkp19r {
        &self.rtc_bkp19r
    }
    #[doc = "0xa0 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp20r(&self) -> &RtcBkp20r {
        &self.rtc_bkp20r
    }
    #[doc = "0xa4 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp21r(&self) -> &RtcBkp21r {
        &self.rtc_bkp21r
    }
    #[doc = "0xa8 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp22r(&self) -> &RtcBkp22r {
        &self.rtc_bkp22r
    }
    #[doc = "0xac - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp23r(&self) -> &RtcBkp23r {
        &self.rtc_bkp23r
    }
    #[doc = "0xb0 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp24r(&self) -> &RtcBkp24r {
        &self.rtc_bkp24r
    }
    #[doc = "0xb4 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp25r(&self) -> &RtcBkp25r {
        &self.rtc_bkp25r
    }
    #[doc = "0xb8 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp26r(&self) -> &RtcBkp26r {
        &self.rtc_bkp26r
    }
    #[doc = "0xbc - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp27r(&self) -> &RtcBkp27r {
        &self.rtc_bkp27r
    }
    #[doc = "0xc0 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp28r(&self) -> &RtcBkp28r {
        &self.rtc_bkp28r
    }
    #[doc = "0xc4 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp29r(&self) -> &RtcBkp29r {
        &self.rtc_bkp29r
    }
    #[doc = "0xc8 - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp30r(&self) -> &RtcBkp30r {
        &self.rtc_bkp30r
    }
    #[doc = "0xcc - RTC backup registers"]
    #[inline(always)]
    pub const fn rtc_bkp31r(&self) -> &RtcBkp31r {
        &self.rtc_bkp31r
    }
}
#[doc = "RTC_TR (rw) register accessor: The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tr`] module"]
#[doc(alias = "RTC_TR")]
pub type RtcTr = crate::Reg<rtc_tr::RtcTrSpec>;
#[doc = "The RTC_TR is the calendar time shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_tr;
#[doc = "RTC_DR (rw) register accessor: The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_dr`] module"]
#[doc(alias = "RTC_DR")]
pub type RtcDr = crate::Reg<rtc_dr::RtcDrSpec>;
#[doc = "The RTC_DR is the calendar date shadow register. This register must be written in initialization mode only. Refer to Calendar initialization and configuration on page9 and Reading the calendar on page10.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_dr;
#[doc = "RTC_CR (rw) register accessor: RTC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_cr`] module"]
#[doc(alias = "RTC_CR")]
pub type RtcCr = crate::Reg<rtc_cr::RtcCrSpec>;
#[doc = "RTC control register"]
pub mod rtc_cr;
#[doc = "RTC_ISR (rw) register accessor: This register is write protected (except for RTC_ISR\\[13:8\\] bits). The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_isr`] module"]
#[doc(alias = "RTC_ISR")]
pub type RtcIsr = crate::Reg<rtc_isr::RtcIsrSpec>;
#[doc = "This register is write protected (except for RTC_ISR\\[13:8\\] bits). The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_isr;
#[doc = "RTC_PRER (rw) register accessor: This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_prer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_prer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_prer`] module"]
#[doc(alias = "RTC_PRER")]
pub type RtcPrer = crate::Reg<rtc_prer::RtcPrerSpec>;
#[doc = "This register must be written in initialization mode only. The initialization must be performed in two separate write accesses. Refer to Calendar initialization and configuration on page9.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_prer;
#[doc = "RTC_WUTR (rw) register accessor: This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_wutr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wutr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wutr`] module"]
#[doc(alias = "RTC_WUTR")]
pub type RtcWutr = crate::Reg<rtc_wutr::RtcWutrSpec>;
#[doc = "This register can be written only when WUTWF is set to 1 in RTC_ISR.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_wutr;
#[doc = "RTC_ALRMAR (rw) register accessor: This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmar`] module"]
#[doc(alias = "RTC_ALRMAR")]
pub type RtcAlrmar = crate::Reg<rtc_alrmar::RtcAlrmarSpec>;
#[doc = "This register can be written only when ALRAWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_alrmar;
#[doc = "RTC_ALRMBR (rw) register accessor: This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbr`] module"]
#[doc(alias = "RTC_ALRMBR")]
pub type RtcAlrmbr = crate::Reg<rtc_alrmbr::RtcAlrmbrSpec>;
#[doc = "This register can be written only when ALRBWF is set to 1 in RTC_ISR, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_alrmbr;
#[doc = "RTC_WPR (w) register accessor: RTC write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_wpr`] module"]
#[doc(alias = "RTC_WPR")]
pub type RtcWpr = crate::Reg<rtc_wpr::RtcWprSpec>;
#[doc = "RTC write protection register"]
pub mod rtc_wpr;
#[doc = "RTC_SSR (r) register accessor: RTC sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_ssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_ssr`] module"]
#[doc(alias = "RTC_SSR")]
pub type RtcSsr = crate::Reg<rtc_ssr::RtcSsrSpec>;
#[doc = "RTC sub second register"]
pub mod rtc_ssr;
#[doc = "RTC_SHIFTR (w) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_shiftr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_shiftr`] module"]
#[doc(alias = "RTC_SHIFTR")]
pub type RtcShiftr = crate::Reg<rtc_shiftr::RtcShiftrSpec>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_shiftr;
#[doc = "RTC_TSTR (r) register accessor: The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tstr`] module"]
#[doc(alias = "RTC_TSTR")]
pub type RtcTstr = crate::Reg<rtc_tstr::RtcTstrSpec>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod rtc_tstr;
#[doc = "RTC_TSDR (r) register accessor: The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsdr`] module"]
#[doc(alias = "RTC_TSDR")]
pub type RtcTsdr = crate::Reg<rtc_tsdr::RtcTsdrSpec>;
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset."]
pub mod rtc_tsdr;
#[doc = "RTC_TSSSR (r) register accessor: The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tsssr`] module"]
#[doc(alias = "RTC_TSSSR")]
pub type RtcTsssr = crate::Reg<rtc_tsssr::RtcTsssrSpec>;
#[doc = "The content of this register is valid only when RTC_ISR/TSF is set. It is cleared when the RTC_ISR/TSF bit is reset."]
pub mod rtc_tsssr;
#[doc = "RTC_CALR (rw) register accessor: This register is write protected. The write access procedure is described in RTC register write protection on page9.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_calr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_calr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_calr`] module"]
#[doc(alias = "RTC_CALR")]
pub type RtcCalr = crate::Reg<rtc_calr::RtcCalrSpec>;
#[doc = "This register is write protected. The write access procedure is described in RTC register write protection on page9."]
pub mod rtc_calr;
#[doc = "RTC_TAMPCR (rw) register accessor: RTC tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_tampcr`] module"]
#[doc(alias = "RTC_TAMPCR")]
pub type RtcTampcr = crate::Reg<rtc_tampcr::RtcTampcrSpec>;
#[doc = "RTC tamper and alternate function configuration register"]
pub mod rtc_tampcr;
#[doc = "RTC_ALRMASSR (rw) register accessor: This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmassr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmassr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmassr`] module"]
#[doc(alias = "RTC_ALRMASSR")]
pub type RtcAlrmassr = crate::Reg<rtc_alrmassr::RtcAlrmassrSpec>;
#[doc = "This register can be written only when ALRAE is reset in RTC_CR register, or in initialization mode.This register is write protected. The write access procedure is described in RTC register write protection on page9"]
pub mod rtc_alrmassr;
#[doc = "RTC_ALRMBSSR (rw) register accessor: This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_alrmbssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_alrmbssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_alrmbssr`] module"]
#[doc(alias = "RTC_ALRMBSSR")]
pub type RtcAlrmbssr = crate::Reg<rtc_alrmbssr::RtcAlrmbssrSpec>;
#[doc = "This register can be written only when ALRBE is reset in RTC_CR register, or in initialization mode.This register is write protected.The write access procedure is described in Section: RTC register write protection."]
pub mod rtc_alrmbssr;
#[doc = "RTC_BKP0R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp0r`] module"]
#[doc(alias = "RTC_BKP0R")]
pub type RtcBkp0r = crate::Reg<rtc_bkp0r::RtcBkp0rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp0r;
#[doc = "RTC_BKP1R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp1r`] module"]
#[doc(alias = "RTC_BKP1R")]
pub type RtcBkp1r = crate::Reg<rtc_bkp1r::RtcBkp1rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp1r;
#[doc = "RTC_BKP2R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp2r`] module"]
#[doc(alias = "RTC_BKP2R")]
pub type RtcBkp2r = crate::Reg<rtc_bkp2r::RtcBkp2rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp2r;
#[doc = "RTC_BKP3R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp3r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp3r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp3r`] module"]
#[doc(alias = "RTC_BKP3R")]
pub type RtcBkp3r = crate::Reg<rtc_bkp3r::RtcBkp3rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp3r;
#[doc = "RTC_BKP4R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp4r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp4r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp4r`] module"]
#[doc(alias = "RTC_BKP4R")]
pub type RtcBkp4r = crate::Reg<rtc_bkp4r::RtcBkp4rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp4r;
#[doc = "RTC_BKP5R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp5r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp5r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp5r`] module"]
#[doc(alias = "RTC_BKP5R")]
pub type RtcBkp5r = crate::Reg<rtc_bkp5r::RtcBkp5rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp5r;
#[doc = "RTC_BKP6R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp6r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp6r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp6r`] module"]
#[doc(alias = "RTC_BKP6R")]
pub type RtcBkp6r = crate::Reg<rtc_bkp6r::RtcBkp6rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp6r;
#[doc = "RTC_BKP7R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp7r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp7r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp7r`] module"]
#[doc(alias = "RTC_BKP7R")]
pub type RtcBkp7r = crate::Reg<rtc_bkp7r::RtcBkp7rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp7r;
#[doc = "RTC_BKP8R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp8r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp8r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp8r`] module"]
#[doc(alias = "RTC_BKP8R")]
pub type RtcBkp8r = crate::Reg<rtc_bkp8r::RtcBkp8rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp8r;
#[doc = "RTC_BKP9R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp9r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp9r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp9r`] module"]
#[doc(alias = "RTC_BKP9R")]
pub type RtcBkp9r = crate::Reg<rtc_bkp9r::RtcBkp9rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp9r;
#[doc = "RTC_BKP10R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp10r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp10r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp10r`] module"]
#[doc(alias = "RTC_BKP10R")]
pub type RtcBkp10r = crate::Reg<rtc_bkp10r::RtcBkp10rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp10r;
#[doc = "RTC_BKP11R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp11r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp11r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp11r`] module"]
#[doc(alias = "RTC_BKP11R")]
pub type RtcBkp11r = crate::Reg<rtc_bkp11r::RtcBkp11rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp11r;
#[doc = "RTC_BKP12R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp12r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp12r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp12r`] module"]
#[doc(alias = "RTC_BKP12R")]
pub type RtcBkp12r = crate::Reg<rtc_bkp12r::RtcBkp12rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp12r;
#[doc = "RTC_BKP13R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp13r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp13r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp13r`] module"]
#[doc(alias = "RTC_BKP13R")]
pub type RtcBkp13r = crate::Reg<rtc_bkp13r::RtcBkp13rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp13r;
#[doc = "RTC_BKP14R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp14r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp14r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp14r`] module"]
#[doc(alias = "RTC_BKP14R")]
pub type RtcBkp14r = crate::Reg<rtc_bkp14r::RtcBkp14rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp14r;
#[doc = "RTC_BKP15R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp15r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp15r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp15r`] module"]
#[doc(alias = "RTC_BKP15R")]
pub type RtcBkp15r = crate::Reg<rtc_bkp15r::RtcBkp15rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp15r;
#[doc = "RTC_OR (rw) register accessor: RTC option register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_or::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_or::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_or`] module"]
#[doc(alias = "RTC_OR")]
pub type RtcOr = crate::Reg<rtc_or::RtcOrSpec>;
#[doc = "RTC option register"]
pub mod rtc_or;
#[doc = "RTC_BKP16R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp16r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp16r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp16r`] module"]
#[doc(alias = "RTC_BKP16R")]
pub type RtcBkp16r = crate::Reg<rtc_bkp16r::RtcBkp16rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp16r;
#[doc = "RTC_BKP17R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp17r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp17r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp17r`] module"]
#[doc(alias = "RTC_BKP17R")]
pub type RtcBkp17r = crate::Reg<rtc_bkp17r::RtcBkp17rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp17r;
#[doc = "RTC_BKP18R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp18r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp18r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp18r`] module"]
#[doc(alias = "RTC_BKP18R")]
pub type RtcBkp18r = crate::Reg<rtc_bkp18r::RtcBkp18rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp18r;
#[doc = "RTC_BKP19R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp19r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp19r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp19r`] module"]
#[doc(alias = "RTC_BKP19R")]
pub type RtcBkp19r = crate::Reg<rtc_bkp19r::RtcBkp19rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp19r;
#[doc = "RTC_BKP20R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp20r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp20r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp20r`] module"]
#[doc(alias = "RTC_BKP20R")]
pub type RtcBkp20r = crate::Reg<rtc_bkp20r::RtcBkp20rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp20r;
#[doc = "RTC_BKP21R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp21r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp21r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp21r`] module"]
#[doc(alias = "RTC_BKP21R")]
pub type RtcBkp21r = crate::Reg<rtc_bkp21r::RtcBkp21rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp21r;
#[doc = "RTC_BKP22R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp22r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp22r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp22r`] module"]
#[doc(alias = "RTC_BKP22R")]
pub type RtcBkp22r = crate::Reg<rtc_bkp22r::RtcBkp22rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp22r;
#[doc = "RTC_BKP23R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp23r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp23r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp23r`] module"]
#[doc(alias = "RTC_BKP23R")]
pub type RtcBkp23r = crate::Reg<rtc_bkp23r::RtcBkp23rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp23r;
#[doc = "RTC_BKP24R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp24r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp24r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp24r`] module"]
#[doc(alias = "RTC_BKP24R")]
pub type RtcBkp24r = crate::Reg<rtc_bkp24r::RtcBkp24rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp24r;
#[doc = "RTC_BKP25R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp25r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp25r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp25r`] module"]
#[doc(alias = "RTC_BKP25R")]
pub type RtcBkp25r = crate::Reg<rtc_bkp25r::RtcBkp25rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp25r;
#[doc = "RTC_BKP26R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp26r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp26r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp26r`] module"]
#[doc(alias = "RTC_BKP26R")]
pub type RtcBkp26r = crate::Reg<rtc_bkp26r::RtcBkp26rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp26r;
#[doc = "RTC_BKP27R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp27r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp27r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp27r`] module"]
#[doc(alias = "RTC_BKP27R")]
pub type RtcBkp27r = crate::Reg<rtc_bkp27r::RtcBkp27rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp27r;
#[doc = "RTC_BKP28R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp28r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp28r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp28r`] module"]
#[doc(alias = "RTC_BKP28R")]
pub type RtcBkp28r = crate::Reg<rtc_bkp28r::RtcBkp28rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp28r;
#[doc = "RTC_BKP29R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp29r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp29r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp29r`] module"]
#[doc(alias = "RTC_BKP29R")]
pub type RtcBkp29r = crate::Reg<rtc_bkp29r::RtcBkp29rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp29r;
#[doc = "RTC_BKP30R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp30r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp30r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp30r`] module"]
#[doc(alias = "RTC_BKP30R")]
pub type RtcBkp30r = crate::Reg<rtc_bkp30r::RtcBkp30rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp30r;
#[doc = "RTC_BKP31R (rw) register accessor: RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp31r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp31r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtc_bkp31r`] module"]
#[doc(alias = "RTC_BKP31R")]
pub type RtcBkp31r = crate::Reg<rtc_bkp31r::RtcBkp31rSpec>;
#[doc = "RTC backup registers"]
pub mod rtc_bkp31r;
