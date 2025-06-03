#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    crel: Crel,
    ccfg: Ccfg,
    cstat: Cstat,
    cwd: Cwd,
    ir: Ir,
    ie: Ie,
}
impl RegisterBlock {
    #[doc = "0x00 - Clock Calibration Unit Core Release Register"]
    #[inline(always)]
    pub const fn crel(&self) -> &Crel {
        &self.crel
    }
    #[doc = "0x04 - Calibration Configuration Register"]
    #[inline(always)]
    pub const fn ccfg(&self) -> &Ccfg {
        &self.ccfg
    }
    #[doc = "0x08 - Calibration Status Register"]
    #[inline(always)]
    pub const fn cstat(&self) -> &Cstat {
        &self.cstat
    }
    #[doc = "0x0c - Calibration Watchdog Register"]
    #[inline(always)]
    pub const fn cwd(&self) -> &Cwd {
        &self.cwd
    }
    #[doc = "0x10 - Clock Calibration Unit Interrupt Register"]
    #[inline(always)]
    pub const fn ir(&self) -> &Ir {
        &self.ir
    }
    #[doc = "0x14 - Clock Calibration Unit Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ie(&self) -> &Ie {
        &self.ie
    }
}
#[doc = "CREL (rw) register accessor: Clock Calibration Unit Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@crel`] module"]
#[doc(alias = "CREL")]
pub type Crel = crate::Reg<crel::CrelSpec>;
#[doc = "Clock Calibration Unit Core Release Register"]
pub mod crel;
#[doc = "CCFG (rw) register accessor: Calibration Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg`] module"]
#[doc(alias = "CCFG")]
pub type Ccfg = crate::Reg<ccfg::CcfgSpec>;
#[doc = "Calibration Configuration Register"]
pub mod ccfg;
#[doc = "CSTAT (rw) register accessor: Calibration Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cstat`] module"]
#[doc(alias = "CSTAT")]
pub type Cstat = crate::Reg<cstat::CstatSpec>;
#[doc = "Calibration Status Register"]
pub mod cstat;
#[doc = "CWD (rw) register accessor: Calibration Watchdog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cwd`] module"]
#[doc(alias = "CWD")]
pub type Cwd = crate::Reg<cwd::CwdSpec>;
#[doc = "Calibration Watchdog Register"]
pub mod cwd;
#[doc = "IR (rw) register accessor: Clock Calibration Unit Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ir`] module"]
#[doc(alias = "IR")]
pub type Ir = crate::Reg<ir::IrSpec>;
#[doc = "Clock Calibration Unit Interrupt Register"]
pub mod ir;
#[doc = "IE (rw) register accessor: Clock Calibration Unit Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ie`] module"]
#[doc(alias = "IE")]
pub type Ie = crate::Reg<ie::IeSpec>;
#[doc = "Clock Calibration Unit Interrupt Enable Register"]
pub mod ie;
