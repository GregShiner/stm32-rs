#[doc = "Register `MDMA_C2MAR` reader"]
pub type R = crate::R<MdmaC2marSpec>;
#[doc = "Register `MDMA_C2MAR` writer"]
pub type W = crate::W<MdmaC2marSpec>;
#[doc = "Field `MAR` reader - Mask address"]
pub type MarR = crate::FieldReader<u32>;
#[doc = "Field `MAR` writer - Mask address"]
pub type MarW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&self) -> MarR {
        MarR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Mask address"]
    #[inline(always)]
    pub fn mar(&mut self) -> MarW<MdmaC2marSpec> {
        MarW::new(self, 0)
    }
}
#[doc = "MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC2marSpec;
impl crate::RegisterSpec for MdmaC2marSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c2mar::R`](R) reader structure"]
impl crate::Readable for MdmaC2marSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c2mar::W`](W) writer structure"]
impl crate::Writable for MdmaC2marSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C2MAR to value 0"]
impl crate::Resettable for MdmaC2marSpec {}
