#[doc = "Register `LTR2` reader"]
pub type R = crate::R<Ltr2Spec>;
#[doc = "Register `LTR2` writer"]
pub type W = crate::W<Ltr2Spec>;
#[doc = "Field `LTR2` reader - Analog watchdog 2 lower threshold"]
pub type Ltr2R = crate::FieldReader<u32>;
#[doc = "Field `LTR2` writer - Analog watchdog 2 lower threshold"]
pub type Ltr2W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&self) -> Ltr2R {
        Ltr2R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 lower threshold"]
    #[inline(always)]
    pub fn ltr2(&mut self) -> Ltr2W<Ltr2Spec> {
        Ltr2W::new(self, 0)
    }
}
#[doc = "ADC watchdog lower threshold register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ltr2Spec;
impl crate::RegisterSpec for Ltr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr2::R`](R) reader structure"]
impl crate::Readable for Ltr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ltr2::W`](W) writer structure"]
impl crate::Writable for Ltr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LTR2 to value 0"]
impl crate::Resettable for Ltr2Spec {}
