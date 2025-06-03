#[doc = "Register `MTLRxQDR` reader"]
pub type R = crate::R<MtlrxQdrSpec>;
#[doc = "Field `RWCSTS` reader - RWCSTS"]
pub type RwcstsR = crate::BitReader;
#[doc = "Field `RRCSTS` reader - RRCSTS"]
pub type RrcstsR = crate::FieldReader;
#[doc = "Field `RXQSTS` reader - RXQSTS"]
pub type RxqstsR = crate::FieldReader;
#[doc = "Field `PRXQ` reader - PRXQ"]
pub type PrxqR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bit 0 - RWCSTS"]
    #[inline(always)]
    pub fn rwcsts(&self) -> RwcstsR {
        RwcstsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RRCSTS"]
    #[inline(always)]
    pub fn rrcsts(&self) -> RrcstsR {
        RrcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 4:5 - RXQSTS"]
    #[inline(always)]
    pub fn rxqsts(&self) -> RxqstsR {
        RxqstsR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 16:29 - PRXQ"]
    #[inline(always)]
    pub fn prxq(&self) -> PrxqR {
        PrxqR::new(((self.bits >> 16) & 0x3fff) as u16)
    }
}
#[doc = "Rx queue debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlrxQdrSpec;
impl crate::RegisterSpec for MtlrxQdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qdr::R`](R) reader structure"]
impl crate::Readable for MtlrxQdrSpec {}
#[doc = "`reset()` method sets MTLRxQDR to value 0"]
impl crate::Resettable for MtlrxQdrSpec {}
