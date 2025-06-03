#[doc = "Register `MDMA_C1LAR` reader"]
pub type R = crate::R<MdmaC1larSpec>;
#[doc = "Register `MDMA_C1LAR` writer"]
pub type W = crate::W<MdmaC1larSpec>;
#[doc = "Field `LAR` reader - Link address register"]
pub type LarR = crate::FieldReader<u32>;
#[doc = "Field `LAR` writer - Link address register"]
pub type LarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&self) -> LarR {
        LarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Link address register"]
    #[inline(always)]
    pub fn lar(&mut self) -> LarW<MdmaC1larSpec> {
        LarW::new(self, 0)
    }
}
#[doc = "MDMA channel x Link Address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c1lar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c1lar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC1larSpec;
impl crate::RegisterSpec for MdmaC1larSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c1lar::R`](R) reader structure"]
impl crate::Readable for MdmaC1larSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c1lar::W`](W) writer structure"]
impl crate::Writable for MdmaC1larSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C1LAR to value 0"]
impl crate::Resettable for MdmaC1larSpec {}
