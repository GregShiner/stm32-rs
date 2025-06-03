#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    hsicfgr: Hsicfgr,
    crrcr: Crrcr,
    _reserved3: [u8; 0x04],
    cfgr: Cfgr,
    _reserved4: [u8; 0x04],
    d1cfgr: D1cfgr,
    d2cfgr: D2cfgr,
    d3cfgr: D3cfgr,
    _reserved7: [u8; 0x04],
    pllckselr: Pllckselr,
    pllcfgr: Pllcfgr,
    pll1divr: Pll1divr,
    pll1fracr: Pll1fracr,
    pll2divr: Pll2divr,
    pll2fracr: Pll2fracr,
    pll3divr: Pll3divr,
    pll3fracr: Pll3fracr,
    _reserved15: [u8; 0x04],
    d1ccipr: D1ccipr,
    d2ccip1r: D2ccip1r,
    d2ccip2r: D2ccip2r,
    d3ccipr: D3ccipr,
    _reserved19: [u8; 0x04],
    cier: Cier,
    cifr: Cifr,
    cicr: Cicr,
    _reserved22: [u8; 0x04],
    bdcr: Bdcr,
    csr: Csr,
    _reserved24: [u8; 0x04],
    ahb3rstr: Ahb3rstr,
    ahb1rstr: Ahb1rstr,
    ahb2rstr: Ahb2rstr,
    ahb4rstr: Ahb4rstr,
    apb3rstr: Apb3rstr,
    apb1lrstr: Apb1lrstr,
    apb1hrstr: Apb1hrstr,
    apb2rstr: Apb2rstr,
    apb4rstr: Apb4rstr,
    gcr: Gcr,
    _reserved34: [u8; 0x04],
    d3amr: D3amr,
    _reserved35: [u8; 0x24],
    rsr: Rsr,
    ahb3enr: Ahb3enr,
    ahb1enr: Ahb1enr,
    ahb2enr: Ahb2enr,
    ahb4enr: Ahb4enr,
    apb3enr: Apb3enr,
    apb1lenr: Apb1lenr,
    apb1henr: Apb1henr,
    apb2enr: Apb2enr,
    apb4enr: Apb4enr,
    _reserved45: [u8; 0x04],
    ahb3lpenr: Ahb3lpenr,
    ahb1lpenr: Ahb1lpenr,
    ahb2lpenr: Ahb2lpenr,
    ahb4lpenr: Ahb4lpenr,
    apb3lpenr: Apb3lpenr,
    apb1llpenr: Apb1llpenr,
    apb1hlpenr: Apb1hlpenr,
    apb2lpenr: Apb2lpenr,
    apb4lpenr: Apb4lpenr,
    _reserved54: [u8; 0x10],
    c1_rsr: C1Rsr,
    c1_ahb3enr: C1Ahb3enr,
    c1_ahb1enr: C1Ahb1enr,
    c1_ahb2enr: C1Ahb2enr,
    c1_ahb4enr: C1Ahb4enr,
    c1_apb3enr: C1Apb3enr,
    c1_apb1lenr: C1Apb1lenr,
    c1_apb1henr: C1Apb1henr,
    c1_apb2enr: C1Apb2enr,
    c1_apb4enr: C1Apb4enr,
    _reserved64: [u8; 0x04],
    c1_ahb3lpenr: C1Ahb3lpenr,
    c1_ahb1lpenr: C1Ahb1lpenr,
    c1_ahb2lpenr: C1Ahb2lpenr,
    c1_ahb4lpenr: C1Ahb4lpenr,
    c1_apb3lpenr: C1Apb3lpenr,
    c1_apb1llpenr: C1Apb1llpenr,
    c1_apb1hlpenr: C1Apb1hlpenr,
    c1_apb2lpenr: C1Apb2lpenr,
    c1_apb4lpenr: C1Apb4lpenr,
}
impl RegisterBlock {
    #[doc = "0x00 - clock control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - RCC HSI configuration register"]
    #[inline(always)]
    pub const fn hsicfgr(&self) -> &Hsicfgr {
        &self.hsicfgr
    }
    #[doc = "0x08 - RCC Clock Recovery RC Register"]
    #[inline(always)]
    pub const fn crrcr(&self) -> &Crrcr {
        &self.crrcr
    }
    #[doc = "0x10 - RCC Clock Configuration Register"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x18 - RCC Domain 1 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1cfgr(&self) -> &D1cfgr {
        &self.d1cfgr
    }
    #[doc = "0x1c - RCC Domain 2 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2cfgr(&self) -> &D2cfgr {
        &self.d2cfgr
    }
    #[doc = "0x20 - RCC Domain 3 Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3cfgr(&self) -> &D3cfgr {
        &self.d3cfgr
    }
    #[doc = "0x28 - RCC PLLs Clock Source Selection Register"]
    #[inline(always)]
    pub const fn pllckselr(&self) -> &Pllckselr {
        &self.pllckselr
    }
    #[doc = "0x2c - RCC PLLs Configuration Register"]
    #[inline(always)]
    pub const fn pllcfgr(&self) -> &Pllcfgr {
        &self.pllcfgr
    }
    #[doc = "0x30 - RCC PLL1 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll1divr(&self) -> &Pll1divr {
        &self.pll1divr
    }
    #[doc = "0x34 - RCC PLL1 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll1fracr(&self) -> &Pll1fracr {
        &self.pll1fracr
    }
    #[doc = "0x38 - RCC PLL2 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll2divr(&self) -> &Pll2divr {
        &self.pll2divr
    }
    #[doc = "0x3c - RCC PLL2 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll2fracr(&self) -> &Pll2fracr {
        &self.pll2fracr
    }
    #[doc = "0x40 - RCC PLL3 Dividers Configuration Register"]
    #[inline(always)]
    pub const fn pll3divr(&self) -> &Pll3divr {
        &self.pll3divr
    }
    #[doc = "0x44 - RCC PLL3 Fractional Divider Register"]
    #[inline(always)]
    pub const fn pll3fracr(&self) -> &Pll3fracr {
        &self.pll3fracr
    }
    #[doc = "0x4c - RCC Domain 1 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d1ccipr(&self) -> &D1ccipr {
        &self.d1ccipr
    }
    #[doc = "0x50 - RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip1r(&self) -> &D2ccip1r {
        &self.d2ccip1r
    }
    #[doc = "0x54 - RCC Domain 2 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d2ccip2r(&self) -> &D2ccip2r {
        &self.d2ccip2r
    }
    #[doc = "0x58 - RCC Domain 3 Kernel Clock Configuration Register"]
    #[inline(always)]
    pub const fn d3ccipr(&self) -> &D3ccipr {
        &self.d3ccipr
    }
    #[doc = "0x60 - RCC Clock Source Interrupt Enable Register"]
    #[inline(always)]
    pub const fn cier(&self) -> &Cier {
        &self.cier
    }
    #[doc = "0x64 - RCC Clock Source Interrupt Flag Register"]
    #[inline(always)]
    pub const fn cifr(&self) -> &Cifr {
        &self.cifr
    }
    #[doc = "0x68 - RCC Clock Source Interrupt Clear Register"]
    #[inline(always)]
    pub const fn cicr(&self) -> &Cicr {
        &self.cicr
    }
    #[doc = "0x70 - RCC Backup Domain Control Register"]
    #[inline(always)]
    pub const fn bdcr(&self) -> &Bdcr {
        &self.bdcr
    }
    #[doc = "0x74 - RCC Clock Control and Status Register"]
    #[inline(always)]
    pub const fn csr(&self) -> &Csr {
        &self.csr
    }
    #[doc = "0x7c - RCC AHB3 Reset Register"]
    #[inline(always)]
    pub const fn ahb3rstr(&self) -> &Ahb3rstr {
        &self.ahb3rstr
    }
    #[doc = "0x80 - RCC AHB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb1rstr(&self) -> &Ahb1rstr {
        &self.ahb1rstr
    }
    #[doc = "0x84 - RCC AHB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb2rstr(&self) -> &Ahb2rstr {
        &self.ahb2rstr
    }
    #[doc = "0x88 - RCC AHB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn ahb4rstr(&self) -> &Ahb4rstr {
        &self.ahb4rstr
    }
    #[doc = "0x8c - RCC APB3 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb3rstr(&self) -> &Apb3rstr {
        &self.apb3rstr
    }
    #[doc = "0x90 - RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1lrstr(&self) -> &Apb1lrstr {
        &self.apb1lrstr
    }
    #[doc = "0x94 - RCC APB1 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb1hrstr(&self) -> &Apb1hrstr {
        &self.apb1hrstr
    }
    #[doc = "0x98 - RCC APB2 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb2rstr(&self) -> &Apb2rstr {
        &self.apb2rstr
    }
    #[doc = "0x9c - RCC APB4 Peripheral Reset Register"]
    #[inline(always)]
    pub const fn apb4rstr(&self) -> &Apb4rstr {
        &self.apb4rstr
    }
    #[doc = "0xa0 - RCC Global Control Register"]
    #[inline(always)]
    pub const fn gcr(&self) -> &Gcr {
        &self.gcr
    }
    #[doc = "0xa8 - RCC D3 Autonomous mode Register"]
    #[inline(always)]
    pub const fn d3amr(&self) -> &D3amr {
        &self.d3amr
    }
    #[doc = "0xd0 - RCC Reset Status Register"]
    #[inline(always)]
    pub const fn rsr(&self) -> &Rsr {
        &self.rsr
    }
    #[doc = "0xd4 - RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn ahb3enr(&self) -> &Ahb3enr {
        &self.ahb3enr
    }
    #[doc = "0xd8 - RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn ahb1enr(&self) -> &Ahb1enr {
        &self.ahb1enr
    }
    #[doc = "0xdc - RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn ahb2enr(&self) -> &Ahb2enr {
        &self.ahb2enr
    }
    #[doc = "0xe0 - RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn ahb4enr(&self) -> &Ahb4enr {
        &self.ahb4enr
    }
    #[doc = "0xe4 - RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn apb3enr(&self) -> &Apb3enr {
        &self.apb3enr
    }
    #[doc = "0xe8 - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1lenr(&self) -> &Apb1lenr {
        &self.apb1lenr
    }
    #[doc = "0xec - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn apb1henr(&self) -> &Apb1henr {
        &self.apb1henr
    }
    #[doc = "0xf0 - RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn apb2enr(&self) -> &Apb2enr {
        &self.apb2enr
    }
    #[doc = "0xf4 - RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn apb4enr(&self) -> &Apb4enr {
        &self.apb4enr
    }
    #[doc = "0xfc - RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb3lpenr(&self) -> &Ahb3lpenr {
        &self.ahb3lpenr
    }
    #[doc = "0x100 - RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb1lpenr(&self) -> &Ahb1lpenr {
        &self.ahb1lpenr
    }
    #[doc = "0x104 - RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb2lpenr(&self) -> &Ahb2lpenr {
        &self.ahb2lpenr
    }
    #[doc = "0x108 - RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn ahb4lpenr(&self) -> &Ahb4lpenr {
        &self.ahb4lpenr
    }
    #[doc = "0x10c - RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb3lpenr(&self) -> &Apb3lpenr {
        &self.apb3lpenr
    }
    #[doc = "0x110 - RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1llpenr(&self) -> &Apb1llpenr {
        &self.apb1llpenr
    }
    #[doc = "0x114 - RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb1hlpenr(&self) -> &Apb1hlpenr {
        &self.apb1hlpenr
    }
    #[doc = "0x118 - RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb2lpenr(&self) -> &Apb2lpenr {
        &self.apb2lpenr
    }
    #[doc = "0x11c - RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn apb4lpenr(&self) -> &Apb4lpenr {
        &self.apb4lpenr
    }
    #[doc = "0x130 - RCC Reset Status Register"]
    #[inline(always)]
    pub const fn c1_rsr(&self) -> &C1Rsr {
        &self.c1_rsr
    }
    #[doc = "0x134 - RCC AHB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3enr(&self) -> &C1Ahb3enr {
        &self.c1_ahb3enr
    }
    #[doc = "0x138 - RCC AHB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1enr(&self) -> &C1Ahb1enr {
        &self.c1_ahb1enr
    }
    #[doc = "0x13c - RCC AHB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2enr(&self) -> &C1Ahb2enr {
        &self.c1_ahb2enr
    }
    #[doc = "0x140 - RCC AHB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4enr(&self) -> &C1Ahb4enr {
        &self.c1_ahb4enr
    }
    #[doc = "0x144 - RCC APB3 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3enr(&self) -> &C1Apb3enr {
        &self.c1_apb3enr
    }
    #[doc = "0x148 - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1lenr(&self) -> &C1Apb1lenr {
        &self.c1_apb1lenr
    }
    #[doc = "0x14c - RCC APB1 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1henr(&self) -> &C1Apb1henr {
        &self.c1_apb1henr
    }
    #[doc = "0x150 - RCC APB2 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2enr(&self) -> &C1Apb2enr {
        &self.c1_apb2enr
    }
    #[doc = "0x154 - RCC APB4 Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4enr(&self) -> &C1Apb4enr {
        &self.c1_apb4enr
    }
    #[doc = "0x15c - RCC AHB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb3lpenr(&self) -> &C1Ahb3lpenr {
        &self.c1_ahb3lpenr
    }
    #[doc = "0x160 - RCC AHB1 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb1lpenr(&self) -> &C1Ahb1lpenr {
        &self.c1_ahb1lpenr
    }
    #[doc = "0x164 - RCC AHB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb2lpenr(&self) -> &C1Ahb2lpenr {
        &self.c1_ahb2lpenr
    }
    #[doc = "0x168 - RCC AHB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_ahb4lpenr(&self) -> &C1Ahb4lpenr {
        &self.c1_ahb4lpenr
    }
    #[doc = "0x16c - RCC APB3 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb3lpenr(&self) -> &C1Apb3lpenr {
        &self.c1_apb3lpenr
    }
    #[doc = "0x170 - RCC APB1 Low Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1llpenr(&self) -> &C1Apb1llpenr {
        &self.c1_apb1llpenr
    }
    #[doc = "0x174 - RCC APB1 High Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb1hlpenr(&self) -> &C1Apb1hlpenr {
        &self.c1_apb1hlpenr
    }
    #[doc = "0x178 - RCC APB2 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb2lpenr(&self) -> &C1Apb2lpenr {
        &self.c1_apb2lpenr
    }
    #[doc = "0x17c - RCC APB4 Sleep Clock Register"]
    #[inline(always)]
    pub const fn c1_apb4lpenr(&self) -> &C1Apb4lpenr {
        &self.c1_apb4lpenr
    }
}
#[doc = "CR (rw) register accessor: clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "clock control register"]
pub mod cr;
#[doc = "HSICFGR (rw) register accessor: RCC HSI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsicfgr`] module"]
#[doc(alias = "HSICFGR")]
pub type Hsicfgr = crate::Reg<hsicfgr::HsicfgrSpec>;
#[doc = "RCC HSI configuration register"]
pub mod hsicfgr;
#[doc = "CRRCR (r) register accessor: RCC Clock Recovery RC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crrcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crrcr`] module"]
#[doc(alias = "CRRCR")]
pub type Crrcr = crate::Reg<crrcr::CrrcrSpec>;
#[doc = "RCC Clock Recovery RC Register"]
pub mod crrcr;
#[doc = "CFGR (rw) register accessor: RCC Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "RCC Clock Configuration Register"]
pub mod cfgr;
#[doc = "D1CFGR (rw) register accessor: RCC Domain 1 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1cfgr`] module"]
#[doc(alias = "D1CFGR")]
pub type D1cfgr = crate::Reg<d1cfgr::D1cfgrSpec>;
#[doc = "RCC Domain 1 Clock Configuration Register"]
pub mod d1cfgr;
#[doc = "D2CFGR (rw) register accessor: RCC Domain 2 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2cfgr`] module"]
#[doc(alias = "D2CFGR")]
pub type D2cfgr = crate::Reg<d2cfgr::D2cfgrSpec>;
#[doc = "RCC Domain 2 Clock Configuration Register"]
pub mod d2cfgr;
#[doc = "D3CFGR (rw) register accessor: RCC Domain 3 Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3cfgr`] module"]
#[doc(alias = "D3CFGR")]
pub type D3cfgr = crate::Reg<d3cfgr::D3cfgrSpec>;
#[doc = "RCC Domain 3 Clock Configuration Register"]
pub mod d3cfgr;
#[doc = "PLLCKSELR (rw) register accessor: RCC PLLs Clock Source Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllckselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllckselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllckselr`] module"]
#[doc(alias = "PLLCKSELR")]
pub type Pllckselr = crate::Reg<pllckselr::PllckselrSpec>;
#[doc = "RCC PLLs Clock Source Selection Register"]
pub mod pllckselr;
#[doc = "PLLCFGR (rw) register accessor: RCC PLLs Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pllcfgr`] module"]
#[doc(alias = "PLLCFGR")]
pub type Pllcfgr = crate::Reg<pllcfgr::PllcfgrSpec>;
#[doc = "RCC PLLs Configuration Register"]
pub mod pllcfgr;
#[doc = "PLL1DIVR (rw) register accessor: RCC PLL1 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1divr`] module"]
#[doc(alias = "PLL1DIVR")]
pub type Pll1divr = crate::Reg<pll1divr::Pll1divrSpec>;
#[doc = "RCC PLL1 Dividers Configuration Register"]
pub mod pll1divr;
#[doc = "PLL1FRACR (rw) register accessor: RCC PLL1 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll1fracr`] module"]
#[doc(alias = "PLL1FRACR")]
pub type Pll1fracr = crate::Reg<pll1fracr::Pll1fracrSpec>;
#[doc = "RCC PLL1 Fractional Divider Register"]
pub mod pll1fracr;
#[doc = "PLL2DIVR (rw) register accessor: RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2divr`] module"]
#[doc(alias = "PLL2DIVR")]
pub type Pll2divr = crate::Reg<pll2divr::Pll2divrSpec>;
#[doc = "RCC PLL2 Dividers Configuration Register"]
pub mod pll2divr;
#[doc = "PLL2FRACR (rw) register accessor: RCC PLL2 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll2fracr`] module"]
#[doc(alias = "PLL2FRACR")]
pub type Pll2fracr = crate::Reg<pll2fracr::Pll2fracrSpec>;
#[doc = "RCC PLL2 Fractional Divider Register"]
pub mod pll2fracr;
#[doc = "PLL3DIVR (rw) register accessor: RCC PLL3 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll3divr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3divr`] module"]
#[doc(alias = "PLL3DIVR")]
pub type Pll3divr = crate::Reg<pll3divr::Pll3divrSpec>;
#[doc = "RCC PLL3 Dividers Configuration Register"]
pub mod pll3divr;
#[doc = "PLL3FRACR (rw) register accessor: RCC PLL3 Fractional Divider Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll3fracr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3fracr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pll3fracr`] module"]
#[doc(alias = "PLL3FRACR")]
pub type Pll3fracr = crate::Reg<pll3fracr::Pll3fracrSpec>;
#[doc = "RCC PLL3 Fractional Divider Register"]
pub mod pll3fracr;
#[doc = "D1CCIPR (rw) register accessor: RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d1ccipr`] module"]
#[doc(alias = "D1CCIPR")]
pub type D1ccipr = crate::Reg<d1ccipr::D1cciprSpec>;
#[doc = "RCC Domain 1 Kernel Clock Configuration Register"]
pub mod d1ccipr;
#[doc = "D2CCIP1R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2ccip1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2ccip1r`] module"]
#[doc(alias = "D2CCIP1R")]
pub type D2ccip1r = crate::Reg<d2ccip1r::D2ccip1rSpec>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip1r;
#[doc = "D2CCIP2R (rw) register accessor: RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2ccip2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d2ccip2r`] module"]
#[doc(alias = "D2CCIP2R")]
pub type D2ccip2r = crate::Reg<d2ccip2r::D2ccip2rSpec>;
#[doc = "RCC Domain 2 Kernel Clock Configuration Register"]
pub mod d2ccip2r;
#[doc = "D3CCIPR (rw) register accessor: RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3ccipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3ccipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3ccipr`] module"]
#[doc(alias = "D3CCIPR")]
pub type D3ccipr = crate::Reg<d3ccipr::D3cciprSpec>;
#[doc = "RCC Domain 3 Kernel Clock Configuration Register"]
pub mod d3ccipr;
#[doc = "CIER (rw) register accessor: RCC Clock Source Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cier`] module"]
#[doc(alias = "CIER")]
pub type Cier = crate::Reg<cier::CierSpec>;
#[doc = "RCC Clock Source Interrupt Enable Register"]
pub mod cier;
#[doc = "CIFR (rw) register accessor: RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cifr`] module"]
#[doc(alias = "CIFR")]
pub type Cifr = crate::Reg<cifr::CifrSpec>;
#[doc = "RCC Clock Source Interrupt Flag Register"]
pub mod cifr;
#[doc = "CICR (rw) register accessor: RCC Clock Source Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cicr`] module"]
#[doc(alias = "CICR")]
pub type Cicr = crate::Reg<cicr::CicrSpec>;
#[doc = "RCC Clock Source Interrupt Clear Register"]
pub mod cicr;
#[doc = "BDCR (rw) register accessor: RCC Backup Domain Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`bdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bdcr`] module"]
#[doc(alias = "BDCR")]
pub type Bdcr = crate::Reg<bdcr::BdcrSpec>;
#[doc = "RCC Backup Domain Control Register"]
pub mod bdcr;
#[doc = "CSR (rw) register accessor: RCC Clock Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`] module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "RCC Clock Control and Status Register"]
pub mod csr;
#[doc = "AHB3RSTR (rw) register accessor: RCC AHB3 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3rstr`] module"]
#[doc(alias = "AHB3RSTR")]
pub type Ahb3rstr = crate::Reg<ahb3rstr::Ahb3rstrSpec>;
#[doc = "RCC AHB3 Reset Register"]
pub mod ahb3rstr;
#[doc = "AHB1RSTR (rw) register accessor: RCC AHB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1rstr`] module"]
#[doc(alias = "AHB1RSTR")]
pub type Ahb1rstr = crate::Reg<ahb1rstr::Ahb1rstrSpec>;
#[doc = "RCC AHB1 Peripheral Reset Register"]
pub mod ahb1rstr;
#[doc = "AHB2RSTR (rw) register accessor: RCC AHB2 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2rstr`] module"]
#[doc(alias = "AHB2RSTR")]
pub type Ahb2rstr = crate::Reg<ahb2rstr::Ahb2rstrSpec>;
#[doc = "RCC AHB2 Peripheral Reset Register"]
pub mod ahb2rstr;
#[doc = "AHB4RSTR (rw) register accessor: RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4rstr`] module"]
#[doc(alias = "AHB4RSTR")]
pub type Ahb4rstr = crate::Reg<ahb4rstr::Ahb4rstrSpec>;
#[doc = "RCC AHB4 Peripheral Reset Register"]
pub mod ahb4rstr;
#[doc = "APB3RSTR (rw) register accessor: RCC APB3 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3rstr`] module"]
#[doc(alias = "APB3RSTR")]
pub type Apb3rstr = crate::Reg<apb3rstr::Apb3rstrSpec>;
#[doc = "RCC APB3 Peripheral Reset Register"]
pub mod apb3rstr;
#[doc = "APB1LRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lrstr`] module"]
#[doc(alias = "APB1LRSTR")]
pub type Apb1lrstr = crate::Reg<apb1lrstr::Apb1lrstrSpec>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1lrstr;
#[doc = "APB1HRSTR (rw) register accessor: RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hrstr`] module"]
#[doc(alias = "APB1HRSTR")]
pub type Apb1hrstr = crate::Reg<apb1hrstr::Apb1hrstrSpec>;
#[doc = "RCC APB1 Peripheral Reset Register"]
pub mod apb1hrstr;
#[doc = "APB2RSTR (rw) register accessor: RCC APB2 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2rstr`] module"]
#[doc(alias = "APB2RSTR")]
pub type Apb2rstr = crate::Reg<apb2rstr::Apb2rstrSpec>;
#[doc = "RCC APB2 Peripheral Reset Register"]
pub mod apb2rstr;
#[doc = "APB4RSTR (rw) register accessor: RCC APB4 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4rstr`] module"]
#[doc(alias = "APB4RSTR")]
pub type Apb4rstr = crate::Reg<apb4rstr::Apb4rstrSpec>;
#[doc = "RCC APB4 Peripheral Reset Register"]
pub mod apb4rstr;
#[doc = "GCR (rw) register accessor: RCC Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@gcr`] module"]
#[doc(alias = "GCR")]
pub type Gcr = crate::Reg<gcr::GcrSpec>;
#[doc = "RCC Global Control Register"]
pub mod gcr;
#[doc = "D3AMR (rw) register accessor: RCC D3 Autonomous mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3amr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3amr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3amr`] module"]
#[doc(alias = "D3AMR")]
pub type D3amr = crate::Reg<d3amr::D3amrSpec>;
#[doc = "RCC D3 Autonomous mode Register"]
pub mod d3amr;
#[doc = "RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rsr`] module"]
#[doc(alias = "RSR")]
pub type Rsr = crate::Reg<rsr::RsrSpec>;
#[doc = "RCC Reset Status Register"]
pub mod rsr;
#[doc = "C1_RSR (rw) register accessor: RCC Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_rsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_rsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_rsr`] module"]
#[doc(alias = "C1_RSR")]
pub type C1Rsr = crate::Reg<c1_rsr::C1RsrSpec>;
#[doc = "RCC Reset Status Register"]
pub mod c1_rsr;
#[doc = "C1_AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb3enr`] module"]
#[doc(alias = "C1_AHB3ENR")]
pub type C1Ahb3enr = crate::Reg<c1_ahb3enr::C1Ahb3enrSpec>;
#[doc = "RCC AHB3 Clock Register"]
pub mod c1_ahb3enr;
#[doc = "AHB3ENR (rw) register accessor: RCC AHB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3enr`] module"]
#[doc(alias = "AHB3ENR")]
pub type Ahb3enr = crate::Reg<ahb3enr::Ahb3enrSpec>;
#[doc = "RCC AHB3 Clock Register"]
pub mod ahb3enr;
#[doc = "AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1enr`] module"]
#[doc(alias = "AHB1ENR")]
pub type Ahb1enr = crate::Reg<ahb1enr::Ahb1enrSpec>;
#[doc = "RCC AHB1 Clock Register"]
pub mod ahb1enr;
#[doc = "C1_AHB1ENR (rw) register accessor: RCC AHB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb1enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb1enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb1enr`] module"]
#[doc(alias = "C1_AHB1ENR")]
pub type C1Ahb1enr = crate::Reg<c1_ahb1enr::C1Ahb1enrSpec>;
#[doc = "RCC AHB1 Clock Register"]
pub mod c1_ahb1enr;
#[doc = "C1_AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb2enr`] module"]
#[doc(alias = "C1_AHB2ENR")]
pub type C1Ahb2enr = crate::Reg<c1_ahb2enr::C1Ahb2enrSpec>;
#[doc = "RCC AHB2 Clock Register"]
pub mod c1_ahb2enr;
#[doc = "AHB2ENR (rw) register accessor: RCC AHB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2enr`] module"]
#[doc(alias = "AHB2ENR")]
pub type Ahb2enr = crate::Reg<ahb2enr::Ahb2enrSpec>;
#[doc = "RCC AHB2 Clock Register"]
pub mod ahb2enr;
#[doc = "AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4enr`] module"]
#[doc(alias = "AHB4ENR")]
pub type Ahb4enr = crate::Reg<ahb4enr::Ahb4enrSpec>;
#[doc = "RCC AHB4 Clock Register"]
pub mod ahb4enr;
#[doc = "C1_AHB4ENR (rw) register accessor: RCC AHB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb4enr`] module"]
#[doc(alias = "C1_AHB4ENR")]
pub type C1Ahb4enr = crate::Reg<c1_ahb4enr::C1Ahb4enrSpec>;
#[doc = "RCC AHB4 Clock Register"]
pub mod c1_ahb4enr;
#[doc = "C1_APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb3enr`] module"]
#[doc(alias = "C1_APB3ENR")]
pub type C1Apb3enr = crate::Reg<c1_apb3enr::C1Apb3enrSpec>;
#[doc = "RCC APB3 Clock Register"]
pub mod c1_apb3enr;
#[doc = "APB3ENR (rw) register accessor: RCC APB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3enr`] module"]
#[doc(alias = "APB3ENR")]
pub type Apb3enr = crate::Reg<apb3enr::Apb3enrSpec>;
#[doc = "RCC APB3 Clock Register"]
pub mod apb3enr;
#[doc = "APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1lenr`] module"]
#[doc(alias = "APB1LENR")]
pub type Apb1lenr = crate::Reg<apb1lenr::Apb1lenrSpec>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1lenr;
#[doc = "C1_APB1LENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb1lenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb1lenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1lenr`] module"]
#[doc(alias = "C1_APB1LENR")]
pub type C1Apb1lenr = crate::Reg<c1_apb1lenr::C1Apb1lenrSpec>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1lenr;
#[doc = "APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1henr`] module"]
#[doc(alias = "APB1HENR")]
pub type Apb1henr = crate::Reg<apb1henr::Apb1henrSpec>;
#[doc = "RCC APB1 Clock Register"]
pub mod apb1henr;
#[doc = "C1_APB1HENR (rw) register accessor: RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb1henr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb1henr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1henr`] module"]
#[doc(alias = "C1_APB1HENR")]
pub type C1Apb1henr = crate::Reg<c1_apb1henr::C1Apb1henrSpec>;
#[doc = "RCC APB1 Clock Register"]
pub mod c1_apb1henr;
#[doc = "C1_APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb2enr`] module"]
#[doc(alias = "C1_APB2ENR")]
pub type C1Apb2enr = crate::Reg<c1_apb2enr::C1Apb2enrSpec>;
#[doc = "RCC APB2 Clock Register"]
pub mod c1_apb2enr;
#[doc = "APB2ENR (rw) register accessor: RCC APB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2enr`] module"]
#[doc(alias = "APB2ENR")]
pub type Apb2enr = crate::Reg<apb2enr::Apb2enrSpec>;
#[doc = "RCC APB2 Clock Register"]
pub mod apb2enr;
#[doc = "APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4enr`] module"]
#[doc(alias = "APB4ENR")]
pub type Apb4enr = crate::Reg<apb4enr::Apb4enrSpec>;
#[doc = "RCC APB4 Clock Register"]
pub mod apb4enr;
#[doc = "C1_APB4ENR (rw) register accessor: RCC APB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb4enr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb4enr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb4enr`] module"]
#[doc(alias = "C1_APB4ENR")]
pub type C1Apb4enr = crate::Reg<c1_apb4enr::C1Apb4enrSpec>;
#[doc = "RCC APB4 Clock Register"]
pub mod c1_apb4enr;
#[doc = "C1_AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb3lpenr`] module"]
#[doc(alias = "C1_AHB3LPENR")]
pub type C1Ahb3lpenr = crate::Reg<c1_ahb3lpenr::C1Ahb3lpenrSpec>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod c1_ahb3lpenr;
#[doc = "AHB3LPENR (rw) register accessor: RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb3lpenr`] module"]
#[doc(alias = "AHB3LPENR")]
pub type Ahb3lpenr = crate::Reg<ahb3lpenr::Ahb3lpenrSpec>;
#[doc = "RCC AHB3 Sleep Clock Register"]
pub mod ahb3lpenr;
#[doc = "AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb1lpenr`] module"]
#[doc(alias = "AHB1LPENR")]
pub type Ahb1lpenr = crate::Reg<ahb1lpenr::Ahb1lpenrSpec>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod ahb1lpenr;
#[doc = "C1_AHB1LPENR (rw) register accessor: RCC AHB1 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb1lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb1lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb1lpenr`] module"]
#[doc(alias = "C1_AHB1LPENR")]
pub type C1Ahb1lpenr = crate::Reg<c1_ahb1lpenr::C1Ahb1lpenrSpec>;
#[doc = "RCC AHB1 Sleep Clock Register"]
pub mod c1_ahb1lpenr;
#[doc = "C1_AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb2lpenr`] module"]
#[doc(alias = "C1_AHB2LPENR")]
pub type C1Ahb2lpenr = crate::Reg<c1_ahb2lpenr::C1Ahb2lpenrSpec>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod c1_ahb2lpenr;
#[doc = "AHB2LPENR (rw) register accessor: RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb2lpenr`] module"]
#[doc(alias = "AHB2LPENR")]
pub type Ahb2lpenr = crate::Reg<ahb2lpenr::Ahb2lpenrSpec>;
#[doc = "RCC AHB2 Sleep Clock Register"]
pub mod ahb2lpenr;
#[doc = "AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ahb4lpenr`] module"]
#[doc(alias = "AHB4LPENR")]
pub type Ahb4lpenr = crate::Reg<ahb4lpenr::Ahb4lpenrSpec>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod ahb4lpenr;
#[doc = "C1_AHB4LPENR (rw) register accessor: RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_ahb4lpenr`] module"]
#[doc(alias = "C1_AHB4LPENR")]
pub type C1Ahb4lpenr = crate::Reg<c1_ahb4lpenr::C1Ahb4lpenrSpec>;
#[doc = "RCC AHB4 Sleep Clock Register"]
pub mod c1_ahb4lpenr;
#[doc = "C1_APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb3lpenr`] module"]
#[doc(alias = "C1_APB3LPENR")]
pub type C1Apb3lpenr = crate::Reg<c1_apb3lpenr::C1Apb3lpenrSpec>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod c1_apb3lpenr;
#[doc = "APB3LPENR (rw) register accessor: RCC APB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb3lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb3lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb3lpenr`] module"]
#[doc(alias = "APB3LPENR")]
pub type Apb3lpenr = crate::Reg<apb3lpenr::Apb3lpenrSpec>;
#[doc = "RCC APB3 Sleep Clock Register"]
pub mod apb3lpenr;
#[doc = "APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1llpenr`] module"]
#[doc(alias = "APB1LLPENR")]
pub type Apb1llpenr = crate::Reg<apb1llpenr::Apb1llpenrSpec>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod apb1llpenr;
#[doc = "C1_APB1LLPENR (rw) register accessor: RCC APB1 Low Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb1llpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb1llpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1llpenr`] module"]
#[doc(alias = "C1_APB1LLPENR")]
pub type C1Apb1llpenr = crate::Reg<c1_apb1llpenr::C1Apb1llpenrSpec>;
#[doc = "RCC APB1 Low Sleep Clock Register"]
pub mod c1_apb1llpenr;
#[doc = "C1_APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb1hlpenr`] module"]
#[doc(alias = "C1_APB1HLPENR")]
pub type C1Apb1hlpenr = crate::Reg<c1_apb1hlpenr::C1Apb1hlpenrSpec>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod c1_apb1hlpenr;
#[doc = "APB1HLPENR (rw) register accessor: RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb1hlpenr`] module"]
#[doc(alias = "APB1HLPENR")]
pub type Apb1hlpenr = crate::Reg<apb1hlpenr::Apb1hlpenrSpec>;
#[doc = "RCC APB1 High Sleep Clock Register"]
pub mod apb1hlpenr;
#[doc = "APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb2lpenr`] module"]
#[doc(alias = "APB2LPENR")]
pub type Apb2lpenr = crate::Reg<apb2lpenr::Apb2lpenrSpec>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod apb2lpenr;
#[doc = "C1_APB2LPENR (rw) register accessor: RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb2lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb2lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb2lpenr`] module"]
#[doc(alias = "C1_APB2LPENR")]
pub type C1Apb2lpenr = crate::Reg<c1_apb2lpenr::C1Apb2lpenrSpec>;
#[doc = "RCC APB2 Sleep Clock Register"]
pub mod c1_apb2lpenr;
#[doc = "C1_APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@c1_apb4lpenr`] module"]
#[doc(alias = "C1_APB4LPENR")]
pub type C1Apb4lpenr = crate::Reg<c1_apb4lpenr::C1Apb4lpenrSpec>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod c1_apb4lpenr;
#[doc = "APB4LPENR (rw) register accessor: RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@apb4lpenr`] module"]
#[doc(alias = "APB4LPENR")]
pub type Apb4lpenr = crate::Reg<apb4lpenr::Apb4lpenrSpec>;
#[doc = "RCC APB4 Sleep Clock Register"]
pub mod apb4lpenr;
