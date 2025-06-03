#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr1: Cr1,
    csr1: Csr1,
    cr2: Cr2,
    cr3: Cr3,
    cpucr: Cpucr,
    _reserved5: [u8; 0x04],
    d3cr: D3cr,
    _reserved6: [u8; 0x04],
    wkupcr: Wkupcr,
    wkupfr: Wkupfr,
    wkupepr: Wkupepr,
}
impl RegisterBlock {
    #[doc = "0x00 - PWR control register 1"]
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    #[doc = "0x04 - PWR control status register 1"]
    #[inline(always)]
    pub const fn csr1(&self) -> &Csr1 {
        &self.csr1
    }
    #[doc = "0x08 - This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    #[doc = "0x0c - Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    #[doc = "0x10 - This register allows controlling CPU1 power."]
    #[inline(always)]
    pub const fn cpucr(&self) -> &Cpucr {
        &self.cpucr
    }
    #[doc = "0x18 - This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
    #[inline(always)]
    pub const fn d3cr(&self) -> &D3cr {
        &self.d3cr
    }
    #[doc = "0x20 - reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
    #[inline(always)]
    pub const fn wkupcr(&self) -> &Wkupcr {
        &self.wkupcr
    }
    #[doc = "0x24 - reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupfr(&self) -> &Wkupfr {
        &self.wkupfr
    }
    #[doc = "0x28 - Reset only by system reset, not reset by wakeup from Standby mode"]
    #[inline(always)]
    pub const fn wkupepr(&self) -> &Wkupepr {
        &self.wkupepr
    }
}
#[doc = "CR1 (rw) register accessor: PWR control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr1`] module"]
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
#[doc = "PWR control register 1"]
pub mod cr1;
#[doc = "CSR1 (r) register accessor: PWR control status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr1`] module"]
#[doc(alias = "CSR1")]
pub type Csr1 = crate::Reg<csr1::Csr1Spec>;
#[doc = "PWR control status register 1"]
pub mod csr1;
#[doc = "CR2 (rw) register accessor: This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr2`] module"]
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
#[doc = "This register is not reset by wakeup from Standby mode, RESET signal and VDD POR. It is only reset by VSW POR and VSWRST reset. This register shall not be accessed when VSWRST bit in RCC_BDCR register resets the VSW domain.After reset, PWR_CR2 register is write-protected. Prior to modifying its content, the DBP bit in PWR_CR1 register must be set to disable the write protection."]
pub mod cr2;
#[doc = "CR3 (rw) register accessor: Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value.\n\nYou can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr3`] module"]
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
#[doc = "Reset only by POR only, not reset by wakeup from Standby mode and RESET pad. The lower byte of this register is written once after POR and shall be written before changing VOS level or ck_sys clock frequency. No limitation applies to the upper bytes.Programming data corresponding to an invalid combination of SDLEVEL, SDEXTHP, SDEN, LDOEN and BYPASS bits (see Table9) will be ignored: data will not be written, the written-once mechanism will lock the register and any further write access will be ignored. The default supply configuration will be kept and the ACTVOSRDY bit in PWR control status register 1 (PWR_CSR1) will go on indicating invalid voltage levels. The system shall be power cycled before writing a new value."]
pub mod cr3;
#[doc = "CPUCR (rw) register accessor: This register allows controlling CPU1 power.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpucr`] module"]
#[doc(alias = "CPUCR")]
pub type Cpucr = crate::Reg<cpucr::CpucrSpec>;
#[doc = "This register allows controlling CPU1 power."]
pub mod cpucr;
#[doc = "D3CR (rw) register accessor: This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software\n\nYou can [`read`](crate::Reg::read) this register and get [`d3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3cr`] module"]
#[doc(alias = "D3CR")]
pub type D3cr = crate::Reg<d3cr::D3crSpec>;
#[doc = "This register allows controlling D3 domain power.Following reset VOSRDY will be read 1 by software"]
pub mod d3cr;
#[doc = "WKUPCR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupcr`] module"]
#[doc(alias = "WKUPCR")]
pub type Wkupcr = crate::Reg<wkupcr::WkupcrSpec>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared)."]
pub mod wkupcr;
#[doc = "WKUPFR (rw) register accessor: reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupfr`] module"]
#[doc(alias = "WKUPFR")]
pub type Wkupfr = crate::Reg<wkupfr::WkupfrSpec>;
#[doc = "reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupfr;
#[doc = "WKUPEPR (rw) register accessor: Reset only by system reset, not reset by wakeup from Standby mode\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupepr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupepr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wkupepr`] module"]
#[doc(alias = "WKUPEPR")]
pub type Wkupepr = crate::Reg<wkupepr::WkupeprSpec>;
#[doc = "Reset only by system reset, not reset by wakeup from Standby mode"]
pub mod wkupepr;
