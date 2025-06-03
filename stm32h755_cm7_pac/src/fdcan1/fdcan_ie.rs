#[doc = "Register `FDCAN_IE` reader"]
pub type R = crate::R<FdcanIeSpec>;
#[doc = "Field `RF0NE` reader - Rx FIFO 0 New Message Enable"]
pub type Rf0neR = crate::BitReader;
#[doc = "Field `RF0WE` reader - Rx FIFO 0 Full Enable"]
pub type Rf0weR = crate::BitReader;
#[doc = "Field `RF0FE` reader - Rx FIFO 0 Full Enable"]
pub type Rf0feR = crate::BitReader;
#[doc = "Field `RF0LE` reader - Rx FIFO 0 Message Lost Enable"]
pub type Rf0leR = crate::BitReader;
#[doc = "Field `RF1NE` reader - Rx FIFO 1 New Message Enable"]
pub type Rf1neR = crate::BitReader;
#[doc = "Field `RF1WE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type Rf1weR = crate::BitReader;
#[doc = "Field `RF1FE` reader - Rx FIFO 1 Watermark Reached Enable"]
pub type Rf1feR = crate::BitReader;
#[doc = "Field `RF1LE` reader - Rx FIFO 1 Message Lost Enable"]
pub type Rf1leR = crate::BitReader;
#[doc = "Field `HPME` reader - High Priority Message Enable"]
pub type HpmeR = crate::BitReader;
#[doc = "Field `TCE` reader - Transmission Completed Enable"]
pub type TceR = crate::BitReader;
#[doc = "Field `TCFE` reader - Transmission Cancellation Finished Enable"]
pub type TcfeR = crate::BitReader;
#[doc = "Field `TEFE` reader - Tx FIFO Empty Enable"]
pub type TefeR = crate::BitReader;
#[doc = "Field `TEFNE` reader - Tx Event FIFO New Entry Enable"]
pub type TefneR = crate::BitReader;
#[doc = "Field `TEFWE` reader - Tx Event FIFO Watermark Reached Enable"]
pub type TefweR = crate::BitReader;
#[doc = "Field `TEFFE` reader - Tx Event FIFO Full Enable"]
pub type TeffeR = crate::BitReader;
#[doc = "Field `TEFLE` reader - Tx Event FIFO Element Lost Enable"]
pub type TefleR = crate::BitReader;
#[doc = "Field `TSWE` reader - Timestamp Wraparound Enable"]
pub type TsweR = crate::BitReader;
#[doc = "Field `MRAFE` reader - Message RAM Access Failure Enable"]
pub type MrafeR = crate::BitReader;
#[doc = "Field `TOOE` reader - Timeout Occurred Enable"]
pub type TooeR = crate::BitReader;
#[doc = "Field `DRXE` reader - Message stored to Dedicated Rx Buffer Enable"]
pub type DrxeR = crate::BitReader;
#[doc = "Field `BECE` reader - Bit Error Corrected Interrupt Enable"]
pub type BeceR = crate::BitReader;
#[doc = "Field `BEUE` reader - Bit Error Uncorrected Interrupt Enable"]
pub type BeueR = crate::BitReader;
#[doc = "Field `ELOE` reader - Error Logging Overflow Enable"]
pub type EloeR = crate::BitReader;
#[doc = "Field `EPE` reader - Error Passive Enable"]
pub type EpeR = crate::BitReader;
#[doc = "Field `EWE` reader - Warning Status Enable"]
pub type EweR = crate::BitReader;
#[doc = "Field `BOE` reader - Bus_Off Status Enable"]
pub type BoeR = crate::BitReader;
#[doc = "Field `WDIE` reader - Watchdog Interrupt Enable"]
pub type WdieR = crate::BitReader;
#[doc = "Field `PEAE` reader - Protocol Error in Arbitration Phase Enable"]
pub type PeaeR = crate::BitReader;
#[doc = "Field `PEDE` reader - Protocol Error in Data Phase Enable"]
pub type PedeR = crate::BitReader;
#[doc = "Field `ARAE` reader - Access to Reserved Address Enable"]
pub type AraeR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Rx FIFO 0 New Message Enable"]
    #[inline(always)]
    pub fn rf0ne(&self) -> Rf0neR {
        Rf0neR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0we(&self) -> Rf0weR {
        Rf0weR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rx FIFO 0 Full Enable"]
    #[inline(always)]
    pub fn rf0fe(&self) -> Rf0feR {
        Rf0feR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rx FIFO 0 Message Lost Enable"]
    #[inline(always)]
    pub fn rf0le(&self) -> Rf0leR {
        Rf0leR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rx FIFO 1 New Message Enable"]
    #[inline(always)]
    pub fn rf1ne(&self) -> Rf1neR {
        Rf1neR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1we(&self) -> Rf1weR {
        Rf1weR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rx FIFO 1 Watermark Reached Enable"]
    #[inline(always)]
    pub fn rf1fe(&self) -> Rf1feR {
        Rf1feR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rx FIFO 1 Message Lost Enable"]
    #[inline(always)]
    pub fn rf1le(&self) -> Rf1leR {
        Rf1leR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - High Priority Message Enable"]
    #[inline(always)]
    pub fn hpme(&self) -> HpmeR {
        HpmeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Transmission Completed Enable"]
    #[inline(always)]
    pub fn tce(&self) -> TceR {
        TceR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Transmission Cancellation Finished Enable"]
    #[inline(always)]
    pub fn tcfe(&self) -> TcfeR {
        TcfeR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx FIFO Empty Enable"]
    #[inline(always)]
    pub fn tefe(&self) -> TefeR {
        TefeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Tx Event FIFO New Entry Enable"]
    #[inline(always)]
    pub fn tefne(&self) -> TefneR {
        TefneR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Tx Event FIFO Watermark Reached Enable"]
    #[inline(always)]
    pub fn tefwe(&self) -> TefweR {
        TefweR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx Event FIFO Full Enable"]
    #[inline(always)]
    pub fn teffe(&self) -> TeffeR {
        TeffeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Tx Event FIFO Element Lost Enable"]
    #[inline(always)]
    pub fn tefle(&self) -> TefleR {
        TefleR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Timestamp Wraparound Enable"]
    #[inline(always)]
    pub fn tswe(&self) -> TsweR {
        TsweR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Message RAM Access Failure Enable"]
    #[inline(always)]
    pub fn mrafe(&self) -> MrafeR {
        MrafeR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Timeout Occurred Enable"]
    #[inline(always)]
    pub fn tooe(&self) -> TooeR {
        TooeR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Message stored to Dedicated Rx Buffer Enable"]
    #[inline(always)]
    pub fn drxe(&self) -> DrxeR {
        DrxeR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Bit Error Corrected Interrupt Enable"]
    #[inline(always)]
    pub fn bece(&self) -> BeceR {
        BeceR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Bit Error Uncorrected Interrupt Enable"]
    #[inline(always)]
    pub fn beue(&self) -> BeueR {
        BeueR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Error Logging Overflow Enable"]
    #[inline(always)]
    pub fn eloe(&self) -> EloeR {
        EloeR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Error Passive Enable"]
    #[inline(always)]
    pub fn epe(&self) -> EpeR {
        EpeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Warning Status Enable"]
    #[inline(always)]
    pub fn ewe(&self) -> EweR {
        EweR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Bus_Off Status Enable"]
    #[inline(always)]
    pub fn boe(&self) -> BoeR {
        BoeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn wdie(&self) -> WdieR {
        WdieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Protocol Error in Arbitration Phase Enable"]
    #[inline(always)]
    pub fn peae(&self) -> PeaeR {
        PeaeR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Protocol Error in Data Phase Enable"]
    #[inline(always)]
    pub fn pede(&self) -> PedeR {
        PedeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Access to Reserved Address Enable"]
    #[inline(always)]
    pub fn arae(&self) -> AraeR {
        AraeR::new(((self.bits >> 29) & 1) != 0)
    }
}
#[doc = "FDCAN Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ie::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanIeSpec;
impl crate::RegisterSpec for FdcanIeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ie::R`](R) reader structure"]
impl crate::Readable for FdcanIeSpec {}
#[doc = "`reset()` method sets FDCAN_IE to value 0"]
impl crate::Resettable for FdcanIeSpec {}
