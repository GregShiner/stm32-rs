#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    cr: Cr,
    isr: Isr,
    ifcr: Ifcr,
    fgmar: Fgmar,
    fgor: Fgor,
    bgmar: Bgmar,
    bgor: Bgor,
    fgpfccr: Fgpfccr,
    fgcolr: Fgcolr,
    bgpfccr: Bgpfccr,
    bgcolr: Bgcolr,
    fgcmar: Fgcmar,
    bgcmar: Bgcmar,
    opfccr: Opfccr,
    ocolr: Ocolr,
    omar: Omar,
    oor: Oor,
    nlr: Nlr,
    lwr: Lwr,
    amtcr: Amtcr,
}
impl RegisterBlock {
    #[doc = "0x00 - DMA2D control register"]
    #[inline(always)]
    pub const fn cr(&self) -> &Cr {
        &self.cr
    }
    #[doc = "0x04 - DMA2D Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x08 - DMA2D interrupt flag clear register"]
    #[inline(always)]
    pub const fn ifcr(&self) -> &Ifcr {
        &self.ifcr
    }
    #[doc = "0x0c - DMA2D foreground memory address register"]
    #[inline(always)]
    pub const fn fgmar(&self) -> &Fgmar {
        &self.fgmar
    }
    #[doc = "0x10 - DMA2D foreground offset register"]
    #[inline(always)]
    pub const fn fgor(&self) -> &Fgor {
        &self.fgor
    }
    #[doc = "0x14 - DMA2D background memory address register"]
    #[inline(always)]
    pub const fn bgmar(&self) -> &Bgmar {
        &self.bgmar
    }
    #[doc = "0x18 - DMA2D background offset register"]
    #[inline(always)]
    pub const fn bgor(&self) -> &Bgor {
        &self.bgor
    }
    #[doc = "0x1c - DMA2D foreground PFC control register"]
    #[inline(always)]
    pub const fn fgpfccr(&self) -> &Fgpfccr {
        &self.fgpfccr
    }
    #[doc = "0x20 - DMA2D foreground color register"]
    #[inline(always)]
    pub const fn fgcolr(&self) -> &Fgcolr {
        &self.fgcolr
    }
    #[doc = "0x24 - DMA2D background PFC control register"]
    #[inline(always)]
    pub const fn bgpfccr(&self) -> &Bgpfccr {
        &self.bgpfccr
    }
    #[doc = "0x28 - DMA2D background color register"]
    #[inline(always)]
    pub const fn bgcolr(&self) -> &Bgcolr {
        &self.bgcolr
    }
    #[doc = "0x2c - DMA2D foreground CLUT memory address register"]
    #[inline(always)]
    pub const fn fgcmar(&self) -> &Fgcmar {
        &self.fgcmar
    }
    #[doc = "0x30 - DMA2D background CLUT memory address register"]
    #[inline(always)]
    pub const fn bgcmar(&self) -> &Bgcmar {
        &self.bgcmar
    }
    #[doc = "0x34 - DMA2D output PFC control register"]
    #[inline(always)]
    pub const fn opfccr(&self) -> &Opfccr {
        &self.opfccr
    }
    #[doc = "0x38 - DMA2D output color register"]
    #[inline(always)]
    pub const fn ocolr(&self) -> &Ocolr {
        &self.ocolr
    }
    #[doc = "0x3c - DMA2D output memory address register"]
    #[inline(always)]
    pub const fn omar(&self) -> &Omar {
        &self.omar
    }
    #[doc = "0x40 - DMA2D output offset register"]
    #[inline(always)]
    pub const fn oor(&self) -> &Oor {
        &self.oor
    }
    #[doc = "0x44 - DMA2D number of line register"]
    #[inline(always)]
    pub const fn nlr(&self) -> &Nlr {
        &self.nlr
    }
    #[doc = "0x48 - DMA2D line watermark register"]
    #[inline(always)]
    pub const fn lwr(&self) -> &Lwr {
        &self.lwr
    }
    #[doc = "0x4c - DMA2D AXI master timer configuration register"]
    #[inline(always)]
    pub const fn amtcr(&self) -> &Amtcr {
        &self.amtcr
    }
}
#[doc = "CR (rw) register accessor: DMA2D control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cr`] module"]
#[doc(alias = "CR")]
pub type Cr = crate::Reg<cr::CrSpec>;
#[doc = "DMA2D control register"]
pub mod cr;
#[doc = "ISR (r) register accessor: DMA2D Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`] module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "DMA2D Interrupt Status Register"]
pub mod isr;
#[doc = "IFCR (rw) register accessor: DMA2D interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`ifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ifcr`] module"]
#[doc(alias = "IFCR")]
pub type Ifcr = crate::Reg<ifcr::IfcrSpec>;
#[doc = "DMA2D interrupt flag clear register"]
pub mod ifcr;
#[doc = "FGMAR (rw) register accessor: DMA2D foreground memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgmar`] module"]
#[doc(alias = "FGMAR")]
pub type Fgmar = crate::Reg<fgmar::FgmarSpec>;
#[doc = "DMA2D foreground memory address register"]
pub mod fgmar;
#[doc = "FGOR (rw) register accessor: DMA2D foreground offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgor`] module"]
#[doc(alias = "FGOR")]
pub type Fgor = crate::Reg<fgor::FgorSpec>;
#[doc = "DMA2D foreground offset register"]
pub mod fgor;
#[doc = "BGMAR (rw) register accessor: DMA2D background memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgmar`] module"]
#[doc(alias = "BGMAR")]
pub type Bgmar = crate::Reg<bgmar::BgmarSpec>;
#[doc = "DMA2D background memory address register"]
pub mod bgmar;
#[doc = "BGOR (rw) register accessor: DMA2D background offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgor`] module"]
#[doc(alias = "BGOR")]
pub type Bgor = crate::Reg<bgor::BgorSpec>;
#[doc = "DMA2D background offset register"]
pub mod bgor;
#[doc = "FGPFCCR (rw) register accessor: DMA2D foreground PFC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgpfccr`] module"]
#[doc(alias = "FGPFCCR")]
pub type Fgpfccr = crate::Reg<fgpfccr::FgpfccrSpec>;
#[doc = "DMA2D foreground PFC control register"]
pub mod fgpfccr;
#[doc = "FGCOLR (rw) register accessor: DMA2D foreground color register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcolr`] module"]
#[doc(alias = "FGCOLR")]
pub type Fgcolr = crate::Reg<fgcolr::FgcolrSpec>;
#[doc = "DMA2D foreground color register"]
pub mod fgcolr;
#[doc = "BGPFCCR (rw) register accessor: DMA2D background PFC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgpfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgpfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgpfccr`] module"]
#[doc(alias = "BGPFCCR")]
pub type Bgpfccr = crate::Reg<bgpfccr::BgpfccrSpec>;
#[doc = "DMA2D background PFC control register"]
pub mod bgpfccr;
#[doc = "BGCOLR (rw) register accessor: DMA2D background color register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcolr`] module"]
#[doc(alias = "BGCOLR")]
pub type Bgcolr = crate::Reg<bgcolr::BgcolrSpec>;
#[doc = "DMA2D background color register"]
pub mod bgcolr;
#[doc = "FGCMAR (rw) register accessor: DMA2D foreground CLUT memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`fgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fgcmar`] module"]
#[doc(alias = "FGCMAR")]
pub type Fgcmar = crate::Reg<fgcmar::FgcmarSpec>;
#[doc = "DMA2D foreground CLUT memory address register"]
pub mod fgcmar;
#[doc = "BGCMAR (rw) register accessor: DMA2D background CLUT memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`bgcmar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bgcmar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@bgcmar`] module"]
#[doc(alias = "BGCMAR")]
pub type Bgcmar = crate::Reg<bgcmar::BgcmarSpec>;
#[doc = "DMA2D background CLUT memory address register"]
pub mod bgcmar;
#[doc = "OPFCCR (rw) register accessor: DMA2D output PFC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`opfccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opfccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@opfccr`] module"]
#[doc(alias = "OPFCCR")]
pub type Opfccr = crate::Reg<opfccr::OpfccrSpec>;
#[doc = "DMA2D output PFC control register"]
pub mod opfccr;
#[doc = "OCOLR (rw) register accessor: DMA2D output color register\n\nYou can [`read`](crate::Reg::read) this register and get [`ocolr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ocolr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ocolr`] module"]
#[doc(alias = "OCOLR")]
pub type Ocolr = crate::Reg<ocolr::OcolrSpec>;
#[doc = "DMA2D output color register"]
pub mod ocolr;
#[doc = "OMAR (rw) register accessor: DMA2D output memory address register\n\nYou can [`read`](crate::Reg::read) this register and get [`omar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`omar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@omar`] module"]
#[doc(alias = "OMAR")]
pub type Omar = crate::Reg<omar::OmarSpec>;
#[doc = "DMA2D output memory address register"]
pub mod omar;
#[doc = "OOR (rw) register accessor: DMA2D output offset register\n\nYou can [`read`](crate::Reg::read) this register and get [`oor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@oor`] module"]
#[doc(alias = "OOR")]
pub type Oor = crate::Reg<oor::OorSpec>;
#[doc = "DMA2D output offset register"]
pub mod oor;
#[doc = "NLR (rw) register accessor: DMA2D number of line register\n\nYou can [`read`](crate::Reg::read) this register and get [`nlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@nlr`] module"]
#[doc(alias = "NLR")]
pub type Nlr = crate::Reg<nlr::NlrSpec>;
#[doc = "DMA2D number of line register"]
pub mod nlr;
#[doc = "LWR (rw) register accessor: DMA2D line watermark register\n\nYou can [`read`](crate::Reg::read) this register and get [`lwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lwr`] module"]
#[doc(alias = "LWR")]
pub type Lwr = crate::Reg<lwr::LwrSpec>;
#[doc = "DMA2D line watermark register"]
pub mod lwr;
#[doc = "AMTCR (rw) register accessor: DMA2D AXI master timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`amtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`amtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@amtcr`] module"]
#[doc(alias = "AMTCR")]
pub type Amtcr = crate::Reg<amtcr::AmtcrSpec>;
#[doc = "DMA2D AXI master timer configuration register"]
pub mod amtcr;
