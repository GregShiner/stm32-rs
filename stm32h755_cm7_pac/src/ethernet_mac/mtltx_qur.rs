#[doc = "Register `MTLTxQUR` reader"]
pub type R = crate::R<MtltxQurSpec>;
#[doc = "Field `UFFRMCNT` reader - Underflow Packet Counter"]
pub type UffrmcntR = crate::FieldReader<u16>;
#[doc = "Field `UFCNTOVF` reader - UFCNTOVF"]
pub type UfcntovfR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Underflow Packet Counter"]
    #[inline(always)]
    pub fn uffrmcnt(&self) -> UffrmcntR {
        UffrmcntR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - UFCNTOVF"]
    #[inline(always)]
    pub fn ufcntovf(&self) -> UfcntovfR {
        UfcntovfR::new(((self.bits >> 11) & 1) != 0)
    }
}
#[doc = "Tx queue underflow register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtltx_qur::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtltxQurSpec;
impl crate::RegisterSpec for MtltxQurSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtltx_qur::R`](R) reader structure"]
impl crate::Readable for MtltxQurSpec {}
#[doc = "`reset()` method sets MTLTxQUR to value 0"]
impl crate::Resettable for MtltxQurSpec {}
