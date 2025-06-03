#[doc = "Register `MACHWF1R` reader"]
pub type R = crate::R<Machwf1rSpec>;
#[doc = "Field `RXFIFOSIZE` reader - RXFIFOSIZE"]
pub type RxfifosizeR = crate::FieldReader;
#[doc = "Field `TXFIFOSIZE` reader - TXFIFOSIZE"]
pub type TxfifosizeR = crate::FieldReader;
#[doc = "Field `OSTEN` reader - OSTEN"]
pub type OstenR = crate::BitReader;
#[doc = "Field `PTOEN` reader - PTOEN"]
pub type PtoenR = crate::BitReader;
#[doc = "Field `ADVTHWORD` reader - ADVTHWORD"]
pub type AdvthwordR = crate::BitReader;
#[doc = "Field `DCBEN` reader - DCBEN"]
pub type DcbenR = crate::BitReader;
#[doc = "Field `SPHEN` reader - SPHEN"]
pub type SphenR = crate::BitReader;
#[doc = "Field `TSOEN` reader - TSOEN"]
pub type TsoenR = crate::BitReader;
#[doc = "Field `DBGMEMA` reader - DBGMEMA"]
pub type DbgmemaR = crate::BitReader;
#[doc = "Field `AVSEL` reader - AVSEL"]
pub type AvselR = crate::BitReader;
#[doc = "Field `HASHTBLSZ` reader - HASHTBLSZ"]
pub type HashtblszR = crate::FieldReader;
#[doc = "Field `L3L4FNUM` reader - L3L4FNUM"]
pub type L3l4fnumR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:4 - RXFIFOSIZE"]
    #[inline(always)]
    pub fn rxfifosize(&self) -> RxfifosizeR {
        RxfifosizeR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 6:10 - TXFIFOSIZE"]
    #[inline(always)]
    pub fn txfifosize(&self) -> TxfifosizeR {
        TxfifosizeR::new(((self.bits >> 6) & 0x1f) as u8)
    }
    #[doc = "Bit 11 - OSTEN"]
    #[inline(always)]
    pub fn osten(&self) -> OstenR {
        OstenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - PTOEN"]
    #[inline(always)]
    pub fn ptoen(&self) -> PtoenR {
        PtoenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADVTHWORD"]
    #[inline(always)]
    pub fn advthword(&self) -> AdvthwordR {
        AdvthwordR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - DCBEN"]
    #[inline(always)]
    pub fn dcben(&self) -> DcbenR {
        DcbenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - SPHEN"]
    #[inline(always)]
    pub fn sphen(&self) -> SphenR {
        SphenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TSOEN"]
    #[inline(always)]
    pub fn tsoen(&self) -> TsoenR {
        TsoenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DBGMEMA"]
    #[inline(always)]
    pub fn dbgmema(&self) -> DbgmemaR {
        DbgmemaR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - AVSEL"]
    #[inline(always)]
    pub fn avsel(&self) -> AvselR {
        AvselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:25 - HASHTBLSZ"]
    #[inline(always)]
    pub fn hashtblsz(&self) -> HashtblszR {
        HashtblszR::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bits 27:30 - L3L4FNUM"]
    #[inline(always)]
    pub fn l3l4fnum(&self) -> L3l4fnumR {
        L3l4fnumR::new(((self.bits >> 27) & 0x0f) as u8)
    }
}
#[doc = "HW feature 1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`machwf1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Machwf1rSpec;
impl crate::RegisterSpec for Machwf1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`machwf1r::R`](R) reader structure"]
impl crate::Readable for Machwf1rSpec {}
#[doc = "`reset()` method sets MACHWF1R to value 0x1184_1904"]
impl crate::Resettable for Machwf1rSpec {
    const RESET_VALUE: u32 = 0x1184_1904;
}
