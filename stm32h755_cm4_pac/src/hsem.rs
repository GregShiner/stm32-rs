#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hsem_r0: HsemR0,
    hsem_r1: HsemR1,
    hsem_r2: HsemR2,
    hsem_r3: HsemR3,
    hsem_r4: HsemR4,
    hsem_r5: HsemR5,
    hsem_r6: HsemR6,
    hsem_r7: HsemR7,
    hsem_r8: HsemR8,
    hsem_r9: HsemR9,
    hsem_r10: HsemR10,
    hsem_r11: HsemR11,
    hsem_r12: HsemR12,
    hsem_r13: HsemR13,
    hsem_r14: HsemR14,
    hsem_r15: HsemR15,
    hsem_r16: HsemR16,
    hsem_r17: HsemR17,
    hsem_r18: HsemR18,
    hsem_r19: HsemR19,
    hsem_r20: HsemR20,
    hsem_r21: HsemR21,
    hsem_r22: HsemR22,
    hsem_r23: HsemR23,
    hsem_r24: HsemR24,
    hsem_r25: HsemR25,
    hsem_r26: HsemR26,
    hsem_r27: HsemR27,
    hsem_r28: HsemR28,
    hsem_r29: HsemR29,
    hsem_r30: HsemR30,
    hsem_r31: HsemR31,
    hsem_rlr0: HsemRlr0,
    hsem_rlr1: HsemRlr1,
    hsem_rlr2: HsemRlr2,
    hsem_rlr3: HsemRlr3,
    hsem_rlr4: HsemRlr4,
    hsem_rlr5: HsemRlr5,
    hsem_rlr6: HsemRlr6,
    hsem_rlr7: HsemRlr7,
    hsem_rlr8: HsemRlr8,
    hsem_rlr9: HsemRlr9,
    hsem_rlr10: HsemRlr10,
    hsem_rlr11: HsemRlr11,
    hsem_rlr12: HsemRlr12,
    hsem_rlr13: HsemRlr13,
    hsem_rlr14: HsemRlr14,
    hsem_rlr15: HsemRlr15,
    hsem_rlr16: HsemRlr16,
    hsem_rlr17: HsemRlr17,
    hsem_rlr18: HsemRlr18,
    hsem_rlr19: HsemRlr19,
    hsem_rlr20: HsemRlr20,
    hsem_rlr21: HsemRlr21,
    hsem_rlr22: HsemRlr22,
    hsem_rlr23: HsemRlr23,
    hsem_rlr24: HsemRlr24,
    hsem_rlr25: HsemRlr25,
    hsem_rlr26: HsemRlr26,
    hsem_rlr27: HsemRlr27,
    hsem_rlr28: HsemRlr28,
    hsem_rlr29: HsemRlr29,
    hsem_rlr30: HsemRlr30,
    hsem_rlr31: HsemRlr31,
    hsem_c1ier: HsemC1ier,
    hsem_c1icr: HsemC1icr,
    hsem_c1isr: HsemC1isr,
    hsem_c1misr: HsemC1misr,
    hsem_c2ier: HsemC2ier,
    hsem_c2icr: HsemC2icr,
    hsem_c2isr: HsemC2isr,
    hsem_c2misr: HsemC2misr,
    _reserved72: [u8; 0x20],
    hsem_cr: HsemCr,
    hsem_keyr: HsemKeyr,
}
impl RegisterBlock {
    #[doc = "0x00 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r0(&self) -> &HsemR0 {
        &self.hsem_r0
    }
    #[doc = "0x04 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r1(&self) -> &HsemR1 {
        &self.hsem_r1
    }
    #[doc = "0x08 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r2(&self) -> &HsemR2 {
        &self.hsem_r2
    }
    #[doc = "0x0c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r3(&self) -> &HsemR3 {
        &self.hsem_r3
    }
    #[doc = "0x10 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r4(&self) -> &HsemR4 {
        &self.hsem_r4
    }
    #[doc = "0x14 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r5(&self) -> &HsemR5 {
        &self.hsem_r5
    }
    #[doc = "0x18 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r6(&self) -> &HsemR6 {
        &self.hsem_r6
    }
    #[doc = "0x1c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r7(&self) -> &HsemR7 {
        &self.hsem_r7
    }
    #[doc = "0x20 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r8(&self) -> &HsemR8 {
        &self.hsem_r8
    }
    #[doc = "0x24 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r9(&self) -> &HsemR9 {
        &self.hsem_r9
    }
    #[doc = "0x28 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r10(&self) -> &HsemR10 {
        &self.hsem_r10
    }
    #[doc = "0x2c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r11(&self) -> &HsemR11 {
        &self.hsem_r11
    }
    #[doc = "0x30 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r12(&self) -> &HsemR12 {
        &self.hsem_r12
    }
    #[doc = "0x34 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r13(&self) -> &HsemR13 {
        &self.hsem_r13
    }
    #[doc = "0x38 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r14(&self) -> &HsemR14 {
        &self.hsem_r14
    }
    #[doc = "0x3c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r15(&self) -> &HsemR15 {
        &self.hsem_r15
    }
    #[doc = "0x40 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r16(&self) -> &HsemR16 {
        &self.hsem_r16
    }
    #[doc = "0x44 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r17(&self) -> &HsemR17 {
        &self.hsem_r17
    }
    #[doc = "0x48 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r18(&self) -> &HsemR18 {
        &self.hsem_r18
    }
    #[doc = "0x4c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r19(&self) -> &HsemR19 {
        &self.hsem_r19
    }
    #[doc = "0x50 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r20(&self) -> &HsemR20 {
        &self.hsem_r20
    }
    #[doc = "0x54 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r21(&self) -> &HsemR21 {
        &self.hsem_r21
    }
    #[doc = "0x58 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r22(&self) -> &HsemR22 {
        &self.hsem_r22
    }
    #[doc = "0x5c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r23(&self) -> &HsemR23 {
        &self.hsem_r23
    }
    #[doc = "0x60 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r24(&self) -> &HsemR24 {
        &self.hsem_r24
    }
    #[doc = "0x64 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r25(&self) -> &HsemR25 {
        &self.hsem_r25
    }
    #[doc = "0x68 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r26(&self) -> &HsemR26 {
        &self.hsem_r26
    }
    #[doc = "0x6c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r27(&self) -> &HsemR27 {
        &self.hsem_r27
    }
    #[doc = "0x70 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r28(&self) -> &HsemR28 {
        &self.hsem_r28
    }
    #[doc = "0x74 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r29(&self) -> &HsemR29 {
        &self.hsem_r29
    }
    #[doc = "0x78 - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r30(&self) -> &HsemR30 {
        &self.hsem_r30
    }
    #[doc = "0x7c - HSEM register HSEM_R0 HSEM_R31"]
    #[inline(always)]
    pub const fn hsem_r31(&self) -> &HsemR31 {
        &self.hsem_r31
    }
    #[doc = "0x80 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr0(&self) -> &HsemRlr0 {
        &self.hsem_rlr0
    }
    #[doc = "0x84 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr1(&self) -> &HsemRlr1 {
        &self.hsem_rlr1
    }
    #[doc = "0x88 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr2(&self) -> &HsemRlr2 {
        &self.hsem_rlr2
    }
    #[doc = "0x8c - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr3(&self) -> &HsemRlr3 {
        &self.hsem_rlr3
    }
    #[doc = "0x90 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr4(&self) -> &HsemRlr4 {
        &self.hsem_rlr4
    }
    #[doc = "0x94 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr5(&self) -> &HsemRlr5 {
        &self.hsem_rlr5
    }
    #[doc = "0x98 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr6(&self) -> &HsemRlr6 {
        &self.hsem_rlr6
    }
    #[doc = "0x9c - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr7(&self) -> &HsemRlr7 {
        &self.hsem_rlr7
    }
    #[doc = "0xa0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr8(&self) -> &HsemRlr8 {
        &self.hsem_rlr8
    }
    #[doc = "0xa4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr9(&self) -> &HsemRlr9 {
        &self.hsem_rlr9
    }
    #[doc = "0xa8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr10(&self) -> &HsemRlr10 {
        &self.hsem_rlr10
    }
    #[doc = "0xac - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr11(&self) -> &HsemRlr11 {
        &self.hsem_rlr11
    }
    #[doc = "0xb0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr12(&self) -> &HsemRlr12 {
        &self.hsem_rlr12
    }
    #[doc = "0xb4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr13(&self) -> &HsemRlr13 {
        &self.hsem_rlr13
    }
    #[doc = "0xb8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr14(&self) -> &HsemRlr14 {
        &self.hsem_rlr14
    }
    #[doc = "0xbc - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr15(&self) -> &HsemRlr15 {
        &self.hsem_rlr15
    }
    #[doc = "0xc0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr16(&self) -> &HsemRlr16 {
        &self.hsem_rlr16
    }
    #[doc = "0xc4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr17(&self) -> &HsemRlr17 {
        &self.hsem_rlr17
    }
    #[doc = "0xc8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr18(&self) -> &HsemRlr18 {
        &self.hsem_rlr18
    }
    #[doc = "0xcc - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr19(&self) -> &HsemRlr19 {
        &self.hsem_rlr19
    }
    #[doc = "0xd0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr20(&self) -> &HsemRlr20 {
        &self.hsem_rlr20
    }
    #[doc = "0xd4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr21(&self) -> &HsemRlr21 {
        &self.hsem_rlr21
    }
    #[doc = "0xd8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr22(&self) -> &HsemRlr22 {
        &self.hsem_rlr22
    }
    #[doc = "0xdc - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr23(&self) -> &HsemRlr23 {
        &self.hsem_rlr23
    }
    #[doc = "0xe0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr24(&self) -> &HsemRlr24 {
        &self.hsem_rlr24
    }
    #[doc = "0xe4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr25(&self) -> &HsemRlr25 {
        &self.hsem_rlr25
    }
    #[doc = "0xe8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr26(&self) -> &HsemRlr26 {
        &self.hsem_rlr26
    }
    #[doc = "0xec - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr27(&self) -> &HsemRlr27 {
        &self.hsem_rlr27
    }
    #[doc = "0xf0 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr28(&self) -> &HsemRlr28 {
        &self.hsem_rlr28
    }
    #[doc = "0xf4 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr29(&self) -> &HsemRlr29 {
        &self.hsem_rlr29
    }
    #[doc = "0xf8 - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr30(&self) -> &HsemRlr30 {
        &self.hsem_rlr30
    }
    #[doc = "0xfc - HSEM Read lock register"]
    #[inline(always)]
    pub const fn hsem_rlr31(&self) -> &HsemRlr31 {
        &self.hsem_rlr31
    }
    #[doc = "0x100 - HSEM Interrupt enable register"]
    #[inline(always)]
    pub const fn hsem_c1ier(&self) -> &HsemC1ier {
        &self.hsem_c1ier
    }
    #[doc = "0x104 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn hsem_c1icr(&self) -> &HsemC1icr {
        &self.hsem_c1icr
    }
    #[doc = "0x108 - HSEM Interrupt status register"]
    #[inline(always)]
    pub const fn hsem_c1isr(&self) -> &HsemC1isr {
        &self.hsem_c1isr
    }
    #[doc = "0x10c - HSEM Masked interrupt status register"]
    #[inline(always)]
    pub const fn hsem_c1misr(&self) -> &HsemC1misr {
        &self.hsem_c1misr
    }
    #[doc = "0x110 - HSEM Interrupt enable register"]
    #[inline(always)]
    pub const fn hsem_c2ier(&self) -> &HsemC2ier {
        &self.hsem_c2ier
    }
    #[doc = "0x114 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn hsem_c2icr(&self) -> &HsemC2icr {
        &self.hsem_c2icr
    }
    #[doc = "0x118 - HSEM Interrupt status register"]
    #[inline(always)]
    pub const fn hsem_c2isr(&self) -> &HsemC2isr {
        &self.hsem_c2isr
    }
    #[doc = "0x11c - HSEM Masked interrupt status register"]
    #[inline(always)]
    pub const fn hsem_c2misr(&self) -> &HsemC2misr {
        &self.hsem_c2misr
    }
    #[doc = "0x140 - HSEM Clear register"]
    #[inline(always)]
    pub const fn hsem_cr(&self) -> &HsemCr {
        &self.hsem_cr
    }
    #[doc = "0x144 - HSEM Interrupt clear register"]
    #[inline(always)]
    pub const fn hsem_keyr(&self) -> &HsemKeyr {
        &self.hsem_keyr
    }
}
#[doc = "HSEM_R0 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r0`] module"]
#[doc(alias = "HSEM_R0")]
pub type HsemR0 = crate::Reg<hsem_r0::HsemR0Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r0;
#[doc = "HSEM_R1 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r1`] module"]
#[doc(alias = "HSEM_R1")]
pub type HsemR1 = crate::Reg<hsem_r1::HsemR1Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r1;
#[doc = "HSEM_R2 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r2`] module"]
#[doc(alias = "HSEM_R2")]
pub type HsemR2 = crate::Reg<hsem_r2::HsemR2Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r2;
#[doc = "HSEM_R3 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r3`] module"]
#[doc(alias = "HSEM_R3")]
pub type HsemR3 = crate::Reg<hsem_r3::HsemR3Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r3;
#[doc = "HSEM_R4 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r4::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r4::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r4`] module"]
#[doc(alias = "HSEM_R4")]
pub type HsemR4 = crate::Reg<hsem_r4::HsemR4Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r4;
#[doc = "HSEM_R5 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r5::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r5::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r5`] module"]
#[doc(alias = "HSEM_R5")]
pub type HsemR5 = crate::Reg<hsem_r5::HsemR5Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r5;
#[doc = "HSEM_R6 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r6::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r6::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r6`] module"]
#[doc(alias = "HSEM_R6")]
pub type HsemR6 = crate::Reg<hsem_r6::HsemR6Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r6;
#[doc = "HSEM_R7 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r7::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r7::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r7`] module"]
#[doc(alias = "HSEM_R7")]
pub type HsemR7 = crate::Reg<hsem_r7::HsemR7Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r7;
#[doc = "HSEM_R8 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r8::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r8::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r8`] module"]
#[doc(alias = "HSEM_R8")]
pub type HsemR8 = crate::Reg<hsem_r8::HsemR8Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r8;
#[doc = "HSEM_R9 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r9::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r9::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r9`] module"]
#[doc(alias = "HSEM_R9")]
pub type HsemR9 = crate::Reg<hsem_r9::HsemR9Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r9;
#[doc = "HSEM_R10 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r10::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r10::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r10`] module"]
#[doc(alias = "HSEM_R10")]
pub type HsemR10 = crate::Reg<hsem_r10::HsemR10Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r10;
#[doc = "HSEM_R11 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r11::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r11::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r11`] module"]
#[doc(alias = "HSEM_R11")]
pub type HsemR11 = crate::Reg<hsem_r11::HsemR11Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r11;
#[doc = "HSEM_R12 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r12::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r12::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r12`] module"]
#[doc(alias = "HSEM_R12")]
pub type HsemR12 = crate::Reg<hsem_r12::HsemR12Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r12;
#[doc = "HSEM_R13 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r13::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r13::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r13`] module"]
#[doc(alias = "HSEM_R13")]
pub type HsemR13 = crate::Reg<hsem_r13::HsemR13Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r13;
#[doc = "HSEM_R14 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r14::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r14::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r14`] module"]
#[doc(alias = "HSEM_R14")]
pub type HsemR14 = crate::Reg<hsem_r14::HsemR14Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r14;
#[doc = "HSEM_R15 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r15::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r15::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r15`] module"]
#[doc(alias = "HSEM_R15")]
pub type HsemR15 = crate::Reg<hsem_r15::HsemR15Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r15;
#[doc = "HSEM_R16 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r16::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r16::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r16`] module"]
#[doc(alias = "HSEM_R16")]
pub type HsemR16 = crate::Reg<hsem_r16::HsemR16Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r16;
#[doc = "HSEM_R17 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r17::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r17::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r17`] module"]
#[doc(alias = "HSEM_R17")]
pub type HsemR17 = crate::Reg<hsem_r17::HsemR17Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r17;
#[doc = "HSEM_R18 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r18::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r18::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r18`] module"]
#[doc(alias = "HSEM_R18")]
pub type HsemR18 = crate::Reg<hsem_r18::HsemR18Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r18;
#[doc = "HSEM_R19 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r19::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r19::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r19`] module"]
#[doc(alias = "HSEM_R19")]
pub type HsemR19 = crate::Reg<hsem_r19::HsemR19Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r19;
#[doc = "HSEM_R20 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r20::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r20::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r20`] module"]
#[doc(alias = "HSEM_R20")]
pub type HsemR20 = crate::Reg<hsem_r20::HsemR20Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r20;
#[doc = "HSEM_R21 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r21::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r21::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r21`] module"]
#[doc(alias = "HSEM_R21")]
pub type HsemR21 = crate::Reg<hsem_r21::HsemR21Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r21;
#[doc = "HSEM_R22 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r22::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r22::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r22`] module"]
#[doc(alias = "HSEM_R22")]
pub type HsemR22 = crate::Reg<hsem_r22::HsemR22Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r22;
#[doc = "HSEM_R23 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r23::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r23::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r23`] module"]
#[doc(alias = "HSEM_R23")]
pub type HsemR23 = crate::Reg<hsem_r23::HsemR23Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r23;
#[doc = "HSEM_R24 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r24::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r24::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r24`] module"]
#[doc(alias = "HSEM_R24")]
pub type HsemR24 = crate::Reg<hsem_r24::HsemR24Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r24;
#[doc = "HSEM_R25 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r25::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r25::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r25`] module"]
#[doc(alias = "HSEM_R25")]
pub type HsemR25 = crate::Reg<hsem_r25::HsemR25Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r25;
#[doc = "HSEM_R26 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r26::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r26::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r26`] module"]
#[doc(alias = "HSEM_R26")]
pub type HsemR26 = crate::Reg<hsem_r26::HsemR26Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r26;
#[doc = "HSEM_R27 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r27::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r27::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r27`] module"]
#[doc(alias = "HSEM_R27")]
pub type HsemR27 = crate::Reg<hsem_r27::HsemR27Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r27;
#[doc = "HSEM_R28 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r28::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r28::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r28`] module"]
#[doc(alias = "HSEM_R28")]
pub type HsemR28 = crate::Reg<hsem_r28::HsemR28Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r28;
#[doc = "HSEM_R29 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r29::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r29::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r29`] module"]
#[doc(alias = "HSEM_R29")]
pub type HsemR29 = crate::Reg<hsem_r29::HsemR29Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r29;
#[doc = "HSEM_R30 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r30::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r30::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r30`] module"]
#[doc(alias = "HSEM_R30")]
pub type HsemR30 = crate::Reg<hsem_r30::HsemR30Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r30;
#[doc = "HSEM_R31 (rw) register accessor: HSEM register HSEM_R0 HSEM_R31\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_r31::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_r31::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_r31`] module"]
#[doc(alias = "HSEM_R31")]
pub type HsemR31 = crate::Reg<hsem_r31::HsemR31Spec>;
#[doc = "HSEM register HSEM_R0 HSEM_R31"]
pub mod hsem_r31;
#[doc = "HSEM_RLR0 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr0`] module"]
#[doc(alias = "HSEM_RLR0")]
pub type HsemRlr0 = crate::Reg<hsem_rlr0::HsemRlr0Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr0;
#[doc = "HSEM_RLR1 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr1`] module"]
#[doc(alias = "HSEM_RLR1")]
pub type HsemRlr1 = crate::Reg<hsem_rlr1::HsemRlr1Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr1;
#[doc = "HSEM_RLR2 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr2`] module"]
#[doc(alias = "HSEM_RLR2")]
pub type HsemRlr2 = crate::Reg<hsem_rlr2::HsemRlr2Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr2;
#[doc = "HSEM_RLR3 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr3`] module"]
#[doc(alias = "HSEM_RLR3")]
pub type HsemRlr3 = crate::Reg<hsem_rlr3::HsemRlr3Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr3;
#[doc = "HSEM_RLR4 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr4`] module"]
#[doc(alias = "HSEM_RLR4")]
pub type HsemRlr4 = crate::Reg<hsem_rlr4::HsemRlr4Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr4;
#[doc = "HSEM_RLR5 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr5::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr5`] module"]
#[doc(alias = "HSEM_RLR5")]
pub type HsemRlr5 = crate::Reg<hsem_rlr5::HsemRlr5Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr5;
#[doc = "HSEM_RLR6 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr6::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr6`] module"]
#[doc(alias = "HSEM_RLR6")]
pub type HsemRlr6 = crate::Reg<hsem_rlr6::HsemRlr6Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr6;
#[doc = "HSEM_RLR7 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr7::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr7`] module"]
#[doc(alias = "HSEM_RLR7")]
pub type HsemRlr7 = crate::Reg<hsem_rlr7::HsemRlr7Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr7;
#[doc = "HSEM_RLR8 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr8::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr8`] module"]
#[doc(alias = "HSEM_RLR8")]
pub type HsemRlr8 = crate::Reg<hsem_rlr8::HsemRlr8Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr8;
#[doc = "HSEM_RLR9 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr9::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr9`] module"]
#[doc(alias = "HSEM_RLR9")]
pub type HsemRlr9 = crate::Reg<hsem_rlr9::HsemRlr9Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr9;
#[doc = "HSEM_RLR10 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr10::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr10`] module"]
#[doc(alias = "HSEM_RLR10")]
pub type HsemRlr10 = crate::Reg<hsem_rlr10::HsemRlr10Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr10;
#[doc = "HSEM_RLR11 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr11::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr11`] module"]
#[doc(alias = "HSEM_RLR11")]
pub type HsemRlr11 = crate::Reg<hsem_rlr11::HsemRlr11Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr11;
#[doc = "HSEM_RLR12 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr12::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr12`] module"]
#[doc(alias = "HSEM_RLR12")]
pub type HsemRlr12 = crate::Reg<hsem_rlr12::HsemRlr12Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr12;
#[doc = "HSEM_RLR13 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr13::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr13`] module"]
#[doc(alias = "HSEM_RLR13")]
pub type HsemRlr13 = crate::Reg<hsem_rlr13::HsemRlr13Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr13;
#[doc = "HSEM_RLR14 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr14::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr14`] module"]
#[doc(alias = "HSEM_RLR14")]
pub type HsemRlr14 = crate::Reg<hsem_rlr14::HsemRlr14Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr14;
#[doc = "HSEM_RLR15 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr15::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr15`] module"]
#[doc(alias = "HSEM_RLR15")]
pub type HsemRlr15 = crate::Reg<hsem_rlr15::HsemRlr15Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr15;
#[doc = "HSEM_RLR16 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr16::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr16`] module"]
#[doc(alias = "HSEM_RLR16")]
pub type HsemRlr16 = crate::Reg<hsem_rlr16::HsemRlr16Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr16;
#[doc = "HSEM_RLR17 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr17::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr17`] module"]
#[doc(alias = "HSEM_RLR17")]
pub type HsemRlr17 = crate::Reg<hsem_rlr17::HsemRlr17Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr17;
#[doc = "HSEM_RLR18 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr18::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr18`] module"]
#[doc(alias = "HSEM_RLR18")]
pub type HsemRlr18 = crate::Reg<hsem_rlr18::HsemRlr18Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr18;
#[doc = "HSEM_RLR19 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr19::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr19`] module"]
#[doc(alias = "HSEM_RLR19")]
pub type HsemRlr19 = crate::Reg<hsem_rlr19::HsemRlr19Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr19;
#[doc = "HSEM_RLR20 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr20::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr20`] module"]
#[doc(alias = "HSEM_RLR20")]
pub type HsemRlr20 = crate::Reg<hsem_rlr20::HsemRlr20Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr20;
#[doc = "HSEM_RLR21 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr21::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr21`] module"]
#[doc(alias = "HSEM_RLR21")]
pub type HsemRlr21 = crate::Reg<hsem_rlr21::HsemRlr21Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr21;
#[doc = "HSEM_RLR22 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr22::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr22`] module"]
#[doc(alias = "HSEM_RLR22")]
pub type HsemRlr22 = crate::Reg<hsem_rlr22::HsemRlr22Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr22;
#[doc = "HSEM_RLR23 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr23::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr23`] module"]
#[doc(alias = "HSEM_RLR23")]
pub type HsemRlr23 = crate::Reg<hsem_rlr23::HsemRlr23Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr23;
#[doc = "HSEM_RLR24 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr24::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr24`] module"]
#[doc(alias = "HSEM_RLR24")]
pub type HsemRlr24 = crate::Reg<hsem_rlr24::HsemRlr24Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr24;
#[doc = "HSEM_RLR25 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr25::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr25`] module"]
#[doc(alias = "HSEM_RLR25")]
pub type HsemRlr25 = crate::Reg<hsem_rlr25::HsemRlr25Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr25;
#[doc = "HSEM_RLR26 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr26::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr26`] module"]
#[doc(alias = "HSEM_RLR26")]
pub type HsemRlr26 = crate::Reg<hsem_rlr26::HsemRlr26Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr26;
#[doc = "HSEM_RLR27 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr27::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr27`] module"]
#[doc(alias = "HSEM_RLR27")]
pub type HsemRlr27 = crate::Reg<hsem_rlr27::HsemRlr27Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr27;
#[doc = "HSEM_RLR28 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr28::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr28`] module"]
#[doc(alias = "HSEM_RLR28")]
pub type HsemRlr28 = crate::Reg<hsem_rlr28::HsemRlr28Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr28;
#[doc = "HSEM_RLR29 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr29::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr29`] module"]
#[doc(alias = "HSEM_RLR29")]
pub type HsemRlr29 = crate::Reg<hsem_rlr29::HsemRlr29Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr29;
#[doc = "HSEM_RLR30 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr30::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr30`] module"]
#[doc(alias = "HSEM_RLR30")]
pub type HsemRlr30 = crate::Reg<hsem_rlr30::HsemRlr30Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr30;
#[doc = "HSEM_RLR31 (r) register accessor: HSEM Read lock register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_rlr31::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_rlr31`] module"]
#[doc(alias = "HSEM_RLR31")]
pub type HsemRlr31 = crate::Reg<hsem_rlr31::HsemRlr31Spec>;
#[doc = "HSEM Read lock register"]
pub mod hsem_rlr31;
#[doc = "HSEM_C1IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c1ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c1ier`] module"]
#[doc(alias = "HSEM_C1IER")]
pub type HsemC1ier = crate::Reg<hsem_c1ier::HsemC1ierSpec>;
#[doc = "HSEM Interrupt enable register"]
pub mod hsem_c1ier;
#[doc = "HSEM_C1ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c1icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c1icr`] module"]
#[doc(alias = "HSEM_C1ICR")]
pub type HsemC1icr = crate::Reg<hsem_c1icr::HsemC1icrSpec>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_c1icr;
#[doc = "HSEM_C1ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c1isr`] module"]
#[doc(alias = "HSEM_C1ISR")]
pub type HsemC1isr = crate::Reg<hsem_c1isr::HsemC1isrSpec>;
#[doc = "HSEM Interrupt status register"]
pub mod hsem_c1isr;
#[doc = "HSEM_C1MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c1misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c1misr`] module"]
#[doc(alias = "HSEM_C1MISR")]
pub type HsemC1misr = crate::Reg<hsem_c1misr::HsemC1misrSpec>;
#[doc = "HSEM Masked interrupt status register"]
pub mod hsem_c1misr;
#[doc = "HSEM_C2IER (rw) register accessor: HSEM Interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2ier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2ier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c2ier`] module"]
#[doc(alias = "HSEM_C2IER")]
pub type HsemC2ier = crate::Reg<hsem_c2ier::HsemC2ierSpec>;
#[doc = "HSEM Interrupt enable register"]
pub mod hsem_c2ier;
#[doc = "HSEM_C2ICR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_c2icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c2icr`] module"]
#[doc(alias = "HSEM_C2ICR")]
pub type HsemC2icr = crate::Reg<hsem_c2icr::HsemC2icrSpec>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_c2icr;
#[doc = "HSEM_C2ISR (r) register accessor: HSEM Interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c2isr`] module"]
#[doc(alias = "HSEM_C2ISR")]
pub type HsemC2isr = crate::Reg<hsem_c2isr::HsemC2isrSpec>;
#[doc = "HSEM Interrupt status register"]
pub mod hsem_c2isr;
#[doc = "HSEM_C2MISR (r) register accessor: HSEM Masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_c2misr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_c2misr`] module"]
#[doc(alias = "HSEM_C2MISR")]
pub type HsemC2misr = crate::Reg<hsem_c2misr::HsemC2misrSpec>;
#[doc = "HSEM Masked interrupt status register"]
pub mod hsem_c2misr;
#[doc = "HSEM_CR (w) register accessor: HSEM Clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_cr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_cr`] module"]
#[doc(alias = "HSEM_CR")]
pub type HsemCr = crate::Reg<hsem_cr::HsemCrSpec>;
#[doc = "HSEM Clear register"]
pub mod hsem_cr;
#[doc = "HSEM_KEYR (rw) register accessor: HSEM Interrupt clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsem_keyr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsem_keyr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hsem_keyr`] module"]
#[doc(alias = "HSEM_KEYR")]
pub type HsemKeyr = crate::Reg<hsem_keyr::HsemKeyrSpec>;
#[doc = "HSEM Interrupt clear register"]
pub mod hsem_keyr;
