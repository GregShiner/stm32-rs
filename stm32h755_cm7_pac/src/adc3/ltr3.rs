#[doc = "Register `LTR3` reader"]
pub type R = crate::R<Ltr3Spec>;
#[doc = "Register `LTR3` writer"]
pub type W = crate::W<Ltr3Spec>;
#[doc = "Field `LTR3` reader - Analog watchdog 3 lower threshold"]
pub type Ltr3R = crate::FieldReader<u32>;
#[doc = "Field `LTR3` writer - Analog watchdog 3 lower threshold"]
pub type Ltr3W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&self) -> Ltr3R {
        Ltr3R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 lower threshold"]
    #[inline(always)]
    pub fn ltr3(&mut self) -> Ltr3W<Ltr3Spec> {
        Ltr3W::new(self, 0)
    }
}
#[doc = "ADC watchdog lower threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ltr3Spec;
impl crate::RegisterSpec for Ltr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr3::R`](R) reader structure"]
impl crate::Readable for Ltr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ltr3::W`](W) writer structure"]
impl crate::Writable for Ltr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LTR3 to value 0"]
impl crate::Resettable for Ltr3Spec {}
