#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    dsi_vr: DsiVr,
    dsi_cr: DsiCr,
    dsi_ccr: DsiCcr,
    dsi_lvcidr: DsiLvcidr,
    dsi_lcolcr: DsiLcolcr,
    dsi_lpcr: DsiLpcr,
    dsi_lpmcr: DsiLpmcr,
    _reserved7: [u8; 0x10],
    dsi_pcr: DsiPcr,
    dsi_gvcidr: DsiGvcidr,
    dsi_mcr: DsiMcr,
    dsi_vmcr: DsiVmcr,
    dsi_vpcr: DsiVpcr,
    dsi_vccr: DsiVccr,
    dsi_vnpcr: DsiVnpcr,
    dsi_vhsacr: DsiVhsacr,
    dsi_vhbpcr: DsiVhbpcr,
    dsi_vlcr: DsiVlcr,
    dsi_vvsacr: DsiVvsacr,
    dsi_vvbpcr: DsiVvbpcr,
    dsi_vvfpcr: DsiVvfpcr,
    dsi_vvacr: DsiVvacr,
    dsi_lccr: DsiLccr,
    dsi_cmcr: DsiCmcr,
    dsi_ghcr: DsiGhcr,
    dsi_gpdr: DsiGpdr,
    dsi_gpsr: DsiGpsr,
    dsi_tccr0: DsiTccr0,
    dsi_tccr1: DsiTccr1,
    dsi_tccr2: DsiTccr2,
    dsi_tccr3: DsiTccr3,
    dsi_tccr4: DsiTccr4,
    dsi_tccr5: DsiTccr5,
    _reserved32: [u8; 0x04],
    dsi_clcr: DsiClcr,
    dsi_cltcr: DsiCltcr,
    dsi_dltcr: DsiDltcr,
    dsi_pctlr: DsiPctlr,
    dsi_pconfr: DsiPconfr,
    dsi_pucr: DsiPucr,
    dsi_pttcr: DsiPttcr,
    dsi_psr: DsiPsr,
    _reserved40: [u8; 0x08],
    dsi_isr0: DsiIsr0,
    dsi_isr1: DsiIsr1,
    dsi_ier0: DsiIer0,
    dsi_ier1: DsiIer1,
    _reserved44: [u8; 0x0c],
    dsi_fir0: DsiFir0,
    dsi_fir1: DsiFir1,
    _reserved46: [u8; 0x20],
    dsi_vscr: DsiVscr,
    _reserved47: [u8; 0x08],
    dsi_lcvcidr: DsiLcvcidr,
    dsi_lcccr: DsiLcccr,
    _reserved49: [u8; 0x04],
    dsi_lpmccr: DsiLpmccr,
    _reserved50: [u8; 0x1c],
    dsi_vmccr: DsiVmccr,
    dsi_vpccr: DsiVpccr,
    dsi_vcccr: DsiVcccr,
    dsi_vnpccr: DsiVnpccr,
    dsi_vhsaccr: DsiVhsaccr,
    dsi_vhbpccr: DsiVhbpccr,
    dsi_vlccr: DsiVlccr,
    dsi_vvsaccr: DsiVvsaccr,
    dsi_vvbpccr: DsiVvbpccr,
    dsi_vvfpccr: DsiVvfpccr,
    dsi_vvaccr: DsiVvaccr,
    _reserved61: [u8; 0x029c],
    dsi_wcfgr: DsiWcfgr,
    dsi_wcr: DsiWcr,
    dsi_wier: DsiWier,
    dsi_wisr: DsiWisr,
    dsi_wifcr: DsiWifcr,
    _reserved66: [u8; 0x04],
    dsi_wpcr0: DsiWpcr0,
    dsi_wpcr1: DsiWpcr1,
    dsi_wpcr2: DsiWpcr2,
    dsi_wpcr3: DsiWpcr3,
    dsi_wpcr4: DsiWpcr4,
    _reserved71: [u8; 0x04],
    dsi_wrpcr: DsiWrpcr,
}
impl RegisterBlock {
    #[doc = "0x00 - DSI Host version register"]
    #[inline(always)]
    pub const fn dsi_vr(&self) -> &DsiVr {
        &self.dsi_vr
    }
    #[doc = "0x04 - DSI Host control register"]
    #[inline(always)]
    pub const fn dsi_cr(&self) -> &DsiCr {
        &self.dsi_cr
    }
    #[doc = "0x08 - DSI Host clock control register"]
    #[inline(always)]
    pub const fn dsi_ccr(&self) -> &DsiCcr {
        &self.dsi_ccr
    }
    #[doc = "0x0c - DSI Host LTDC VCID register"]
    #[inline(always)]
    pub const fn dsi_lvcidr(&self) -> &DsiLvcidr {
        &self.dsi_lvcidr
    }
    #[doc = "0x10 - DSI Host LTDC color coding register"]
    #[inline(always)]
    pub const fn dsi_lcolcr(&self) -> &DsiLcolcr {
        &self.dsi_lcolcr
    }
    #[doc = "0x14 - DSI Host LTDC polarity configuration register"]
    #[inline(always)]
    pub const fn dsi_lpcr(&self) -> &DsiLpcr {
        &self.dsi_lpcr
    }
    #[doc = "0x18 - DSI Host low-power mode configuration register"]
    #[inline(always)]
    pub const fn dsi_lpmcr(&self) -> &DsiLpmcr {
        &self.dsi_lpmcr
    }
    #[doc = "0x2c - DSI Host protocol configuration register"]
    #[inline(always)]
    pub const fn dsi_pcr(&self) -> &DsiPcr {
        &self.dsi_pcr
    }
    #[doc = "0x30 - DSI Host generic VCID register"]
    #[inline(always)]
    pub const fn dsi_gvcidr(&self) -> &DsiGvcidr {
        &self.dsi_gvcidr
    }
    #[doc = "0x34 - DSI Host mode configuration register"]
    #[inline(always)]
    pub const fn dsi_mcr(&self) -> &DsiMcr {
        &self.dsi_mcr
    }
    #[doc = "0x38 - DSI Host video mode configuration register"]
    #[inline(always)]
    pub const fn dsi_vmcr(&self) -> &DsiVmcr {
        &self.dsi_vmcr
    }
    #[doc = "0x3c - DSI Host video packet configuration register"]
    #[inline(always)]
    pub const fn dsi_vpcr(&self) -> &DsiVpcr {
        &self.dsi_vpcr
    }
    #[doc = "0x40 - DSI Host video chunks configuration register"]
    #[inline(always)]
    pub const fn dsi_vccr(&self) -> &DsiVccr {
        &self.dsi_vccr
    }
    #[doc = "0x44 - DSI Host video null packet configuration register"]
    #[inline(always)]
    pub const fn dsi_vnpcr(&self) -> &DsiVnpcr {
        &self.dsi_vnpcr
    }
    #[doc = "0x48 - DSI Host video HSA configuration register"]
    #[inline(always)]
    pub const fn dsi_vhsacr(&self) -> &DsiVhsacr {
        &self.dsi_vhsacr
    }
    #[doc = "0x4c - DSI Host video HBP configuration register"]
    #[inline(always)]
    pub const fn dsi_vhbpcr(&self) -> &DsiVhbpcr {
        &self.dsi_vhbpcr
    }
    #[doc = "0x50 - DSI Host video line configuration register"]
    #[inline(always)]
    pub const fn dsi_vlcr(&self) -> &DsiVlcr {
        &self.dsi_vlcr
    }
    #[doc = "0x54 - DSI Host video VSA configuration register"]
    #[inline(always)]
    pub const fn dsi_vvsacr(&self) -> &DsiVvsacr {
        &self.dsi_vvsacr
    }
    #[doc = "0x58 - DSI Host video VBP configuration register"]
    #[inline(always)]
    pub const fn dsi_vvbpcr(&self) -> &DsiVvbpcr {
        &self.dsi_vvbpcr
    }
    #[doc = "0x5c - DSI Host video VFP configuration register"]
    #[inline(always)]
    pub const fn dsi_vvfpcr(&self) -> &DsiVvfpcr {
        &self.dsi_vvfpcr
    }
    #[doc = "0x60 - DSI Host video VA configuration register"]
    #[inline(always)]
    pub const fn dsi_vvacr(&self) -> &DsiVvacr {
        &self.dsi_vvacr
    }
    #[doc = "0x64 - DSI Host LTDC command configuration register"]
    #[inline(always)]
    pub const fn dsi_lccr(&self) -> &DsiLccr {
        &self.dsi_lccr
    }
    #[doc = "0x68 - DSI Host command mode configuration register"]
    #[inline(always)]
    pub const fn dsi_cmcr(&self) -> &DsiCmcr {
        &self.dsi_cmcr
    }
    #[doc = "0x6c - DSI Host generic header configuration register"]
    #[inline(always)]
    pub const fn dsi_ghcr(&self) -> &DsiGhcr {
        &self.dsi_ghcr
    }
    #[doc = "0x70 - DSI Host generic payload data register"]
    #[inline(always)]
    pub const fn dsi_gpdr(&self) -> &DsiGpdr {
        &self.dsi_gpdr
    }
    #[doc = "0x74 - DSI Host generic packet status register"]
    #[inline(always)]
    pub const fn dsi_gpsr(&self) -> &DsiGpsr {
        &self.dsi_gpsr
    }
    #[doc = "0x78 - DSI Host timeout counter configuration register 0"]
    #[inline(always)]
    pub const fn dsi_tccr0(&self) -> &DsiTccr0 {
        &self.dsi_tccr0
    }
    #[doc = "0x7c - DSI Host timeout counter configuration register 1"]
    #[inline(always)]
    pub const fn dsi_tccr1(&self) -> &DsiTccr1 {
        &self.dsi_tccr1
    }
    #[doc = "0x80 - DSI Host timeout counter configuration register 2"]
    #[inline(always)]
    pub const fn dsi_tccr2(&self) -> &DsiTccr2 {
        &self.dsi_tccr2
    }
    #[doc = "0x84 - DSI Host timeout counter configuration register 3"]
    #[inline(always)]
    pub const fn dsi_tccr3(&self) -> &DsiTccr3 {
        &self.dsi_tccr3
    }
    #[doc = "0x88 - DSI Host timeout counter configuration register 4"]
    #[inline(always)]
    pub const fn dsi_tccr4(&self) -> &DsiTccr4 {
        &self.dsi_tccr4
    }
    #[doc = "0x8c - DSI Host timeout counter configuration register 5"]
    #[inline(always)]
    pub const fn dsi_tccr5(&self) -> &DsiTccr5 {
        &self.dsi_tccr5
    }
    #[doc = "0x94 - DSI Host clock lane configuration register"]
    #[inline(always)]
    pub const fn dsi_clcr(&self) -> &DsiClcr {
        &self.dsi_clcr
    }
    #[doc = "0x98 - DSI Host clock lane timer configuration register"]
    #[inline(always)]
    pub const fn dsi_cltcr(&self) -> &DsiCltcr {
        &self.dsi_cltcr
    }
    #[doc = "0x9c - DSI Host data lane timer configuration register"]
    #[inline(always)]
    pub const fn dsi_dltcr(&self) -> &DsiDltcr {
        &self.dsi_dltcr
    }
    #[doc = "0xa0 - DSI Host PHY control register"]
    #[inline(always)]
    pub const fn dsi_pctlr(&self) -> &DsiPctlr {
        &self.dsi_pctlr
    }
    #[doc = "0xa4 - DSI Host PHY configuration register"]
    #[inline(always)]
    pub const fn dsi_pconfr(&self) -> &DsiPconfr {
        &self.dsi_pconfr
    }
    #[doc = "0xa8 - DSI Host PHY ULPS control register"]
    #[inline(always)]
    pub const fn dsi_pucr(&self) -> &DsiPucr {
        &self.dsi_pucr
    }
    #[doc = "0xac - DSI Host PHY TX triggers configuration register"]
    #[inline(always)]
    pub const fn dsi_pttcr(&self) -> &DsiPttcr {
        &self.dsi_pttcr
    }
    #[doc = "0xb0 - DSI Host PHY status register"]
    #[inline(always)]
    pub const fn dsi_psr(&self) -> &DsiPsr {
        &self.dsi_psr
    }
    #[doc = "0xbc - DSI Host interrupt and status register 0"]
    #[inline(always)]
    pub const fn dsi_isr0(&self) -> &DsiIsr0 {
        &self.dsi_isr0
    }
    #[doc = "0xc0 - DSI Host interrupt and status register 1"]
    #[inline(always)]
    pub const fn dsi_isr1(&self) -> &DsiIsr1 {
        &self.dsi_isr1
    }
    #[doc = "0xc4 - DSI Host interrupt enable register 0"]
    #[inline(always)]
    pub const fn dsi_ier0(&self) -> &DsiIer0 {
        &self.dsi_ier0
    }
    #[doc = "0xc8 - DSI Host interrupt enable register 1"]
    #[inline(always)]
    pub const fn dsi_ier1(&self) -> &DsiIer1 {
        &self.dsi_ier1
    }
    #[doc = "0xd8 - DSI Host force interrupt register 0"]
    #[inline(always)]
    pub const fn dsi_fir0(&self) -> &DsiFir0 {
        &self.dsi_fir0
    }
    #[doc = "0xdc - DSI Host force interrupt register 1"]
    #[inline(always)]
    pub const fn dsi_fir1(&self) -> &DsiFir1 {
        &self.dsi_fir1
    }
    #[doc = "0x100 - DSI Host video shadow control register"]
    #[inline(always)]
    pub const fn dsi_vscr(&self) -> &DsiVscr {
        &self.dsi_vscr
    }
    #[doc = "0x10c - DSI Host LTDC current VCID register"]
    #[inline(always)]
    pub const fn dsi_lcvcidr(&self) -> &DsiLcvcidr {
        &self.dsi_lcvcidr
    }
    #[doc = "0x110 - DSI Host LTDC current color coding register"]
    #[inline(always)]
    pub const fn dsi_lcccr(&self) -> &DsiLcccr {
        &self.dsi_lcccr
    }
    #[doc = "0x118 - DSI Host low-power mode current configuration register"]
    #[inline(always)]
    pub const fn dsi_lpmccr(&self) -> &DsiLpmccr {
        &self.dsi_lpmccr
    }
    #[doc = "0x138 - DSI Host video mode current configuration register"]
    #[inline(always)]
    pub const fn dsi_vmccr(&self) -> &DsiVmccr {
        &self.dsi_vmccr
    }
    #[doc = "0x13c - DSI Host video packet current configuration register"]
    #[inline(always)]
    pub const fn dsi_vpccr(&self) -> &DsiVpccr {
        &self.dsi_vpccr
    }
    #[doc = "0x140 - DSI Host video chunks current configuration register"]
    #[inline(always)]
    pub const fn dsi_vcccr(&self) -> &DsiVcccr {
        &self.dsi_vcccr
    }
    #[doc = "0x144 - DSI Host video null packet current configuration register"]
    #[inline(always)]
    pub const fn dsi_vnpccr(&self) -> &DsiVnpccr {
        &self.dsi_vnpccr
    }
    #[doc = "0x148 - DSI Host video HSA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vhsaccr(&self) -> &DsiVhsaccr {
        &self.dsi_vhsaccr
    }
    #[doc = "0x14c - DSI Host video HBP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vhbpccr(&self) -> &DsiVhbpccr {
        &self.dsi_vhbpccr
    }
    #[doc = "0x150 - DSI Host video line current configuration register"]
    #[inline(always)]
    pub const fn dsi_vlccr(&self) -> &DsiVlccr {
        &self.dsi_vlccr
    }
    #[doc = "0x154 - DSI Host video VSA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvsaccr(&self) -> &DsiVvsaccr {
        &self.dsi_vvsaccr
    }
    #[doc = "0x158 - DSI Host video VBP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvbpccr(&self) -> &DsiVvbpccr {
        &self.dsi_vvbpccr
    }
    #[doc = "0x15c - DSI Host video VFP current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvfpccr(&self) -> &DsiVvfpccr {
        &self.dsi_vvfpccr
    }
    #[doc = "0x160 - DSI Host video VA current configuration register"]
    #[inline(always)]
    pub const fn dsi_vvaccr(&self) -> &DsiVvaccr {
        &self.dsi_vvaccr
    }
    #[doc = "0x400 - DSI wrapper configuration register"]
    #[inline(always)]
    pub const fn dsi_wcfgr(&self) -> &DsiWcfgr {
        &self.dsi_wcfgr
    }
    #[doc = "0x404 - DSI wrapper control register"]
    #[inline(always)]
    pub const fn dsi_wcr(&self) -> &DsiWcr {
        &self.dsi_wcr
    }
    #[doc = "0x408 - DSI wrapper interrupt enable register"]
    #[inline(always)]
    pub const fn dsi_wier(&self) -> &DsiWier {
        &self.dsi_wier
    }
    #[doc = "0x40c - DSI wrapper interrupt and status register"]
    #[inline(always)]
    pub const fn dsi_wisr(&self) -> &DsiWisr {
        &self.dsi_wisr
    }
    #[doc = "0x410 - DSI wrapper interrupt flag clear register"]
    #[inline(always)]
    pub const fn dsi_wifcr(&self) -> &DsiWifcr {
        &self.dsi_wifcr
    }
    #[doc = "0x418 - DSI wrapper PHY configuration register 0"]
    #[inline(always)]
    pub const fn dsi_wpcr0(&self) -> &DsiWpcr0 {
        &self.dsi_wpcr0
    }
    #[doc = "0x41c - This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
    #[inline(always)]
    pub const fn dsi_wpcr1(&self) -> &DsiWpcr1 {
        &self.dsi_wpcr1
    }
    #[doc = "0x420 - DSI wrapper PHY configuration register 2"]
    #[inline(always)]
    pub const fn dsi_wpcr2(&self) -> &DsiWpcr2 {
        &self.dsi_wpcr2
    }
    #[doc = "0x424 - DSI wrapper PHY configuration register 3"]
    #[inline(always)]
    pub const fn dsi_wpcr3(&self) -> &DsiWpcr3 {
        &self.dsi_wpcr3
    }
    #[doc = "0x428 - DSI wrapper PHY configuration register 4"]
    #[inline(always)]
    pub const fn dsi_wpcr4(&self) -> &DsiWpcr4 {
        &self.dsi_wpcr4
    }
    #[doc = "0x430 - DSI wrapper regulator and PLL control register"]
    #[inline(always)]
    pub const fn dsi_wrpcr(&self) -> &DsiWrpcr {
        &self.dsi_wrpcr
    }
}
#[doc = "DSI_VR (r) register accessor: DSI Host version register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vr`] module"]
#[doc(alias = "DSI_VR")]
pub type DsiVr = crate::Reg<dsi_vr::DsiVrSpec>;
#[doc = "DSI Host version register"]
pub mod dsi_vr;
#[doc = "DSI_CR (rw) register accessor: DSI Host control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cr`] module"]
#[doc(alias = "DSI_CR")]
pub type DsiCr = crate::Reg<dsi_cr::DsiCrSpec>;
#[doc = "DSI Host control register"]
pub mod dsi_cr;
#[doc = "DSI_CCR (rw) register accessor: DSI Host clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ccr`] module"]
#[doc(alias = "DSI_CCR")]
pub type DsiCcr = crate::Reg<dsi_ccr::DsiCcrSpec>;
#[doc = "DSI Host clock control register"]
pub mod dsi_ccr;
#[doc = "DSI_LVCIDR (rw) register accessor: DSI Host LTDC VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lvcidr`] module"]
#[doc(alias = "DSI_LVCIDR")]
pub type DsiLvcidr = crate::Reg<dsi_lvcidr::DsiLvcidrSpec>;
#[doc = "DSI Host LTDC VCID register"]
pub mod dsi_lvcidr;
#[doc = "DSI_LCOLCR (rw) register accessor: DSI Host LTDC color coding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcolcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lcolcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcolcr`] module"]
#[doc(alias = "DSI_LCOLCR")]
pub type DsiLcolcr = crate::Reg<dsi_lcolcr::DsiLcolcrSpec>;
#[doc = "DSI Host LTDC color coding register"]
pub mod dsi_lcolcr;
#[doc = "DSI_LPCR (rw) register accessor: DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpcr`] module"]
#[doc(alias = "DSI_LPCR")]
pub type DsiLpcr = crate::Reg<dsi_lpcr::DsiLpcrSpec>;
#[doc = "DSI Host LTDC polarity configuration register"]
pub mod dsi_lpcr;
#[doc = "DSI_LPMCR (rw) register accessor: DSI Host low-power mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpmcr`] module"]
#[doc(alias = "DSI_LPMCR")]
pub type DsiLpmcr = crate::Reg<dsi_lpmcr::DsiLpmcrSpec>;
#[doc = "DSI Host low-power mode configuration register"]
pub mod dsi_lpmcr;
#[doc = "DSI_PCR (rw) register accessor: DSI Host protocol configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pcr`] module"]
#[doc(alias = "DSI_PCR")]
pub type DsiPcr = crate::Reg<dsi_pcr::DsiPcrSpec>;
#[doc = "DSI Host protocol configuration register"]
pub mod dsi_pcr;
#[doc = "DSI_GVCIDR (r) register accessor: DSI Host generic VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gvcidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gvcidr`] module"]
#[doc(alias = "DSI_GVCIDR")]
pub type DsiGvcidr = crate::Reg<dsi_gvcidr::DsiGvcidrSpec>;
#[doc = "DSI Host generic VCID register"]
pub mod dsi_gvcidr;
#[doc = "DSI_MCR (rw) register accessor: DSI Host mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_mcr`] module"]
#[doc(alias = "DSI_MCR")]
pub type DsiMcr = crate::Reg<dsi_mcr::DsiMcrSpec>;
#[doc = "DSI Host mode configuration register"]
pub mod dsi_mcr;
#[doc = "DSI_VMCR (rw) register accessor: DSI Host video mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vmcr`] module"]
#[doc(alias = "DSI_VMCR")]
pub type DsiVmcr = crate::Reg<dsi_vmcr::DsiVmcrSpec>;
#[doc = "DSI Host video mode configuration register"]
pub mod dsi_vmcr;
#[doc = "DSI_VPCR (rw) register accessor: DSI Host video packet configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vpcr`] module"]
#[doc(alias = "DSI_VPCR")]
pub type DsiVpcr = crate::Reg<dsi_vpcr::DsiVpcrSpec>;
#[doc = "DSI Host video packet configuration register"]
pub mod dsi_vpcr;
#[doc = "DSI_VCCR (rw) register accessor: DSI Host video chunks configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vccr`] module"]
#[doc(alias = "DSI_VCCR")]
pub type DsiVccr = crate::Reg<dsi_vccr::DsiVccrSpec>;
#[doc = "DSI Host video chunks configuration register"]
pub mod dsi_vccr;
#[doc = "DSI_VNPCR (rw) register accessor: DSI Host video null packet configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vnpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vnpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vnpcr`] module"]
#[doc(alias = "DSI_VNPCR")]
pub type DsiVnpcr = crate::Reg<dsi_vnpcr::DsiVnpcrSpec>;
#[doc = "DSI Host video null packet configuration register"]
pub mod dsi_vnpcr;
#[doc = "DSI_VHSACR (rw) register accessor: DSI Host video HSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vhsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhsacr`] module"]
#[doc(alias = "DSI_VHSACR")]
pub type DsiVhsacr = crate::Reg<dsi_vhsacr::DsiVhsacrSpec>;
#[doc = "DSI Host video HSA configuration register"]
pub mod dsi_vhsacr;
#[doc = "DSI_VHBPCR (rw) register accessor: DSI Host video HBP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vhbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhbpcr`] module"]
#[doc(alias = "DSI_VHBPCR")]
pub type DsiVhbpcr = crate::Reg<dsi_vhbpcr::DsiVhbpcrSpec>;
#[doc = "DSI Host video HBP configuration register"]
pub mod dsi_vhbpcr;
#[doc = "DSI_VLCR (rw) register accessor: DSI Host video line configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vlcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vlcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vlcr`] module"]
#[doc(alias = "DSI_VLCR")]
pub type DsiVlcr = crate::Reg<dsi_vlcr::DsiVlcrSpec>;
#[doc = "DSI Host video line configuration register"]
pub mod dsi_vlcr;
#[doc = "DSI_VVSACR (rw) register accessor: DSI Host video VSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvsacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvsacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvsacr`] module"]
#[doc(alias = "DSI_VVSACR")]
pub type DsiVvsacr = crate::Reg<dsi_vvsacr::DsiVvsacrSpec>;
#[doc = "DSI Host video VSA configuration register"]
pub mod dsi_vvsacr;
#[doc = "DSI_VVBPCR (rw) register accessor: DSI Host video VBP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvbpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvbpcr`] module"]
#[doc(alias = "DSI_VVBPCR")]
pub type DsiVvbpcr = crate::Reg<dsi_vvbpcr::DsiVvbpcrSpec>;
#[doc = "DSI Host video VBP configuration register"]
pub mod dsi_vvbpcr;
#[doc = "DSI_VVFPCR (rw) register accessor: DSI Host video VFP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvfpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvfpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvfpcr`] module"]
#[doc(alias = "DSI_VVFPCR")]
pub type DsiVvfpcr = crate::Reg<dsi_vvfpcr::DsiVvfpcrSpec>;
#[doc = "DSI Host video VFP configuration register"]
pub mod dsi_vvfpcr;
#[doc = "DSI_VVACR (rw) register accessor: DSI Host video VA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvacr`] module"]
#[doc(alias = "DSI_VVACR")]
pub type DsiVvacr = crate::Reg<dsi_vvacr::DsiVvacrSpec>;
#[doc = "DSI Host video VA configuration register"]
pub mod dsi_vvacr;
#[doc = "DSI_LCCR (rw) register accessor: DSI Host LTDC command configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lccr`] module"]
#[doc(alias = "DSI_LCCR")]
pub type DsiLccr = crate::Reg<dsi_lccr::DsiLccrSpec>;
#[doc = "DSI Host LTDC command configuration register"]
pub mod dsi_lccr;
#[doc = "DSI_CMCR (rw) register accessor: DSI Host command mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cmcr`] module"]
#[doc(alias = "DSI_CMCR")]
pub type DsiCmcr = crate::Reg<dsi_cmcr::DsiCmcrSpec>;
#[doc = "DSI Host command mode configuration register"]
pub mod dsi_cmcr;
#[doc = "DSI_GHCR (rw) register accessor: DSI Host generic header configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ghcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ghcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ghcr`] module"]
#[doc(alias = "DSI_GHCR")]
pub type DsiGhcr = crate::Reg<dsi_ghcr::DsiGhcrSpec>;
#[doc = "DSI Host generic header configuration register"]
pub mod dsi_ghcr;
#[doc = "DSI_GPDR (rw) register accessor: DSI Host generic payload data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_gpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gpdr`] module"]
#[doc(alias = "DSI_GPDR")]
pub type DsiGpdr = crate::Reg<dsi_gpdr::DsiGpdrSpec>;
#[doc = "DSI Host generic payload data register"]
pub mod dsi_gpdr;
#[doc = "DSI_GPSR (r) register accessor: DSI Host generic packet status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_gpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_gpsr`] module"]
#[doc(alias = "DSI_GPSR")]
pub type DsiGpsr = crate::Reg<dsi_gpsr::DsiGpsrSpec>;
#[doc = "DSI Host generic packet status register"]
pub mod dsi_gpsr;
#[doc = "DSI_TCCR0 (rw) register accessor: DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr0`] module"]
#[doc(alias = "DSI_TCCR0")]
pub type DsiTccr0 = crate::Reg<dsi_tccr0::DsiTccr0Spec>;
#[doc = "DSI Host timeout counter configuration register 0"]
pub mod dsi_tccr0;
#[doc = "DSI_TCCR1 (rw) register accessor: DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr1`] module"]
#[doc(alias = "DSI_TCCR1")]
pub type DsiTccr1 = crate::Reg<dsi_tccr1::DsiTccr1Spec>;
#[doc = "DSI Host timeout counter configuration register 1"]
pub mod dsi_tccr1;
#[doc = "DSI_TCCR2 (rw) register accessor: DSI Host timeout counter configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr2`] module"]
#[doc(alias = "DSI_TCCR2")]
pub type DsiTccr2 = crate::Reg<dsi_tccr2::DsiTccr2Spec>;
#[doc = "DSI Host timeout counter configuration register 2"]
pub mod dsi_tccr2;
#[doc = "DSI_TCCR3 (rw) register accessor: DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr3`] module"]
#[doc(alias = "DSI_TCCR3")]
pub type DsiTccr3 = crate::Reg<dsi_tccr3::DsiTccr3Spec>;
#[doc = "DSI Host timeout counter configuration register 3"]
pub mod dsi_tccr3;
#[doc = "DSI_TCCR4 (rw) register accessor: DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr4`] module"]
#[doc(alias = "DSI_TCCR4")]
pub type DsiTccr4 = crate::Reg<dsi_tccr4::DsiTccr4Spec>;
#[doc = "DSI Host timeout counter configuration register 4"]
pub mod dsi_tccr4;
#[doc = "DSI_TCCR5 (rw) register accessor: DSI Host timeout counter configuration register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_tccr5`] module"]
#[doc(alias = "DSI_TCCR5")]
pub type DsiTccr5 = crate::Reg<dsi_tccr5::DsiTccr5Spec>;
#[doc = "DSI Host timeout counter configuration register 5"]
pub mod dsi_tccr5;
#[doc = "DSI_CLCR (rw) register accessor: DSI Host clock lane configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_clcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_clcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_clcr`] module"]
#[doc(alias = "DSI_CLCR")]
pub type DsiClcr = crate::Reg<dsi_clcr::DsiClcrSpec>;
#[doc = "DSI Host clock lane configuration register"]
pub mod dsi_clcr;
#[doc = "DSI_CLTCR (rw) register accessor: DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_cltcr`] module"]
#[doc(alias = "DSI_CLTCR")]
pub type DsiCltcr = crate::Reg<dsi_cltcr::DsiCltcrSpec>;
#[doc = "DSI Host clock lane timer configuration register"]
pub mod dsi_cltcr;
#[doc = "DSI_DLTCR (rw) register accessor: DSI Host data lane timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_dltcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dltcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_dltcr`] module"]
#[doc(alias = "DSI_DLTCR")]
pub type DsiDltcr = crate::Reg<dsi_dltcr::DsiDltcrSpec>;
#[doc = "DSI Host data lane timer configuration register"]
pub mod dsi_dltcr;
#[doc = "DSI_PCTLR (rw) register accessor: DSI Host PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pctlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pctlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pctlr`] module"]
#[doc(alias = "DSI_PCTLR")]
pub type DsiPctlr = crate::Reg<dsi_pctlr::DsiPctlrSpec>;
#[doc = "DSI Host PHY control register"]
pub mod dsi_pctlr;
#[doc = "DSI_PCONFR (rw) register accessor: DSI Host PHY configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pconfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pconfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pconfr`] module"]
#[doc(alias = "DSI_PCONFR")]
pub type DsiPconfr = crate::Reg<dsi_pconfr::DsiPconfrSpec>;
#[doc = "DSI Host PHY configuration register"]
pub mod dsi_pconfr;
#[doc = "DSI_PUCR (rw) register accessor: DSI Host PHY ULPS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pucr`] module"]
#[doc(alias = "DSI_PUCR")]
pub type DsiPucr = crate::Reg<dsi_pucr::DsiPucrSpec>;
#[doc = "DSI Host PHY ULPS control register"]
pub mod dsi_pucr;
#[doc = "DSI_PTTCR (rw) register accessor: DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pttcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pttcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_pttcr`] module"]
#[doc(alias = "DSI_PTTCR")]
pub type DsiPttcr = crate::Reg<dsi_pttcr::DsiPttcrSpec>;
#[doc = "DSI Host PHY TX triggers configuration register"]
pub mod dsi_pttcr;
#[doc = "DSI_PSR (r) register accessor: DSI Host PHY status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_psr`] module"]
#[doc(alias = "DSI_PSR")]
pub type DsiPsr = crate::Reg<dsi_psr::DsiPsrSpec>;
#[doc = "DSI Host PHY status register"]
pub mod dsi_psr;
#[doc = "DSI_ISR0 (r) register accessor: DSI Host interrupt and status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_isr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_isr0`] module"]
#[doc(alias = "DSI_ISR0")]
pub type DsiIsr0 = crate::Reg<dsi_isr0::DsiIsr0Spec>;
#[doc = "DSI Host interrupt and status register 0"]
pub mod dsi_isr0;
#[doc = "DSI_ISR1 (r) register accessor: DSI Host interrupt and status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_isr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_isr1`] module"]
#[doc(alias = "DSI_ISR1")]
pub type DsiIsr1 = crate::Reg<dsi_isr1::DsiIsr1Spec>;
#[doc = "DSI Host interrupt and status register 1"]
pub mod dsi_isr1;
#[doc = "DSI_IER0 (rw) register accessor: DSI Host interrupt enable register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ier0`] module"]
#[doc(alias = "DSI_IER0")]
pub type DsiIer0 = crate::Reg<dsi_ier0::DsiIer0Spec>;
#[doc = "DSI Host interrupt enable register 0"]
pub mod dsi_ier0;
#[doc = "DSI_IER1 (rw) register accessor: DSI Host interrupt enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_ier1`] module"]
#[doc(alias = "DSI_IER1")]
pub type DsiIer1 = crate::Reg<dsi_ier1::DsiIer1Spec>;
#[doc = "DSI Host interrupt enable register 1"]
pub mod dsi_ier1;
#[doc = "DSI_FIR0 (w) register accessor: DSI Host force interrupt register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fir0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fir0`] module"]
#[doc(alias = "DSI_FIR0")]
pub type DsiFir0 = crate::Reg<dsi_fir0::DsiFir0Spec>;
#[doc = "DSI Host force interrupt register 0"]
pub mod dsi_fir0;
#[doc = "DSI_FIR1 (w) register accessor: DSI Host force interrupt register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fir1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_fir1`] module"]
#[doc(alias = "DSI_FIR1")]
pub type DsiFir1 = crate::Reg<dsi_fir1::DsiFir1Spec>;
#[doc = "DSI Host force interrupt register 1"]
pub mod dsi_fir1;
#[doc = "DSI_VSCR (rw) register accessor: DSI Host video shadow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vscr`] module"]
#[doc(alias = "DSI_VSCR")]
pub type DsiVscr = crate::Reg<dsi_vscr::DsiVscrSpec>;
#[doc = "DSI Host video shadow control register"]
pub mod dsi_vscr;
#[doc = "DSI_LCVCIDR (rw) register accessor: DSI Host LTDC current VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcvcidr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcvcidr`] module"]
#[doc(alias = "DSI_LCVCIDR")]
pub type DsiLcvcidr = crate::Reg<dsi_lcvcidr::DsiLcvcidrSpec>;
#[doc = "DSI Host LTDC current VCID register"]
pub mod dsi_lcvcidr;
#[doc = "DSI_LCCCR (r) register accessor: DSI Host LTDC current color coding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lcccr`] module"]
#[doc(alias = "DSI_LCCCR")]
pub type DsiLcccr = crate::Reg<dsi_lcccr::DsiLcccrSpec>;
#[doc = "DSI Host LTDC current color coding register"]
pub mod dsi_lcccr;
#[doc = "DSI_LPMCCR (r) register accessor: DSI Host low-power mode current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_lpmccr`] module"]
#[doc(alias = "DSI_LPMCCR")]
pub type DsiLpmccr = crate::Reg<dsi_lpmccr::DsiLpmccrSpec>;
#[doc = "DSI Host low-power mode current configuration register"]
pub mod dsi_lpmccr;
#[doc = "DSI_VMCCR (r) register accessor: DSI Host video mode current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vmccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vmccr`] module"]
#[doc(alias = "DSI_VMCCR")]
pub type DsiVmccr = crate::Reg<dsi_vmccr::DsiVmccrSpec>;
#[doc = "DSI Host video mode current configuration register"]
pub mod dsi_vmccr;
#[doc = "DSI_VPCCR (r) register accessor: DSI Host video packet current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vpccr`] module"]
#[doc(alias = "DSI_VPCCR")]
pub type DsiVpccr = crate::Reg<dsi_vpccr::DsiVpccrSpec>;
#[doc = "DSI Host video packet current configuration register"]
pub mod dsi_vpccr;
#[doc = "DSI_VCCCR (r) register accessor: DSI Host video chunks current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vcccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vcccr`] module"]
#[doc(alias = "DSI_VCCCR")]
pub type DsiVcccr = crate::Reg<dsi_vcccr::DsiVcccrSpec>;
#[doc = "DSI Host video chunks current configuration register"]
pub mod dsi_vcccr;
#[doc = "DSI_VNPCCR (r) register accessor: DSI Host video null packet current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vnpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vnpccr`] module"]
#[doc(alias = "DSI_VNPCCR")]
pub type DsiVnpccr = crate::Reg<dsi_vnpccr::DsiVnpccrSpec>;
#[doc = "DSI Host video null packet current configuration register"]
pub mod dsi_vnpccr;
#[doc = "DSI_VHSACCR (r) register accessor: DSI Host video HSA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhsaccr`] module"]
#[doc(alias = "DSI_VHSACCR")]
pub type DsiVhsaccr = crate::Reg<dsi_vhsaccr::DsiVhsaccrSpec>;
#[doc = "DSI Host video HSA current configuration register"]
pub mod dsi_vhsaccr;
#[doc = "DSI_VHBPCCR (r) register accessor: DSI Host video HBP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vhbpccr`] module"]
#[doc(alias = "DSI_VHBPCCR")]
pub type DsiVhbpccr = crate::Reg<dsi_vhbpccr::DsiVhbpccrSpec>;
#[doc = "DSI Host video HBP current configuration register"]
pub mod dsi_vhbpccr;
#[doc = "DSI_VLCCR (r) register accessor: DSI Host video line current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vlccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vlccr`] module"]
#[doc(alias = "DSI_VLCCR")]
pub type DsiVlccr = crate::Reg<dsi_vlccr::DsiVlccrSpec>;
#[doc = "DSI Host video line current configuration register"]
pub mod dsi_vlccr;
#[doc = "DSI_VVSACCR (r) register accessor: DSI Host video VSA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvsaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvsaccr`] module"]
#[doc(alias = "DSI_VVSACCR")]
pub type DsiVvsaccr = crate::Reg<dsi_vvsaccr::DsiVvsaccrSpec>;
#[doc = "DSI Host video VSA current configuration register"]
pub mod dsi_vvsaccr;
#[doc = "DSI_VVBPCCR (r) register accessor: DSI Host video VBP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvbpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvbpccr`] module"]
#[doc(alias = "DSI_VVBPCCR")]
pub type DsiVvbpccr = crate::Reg<dsi_vvbpccr::DsiVvbpccrSpec>;
#[doc = "DSI Host video VBP current configuration register"]
pub mod dsi_vvbpccr;
#[doc = "DSI_VVFPCCR (r) register accessor: DSI Host video VFP current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvfpccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvfpccr`] module"]
#[doc(alias = "DSI_VVFPCCR")]
pub type DsiVvfpccr = crate::Reg<dsi_vvfpccr::DsiVvfpccrSpec>;
#[doc = "DSI Host video VFP current configuration register"]
pub mod dsi_vvfpccr;
#[doc = "DSI_VVACCR (r) register accessor: DSI Host video VA current configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvaccr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_vvaccr`] module"]
#[doc(alias = "DSI_VVACCR")]
pub type DsiVvaccr = crate::Reg<dsi_vvaccr::DsiVvaccrSpec>;
#[doc = "DSI Host video VA current configuration register"]
pub mod dsi_vvaccr;
#[doc = "DSI_WCFGR (rw) register accessor: DSI wrapper configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wcfgr`] module"]
#[doc(alias = "DSI_WCFGR")]
pub type DsiWcfgr = crate::Reg<dsi_wcfgr::DsiWcfgrSpec>;
#[doc = "DSI wrapper configuration register"]
pub mod dsi_wcfgr;
#[doc = "DSI_WCR (rw) register accessor: DSI wrapper control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wcr`] module"]
#[doc(alias = "DSI_WCR")]
pub type DsiWcr = crate::Reg<dsi_wcr::DsiWcrSpec>;
#[doc = "DSI wrapper control register"]
pub mod dsi_wcr;
#[doc = "DSI_WIER (rw) register accessor: DSI wrapper interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wier`] module"]
#[doc(alias = "DSI_WIER")]
pub type DsiWier = crate::Reg<dsi_wier::DsiWierSpec>;
#[doc = "DSI wrapper interrupt enable register"]
pub mod dsi_wier;
#[doc = "DSI_WISR (r) register accessor: DSI wrapper interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wisr`] module"]
#[doc(alias = "DSI_WISR")]
pub type DsiWisr = crate::Reg<dsi_wisr::DsiWisrSpec>;
#[doc = "DSI wrapper interrupt and status register"]
pub mod dsi_wisr;
#[doc = "DSI_WIFCR (w) register accessor: DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wifcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wifcr`] module"]
#[doc(alias = "DSI_WIFCR")]
pub type DsiWifcr = crate::Reg<dsi_wifcr::DsiWifcrSpec>;
#[doc = "DSI wrapper interrupt flag clear register"]
pub mod dsi_wifcr;
#[doc = "DSI_WPCR0 (rw) register accessor: DSI wrapper PHY configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr0`] module"]
#[doc(alias = "DSI_WPCR0")]
pub type DsiWpcr0 = crate::Reg<dsi_wpcr0::DsiWpcr0Spec>;
#[doc = "DSI wrapper PHY configuration register 0"]
pub mod dsi_wpcr0;
#[doc = "DSI_WPCR1 (rw) register accessor: This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr1`] module"]
#[doc(alias = "DSI_WPCR1")]
pub type DsiWpcr1 = crate::Reg<dsi_wpcr1::DsiWpcr1Spec>;
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0)."]
pub mod dsi_wpcr1;
#[doc = "DSI_WPCR2 (rw) register accessor: DSI wrapper PHY configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr2`] module"]
#[doc(alias = "DSI_WPCR2")]
pub type DsiWpcr2 = crate::Reg<dsi_wpcr2::DsiWpcr2Spec>;
#[doc = "DSI wrapper PHY configuration register 2"]
pub mod dsi_wpcr2;
#[doc = "DSI_WPCR3 (rw) register accessor: DSI wrapper PHY configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr3`] module"]
#[doc(alias = "DSI_WPCR3")]
pub type DsiWpcr3 = crate::Reg<dsi_wpcr3::DsiWpcr3Spec>;
#[doc = "DSI wrapper PHY configuration register 3"]
pub mod dsi_wpcr3;
#[doc = "DSI_WPCR4 (rw) register accessor: DSI wrapper PHY configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wpcr4`] module"]
#[doc(alias = "DSI_WPCR4")]
pub type DsiWpcr4 = crate::Reg<dsi_wpcr4::DsiWpcr4Spec>;
#[doc = "DSI wrapper PHY configuration register 4"]
pub mod dsi_wpcr4;
#[doc = "DSI_WRPCR (rw) register accessor: DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wrpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wrpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dsi_wrpcr`] module"]
#[doc(alias = "DSI_WRPCR")]
pub type DsiWrpcr = crate::Reg<dsi_wrpcr::DsiWrpcrSpec>;
#[doc = "DSI wrapper regulator and PLL control register"]
pub mod dsi_wrpcr;
