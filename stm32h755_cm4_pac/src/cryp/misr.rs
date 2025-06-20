#[doc = "Register `MISR` reader"]
pub type R = crate::R<MisrSpec>;
#[doc = "Field `INMIS` reader - Input FIFO service masked interrupt status"]
pub type InmisR = crate::BitReader;
#[doc = "Field `OUTMIS` reader - Output FIFO service masked interrupt status"]
pub type OutmisR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Input FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn inmis(&self) -> InmisR {
        InmisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Output FIFO service masked interrupt status"]
    #[inline(always)]
    pub fn outmis(&self) -> OutmisR {
        OutmisR::new(((self.bits >> 1) & 1) != 0)
    }
}
#[doc = "masked interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`misr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MisrSpec;
impl crate::RegisterSpec for MisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misr::R`](R) reader structure"]
impl crate::Readable for MisrSpec {}
#[doc = "`reset()` method sets MISR to value 0"]
impl crate::Resettable for MisrSpec {}
