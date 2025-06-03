#[doc = "Register `FDCAN_ILS` reader"]
pub type R = crate::R<FdcanIlsSpec>;
#[doc = "Field `RF0NL` reader - Rx FIFO 0 New Message Interrupt Line"]
pub type Rf0nlR = crate::BitReader;
#[doc = "Field `RF0WL` reader - Rx FIFO 0 Watermark Reached Interrupt Line"]
pub type Rf0wlR = crate::BitReader;
#[doc = "Field `RF0FL` reader - Rx FIFO 0 Full Interrupt Line"]
pub type Rf0flR = crate::BitReader;
#[doc = "Field `RF0LL` reader - Rx FIFO 0 Message Lost Interrupt Line"]
pub type Rf0llR = crate::BitReader;
#[doc = "Field `RF1NL` reader - Rx FIFO 1 New Message Interrupt Line"]
pub type Rf1nlR = crate::BitReader;
#[doc = "Field `RF1WL` reader - Rx FIFO 1 Watermark Reached Interrupt Line"]
pub type Rf1wlR = crate::BitReader;
#[doc = "Field `RF1FL` reader - Rx FIFO 1 Full Interrupt Line"]
pub type Rf1flR = crate::BitReader;
#[doc = "Field `RF1LL` reader - Rx FIFO 1 Message Lost Interrupt Line"]
pub type Rf1llR = crate::BitReader;
#[doc = "Field `HPML` reader - High Priority Message Interrupt Line"]
pub type HpmlR = crate::BitReader;
#[doc = "Field `TCL` reader - Transmission Completed Interrupt Line"]
pub type TclR = crate::BitReader;
#[doc = "Field `TCFL` reader - Transmission Cancellation Finished Interrupt Line"]
pub type TcflR = crate::BitReader;
#[doc = "Field `TEFL` reader - Tx FIFO Empty Interrupt Line"]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFNL` reader - Tx Event FIFO New Entry Interrupt Line"]
pub type TefnlR = crate::BitReader;
#[doc = "Field `TEFWL` reader - Tx Event FIFO Watermark Reached Interrupt Line"]
pub type TefwlR = crate::BitReader;
#[doc = "Field `TEFFL` reader - Tx Event FIFO Full Interrupt Line"]
pub type TefflR = crate::BitReader;
#[doc = "Field `TEFLL` reader - Tx Event FIFO Element Lost Interrupt Line"]
pub type TefllR = crate::BitReader;
#[doc = "Field `TSWL` reader - Timestamp Wraparound Interrupt Line"]
pub type TswlR = crate::BitReader;
#[doc = "Field `MRAFL` reader - Message RAM Access Failure Interrupt Line"]
pub type MraflR = crate::BitReader;
#[doc = "Field `TOOL` reader - Timeout Occurred Interrupt Line"]
pub type ToolR = crate::BitReader;
#[doc = "Field `DRXL` reader - Message stored to Dedicated Rx Buffer Interrupt Line"]
pub type DrxlR = crate::BitReader;
#[doc = "Field `BECL` reader - Bit Error Corrected Interrupt Line"]
pub type BeclR = crate::BitReader;
#[doc = "Field `BEUL` reader - Bit Error Uncorrected Interrupt Line"]
pub type BeulR = crate::BitReader;
#[doc = "Field `ELOL` reader - Error Logging Overflow Interrupt Line"]
pub type ElolR = crate::BitReader;
#[doc = "Field `EPL` reader - Error Passive Interrupt Line"]
pub type EplR = crate::BitReader;
#[doc = "Field `EWL` reader - Warning Status Interrupt Line"]
pub type EwlR = crate::BitReader;
#[doc = "Field `BOL` reader - Bus_Off Status"]
pub type BolR = crate::BitReader;
#[doc = "Field `WDIL` reader - Watchdog Interrupt Line"]
pub type WdilR = crate::BitReader;
#[doc = "Field `PEAL` reader - Protocol Error in Arbitration Phase Line"]
pub type PealR = crate::BitReader;
#[doc = "Field `PEDL` reader - Protocol Error in Data Phase Line"]
pub type PedlR = crate::BitReader;
#[doc = "Field `ARAL` reader - Access to Reserved Address Line"]
pub type AralR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf0nl(&self) -> Rf0nlR {
        Rf0nlR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf0wl(&self) -> Rf0wlR {
        Rf0wlR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf0fl(&self) -> Rf0flR {
        Rf0flR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf0ll(&self) -> Rf0llR {
        Rf0llR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Interrupt Line"]
    #[inline(always)]
    pub fn rf1nl(&self) -> Rf1nlR {
        Rf1nlR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn rf1wl(&self) -> Rf1wlR {
        Rf1wlR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Full Interrupt Line"]
    #[inline(always)]
    pub fn rf1fl(&self) -> Rf1flR {
        Rf1flR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Interrupt Line"]
    #[inline(always)]
    pub fn rf1ll(&self) -> Rf1llR {
        Rf1llR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Interrupt Line"]
    #[inline(always)]
    pub fn hpml(&self) -> HpmlR {
        HpmlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Interrupt Line"]
    #[inline(always)]
    pub fn tcl(&self) -> TclR {
        TclR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Interrupt Line"]
    #[inline(always)]
    pub fn tcfl(&self) -> TcflR {
        TcflR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Interrupt Line"]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Interrupt Line"]
    #[inline(always)]
    pub fn tefnl(&self) -> TefnlR {
        TefnlR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Interrupt Line"]
    #[inline(always)]
    pub fn tefwl(&self) -> TefwlR {
        TefwlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Interrupt Line"]
    #[inline(always)]
    pub fn teffl(&self) -> TefflR {
        TefflR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Interrupt Line"]
    #[inline(always)]
    pub fn tefll(&self) -> TefllR {
        TefllR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Interrupt Line"]
    #[inline(always)]
    pub fn tswl(&self) -> TswlR {
        TswlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Interrupt Line"]
    #[inline(always)]
    pub fn mrafl(&self) -> MraflR {
        MraflR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Interrupt Line"]
    #[inline(always)]
    pub fn tool(&self) -> ToolR {
        ToolR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Interrupt Line"]
    #[inline(always)]
    pub fn drxl(&self) -> DrxlR {
        DrxlR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Line"]
    #[inline(always)]
    pub fn becl(&self) -> BeclR {
        BeclR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Line"]
    #[inline(always)]
    pub fn beul(&self) -> BeulR {
        BeulR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Interrupt Line"]
    #[inline(always)]
    pub fn elol(&self) -> ElolR {
        ElolR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Interrupt Line"]
    #[inline(always)]
    pub fn epl(&self) -> EplR {
        EplR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Interrupt Line"]
    #[inline(always)]
    pub fn ewl(&self) -> EwlR {
        EwlR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status"]
    #[inline(always)]
    pub fn bol(&self) -> BolR {
        BolR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn wdil(&self) -> WdilR {
        WdilR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Line"]
    #[inline(always)]
    pub fn peal(&self) -> PealR {
        PealR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Line"]
    #[inline(always)]
    pub fn pedl(&self) -> PedlR {
        PedlR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Line"]
    #[inline(always)]
    pub fn aral(&self) -> AralR {
        AralR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "FDCAN Interrupt Line Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ils::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIlsSpec;
impl crate::RegisterSpec for FdcanIlsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ils::R`](R) reader structure"]
impl crate::Readable for FdcanIlsSpec {}
#[doc = "`reset()` method sets FDCAN_ILS to value 0"]
impl crate::Resettable for FdcanIlsSpec {}
