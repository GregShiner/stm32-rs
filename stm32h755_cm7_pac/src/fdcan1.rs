#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fdcan_crel: FdcanCrel,
    fdcan_endn: FdcanEndn,
    _reserved2: [u8; 0x04],
    fdcan_dbtp: FdcanDbtp,
    fdcan_test: FdcanTest,
    fdcan_rwd: FdcanRwd,
    fdcan_cccr: FdcanCccr,
    fdcan_nbtp: FdcanNbtp,
    fdcan_tscc: FdcanTscc,
    fdcan_tscv: FdcanTscv,
    fdcan_tocc: FdcanTocc,
    fdcan_tocv: FdcanTocv,
    _reserved11: [u8; 0x10],
    fdcan_ecr: FdcanEcr,
    fdcan_psr: FdcanPsr,
    fdcan_tdcr: FdcanTdcr,
    _reserved14: [u8; 0x04],
    fdcan_ir: FdcanIr,
    fdcan_ie: FdcanIe,
    fdcan_ils: FdcanIls,
    fdcan_ile: FdcanIle,
    _reserved18: [u8; 0x20],
    fdcan_gfc: FdcanGfc,
    fdcan_sidfc: FdcanSidfc,
    fdcan_xidfc: FdcanXidfc,
    _reserved21: [u8; 0x04],
    fdcan_xidam: FdcanXidam,
    fdcan_hpms: FdcanHpms,
    fdcan_ndat1: FdcanNdat1,
    fdcan_ndat2: FdcanNdat2,
    fdcan_rxf0c: FdcanRxf0c,
    fdcan_rxf0s: FdcanRxf0s,
    fdcan_rxf0a: FdcanRxf0a,
    fdcan_rxbc: FdcanRxbc,
    fdcan_rxf1c: FdcanRxf1c,
    fdcan_rxf1s: FdcanRxf1s,
    fdcan_rxf1a: FdcanRxf1a,
    fdcan_rxesc: FdcanRxesc,
    fdcan_txbc: FdcanTxbc,
    fdcan_txfqs: FdcanTxfqs,
    fdcan_txesc: FdcanTxesc,
    fdcan_txbrp: FdcanTxbrp,
    fdcan_txbar: FdcanTxbar,
    fdcan_txbcr: FdcanTxbcr,
    fdcan_txbto: FdcanTxbto,
    fdcan_txbcf: FdcanTxbcf,
    fdcan_txbtie: FdcanTxbtie,
    fdcan_txbcie: FdcanTxbcie,
    _reserved43: [u8; 0x08],
    fdcan_txefc: FdcanTxefc,
    fdcan_txefs: FdcanTxefs,
    fdcan_txefa: FdcanTxefa,
    _reserved46: [u8; 0x04],
    fdcan_tttmc: FdcanTttmc,
    fdcan_ttrmc: FdcanTtrmc,
    fdcan_ttocf: FdcanTtocf,
    fdcan_ttmlm: FdcanTtmlm,
    fdcan_turcf: FdcanTurcf,
    fdcan_ttocn: FdcanTtocn,
    can_ttgtp: CanTtgtp,
    fdcan_tttmk: FdcanTttmk,
    fdcan_ttir: FdcanTtir,
    fdcan_ttie: FdcanTtie,
    fdcan_ttils: FdcanTtils,
    fdcan_ttost: FdcanTtost,
    fdcan_turna: FdcanTurna,
    fdcan_ttlgt: FdcanTtlgt,
    fdcan_ttctc: FdcanTtctc,
    fdcan_ttcpt: FdcanTtcpt,
    fdcan_ttcsm: FdcanTtcsm,
    _reserved63: [u8; 0x01bc],
    fdcan_ttts: FdcanTtts,
}
impl RegisterBlock {
    #[doc = "0x00 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn fdcan_crel(&self) -> &FdcanCrel {
        &self.fdcan_crel
    }
    #[doc = "0x04 - FDCAN Core Release Register"]
    #[inline(always)]
    pub const fn fdcan_endn(&self) -> &FdcanEndn {
        &self.fdcan_endn
    }
    #[doc = "0x0c - FDCAN Data Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn fdcan_dbtp(&self) -> &FdcanDbtp {
        &self.fdcan_dbtp
    }
    #[doc = "0x10 - FDCAN Test Register"]
    #[inline(always)]
    pub const fn fdcan_test(&self) -> &FdcanTest {
        &self.fdcan_test
    }
    #[doc = "0x14 - FDCAN RAM Watchdog Register"]
    #[inline(always)]
    pub const fn fdcan_rwd(&self) -> &FdcanRwd {
        &self.fdcan_rwd
    }
    #[doc = "0x18 - FDCAN CC Control Register"]
    #[inline(always)]
    pub const fn fdcan_cccr(&self) -> &FdcanCccr {
        &self.fdcan_cccr
    }
    #[doc = "0x1c - FDCAN Nominal Bit Timing and Prescaler Register"]
    #[inline(always)]
    pub const fn fdcan_nbtp(&self) -> &FdcanNbtp {
        &self.fdcan_nbtp
    }
    #[doc = "0x20 - FDCAN Timestamp Counter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tscc(&self) -> &FdcanTscc {
        &self.fdcan_tscc
    }
    #[doc = "0x24 - FDCAN Timestamp Counter Value Register"]
    #[inline(always)]
    pub const fn fdcan_tscv(&self) -> &FdcanTscv {
        &self.fdcan_tscv
    }
    #[doc = "0x28 - FDCAN Timeout Counter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tocc(&self) -> &FdcanTocc {
        &self.fdcan_tocc
    }
    #[doc = "0x2c - FDCAN Timeout Counter Value Register"]
    #[inline(always)]
    pub const fn fdcan_tocv(&self) -> &FdcanTocv {
        &self.fdcan_tocv
    }
    #[doc = "0x40 - FDCAN Error Counter Register"]
    #[inline(always)]
    pub const fn fdcan_ecr(&self) -> &FdcanEcr {
        &self.fdcan_ecr
    }
    #[doc = "0x44 - FDCAN Protocol Status Register"]
    #[inline(always)]
    pub const fn fdcan_psr(&self) -> &FdcanPsr {
        &self.fdcan_psr
    }
    #[doc = "0x48 - FDCAN Transmitter Delay Compensation Register"]
    #[inline(always)]
    pub const fn fdcan_tdcr(&self) -> &FdcanTdcr {
        &self.fdcan_tdcr
    }
    #[doc = "0x50 - FDCAN Interrupt Register"]
    #[inline(always)]
    pub const fn fdcan_ir(&self) -> &FdcanIr {
        &self.fdcan_ir
    }
    #[doc = "0x54 - FDCAN Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ie(&self) -> &FdcanIe {
        &self.fdcan_ie
    }
    #[doc = "0x58 - FDCAN Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn fdcan_ils(&self) -> &FdcanIls {
        &self.fdcan_ils
    }
    #[doc = "0x5c - FDCAN Interrupt Line Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ile(&self) -> &FdcanIle {
        &self.fdcan_ile
    }
    #[doc = "0x80 - FDCAN Global Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_gfc(&self) -> &FdcanGfc {
        &self.fdcan_gfc
    }
    #[doc = "0x84 - FDCAN Standard ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_sidfc(&self) -> &FdcanSidfc {
        &self.fdcan_sidfc
    }
    #[doc = "0x88 - FDCAN Extended ID Filter Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_xidfc(&self) -> &FdcanXidfc {
        &self.fdcan_xidfc
    }
    #[doc = "0x90 - FDCAN Extended ID and Mask Register"]
    #[inline(always)]
    pub const fn fdcan_xidam(&self) -> &FdcanXidam {
        &self.fdcan_xidam
    }
    #[doc = "0x94 - FDCAN High Priority Message Status Register"]
    #[inline(always)]
    pub const fn fdcan_hpms(&self) -> &FdcanHpms {
        &self.fdcan_hpms
    }
    #[doc = "0x98 - FDCAN New Data 1 Register"]
    #[inline(always)]
    pub const fn fdcan_ndat1(&self) -> &FdcanNdat1 {
        &self.fdcan_ndat1
    }
    #[doc = "0x9c - FDCAN New Data 2 Register"]
    #[inline(always)]
    pub const fn fdcan_ndat2(&self) -> &FdcanNdat2 {
        &self.fdcan_ndat2
    }
    #[doc = "0xa0 - FDCAN Rx FIFO 0 Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0c(&self) -> &FdcanRxf0c {
        &self.fdcan_rxf0c
    }
    #[doc = "0xa4 - FDCAN Rx FIFO 0 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0s(&self) -> &FdcanRxf0s {
        &self.fdcan_rxf0s
    }
    #[doc = "0xa8 - CAN Rx FIFO 0 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf0a(&self) -> &FdcanRxf0a {
        &self.fdcan_rxf0a
    }
    #[doc = "0xac - FDCAN Rx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxbc(&self) -> &FdcanRxbc {
        &self.fdcan_rxbc
    }
    #[doc = "0xb0 - FDCAN Rx FIFO 1 Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1c(&self) -> &FdcanRxf1c {
        &self.fdcan_rxf1c
    }
    #[doc = "0xb4 - FDCAN Rx FIFO 1 Status Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1s(&self) -> &FdcanRxf1s {
        &self.fdcan_rxf1s
    }
    #[doc = "0xb8 - FDCAN Rx FIFO 1 Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_rxf1a(&self) -> &FdcanRxf1a {
        &self.fdcan_rxf1a
    }
    #[doc = "0xbc - FDCAN Rx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_rxesc(&self) -> &FdcanRxesc {
        &self.fdcan_rxesc
    }
    #[doc = "0xc0 - FDCAN Tx Buffer Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txbc(&self) -> &FdcanTxbc {
        &self.fdcan_txbc
    }
    #[doc = "0xc4 - FDCAN Tx FIFO/Queue Status Register"]
    #[inline(always)]
    pub const fn fdcan_txfqs(&self) -> &FdcanTxfqs {
        &self.fdcan_txfqs
    }
    #[doc = "0xc8 - FDCAN Tx Buffer Element Size Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txesc(&self) -> &FdcanTxesc {
        &self.fdcan_txesc
    }
    #[doc = "0xcc - FDCAN Tx Buffer Request Pending Register"]
    #[inline(always)]
    pub const fn fdcan_txbrp(&self) -> &FdcanTxbrp {
        &self.fdcan_txbrp
    }
    #[doc = "0xd0 - FDCAN Tx Buffer Add Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbar(&self) -> &FdcanTxbar {
        &self.fdcan_txbar
    }
    #[doc = "0xd4 - FDCAN Tx Buffer Cancellation Request Register"]
    #[inline(always)]
    pub const fn fdcan_txbcr(&self) -> &FdcanTxbcr {
        &self.fdcan_txbcr
    }
    #[doc = "0xd8 - FDCAN Tx Buffer Transmission Occurred Register"]
    #[inline(always)]
    pub const fn fdcan_txbto(&self) -> &FdcanTxbto {
        &self.fdcan_txbto
    }
    #[doc = "0xdc - FDCAN Tx Buffer Cancellation Finished Register"]
    #[inline(always)]
    pub const fn fdcan_txbcf(&self) -> &FdcanTxbcf {
        &self.fdcan_txbcf
    }
    #[doc = "0xe0 - FDCAN Tx Buffer Transmission Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbtie(&self) -> &FdcanTxbtie {
        &self.fdcan_txbtie
    }
    #[doc = "0xe4 - FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_txbcie(&self) -> &FdcanTxbcie {
        &self.fdcan_txbcie
    }
    #[doc = "0xf0 - FDCAN Tx Event FIFO Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_txefc(&self) -> &FdcanTxefc {
        &self.fdcan_txefc
    }
    #[doc = "0xf4 - FDCAN Tx Event FIFO Status Register"]
    #[inline(always)]
    pub const fn fdcan_txefs(&self) -> &FdcanTxefs {
        &self.fdcan_txefs
    }
    #[doc = "0xf8 - FDCAN Tx Event FIFO Acknowledge Register"]
    #[inline(always)]
    pub const fn fdcan_txefa(&self) -> &FdcanTxefa {
        &self.fdcan_txefa
    }
    #[doc = "0x100 - FDCAN TT Trigger Memory Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_tttmc(&self) -> &FdcanTttmc {
        &self.fdcan_tttmc
    }
    #[doc = "0x104 - FDCAN TT Reference Message Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_ttrmc(&self) -> &FdcanTtrmc {
        &self.fdcan_ttrmc
    }
    #[doc = "0x108 - FDCAN TT Operation Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_ttocf(&self) -> &FdcanTtocf {
        &self.fdcan_ttocf
    }
    #[doc = "0x10c - FDCAN TT Matrix Limits Register"]
    #[inline(always)]
    pub const fn fdcan_ttmlm(&self) -> &FdcanTtmlm {
        &self.fdcan_ttmlm
    }
    #[doc = "0x110 - FDCAN TUR Configuration Register"]
    #[inline(always)]
    pub const fn fdcan_turcf(&self) -> &FdcanTurcf {
        &self.fdcan_turcf
    }
    #[doc = "0x114 - FDCAN TT Operation Control Register"]
    #[inline(always)]
    pub const fn fdcan_ttocn(&self) -> &FdcanTtocn {
        &self.fdcan_ttocn
    }
    #[doc = "0x118 - FDCAN TT Global Time Preset Register"]
    #[inline(always)]
    pub const fn can_ttgtp(&self) -> &CanTtgtp {
        &self.can_ttgtp
    }
    #[doc = "0x11c - FDCAN TT Time Mark Register"]
    #[inline(always)]
    pub const fn fdcan_tttmk(&self) -> &FdcanTttmk {
        &self.fdcan_tttmk
    }
    #[doc = "0x120 - FDCAN TT Interrupt Register"]
    #[inline(always)]
    pub const fn fdcan_ttir(&self) -> &FdcanTtir {
        &self.fdcan_ttir
    }
    #[doc = "0x124 - FDCAN TT Interrupt Enable Register"]
    #[inline(always)]
    pub const fn fdcan_ttie(&self) -> &FdcanTtie {
        &self.fdcan_ttie
    }
    #[doc = "0x128 - FDCAN TT Interrupt Line Select Register"]
    #[inline(always)]
    pub const fn fdcan_ttils(&self) -> &FdcanTtils {
        &self.fdcan_ttils
    }
    #[doc = "0x12c - FDCAN TT Operation Status Register"]
    #[inline(always)]
    pub const fn fdcan_ttost(&self) -> &FdcanTtost {
        &self.fdcan_ttost
    }
    #[doc = "0x130 - FDCAN TUR Numerator Actual Register"]
    #[inline(always)]
    pub const fn fdcan_turna(&self) -> &FdcanTurna {
        &self.fdcan_turna
    }
    #[doc = "0x134 - FDCAN TT Local and Global Time Register"]
    #[inline(always)]
    pub const fn fdcan_ttlgt(&self) -> &FdcanTtlgt {
        &self.fdcan_ttlgt
    }
    #[doc = "0x138 - FDCAN TT Cycle Time and Count Register"]
    #[inline(always)]
    pub const fn fdcan_ttctc(&self) -> &FdcanTtctc {
        &self.fdcan_ttctc
    }
    #[doc = "0x13c - FDCAN TT Capture Time Register"]
    #[inline(always)]
    pub const fn fdcan_ttcpt(&self) -> &FdcanTtcpt {
        &self.fdcan_ttcpt
    }
    #[doc = "0x140 - FDCAN TT Cycle Sync Mark Register"]
    #[inline(always)]
    pub const fn fdcan_ttcsm(&self) -> &FdcanTtcsm {
        &self.fdcan_ttcsm
    }
    #[doc = "0x300 - FDCAN TT Trigger Select Register"]
    #[inline(always)]
    pub const fn fdcan_ttts(&self) -> &FdcanTtts {
        &self.fdcan_ttts
    }
}
#[doc = "FDCAN_CREL (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_crel::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_crel`] module"]
#[doc(alias = "FDCAN_CREL")]
pub type FdcanCrel = crate::Reg<fdcan_crel::FdcanCrelSpec>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_crel;
#[doc = "FDCAN_ENDN (r) register accessor: FDCAN Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_endn::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_endn`] module"]
#[doc(alias = "FDCAN_ENDN")]
pub type FdcanEndn = crate::Reg<fdcan_endn::FdcanEndnSpec>;
#[doc = "FDCAN Core Release Register"]
pub mod fdcan_endn;
#[doc = "FDCAN_DBTP (r) register accessor: FDCAN Data Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_dbtp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_dbtp`] module"]
#[doc(alias = "FDCAN_DBTP")]
pub type FdcanDbtp = crate::Reg<fdcan_dbtp::FdcanDbtpSpec>;
#[doc = "FDCAN Data Bit Timing and Prescaler Register"]
pub mod fdcan_dbtp;
#[doc = "FDCAN_TEST (r) register accessor: FDCAN Test Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_test::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_test`] module"]
#[doc(alias = "FDCAN_TEST")]
pub type FdcanTest = crate::Reg<fdcan_test::FdcanTestSpec>;
#[doc = "FDCAN Test Register"]
pub mod fdcan_test;
#[doc = "FDCAN_RWD (r) register accessor: FDCAN RAM Watchdog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rwd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rwd`] module"]
#[doc(alias = "FDCAN_RWD")]
pub type FdcanRwd = crate::Reg<fdcan_rwd::FdcanRwdSpec>;
#[doc = "FDCAN RAM Watchdog Register"]
pub mod fdcan_rwd;
#[doc = "FDCAN_CCCR (rw) register accessor: FDCAN CC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_cccr`] module"]
#[doc(alias = "FDCAN_CCCR")]
pub type FdcanCccr = crate::Reg<fdcan_cccr::FdcanCccrSpec>;
#[doc = "FDCAN CC Control Register"]
pub mod fdcan_cccr;
#[doc = "FDCAN_NBTP (rw) register accessor: FDCAN Nominal Bit Timing and Prescaler Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_nbtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_nbtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_nbtp`] module"]
#[doc(alias = "FDCAN_NBTP")]
pub type FdcanNbtp = crate::Reg<fdcan_nbtp::FdcanNbtpSpec>;
#[doc = "FDCAN Nominal Bit Timing and Prescaler Register"]
pub mod fdcan_nbtp;
#[doc = "FDCAN_TSCC (rw) register accessor: FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscc`] module"]
#[doc(alias = "FDCAN_TSCC")]
pub type FdcanTscc = crate::Reg<fdcan_tscc::FdcanTsccSpec>;
#[doc = "FDCAN Timestamp Counter Configuration Register"]
pub mod fdcan_tscc;
#[doc = "FDCAN_TSCV (rw) register accessor: FDCAN Timestamp Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tscv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tscv`] module"]
#[doc(alias = "FDCAN_TSCV")]
pub type FdcanTscv = crate::Reg<fdcan_tscv::FdcanTscvSpec>;
#[doc = "FDCAN Timestamp Counter Value Register"]
pub mod fdcan_tscv;
#[doc = "FDCAN_TOCC (rw) register accessor: FDCAN Timeout Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocc`] module"]
#[doc(alias = "FDCAN_TOCC")]
pub type FdcanTocc = crate::Reg<fdcan_tocc::FdcanToccSpec>;
#[doc = "FDCAN Timeout Counter Configuration Register"]
pub mod fdcan_tocc;
#[doc = "FDCAN_TOCV (rw) register accessor: FDCAN Timeout Counter Value Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tocv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tocv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tocv`] module"]
#[doc(alias = "FDCAN_TOCV")]
pub type FdcanTocv = crate::Reg<fdcan_tocv::FdcanTocvSpec>;
#[doc = "FDCAN Timeout Counter Value Register"]
pub mod fdcan_tocv;
#[doc = "FDCAN_ECR (rw) register accessor: FDCAN Error Counter Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ecr`] module"]
#[doc(alias = "FDCAN_ECR")]
pub type FdcanEcr = crate::Reg<fdcan_ecr::FdcanEcrSpec>;
#[doc = "FDCAN Error Counter Register"]
pub mod fdcan_ecr;
#[doc = "FDCAN_PSR (rw) register accessor: FDCAN Protocol Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_psr`] module"]
#[doc(alias = "FDCAN_PSR")]
pub type FdcanPsr = crate::Reg<fdcan_psr::FdcanPsrSpec>;
#[doc = "FDCAN Protocol Status Register"]
pub mod fdcan_psr;
#[doc = "FDCAN_TDCR (r) register accessor: FDCAN Transmitter Delay Compensation Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tdcr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tdcr`] module"]
#[doc(alias = "FDCAN_TDCR")]
pub type FdcanTdcr = crate::Reg<fdcan_tdcr::FdcanTdcrSpec>;
#[doc = "FDCAN Transmitter Delay Compensation Register"]
pub mod fdcan_tdcr;
#[doc = "FDCAN_IR (r) register accessor: FDCAN Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ir`] module"]
#[doc(alias = "FDCAN_IR")]
pub type FdcanIr = crate::Reg<fdcan_ir::FdcanIrSpec>;
#[doc = "FDCAN Interrupt Register"]
pub mod fdcan_ir;
#[doc = "FDCAN_IE (r) register accessor: FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ie`] module"]
#[doc(alias = "FDCAN_IE")]
pub type FdcanIe = crate::Reg<fdcan_ie::FdcanIeSpec>;
#[doc = "FDCAN Interrupt Enable Register"]
pub mod fdcan_ie;
#[doc = "FDCAN_ILS (r) register accessor: FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ils`] module"]
#[doc(alias = "FDCAN_ILS")]
pub type FdcanIls = crate::Reg<fdcan_ils::FdcanIlsSpec>;
#[doc = "FDCAN Interrupt Line Select Register"]
pub mod fdcan_ils;
#[doc = "FDCAN_ILE (rw) register accessor: FDCAN Interrupt Line Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ile::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ile::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ile`] module"]
#[doc(alias = "FDCAN_ILE")]
pub type FdcanIle = crate::Reg<fdcan_ile::FdcanIleSpec>;
#[doc = "FDCAN Interrupt Line Enable Register"]
pub mod fdcan_ile;
#[doc = "FDCAN_GFC (rw) register accessor: FDCAN Global Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_gfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_gfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_gfc`] module"]
#[doc(alias = "FDCAN_GFC")]
pub type FdcanGfc = crate::Reg<fdcan_gfc::FdcanGfcSpec>;
#[doc = "FDCAN Global Filter Configuration Register"]
pub mod fdcan_gfc;
#[doc = "FDCAN_SIDFC (rw) register accessor: FDCAN Standard ID Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_sidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_sidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_sidfc`] module"]
#[doc(alias = "FDCAN_SIDFC")]
pub type FdcanSidfc = crate::Reg<fdcan_sidfc::FdcanSidfcSpec>;
#[doc = "FDCAN Standard ID Filter Configuration Register"]
pub mod fdcan_sidfc;
#[doc = "FDCAN_XIDFC (rw) register accessor: FDCAN Extended ID Filter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_xidfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidfc`] module"]
#[doc(alias = "FDCAN_XIDFC")]
pub type FdcanXidfc = crate::Reg<fdcan_xidfc::FdcanXidfcSpec>;
#[doc = "FDCAN Extended ID Filter Configuration Register"]
pub mod fdcan_xidfc;
#[doc = "FDCAN_XIDAM (rw) register accessor: FDCAN Extended ID and Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_xidam::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_xidam::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_xidam`] module"]
#[doc(alias = "FDCAN_XIDAM")]
pub type FdcanXidam = crate::Reg<fdcan_xidam::FdcanXidamSpec>;
#[doc = "FDCAN Extended ID and Mask Register"]
pub mod fdcan_xidam;
#[doc = "FDCAN_HPMS (r) register accessor: FDCAN High Priority Message Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_hpms::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_hpms`] module"]
#[doc(alias = "FDCAN_HPMS")]
pub type FdcanHpms = crate::Reg<fdcan_hpms::FdcanHpmsSpec>;
#[doc = "FDCAN High Priority Message Status Register"]
pub mod fdcan_hpms;
#[doc = "FDCAN_NDAT1 (r) register accessor: FDCAN New Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ndat1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat1`] module"]
#[doc(alias = "FDCAN_NDAT1")]
pub type FdcanNdat1 = crate::Reg<fdcan_ndat1::FdcanNdat1Spec>;
#[doc = "FDCAN New Data 1 Register"]
pub mod fdcan_ndat1;
#[doc = "FDCAN_NDAT2 (r) register accessor: FDCAN New Data 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ndat2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ndat2`] module"]
#[doc(alias = "FDCAN_NDAT2")]
pub type FdcanNdat2 = crate::Reg<fdcan_ndat2::FdcanNdat2Spec>;
#[doc = "FDCAN New Data 2 Register"]
pub mod fdcan_ndat2;
#[doc = "FDCAN_RXF0C (rw) register accessor: FDCAN Rx FIFO 0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0c`] module"]
#[doc(alias = "FDCAN_RXF0C")]
pub type FdcanRxf0c = crate::Reg<fdcan_rxf0c::FdcanRxf0cSpec>;
#[doc = "FDCAN Rx FIFO 0 Configuration Register"]
pub mod fdcan_rxf0c;
#[doc = "FDCAN_RXF0S (rw) register accessor: FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0s`] module"]
#[doc(alias = "FDCAN_RXF0S")]
pub type FdcanRxf0s = crate::Reg<fdcan_rxf0s::FdcanRxf0sSpec>;
#[doc = "FDCAN Rx FIFO 0 Status Register"]
pub mod fdcan_rxf0s;
#[doc = "FDCAN_RXF0A (rw) register accessor: CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf0a`] module"]
#[doc(alias = "FDCAN_RXF0A")]
pub type FdcanRxf0a = crate::Reg<fdcan_rxf0a::FdcanRxf0aSpec>;
#[doc = "CAN Rx FIFO 0 Acknowledge Register"]
pub mod fdcan_rxf0a;
#[doc = "FDCAN_RXBC (rw) register accessor: FDCAN Rx Buffer Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxbc`] module"]
#[doc(alias = "FDCAN_RXBC")]
pub type FdcanRxbc = crate::Reg<fdcan_rxbc::FdcanRxbcSpec>;
#[doc = "FDCAN Rx Buffer Configuration Register"]
pub mod fdcan_rxbc;
#[doc = "FDCAN_RXF1C (rw) register accessor: FDCAN Rx FIFO 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1c::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1c`] module"]
#[doc(alias = "FDCAN_RXF1C")]
pub type FdcanRxf1c = crate::Reg<fdcan_rxf1c::FdcanRxf1cSpec>;
#[doc = "FDCAN Rx FIFO 1 Configuration Register"]
pub mod fdcan_rxf1c;
#[doc = "FDCAN_RXF1S (rw) register accessor: FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1s::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1s`] module"]
#[doc(alias = "FDCAN_RXF1S")]
pub type FdcanRxf1s = crate::Reg<fdcan_rxf1s::FdcanRxf1sSpec>;
#[doc = "FDCAN Rx FIFO 1 Status Register"]
pub mod fdcan_rxf1s;
#[doc = "FDCAN_RXF1A (rw) register accessor: FDCAN Rx FIFO 1 Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1a::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1a::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxf1a`] module"]
#[doc(alias = "FDCAN_RXF1A")]
pub type FdcanRxf1a = crate::Reg<fdcan_rxf1a::FdcanRxf1aSpec>;
#[doc = "FDCAN Rx FIFO 1 Acknowledge Register"]
pub mod fdcan_rxf1a;
#[doc = "FDCAN_RXESC (rw) register accessor: FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_rxesc`] module"]
#[doc(alias = "FDCAN_RXESC")]
pub type FdcanRxesc = crate::Reg<fdcan_rxesc::FdcanRxescSpec>;
#[doc = "FDCAN Rx Buffer Element Size Configuration Register"]
pub mod fdcan_rxesc;
#[doc = "FDCAN_TXBC (rw) register accessor: FDCAN Tx Buffer Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbc`] module"]
#[doc(alias = "FDCAN_TXBC")]
pub type FdcanTxbc = crate::Reg<fdcan_txbc::FdcanTxbcSpec>;
#[doc = "FDCAN Tx Buffer Configuration Register"]
pub mod fdcan_txbc;
#[doc = "FDCAN_TXFQS (r) register accessor: FDCAN Tx FIFO/Queue Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txfqs::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txfqs`] module"]
#[doc(alias = "FDCAN_TXFQS")]
pub type FdcanTxfqs = crate::Reg<fdcan_txfqs::FdcanTxfqsSpec>;
#[doc = "FDCAN Tx FIFO/Queue Status Register"]
pub mod fdcan_txfqs;
#[doc = "FDCAN_TXESC (rw) register accessor: FDCAN Tx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txesc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txesc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txesc`] module"]
#[doc(alias = "FDCAN_TXESC")]
pub type FdcanTxesc = crate::Reg<fdcan_txesc::FdcanTxescSpec>;
#[doc = "FDCAN Tx Buffer Element Size Configuration Register"]
pub mod fdcan_txesc;
#[doc = "FDCAN_TXBRP (r) register accessor: FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbrp`] module"]
#[doc(alias = "FDCAN_TXBRP")]
pub type FdcanTxbrp = crate::Reg<fdcan_txbrp::FdcanTxbrpSpec>;
#[doc = "FDCAN Tx Buffer Request Pending Register"]
pub mod fdcan_txbrp;
#[doc = "FDCAN_TXBAR (rw) register accessor: FDCAN Tx Buffer Add Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbar`] module"]
#[doc(alias = "FDCAN_TXBAR")]
pub type FdcanTxbar = crate::Reg<fdcan_txbar::FdcanTxbarSpec>;
#[doc = "FDCAN Tx Buffer Add Request Register"]
pub mod fdcan_txbar;
#[doc = "FDCAN_TXBCR (rw) register accessor: FDCAN Tx Buffer Cancellation Request Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcr`] module"]
#[doc(alias = "FDCAN_TXBCR")]
pub type FdcanTxbcr = crate::Reg<fdcan_txbcr::FdcanTxbcrSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Request Register"]
pub mod fdcan_txbcr;
#[doc = "FDCAN_TXBTO (rw) register accessor: FDCAN Tx Buffer Transmission Occurred Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbto::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbto::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbto`] module"]
#[doc(alias = "FDCAN_TXBTO")]
pub type FdcanTxbto = crate::Reg<fdcan_txbto::FdcanTxbtoSpec>;
#[doc = "FDCAN Tx Buffer Transmission Occurred Register"]
pub mod fdcan_txbto;
#[doc = "FDCAN_TXBCF (r) register accessor: FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcf`] module"]
#[doc(alias = "FDCAN_TXBCF")]
pub type FdcanTxbcf = crate::Reg<fdcan_txbcf::FdcanTxbcfSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Register"]
pub mod fdcan_txbcf;
#[doc = "FDCAN_TXBTIE (rw) register accessor: FDCAN Tx Buffer Transmission Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbtie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbtie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbtie`] module"]
#[doc(alias = "FDCAN_TXBTIE")]
pub type FdcanTxbtie = crate::Reg<fdcan_txbtie::FdcanTxbtieSpec>;
#[doc = "FDCAN Tx Buffer Transmission Interrupt Enable Register"]
pub mod fdcan_txbtie;
#[doc = "FDCAN_TXBCIE (rw) register accessor: FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txbcie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txbcie`] module"]
#[doc(alias = "FDCAN_TXBCIE")]
pub type FdcanTxbcie = crate::Reg<fdcan_txbcie::FdcanTxbcieSpec>;
#[doc = "FDCAN Tx Buffer Cancellation Finished Interrupt Enable Register"]
pub mod fdcan_txbcie;
#[doc = "FDCAN_TXEFC (rw) register accessor: FDCAN Tx Event FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefc`] module"]
#[doc(alias = "FDCAN_TXEFC")]
pub type FdcanTxefc = crate::Reg<fdcan_txefc::FdcanTxefcSpec>;
#[doc = "FDCAN Tx Event FIFO Configuration Register"]
pub mod fdcan_txefc;
#[doc = "FDCAN_TXEFS (rw) register accessor: FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefs`] module"]
#[doc(alias = "FDCAN_TXEFS")]
pub type FdcanTxefs = crate::Reg<fdcan_txefs::FdcanTxefsSpec>;
#[doc = "FDCAN Tx Event FIFO Status Register"]
pub mod fdcan_txefs;
#[doc = "FDCAN_TXEFA (rw) register accessor: FDCAN Tx Event FIFO Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefa::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefa::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_txefa`] module"]
#[doc(alias = "FDCAN_TXEFA")]
pub type FdcanTxefa = crate::Reg<fdcan_txefa::FdcanTxefaSpec>;
#[doc = "FDCAN Tx Event FIFO Acknowledge Register"]
pub mod fdcan_txefa;
#[doc = "FDCAN_TTTMC (rw) register accessor: FDCAN TT Trigger Memory Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tttmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmc`] module"]
#[doc(alias = "FDCAN_TTTMC")]
pub type FdcanTttmc = crate::Reg<fdcan_tttmc::FdcanTttmcSpec>;
#[doc = "FDCAN TT Trigger Memory Configuration Register"]
pub mod fdcan_tttmc;
#[doc = "FDCAN_TTRMC (rw) register accessor: FDCAN TT Reference Message Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttrmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttrmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttrmc`] module"]
#[doc(alias = "FDCAN_TTRMC")]
pub type FdcanTtrmc = crate::Reg<fdcan_ttrmc::FdcanTtrmcSpec>;
#[doc = "FDCAN TT Reference Message Configuration Register"]
pub mod fdcan_ttrmc;
#[doc = "FDCAN_TTOCF (rw) register accessor: FDCAN TT Operation Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttocf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocf`] module"]
#[doc(alias = "FDCAN_TTOCF")]
pub type FdcanTtocf = crate::Reg<fdcan_ttocf::FdcanTtocfSpec>;
#[doc = "FDCAN TT Operation Configuration Register"]
pub mod fdcan_ttocf;
#[doc = "FDCAN_TTMLM (rw) register accessor: FDCAN TT Matrix Limits Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttmlm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttmlm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttmlm`] module"]
#[doc(alias = "FDCAN_TTMLM")]
pub type FdcanTtmlm = crate::Reg<fdcan_ttmlm::FdcanTtmlmSpec>;
#[doc = "FDCAN TT Matrix Limits Register"]
pub mod fdcan_ttmlm;
#[doc = "FDCAN_TURCF (rw) register accessor: FDCAN TUR Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_turcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_turcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turcf`] module"]
#[doc(alias = "FDCAN_TURCF")]
pub type FdcanTurcf = crate::Reg<fdcan_turcf::FdcanTurcfSpec>;
#[doc = "FDCAN TUR Configuration Register"]
pub mod fdcan_turcf;
#[doc = "FDCAN_TTOCN (rw) register accessor: FDCAN TT Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttocn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttocn`] module"]
#[doc(alias = "FDCAN_TTOCN")]
pub type FdcanTtocn = crate::Reg<fdcan_ttocn::FdcanTtocnSpec>;
#[doc = "FDCAN TT Operation Control Register"]
pub mod fdcan_ttocn;
#[doc = "CAN_TTGTP (rw) register accessor: FDCAN TT Global Time Preset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ttgtp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ttgtp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@can_ttgtp`] module"]
#[doc(alias = "CAN_TTGTP")]
pub type CanTtgtp = crate::Reg<can_ttgtp::CanTtgtpSpec>;
#[doc = "FDCAN TT Global Time Preset Register"]
pub mod can_ttgtp;
#[doc = "FDCAN_TTTMK (rw) register accessor: FDCAN TT Time Mark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tttmk::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tttmk::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_tttmk`] module"]
#[doc(alias = "FDCAN_TTTMK")]
pub type FdcanTttmk = crate::Reg<fdcan_tttmk::FdcanTttmkSpec>;
#[doc = "FDCAN TT Time Mark Register"]
pub mod fdcan_tttmk;
#[doc = "FDCAN_TTIR (rw) register accessor: FDCAN TT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttir`] module"]
#[doc(alias = "FDCAN_TTIR")]
pub type FdcanTtir = crate::Reg<fdcan_ttir::FdcanTtirSpec>;
#[doc = "FDCAN TT Interrupt Register"]
pub mod fdcan_ttir;
#[doc = "FDCAN_TTIE (rw) register accessor: FDCAN TT Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttie`] module"]
#[doc(alias = "FDCAN_TTIE")]
pub type FdcanTtie = crate::Reg<fdcan_ttie::FdcanTtieSpec>;
#[doc = "FDCAN TT Interrupt Enable Register"]
pub mod fdcan_ttie;
#[doc = "FDCAN_TTILS (rw) register accessor: FDCAN TT Interrupt Line Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttils::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttils::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttils`] module"]
#[doc(alias = "FDCAN_TTILS")]
pub type FdcanTtils = crate::Reg<fdcan_ttils::FdcanTtilsSpec>;
#[doc = "FDCAN TT Interrupt Line Select Register"]
pub mod fdcan_ttils;
#[doc = "FDCAN_TTOST (rw) register accessor: FDCAN TT Operation Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttost::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttost::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttost`] module"]
#[doc(alias = "FDCAN_TTOST")]
pub type FdcanTtost = crate::Reg<fdcan_ttost::FdcanTtostSpec>;
#[doc = "FDCAN TT Operation Status Register"]
pub mod fdcan_ttost;
#[doc = "FDCAN_TURNA (r) register accessor: FDCAN TUR Numerator Actual Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_turna::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_turna`] module"]
#[doc(alias = "FDCAN_TURNA")]
pub type FdcanTurna = crate::Reg<fdcan_turna::FdcanTurnaSpec>;
#[doc = "FDCAN TUR Numerator Actual Register"]
pub mod fdcan_turna;
#[doc = "FDCAN_TTLGT (r) register accessor: FDCAN TT Local and Global Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttlgt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttlgt`] module"]
#[doc(alias = "FDCAN_TTLGT")]
pub type FdcanTtlgt = crate::Reg<fdcan_ttlgt::FdcanTtlgtSpec>;
#[doc = "FDCAN TT Local and Global Time Register"]
pub mod fdcan_ttlgt;
#[doc = "FDCAN_TTCTC (r) register accessor: FDCAN TT Cycle Time and Count Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttctc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttctc`] module"]
#[doc(alias = "FDCAN_TTCTC")]
pub type FdcanTtctc = crate::Reg<fdcan_ttctc::FdcanTtctcSpec>;
#[doc = "FDCAN TT Cycle Time and Count Register"]
pub mod fdcan_ttctc;
#[doc = "FDCAN_TTCPT (r) register accessor: FDCAN TT Capture Time Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttcpt::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcpt`] module"]
#[doc(alias = "FDCAN_TTCPT")]
pub type FdcanTtcpt = crate::Reg<fdcan_ttcpt::FdcanTtcptSpec>;
#[doc = "FDCAN TT Capture Time Register"]
pub mod fdcan_ttcpt;
#[doc = "FDCAN_TTCSM (r) register accessor: FDCAN TT Cycle Sync Mark Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttcsm::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttcsm`] module"]
#[doc(alias = "FDCAN_TTCSM")]
pub type FdcanTtcsm = crate::Reg<fdcan_ttcsm::FdcanTtcsmSpec>;
#[doc = "FDCAN TT Cycle Sync Mark Register"]
pub mod fdcan_ttcsm;
#[doc = "FDCAN_TTTS (rw) register accessor: FDCAN TT Trigger Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdcan_ttts`] module"]
#[doc(alias = "FDCAN_TTTS")]
pub type FdcanTtts = crate::Reg<fdcan_ttts::FdcanTttsSpec>;
#[doc = "FDCAN TT Trigger Select Register"]
pub mod fdcan_ttts;
