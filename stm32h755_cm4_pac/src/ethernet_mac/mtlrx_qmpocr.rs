#[doc = "Register `MTLRxQMPOCR` reader"]
pub type R = crate::R<MtlrxQmpocrSpec>;
#[doc = "Field `OVFPKTCNT` reader - OVFPKTCNT"]
pub type OvfpktcntR = crate::FieldReader<u16>;
#[doc = "Field `OVFCNTOVF` reader - OVFCNTOVF"]
pub type OvfcntovfR = crate::BitReader;
#[doc = "Field `MISPKTCNT` reader - MISPKTCNT"]
pub type MispktcntR = crate::FieldReader<u16>;
#[doc = "Field `MISCNTOVF` reader - MISCNTOVF"]
pub type MiscntovfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - OVFPKTCNT"]
    #[inline(always)]
    pub fn ovfpktcnt(&self) -> OvfpktcntR {
        OvfpktcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - OVFCNTOVF"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OvfcntovfR {
        OvfcntovfR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 16:26 - MISPKTCNT"]
    #[inline(always)]
    pub fn mispktcnt(&self) -> MispktcntR {
        MispktcntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
    #[doc = "Bit 27 - MISCNTOVF"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MiscntovfR {
        MiscntovfR::new(((self.bits >> 27) & 1) != 0)
    }
}
#[doc = "Rx queue missed packet and overflow counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlrx_qmpocr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlrxQmpocrSpec;
impl crate::RegisterSpec for MtlrxQmpocrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlrx_qmpocr::R`](R) reader structure"]
impl crate::Readable for MtlrxQmpocrSpec {}
#[doc = "`reset()` method sets MTLRxQMPOCR to value 0"]
impl crate::Resettable for MtlrxQmpocrSpec {}
