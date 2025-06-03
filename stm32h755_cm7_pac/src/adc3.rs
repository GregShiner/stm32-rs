#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    isr: Isr,
    ier: Ier,
    cr: Cr,
    cfgr: Cfgr,
    cfgr2: Cfgr2,
    smpr1: Smpr1,
    smpr2: Smpr2,
    pcsel: Pcsel,
    ltr1: Ltr1,
    lhtr1: Lhtr1,
    _reserved10: [u8; 0x08],
    sqr1: Sqr1,
    sqr2: Sqr2,
    sqr3: Sqr3,
    sqr4: Sqr4,
    dr: Dr,
    _reserved15: [u8; 0x08],
    jsqr: Jsqr,
    _reserved16: [u8; 0x10],
    ofr1: Ofr1,
    ofr2: Ofr2,
    ofr3: Ofr3,
    ofr4: Ofr4,
    _reserved20: [u8; 0x10],
    jdr1: Jdr1,
    jdr2: Jdr2,
    jdr3: Jdr3,
    jdr4: Jdr4,
    _reserved24: [u8; 0x10],
    awd2cr: Awd2cr,
    awd3cr: Awd3cr,
    _reserved26: [u8; 0x08],
    ltr2: Ltr2,
    htr2: Htr2,
    ltr3: Ltr3,
    htr3: Htr3,
    difsel: Difsel,
    calfact: Calfact,
    calfact2: Calfact2,
}
impl RegisterBlock {
    #[doc = "0x00 - ADC interrupt and status register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x04 - ADC interrupt enable register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x08 - ADC control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x0c - ADC configuration register 1"]
    #[inline(always)]
    pub const fn cfgr(&self) -> &Cfgr {
        &self.cfgr
    }
    #[doc = "0x10 - ADC configuration register 2"]
    #[inline(always)]
    pub const fn cfgr2(&self) -> &Cfgr2 {
        &self.cfgr2
    }
    #[doc = "0x14 - ADC sampling time register 1"]
    #[inline(always)]
    pub const fn smpr1(&self) -> &Smpr1 {
        &self.smpr1
    }
    #[doc = "0x18 - ADC sampling time register 2"]
    #[inline(always)]
    pub const fn smpr2(&self) -> &Smpr2 {
        &self.smpr2
    }
    #[doc = "0x1c - ADC pre channel selection register"]
    #[inline(always)]
    pub const fn pcsel(&self) -> &Pcsel {
        &self.pcsel
    }
    #[doc = "0x20 - ADC analog watchdog 1 threshold register"]
    #[inline(always)]
    pub const fn ltr1(&self) -> &Ltr1 {
        &self.ltr1
    }
    #[doc = "0x24 - ADC analog watchdog 2 threshold register"]
    #[inline(always)]
    pub const fn lhtr1(&self) -> &Lhtr1 {
        &self.lhtr1
    }
    #[doc = "0x30 - ADC group regular sequencer ranks register 1"]
    #[inline(always)]
    pub const fn sqr1(&self) -> &Sqr1 {
        &self.sqr1
    }
    #[doc = "0x34 - ADC group regular sequencer ranks register 2"]
    #[inline(always)]
    pub const fn sqr2(&self) -> &Sqr2 {
        &self.sqr2
    }
    #[doc = "0x38 - ADC group regular sequencer ranks register 3"]
    #[inline(always)]
    pub const fn sqr3(&self) -> &Sqr3 {
        &self.sqr3
    }
    #[doc = "0x3c - ADC group regular sequencer ranks register 4"]
    #[inline(always)]
    pub const fn sqr4(&self) -> &Sqr4 {
        &self.sqr4
    }
    #[doc = "0x40 - ADC group regular conversion data register"]
    #[inline(always)]
    pub const fn dr(&self) -> &Dr {
        &self.dr
    }
    #[doc = "0x4c - ADC group injected sequencer register"]
    #[inline(always)]
    pub const fn jsqr(&self) -> &Jsqr {
        &self.jsqr
    }
    #[doc = "0x60 - ADC offset number 1 register"]
    #[inline(always)]
    pub const fn ofr1(&self) -> &Ofr1 {
        &self.ofr1
    }
    #[doc = "0x64 - ADC offset number 2 register"]
    #[inline(always)]
    pub const fn ofr2(&self) -> &Ofr2 {
        &self.ofr2
    }
    #[doc = "0x68 - ADC offset number 3 register"]
    #[inline(always)]
    pub const fn ofr3(&self) -> &Ofr3 {
        &self.ofr3
    }
    #[doc = "0x6c - ADC offset number 4 register"]
    #[inline(always)]
    pub const fn ofr4(&self) -> &Ofr4 {
        &self.ofr4
    }
    #[doc = "0x80 - ADC group injected sequencer rank 1 register"]
    #[inline(always)]
    pub const fn jdr1(&self) -> &Jdr1 {
        &self.jdr1
    }
    #[doc = "0x84 - ADC group injected sequencer rank 2 register"]
    #[inline(always)]
    pub const fn jdr2(&self) -> &Jdr2 {
        &self.jdr2
    }
    #[doc = "0x88 - ADC group injected sequencer rank 3 register"]
    #[inline(always)]
    pub const fn jdr3(&self) -> &Jdr3 {
        &self.jdr3
    }
    #[doc = "0x8c - ADC group injected sequencer rank 4 register"]
    #[inline(always)]
    pub const fn jdr4(&self) -> &Jdr4 {
        &self.jdr4
    }
    #[doc = "0xa0 - ADC analog watchdog 2 configuration register"]
    #[inline(always)]
    pub const fn awd2cr(&self) -> &Awd2cr {
        &self.awd2cr
    }
    #[doc = "0xa4 - ADC analog watchdog 3 configuration register"]
    #[inline(always)]
    pub const fn awd3cr(&self) -> &Awd3cr {
        &self.awd3cr
    }
    #[doc = "0xb0 - ADC watchdog lower threshold register 2"]
    #[inline(always)]
    pub const fn ltr2(&self) -> &Ltr2 {
        &self.ltr2
    }
    #[doc = "0xb4 - ADC watchdog higher threshold register 2"]
    #[inline(always)]
    pub const fn htr2(&self) -> &Htr2 {
        &self.htr2
    }
    #[doc = "0xb8 - ADC watchdog lower threshold register 3"]
    #[inline(always)]
    pub const fn ltr3(&self) -> &Ltr3 {
        &self.ltr3
    }
    #[doc = "0xbc - ADC watchdog higher threshold register 3"]
    #[inline(always)]
    pub const fn htr3(&self) -> &Htr3 {
        &self.htr3
    }
    #[doc = "0xc0 - ADC channel differential or single-ended mode selection register"]
    #[inline(always)]
    pub const fn difsel(&self) -> &Difsel {
        &self.difsel
    }
    #[doc = "0xc4 - ADC calibration factors register"]
    #[inline(always)]
    pub const fn calfact(&self) -> &Calfact {
        &self.calfact
    }
    #[doc = "0xc8 - ADC Calibration Factor register 2"]
    #[inline(always)]
    pub const fn calfact2(&self) -> &Calfact2 {
        &self.calfact2
    }
}
#[doc = "ISR (rw) register accessor: ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "ADC interrupt and status register"]
pub mod isr;
#[doc = "IER (rw) register accessor: ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`] module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "ADC interrupt enable register"]
pub mod ier;
#[doc = "CR (rw) register accessor: ADC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "ADC control register"]
pub mod cr;
#[doc = "CFGR (rw) register accessor: ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr`] module"]
#[doc(alias = "CFGR")]
pub type Cfgr = crate::Reg<cfgr::CfgrSpec>;
#[doc = "ADC configuration register 1"]
pub mod cfgr;
#[doc = "CFGR2 (rw) register accessor: ADC configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfgr2`] module"]
#[doc(alias = "CFGR2")]
pub type Cfgr2 = crate::Reg<cfgr2::Cfgr2Spec>;
#[doc = "ADC configuration register 2"]
pub mod cfgr2;
#[doc = "SMPR1 (rw) register accessor: ADC sampling time register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr1`] module"]
#[doc(alias = "SMPR1")]
pub type Smpr1 = crate::Reg<smpr1::Smpr1Spec>;
#[doc = "ADC sampling time register 1"]
pub mod smpr1;
#[doc = "SMPR2 (rw) register accessor: ADC sampling time register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`smpr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smpr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@smpr2`] module"]
#[doc(alias = "SMPR2")]
pub type Smpr2 = crate::Reg<smpr2::Smpr2Spec>;
#[doc = "ADC sampling time register 2"]
pub mod smpr2;
#[doc = "LTR1 (rw) register accessor: ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr1`] module"]
#[doc(alias = "LTR1")]
pub type Ltr1 = crate::Reg<ltr1::Ltr1Spec>;
#[doc = "ADC analog watchdog 1 threshold register"]
pub mod ltr1;
#[doc = "LHTR1 (rw) register accessor: ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`lhtr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lhtr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lhtr1`] module"]
#[doc(alias = "LHTR1")]
pub type Lhtr1 = crate::Reg<lhtr1::Lhtr1Spec>;
#[doc = "ADC analog watchdog 2 threshold register"]
pub mod lhtr1;
#[doc = "SQR1 (rw) register accessor: ADC group regular sequencer ranks register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr1`] module"]
#[doc(alias = "SQR1")]
pub type Sqr1 = crate::Reg<sqr1::Sqr1Spec>;
#[doc = "ADC group regular sequencer ranks register 1"]
pub mod sqr1;
#[doc = "SQR2 (rw) register accessor: ADC group regular sequencer ranks register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr2`] module"]
#[doc(alias = "SQR2")]
pub type Sqr2 = crate::Reg<sqr2::Sqr2Spec>;
#[doc = "ADC group regular sequencer ranks register 2"]
pub mod sqr2;
#[doc = "SQR3 (rw) register accessor: ADC group regular sequencer ranks register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr3`] module"]
#[doc(alias = "SQR3")]
pub type Sqr3 = crate::Reg<sqr3::Sqr3Spec>;
#[doc = "ADC group regular sequencer ranks register 3"]
pub mod sqr3;
#[doc = "SQR4 (rw) register accessor: ADC group regular sequencer ranks register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`sqr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sqr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sqr4`] module"]
#[doc(alias = "SQR4")]
pub type Sqr4 = crate::Reg<sqr4::Sqr4Spec>;
#[doc = "ADC group regular sequencer ranks register 4"]
pub mod sqr4;
#[doc = "DR (r) register accessor: ADC group regular conversion data register\n\nYou can [`read`](crate::Reg::read) this register and get [`dr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@dr`] module"]
#[doc(alias = "DR")]
pub type Dr = crate::Reg<dr::DrSpec>;
#[doc = "ADC group regular conversion data register"]
pub mod dr;
#[doc = "JSQR (rw) register accessor: ADC group injected sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jsqr`] module"]
#[doc(alias = "JSQR")]
pub type Jsqr = crate::Reg<jsqr::JsqrSpec>;
#[doc = "ADC group injected sequencer register"]
pub mod jsqr;
#[doc = "OFR1 (rw) register accessor: ADC offset number 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr1`] module"]
#[doc(alias = "OFR1")]
pub type Ofr1 = crate::Reg<ofr1::Ofr1Spec>;
#[doc = "ADC offset number 1 register"]
pub mod ofr1;
#[doc = "OFR2 (rw) register accessor: ADC offset number 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr2`] module"]
#[doc(alias = "OFR2")]
pub type Ofr2 = crate::Reg<ofr2::Ofr2Spec>;
#[doc = "ADC offset number 2 register"]
pub mod ofr2;
#[doc = "OFR3 (rw) register accessor: ADC offset number 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr3`] module"]
#[doc(alias = "OFR3")]
pub type Ofr3 = crate::Reg<ofr3::Ofr3Spec>;
#[doc = "ADC offset number 3 register"]
pub mod ofr3;
#[doc = "OFR4 (rw) register accessor: ADC offset number 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`ofr4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ofr4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ofr4`] module"]
#[doc(alias = "OFR4")]
pub type Ofr4 = crate::Reg<ofr4::Ofr4Spec>;
#[doc = "ADC offset number 4 register"]
pub mod ofr4;
#[doc = "JDR1 (r) register accessor: ADC group injected sequencer rank 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr1`] module"]
#[doc(alias = "JDR1")]
pub type Jdr1 = crate::Reg<jdr1::Jdr1Spec>;
#[doc = "ADC group injected sequencer rank 1 register"]
pub mod jdr1;
#[doc = "JDR2 (r) register accessor: ADC group injected sequencer rank 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr2`] module"]
#[doc(alias = "JDR2")]
pub type Jdr2 = crate::Reg<jdr2::Jdr2Spec>;
#[doc = "ADC group injected sequencer rank 2 register"]
pub mod jdr2;
#[doc = "JDR3 (r) register accessor: ADC group injected sequencer rank 3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr3`] module"]
#[doc(alias = "JDR3")]
pub type Jdr3 = crate::Reg<jdr3::Jdr3Spec>;
#[doc = "ADC group injected sequencer rank 3 register"]
pub mod jdr3;
#[doc = "JDR4 (r) register accessor: ADC group injected sequencer rank 4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`jdr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@jdr4`] module"]
#[doc(alias = "JDR4")]
pub type Jdr4 = crate::Reg<jdr4::Jdr4Spec>;
#[doc = "ADC group injected sequencer rank 4 register"]
pub mod jdr4;
#[doc = "AWD2CR (rw) register accessor: ADC analog watchdog 2 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd2cr`] module"]
#[doc(alias = "AWD2CR")]
pub type Awd2cr = crate::Reg<awd2cr::Awd2crSpec>;
#[doc = "ADC analog watchdog 2 configuration register"]
pub mod awd2cr;
#[doc = "AWD3CR (rw) register accessor: ADC analog watchdog 3 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`awd3cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`awd3cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@awd3cr`] module"]
#[doc(alias = "AWD3CR")]
pub type Awd3cr = crate::Reg<awd3cr::Awd3crSpec>;
#[doc = "ADC analog watchdog 3 configuration register"]
pub mod awd3cr;
#[doc = "DIFSEL (rw) register accessor: ADC channel differential or single-ended mode selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`difsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`difsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@difsel`] module"]
#[doc(alias = "DIFSEL")]
pub type Difsel = crate::Reg<difsel::DifselSpec>;
#[doc = "ADC channel differential or single-ended mode selection register"]
pub mod difsel;
#[doc = "CALFACT (rw) register accessor: ADC calibration factors register\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact`] module"]
#[doc(alias = "CALFACT")]
pub type Calfact = crate::Reg<calfact::CalfactSpec>;
#[doc = "ADC calibration factors register"]
pub mod calfact;
#[doc = "PCSEL (rw) register accessor: ADC pre channel selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`pcsel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcsel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@pcsel`] module"]
#[doc(alias = "PCSEL")]
pub type Pcsel = crate::Reg<pcsel::PcselSpec>;
#[doc = "ADC pre channel selection register"]
pub mod pcsel;
#[doc = "LTR2 (rw) register accessor: ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr2`] module"]
#[doc(alias = "LTR2")]
pub type Ltr2 = crate::Reg<ltr2::Ltr2Spec>;
#[doc = "ADC watchdog lower threshold register 2"]
pub mod ltr2;
#[doc = "HTR2 (rw) register accessor: ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`htr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htr2`] module"]
#[doc(alias = "HTR2")]
pub type Htr2 = crate::Reg<htr2::Htr2Spec>;
#[doc = "ADC watchdog higher threshold register 2"]
pub mod htr2;
#[doc = "LTR3 (rw) register accessor: ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ltr3`] module"]
#[doc(alias = "LTR3")]
pub type Ltr3 = crate::Reg<ltr3::Ltr3Spec>;
#[doc = "ADC watchdog lower threshold register 3"]
pub mod ltr3;
#[doc = "HTR3 (rw) register accessor: ADC watchdog higher threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`htr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@htr3`] module"]
#[doc(alias = "HTR3")]
pub type Htr3 = crate::Reg<htr3::Htr3Spec>;
#[doc = "ADC watchdog higher threshold register 3"]
pub mod htr3;
#[doc = "CALFACT2 (rw) register accessor: ADC Calibration Factor register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@calfact2`] module"]
#[doc(alias = "CALFACT2")]
pub type Calfact2 = crate::Reg<calfact2::Calfact2Spec>;
#[doc = "ADC Calibration Factor register 2"]
pub mod calfact2;
