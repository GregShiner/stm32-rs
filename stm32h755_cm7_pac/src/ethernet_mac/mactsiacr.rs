#[doc = "Register `MACTSIACR` reader"]
pub type R = crate::R<MactsiacrSpec>;
#[doc = "Register `MACTSIACR` writer"]
pub type W = crate::W<MactsiacrSpec>;
#[doc = "Field `OSTIAC` reader - OSTIAC"]
pub type OstiacR = crate::FieldReader<u32>;
#[doc = "Field `OSTIAC` writer - OSTIAC"]
pub type OstiacW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&self) -> OstiacR {
        OstiacR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTIAC"]
    #[inline(always)]
    pub fn ostiac(&mut self) -> OstiacW<MactsiacrSpec> {
        OstiacW::new(self, 0)
    }
}
#[doc = "Timestamp Ingress asymmetric correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactsiacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactsiacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactsiacrSpec;
impl crate::RegisterSpec for MactsiacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactsiacr::R`](R) reader structure"]
impl crate::Readable for MactsiacrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactsiacr::W`](W) writer structure"]
impl crate::Writable for MactsiacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSIACR to value 0"]
impl crate::Resettable for MactsiacrSpec {}
