#[doc = "Register `OPTKEYR_` reader"]
pub type R = crate::R<Optkeyr_Spec>;
#[doc = "Register `OPTKEYR_` writer"]
pub type W = crate::W<Optkeyr_Spec>;
#[doc = "Field `OPTKEYR` reader - Unlock key option bytes"]
pub type OptkeyrR = crate::FieldReader<u32>;
#[doc = "Field `OPTKEYR` writer - Unlock key option bytes"]
pub type OptkeyrW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&self) -> OptkeyrR {
        OptkeyrR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Unlock key option bytes"]
    #[inline(always)]
    pub fn optkeyr(&mut self) -> OptkeyrW<Optkeyr_Spec> {
        OptkeyrW::new(self, 0)
    }
}
#[doc = "FLASH option key register\n\nYou can [`read`](crate::Reg::read) this register and get [`optkeyr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optkeyr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Optkeyr_Spec;
impl crate::RegisterSpec for Optkeyr_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`optkeyr_::R`](R) reader structure"]
impl crate::Readable for Optkeyr_Spec {}
#[doc = "`write(|w| ..)` method takes [`optkeyr_::W`](W) writer structure"]
impl crate::Writable for Optkeyr_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTKEYR_ to value 0"]
impl crate::Resettable for Optkeyr_Spec {}
