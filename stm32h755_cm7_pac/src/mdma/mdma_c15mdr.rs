#[doc = "Register `MDMA_C15MDR` reader"]
pub type R = crate::R<MdmaC15mdrSpec>;
#[doc = "Register `MDMA_C15MDR` writer"]
pub type W = crate::W<MdmaC15mdrSpec>;
#[doc = "Field `MDR` reader - Mask data"]
pub type MdrR = crate::FieldReader<u32>;
#[doc = "Field `MDR` writer - Mask data"]
pub type MdrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&self) -> MdrR {
        MdrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask data"]
    #[inline(always)]
    pub fn mdr(&mut self) -> MdrW<MdmaC15mdrSpec> {
        MdrW::new(self, 0)
    }
}
#[doc = "MDMA channel x Mask Data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c15mdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c15mdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC15mdrSpec;
impl crate::RegisterSpec for MdmaC15mdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c15mdr::R`](R) reader structure"]
impl crate::Readable for MdmaC15mdrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c15mdr::W`](W) writer structure"]
impl crate::Writable for MdmaC15mdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C15MDR to value 0"]
impl crate::Resettable for MdmaC15mdrSpec {}
