#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    swtrgr: Swtrgr,
    dhr12r1: Dhr12r1,
    dhr12l1: Dhr12l1,
    dhr8r1: Dhr8r1,
    dhr12r2: Dhr12r2,
    dhr12l2: Dhr12l2,
    dhr8r2: Dhr8r2,
    dhr12rd: Dhr12rd,
    dhr12ld: Dhr12ld,
    dhr8rd: Dhr8rd,
    dor1: Dor1,
    dor2: Dor2,
    sr: Sr,
    ccr: Ccr,
    mcr: Mcr,
    shsr1: Shsr1,
    shsr2: Shsr2,
    shhr: Shhr,
    shrr: Shrr,
}
impl RegisterBlock {
    #[doc = "0x00 - DAC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DAC software trigger register"]
    #[inline(always)]
    pub const fn swtrgr(&self) -> &Swtrgr {
        &self.swtrgr
    }
    #[doc = "0x08 - DAC channel1 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r1(&self) -> &Dhr12r1 {
        &self.dhr12r1
    }
    #[doc = "0x0c - DAC channel1 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l1(&self) -> &Dhr12l1 {
        &self.dhr12l1
    }
    #[doc = "0x10 - DAC channel1 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r1(&self) -> &Dhr8r1 {
        &self.dhr8r1
    }
    #[doc = "0x14 - DAC channel2 12-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12r2(&self) -> &Dhr12r2 {
        &self.dhr12r2
    }
    #[doc = "0x18 - DAC channel2 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12l2(&self) -> &Dhr12l2 {
        &self.dhr12l2
    }
    #[doc = "0x1c - DAC channel2 8-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8r2(&self) -> &Dhr8r2 {
        &self.dhr8r2
    }
    #[doc = "0x20 - Dual DAC 12-bit right-aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12rd(&self) -> &Dhr12rd {
        &self.dhr12rd
    }
    #[doc = "0x24 - DUAL DAC 12-bit left aligned data holding register"]
    #[inline(always)]
    pub const fn dhr12ld(&self) -> &Dhr12ld {
        &self.dhr12ld
    }
    #[doc = "0x28 - DUAL DAC 8-bit right aligned data holding register"]
    #[inline(always)]
    pub const fn dhr8rd(&self) -> &Dhr8rd {
        &self.dhr8rd
    }
    #[doc = "0x2c - DAC channel1 data output register"]
    #[inline(always)]
    pub const fn dor1(&self) -> &Dor1 {
        &self.dor1
    }
    #[doc = "0x30 - DAC channel2 data output register"]
    #[inline(always)]
    pub const fn dor2(&self) -> &Dor2 {
        &self.dor2
    }
    #[doc = "0x34 - DAC status register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x38 - DAC calibration control register"]
    #[inline(always)]
    pub const fn ccr(&self) -> &Ccr {
        &self.ccr
    }
    #[doc = "0x3c - DAC mode control register"]
    #[inline(always)]
    pub const fn mcr(&self) -> &Mcr {
        &self.mcr
    }
    #[doc = "0x40 - DAC Sample and Hold sample time register 1"]
    #[inline(always)]
    pub const fn shsr1(&self) -> &Shsr1 {
        &self.shsr1
    }
    #[doc = "0x44 - DAC Sample and Hold sample time register 2"]
    #[inline(always)]
    pub const fn shsr2(&self) -> &Shsr2 {
        &self.shsr2
    }
    #[doc = "0x48 - DAC Sample and Hold hold time register"]
    #[inline(always)]
    pub const fn shhr(&self) -> &Shhr {
        &self.shhr
    }
    #[doc = "0x4c - DAC Sample and Hold refresh time register"]
    #[inline(always)]
    pub const fn shrr(&self) -> &Shrr {
        &self.shrr
    }
}
#[doc = "CR (rw) register accessor: DAC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "DAC control register"]
pub mod cr;
#[doc = "SWTRGR (w) register accessor: DAC software trigger register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swtrgr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swtrgr`] module"]
#[doc(alias = "SWTRGR")]
pub type Swtrgr = crate::Reg<swtrgr::SwtrgrSpec>;
#[doc = "DAC software trigger register"]
pub mod swtrgr;
#[doc = "DHR12R1 (rw) register accessor: DAC channel1 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r1`] module"]
#[doc(alias = "DHR12R1")]
pub type Dhr12r1 = crate::Reg<dhr12r1::Dhr12r1Spec>;
#[doc = "DAC channel1 12-bit right-aligned data holding register"]
pub mod dhr12r1;
#[doc = "DHR12L1 (rw) register accessor: DAC channel1 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l1`] module"]
#[doc(alias = "DHR12L1")]
pub type Dhr12l1 = crate::Reg<dhr12l1::Dhr12l1Spec>;
#[doc = "DAC channel1 12-bit left aligned data holding register"]
pub mod dhr12l1;
#[doc = "DHR8R1 (rw) register accessor: DAC channel1 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r1`] module"]
#[doc(alias = "DHR8R1")]
pub type Dhr8r1 = crate::Reg<dhr8r1::Dhr8r1Spec>;
#[doc = "DAC channel1 8-bit right aligned data holding register"]
pub mod dhr8r1;
#[doc = "DHR12R2 (rw) register accessor: DAC channel2 12-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12r2`] module"]
#[doc(alias = "DHR12R2")]
pub type Dhr12r2 = crate::Reg<dhr12r2::Dhr12r2Spec>;
#[doc = "DAC channel2 12-bit right aligned data holding register"]
pub mod dhr12r2;
#[doc = "DHR12L2 (rw) register accessor: DAC channel2 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12l2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12l2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12l2`] module"]
#[doc(alias = "DHR12L2")]
pub type Dhr12l2 = crate::Reg<dhr12l2::Dhr12l2Spec>;
#[doc = "DAC channel2 12-bit left aligned data holding register"]
pub mod dhr12l2;
#[doc = "DHR8R2 (rw) register accessor: DAC channel2 8-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8r2`] module"]
#[doc(alias = "DHR8R2")]
pub type Dhr8r2 = crate::Reg<dhr8r2::Dhr8r2Spec>;
#[doc = "DAC channel2 8-bit right-aligned data holding register"]
pub mod dhr8r2;
#[doc = "DHR12RD (rw) register accessor: Dual DAC 12-bit right-aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12rd`] module"]
#[doc(alias = "DHR12RD")]
pub type Dhr12rd = crate::Reg<dhr12rd::Dhr12rdSpec>;
#[doc = "Dual DAC 12-bit right-aligned data holding register"]
pub mod dhr12rd;
#[doc = "DHR12LD (rw) register accessor: DUAL DAC 12-bit left aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr12ld::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr12ld::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr12ld`] module"]
#[doc(alias = "DHR12LD")]
pub type Dhr12ld = crate::Reg<dhr12ld::Dhr12ldSpec>;
#[doc = "DUAL DAC 12-bit left aligned data holding register"]
pub mod dhr12ld;
#[doc = "DHR8RD (rw) register accessor: DUAL DAC 8-bit right aligned data holding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dhr8rd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dhr8rd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dhr8rd`] module"]
#[doc(alias = "DHR8RD")]
pub type Dhr8rd = crate::Reg<dhr8rd::Dhr8rdSpec>;
#[doc = "DUAL DAC 8-bit right aligned data holding register"]
pub mod dhr8rd;
#[doc = "DOR1 (r) register accessor: DAC channel1 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor1`] module"]
#[doc(alias = "DOR1")]
pub type Dor1 = crate::Reg<dor1::Dor1Spec>;
#[doc = "DAC channel1 data output register"]
pub mod dor1;
#[doc = "DOR2 (r) register accessor: DAC channel2 data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dor2`] module"]
#[doc(alias = "DOR2")]
pub type Dor2 = crate::Reg<dor2::Dor2Spec>;
#[doc = "DAC channel2 data output register"]
pub mod dor2;
#[doc = "SR (rw) register accessor: DAC status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`] module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "DAC status register"]
pub mod sr;
#[doc = "CCR (rw) register accessor: DAC calibration control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccr`] module"]
#[doc(alias = "CCR")]
pub type Ccr = crate::Reg<ccr::CcrSpec>;
#[doc = "DAC calibration control register"]
pub mod ccr;
#[doc = "MCR (rw) register accessor: DAC mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcr`] module"]
#[doc(alias = "MCR")]
pub type Mcr = crate::Reg<mcr::McrSpec>;
#[doc = "DAC mode control register"]
pub mod mcr;
#[doc = "SHSR1 (rw) register accessor: DAC Sample and Hold sample time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr1`] module"]
#[doc(alias = "SHSR1")]
pub type Shsr1 = crate::Reg<shsr1::Shsr1Spec>;
#[doc = "DAC Sample and Hold sample time register 1"]
pub mod shsr1;
#[doc = "SHSR2 (rw) register accessor: DAC Sample and Hold sample time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`shsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shsr2`] module"]
#[doc(alias = "SHSR2")]
pub type Shsr2 = crate::Reg<shsr2::Shsr2Spec>;
#[doc = "DAC Sample and Hold sample time register 2"]
pub mod shsr2;
#[doc = "SHHR (rw) register accessor: DAC Sample and Hold hold time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shhr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shhr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shhr`] module"]
#[doc(alias = "SHHR")]
pub type Shhr = crate::Reg<shhr::ShhrSpec>;
#[doc = "DAC Sample and Hold hold time register"]
pub mod shhr;
#[doc = "SHRR (rw) register accessor: DAC Sample and Hold refresh time register\n\nYou can [`read`](crate::Reg::read) this register and get [`shrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@shrr`] module"]
#[doc(alias = "SHRR")]
pub type Shrr = crate::Reg<shrr::ShrrSpec>;
#[doc = "DAC Sample and Hold refresh time register"]
pub mod shrr;
