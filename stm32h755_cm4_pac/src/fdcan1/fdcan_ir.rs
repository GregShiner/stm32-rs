#[doc = "Register `FDCAN_IR` reader"]
pub type R = crate::R<FdcanIrSpec>;
#[doc = "Field `RF0N` reader - Rx FIFO 0 New Message"]
pub type Rf0nR = crate::BitReader;
#[doc = "Field `RF0W` reader - Rx FIFO 0 Full"]
pub type Rf0wR = crate::BitReader;
#[doc = "Field `RF0F` reader - Rx FIFO 0 Full"]
pub type Rf0fR = crate::BitReader;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF1N` reader - Rx FIFO 1 New Message"]
pub type Rf1nR = crate::BitReader;
#[doc = "Field `RF1W` reader - Rx FIFO 1 Watermark Reached"]
pub type Rf1wR = crate::BitReader;
#[doc = "Field `RF1F` reader - Rx FIFO 1 Watermark Reached"]
pub type Rf1fR = crate::BitReader;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `HPM` reader - High Priority Message"]
pub type HpmR = crate::BitReader;
#[doc = "Field `TC` reader - Transmission Completed"]
pub type TcR = crate::BitReader;
#[doc = "Field `TCF` reader - Transmission Cancellation Finished"]
pub type TcfR = crate::BitReader;
#[doc = "Field `TEF` reader - Tx FIFO Empty"]
pub type TefR = crate::BitReader;
#[doc = "Field `TEFN` reader - Tx Event FIFO New Entry"]
pub type TefnR = crate::BitReader;
#[doc = "Field `TEFW` reader - Tx Event FIFO Watermark Reached"]
pub type TefwR = crate::BitReader;
#[doc = "Field `TEFF` reader - Tx Event FIFO Full"]
pub type TeffR = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TSW` reader - Timestamp Wraparound"]
pub type TswR = crate::BitReader;
#[doc = "Field `MRAF` reader - Message RAM Access Failure"]
pub type MrafR = crate::BitReader;
#[doc = "Field `TOO` reader - Timeout Occurred"]
pub type TooR = crate::BitReader;
#[doc = "Field `DRX` reader - Message stored to Dedicated Rx Buffer"]
pub type DrxR = crate::BitReader;
#[doc = "Field `ELO` reader - Error Logging Overflow"]
pub type EloR = crate::BitReader;
#[doc = "Field `EP` reader - Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EW` reader - Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Field `WDI` reader - Watchdog Interrupt"]
pub type WdiR = crate::BitReader;
#[doc = "Field `PEA` reader - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
pub type PeaR = crate::BitReader;
#[doc = "Field `PED` reader - Protocol Error in Data Phase (Data Bit Time is used)"]
pub type PedR = crate::BitReader;
#[doc = "Field `ARA` reader - Access to Reserved Address"]
pub type AraR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message"]
    #[inline(always)]
    pub fn rf0n(&self) -> Rf0nR {
        Rf0nR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0w(&self) -> Rf0wR {
        Rf0wR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn rf0f(&self) -> Rf0fR {
        Rf0fR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message"]
    #[inline(always)]
    pub fn rf1n(&self) -> Rf1nR {
        Rf1nR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1w(&self) -> Rf1wR {
        Rf1wR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached"]
    #[inline(always)]
    pub fn rf1f(&self) -> Rf1fR {
        Rf1fR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message"]
    #[inline(always)]
    pub fn hpm(&self) -> HpmR {
        HpmR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed"]
    #[inline(always)]
    pub fn tc(&self) -> TcR {
        TcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished"]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty"]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry"]
    #[inline(always)]
    pub fn tefn(&self) -> TefnR {
        TefnR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached"]
    #[inline(always)]
    pub fn tefw(&self) -> TefwR {
        TefwR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full"]
    #[inline(always)]
    pub fn teff(&self) -> TeffR {
        TeffR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound"]
    #[inline(always)]
    pub fn tsw(&self) -> TswR {
        TswR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure"]
    #[inline(always)]
    pub fn mraf(&self) -> MrafR {
        MrafR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred"]
    #[inline(always)]
    pub fn too(&self) -> TooR {
        TooR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer"]
    #[inline(always)]
    pub fn drx(&self) -> DrxR {
        DrxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow"]
    #[inline(always)]
    pub fn elo(&self) -> EloR {
        EloR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt"]
    #[inline(always)]
    pub fn wdi(&self) -> WdiR {
        WdiR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase (Nominal Bit Time is used)"]
    #[inline(always)]
    pub fn pea(&self) -> PeaR {
        PeaR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase (Data Bit Time is used)"]
    #[inline(always)]
    pub fn ped(&self) -> PedR {
        PedR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address"]
    #[inline(always)]
    pub fn ara(&self) -> AraR {
        AraR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "FDCAN Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ir::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIrSpec;
impl crate::RegisterSpec for FdcanIrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ir::R`](R) reader structure"]
impl crate::Readable for FdcanIrSpec {}
#[doc = "`reset()` method sets FDCAN_IR to value 0"]
impl crate::Resettable for FdcanIrSpec {}
