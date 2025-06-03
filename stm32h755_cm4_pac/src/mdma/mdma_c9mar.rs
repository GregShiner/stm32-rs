#[doc = "Register `MDMA_C9MAR` reader"]
pub type R = crate::R<MdmaC9marSpec>;
#[doc = "Register `MDMA_C9MAR` writer"]
pub type W = crate::W<MdmaC9marSpec>;
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
    pub fn mar(&mut self) -> MarW<MdmaC9marSpec> {
        MarW::new(self, 0)
    }
}
#[doc = "MDMA channel x Mask address register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c9mar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c9mar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC9marSpec;
impl crate::RegisterSpec for MdmaC9marSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c9mar::R`](R) reader structure"]
impl crate::Readable for MdmaC9marSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c9mar::W`](W) writer structure"]
impl crate::Writable for MdmaC9marSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C9MAR to value 0"]
impl crate::Resettable for MdmaC9marSpec {}
