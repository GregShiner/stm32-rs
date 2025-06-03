#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    rtsr1: Rtsr1,
    ftsr1: Ftsr1,
    swier1: Swier1,
    d3pmr1: D3pmr1,
    d3pcr1l: D3pcr1l,
    d3pcr1h: D3pcr1h,
    _reserved6: [u8; 0x08],
    rtsr2: Rtsr2,
    ftsr2: Ftsr2,
    swier2: Swier2,
    d3pmr2: D3pmr2,
    d3pcr2l: D3pcr2l,
    d3pcr2h: D3pcr2h,
    _reserved12: [u8; 0x08],
    rtsr3: Rtsr3,
    ftsr3: Ftsr3,
    swier3: Swier3,
    d3pmr3: D3pmr3,
    _reserved16: [u8; 0x04],
    d3pcr3h: D3pcr3h,
    _reserved17: [u8; 0x28],
    cpuimr1: Cpuimr1,
    cpuemr1: Cpuemr1,
    cpupr1: Cpupr1,
    _reserved20: [u8; 0x04],
    cpuimr2: Cpuimr2,
    cpuemr2: Cpuemr2,
    cpupr2: Cpupr2,
    _reserved23: [u8; 0x04],
    cpuimr3: Cpuimr3,
    cpuemr3: Cpuemr3,
    cpupr3: Cpupr3,
}
impl RegisterBlock {
    #[doc = "0x00 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr1(&self) -> &Rtsr1 {
        &self.rtsr1
    }
    #[doc = "0x04 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr1(&self) -> &Ftsr1 {
        &self.ftsr1
    }
    #[doc = "0x08 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier1(&self) -> &Swier1 {
        &self.swier1
    }
    #[doc = "0x0c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr1(&self) -> &D3pmr1 {
        &self.d3pmr1
    }
    #[doc = "0x10 - EXTI D3 pending clear selection register low"]
    #[inline(always)]
    pub const fn d3pcr1l(&self) -> &D3pcr1l {
        &self.d3pcr1l
    }
    #[doc = "0x14 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr1h(&self) -> &D3pcr1h {
        &self.d3pcr1h
    }
    #[doc = "0x20 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr2(&self) -> &Rtsr2 {
        &self.rtsr2
    }
    #[doc = "0x24 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr2(&self) -> &Ftsr2 {
        &self.ftsr2
    }
    #[doc = "0x28 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier2(&self) -> &Swier2 {
        &self.swier2
    }
    #[doc = "0x2c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr2(&self) -> &D3pmr2 {
        &self.d3pmr2
    }
    #[doc = "0x30 - EXTI D3 pending clear selection register low"]
    #[inline(always)]
    pub const fn d3pcr2l(&self) -> &D3pcr2l {
        &self.d3pcr2l
    }
    #[doc = "0x34 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr2h(&self) -> &D3pcr2h {
        &self.d3pcr2h
    }
    #[doc = "0x40 - EXTI rising trigger selection register"]
    #[inline(always)]
    pub const fn rtsr3(&self) -> &Rtsr3 {
        &self.rtsr3
    }
    #[doc = "0x44 - EXTI falling trigger selection register"]
    #[inline(always)]
    pub const fn ftsr3(&self) -> &Ftsr3 {
        &self.ftsr3
    }
    #[doc = "0x48 - EXTI software interrupt event register"]
    #[inline(always)]
    pub const fn swier3(&self) -> &Swier3 {
        &self.swier3
    }
    #[doc = "0x4c - EXTI D3 pending mask register"]
    #[inline(always)]
    pub const fn d3pmr3(&self) -> &D3pmr3 {
        &self.d3pmr3
    }
    #[doc = "0x54 - EXTI D3 pending clear selection register high"]
    #[inline(always)]
    pub const fn d3pcr3h(&self) -> &D3pcr3h {
        &self.d3pcr3h
    }
    #[doc = "0x80 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn cpuimr1(&self) -> &Cpuimr1 {
        &self.cpuimr1
    }
    #[doc = "0x84 - EXTI event mask register"]
    #[inline(always)]
    pub const fn cpuemr1(&self) -> &Cpuemr1 {
        &self.cpuemr1
    }
    #[doc = "0x88 - EXTI pending register"]
    #[inline(always)]
    pub const fn cpupr1(&self) -> &Cpupr1 {
        &self.cpupr1
    }
    #[doc = "0x90 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn cpuimr2(&self) -> &Cpuimr2 {
        &self.cpuimr2
    }
    #[doc = "0x94 - EXTI event mask register"]
    #[inline(always)]
    pub const fn cpuemr2(&self) -> &Cpuemr2 {
        &self.cpuemr2
    }
    #[doc = "0x98 - EXTI pending register"]
    #[inline(always)]
    pub const fn cpupr2(&self) -> &Cpupr2 {
        &self.cpupr2
    }
    #[doc = "0xa0 - EXTI interrupt mask register"]
    #[inline(always)]
    pub const fn cpuimr3(&self) -> &Cpuimr3 {
        &self.cpuimr3
    }
    #[doc = "0xa4 - EXTI event mask register"]
    #[inline(always)]
    pub const fn cpuemr3(&self) -> &Cpuemr3 {
        &self.cpuemr3
    }
    #[doc = "0xa8 - EXTI pending register"]
    #[inline(always)]
    pub const fn cpupr3(&self) -> &Cpupr3 {
        &self.cpupr3
    }
}
#[doc = "RTSR1 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr1`] module"]
#[doc(alias = "RTSR1")]
pub type Rtsr1 = crate::Reg<rtsr1::Rtsr1Spec>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr1;
#[doc = "FTSR1 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr1`] module"]
#[doc(alias = "FTSR1")]
pub type Ftsr1 = crate::Reg<ftsr1::Ftsr1Spec>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr1;
#[doc = "SWIER1 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier1`] module"]
#[doc(alias = "SWIER1")]
pub type Swier1 = crate::Reg<swier1::Swier1Spec>;
#[doc = "EXTI software interrupt event register"]
pub mod swier1;
#[doc = "D3PMR1 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr1`] module"]
#[doc(alias = "D3PMR1")]
pub type D3pmr1 = crate::Reg<d3pmr1::D3pmr1Spec>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr1;
#[doc = "D3PCR1L (rw) register accessor: EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr1l`] module"]
#[doc(alias = "D3PCR1L")]
pub type D3pcr1l = crate::Reg<d3pcr1l::D3pcr1lSpec>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr1l;
#[doc = "D3PCR1H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr1h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr1h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr1h`] module"]
#[doc(alias = "D3PCR1H")]
pub type D3pcr1h = crate::Reg<d3pcr1h::D3pcr1hSpec>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr1h;
#[doc = "RTSR2 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr2`] module"]
#[doc(alias = "RTSR2")]
pub type Rtsr2 = crate::Reg<rtsr2::Rtsr2Spec>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr2;
#[doc = "FTSR2 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr2`] module"]
#[doc(alias = "FTSR2")]
pub type Ftsr2 = crate::Reg<ftsr2::Ftsr2Spec>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr2;
#[doc = "SWIER2 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier2`] module"]
#[doc(alias = "SWIER2")]
pub type Swier2 = crate::Reg<swier2::Swier2Spec>;
#[doc = "EXTI software interrupt event register"]
pub mod swier2;
#[doc = "D3PMR2 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr2`] module"]
#[doc(alias = "D3PMR2")]
pub type D3pmr2 = crate::Reg<d3pmr2::D3pmr2Spec>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr2;
#[doc = "D3PCR2L (rw) register accessor: EXTI D3 pending clear selection register low\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr2l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr2l`] module"]
#[doc(alias = "D3PCR2L")]
pub type D3pcr2l = crate::Reg<d3pcr2l::D3pcr2lSpec>;
#[doc = "EXTI D3 pending clear selection register low"]
pub mod d3pcr2l;
#[doc = "D3PCR2H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr2h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr2h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr2h`] module"]
#[doc(alias = "D3PCR2H")]
pub type D3pcr2h = crate::Reg<d3pcr2h::D3pcr2hSpec>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr2h;
#[doc = "RTSR3 (rw) register accessor: EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rtsr3`] module"]
#[doc(alias = "RTSR3")]
pub type Rtsr3 = crate::Reg<rtsr3::Rtsr3Spec>;
#[doc = "EXTI rising trigger selection register"]
pub mod rtsr3;
#[doc = "FTSR3 (rw) register accessor: EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ftsr3`] module"]
#[doc(alias = "FTSR3")]
pub type Ftsr3 = crate::Reg<ftsr3::Ftsr3Spec>;
#[doc = "EXTI falling trigger selection register"]
pub mod ftsr3;
#[doc = "SWIER3 (rw) register accessor: EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@swier3`] module"]
#[doc(alias = "SWIER3")]
pub type Swier3 = crate::Reg<swier3::Swier3Spec>;
#[doc = "EXTI software interrupt event register"]
pub mod swier3;
#[doc = "D3PMR3 (rw) register accessor: EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pmr3`] module"]
#[doc(alias = "D3PMR3")]
pub type D3pmr3 = crate::Reg<d3pmr3::D3pmr3Spec>;
#[doc = "EXTI D3 pending mask register"]
pub mod d3pmr3;
#[doc = "D3PCR3H (rw) register accessor: EXTI D3 pending clear selection register high\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pcr3h::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pcr3h::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@d3pcr3h`] module"]
#[doc(alias = "D3PCR3H")]
pub type D3pcr3h = crate::Reg<d3pcr3h::D3pcr3hSpec>;
#[doc = "EXTI D3 pending clear selection register high"]
pub mod d3pcr3h;
#[doc = "CPUIMR1 (rw) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuimr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuimr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuimr1`] module"]
#[doc(alias = "CPUIMR1")]
pub type Cpuimr1 = crate::Reg<cpuimr1::Cpuimr1Spec>;
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr1;
#[doc = "CPUEMR1 (rw) register accessor: EXTI event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuemr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuemr1`] module"]
#[doc(alias = "CPUEMR1")]
pub type Cpuemr1 = crate::Reg<cpuemr1::Cpuemr1Spec>;
#[doc = "EXTI event mask register"]
pub mod cpuemr1;
#[doc = "CPUPR1 (rw) register accessor: EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpupr1`] module"]
#[doc(alias = "CPUPR1")]
pub type Cpupr1 = crate::Reg<cpupr1::Cpupr1Spec>;
#[doc = "EXTI pending register"]
pub mod cpupr1;
#[doc = "CPUIMR2 (rw) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuimr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuimr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuimr2`] module"]
#[doc(alias = "CPUIMR2")]
pub type Cpuimr2 = crate::Reg<cpuimr2::Cpuimr2Spec>;
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr2;
#[doc = "CPUEMR2 (rw) register accessor: EXTI event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuemr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpuemr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuemr2`] module"]
#[doc(alias = "CPUEMR2")]
pub type Cpuemr2 = crate::Reg<cpuemr2::Cpuemr2Spec>;
#[doc = "EXTI event mask register"]
pub mod cpuemr2;
#[doc = "CPUPR2 (r) register accessor: EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpupr2`] module"]
#[doc(alias = "CPUPR2")]
pub type Cpupr2 = crate::Reg<cpupr2::Cpupr2Spec>;
#[doc = "EXTI pending register"]
pub mod cpupr2;
#[doc = "CPUIMR3 (r) register accessor: EXTI interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuimr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuimr3`] module"]
#[doc(alias = "CPUIMR3")]
pub type Cpuimr3 = crate::Reg<cpuimr3::Cpuimr3Spec>;
#[doc = "EXTI interrupt mask register"]
pub mod cpuimr3;
#[doc = "CPUEMR3 (r) register accessor: EXTI event mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuemr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuemr3`] module"]
#[doc(alias = "CPUEMR3")]
pub type Cpuemr3 = crate::Reg<cpuemr3::Cpuemr3Spec>;
#[doc = "EXTI event mask register"]
pub mod cpuemr3;
#[doc = "CPUPR3 (r) register accessor: EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpupr3`] module"]
#[doc(alias = "CPUPR3")]
pub type Cpupr3 = crate::Reg<cpupr3::Cpupr3Spec>;
#[doc = "EXTI pending register"]
pub mod cpupr3;
