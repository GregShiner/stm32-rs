#[doc = "Register `MDMA_C1SAR` reader"]
pub type R = crate::R<MdmaC1sarSpec>;
#[doc = "Register `MDMA_C1SAR` writer"]
pub type W = crate::W<MdmaC1sarSpec>;
#[doc = "Field `SAR` reader - source adr base"]
pub type SarR = crate::FieldReader<u32>;
#[doc = "Field `SAR` writer - source adr base"]
pub type SarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    pub fn sar(&self) -> SarR {
        SarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - source adr base"]
    #[inline(always)]
    pub fn sar(&mut self) -> SarW<MdmaC1sarSpec> {
        SarW::new(self, 0)
    }
}
#[doc = "MDMA channel x source address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1sar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1sar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC1sarSpec;
impl crate::RegisterSpec for MdmaC1sarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c1sar::R`](R) reader structure"]
impl crate::Readable for MdmaC1sarSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c1sar::W`](W) writer structure"]
impl crate::Writable for MdmaC1sarSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C1SAR to value 0"]
impl crate::Resettable for MdmaC1sarSpec {}
