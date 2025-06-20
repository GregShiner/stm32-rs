#[doc = "Register `S2M0AR` reader"]
pub type R = crate::R<S2m0arSpec>;
#[doc = "Register `S2M0AR` writer"]
pub type W = crate::W<S2m0arSpec>;
#[doc = "Field `M0A` reader - Memory 0 address"]
pub type M0aR = crate::FieldReader<u32>;
#[doc = "Field `M0A` writer - Memory 0 address"]
pub type M0aW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&self) -> M0aR {
        M0aR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Memory 0 address"]
    #[inline(always)]
    pub fn m0a(&mut self) -> M0aW<S2m0arSpec> {
        M0aW::new(self, 0)
    }
}
#[doc = "stream x memory 0 address register\n\nYou can [`read`](crate::Reg::read) this register and get [`s2m0ar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`s2m0ar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S2m0arSpec;
impl crate::RegisterSpec for S2m0arSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s2m0ar::R`](R) reader structure"]
impl crate::Readable for S2m0arSpec {}
#[doc = "`write(|w| ..)` method takes [`s2m0ar::W`](W) writer structure"]
impl crate::Writable for S2m0arSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets S2M0AR to value 0"]
impl crate::Resettable for S2m0arSpec {}
