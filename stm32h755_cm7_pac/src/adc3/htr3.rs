#[doc = "Register `HTR3` reader"]
pub type R = crate::R<Htr3Spec>;
#[doc = "Register `HTR3` writer"]
pub type W = crate::W<Htr3Spec>;
#[doc = "Field `HTR3` reader - Analog watchdog 3 higher threshold"]
pub type Htr3R = crate::FieldReader<u32>;
#[doc = "Field `HTR3` writer - Analog watchdog 3 higher threshold"]
pub type Htr3W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&self) -> Htr3R {
        Htr3R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 3 higher threshold"]
    #[inline(always)]
    pub fn htr3(&mut self) -> Htr3W<Htr3Spec> {
        Htr3W::new(self, 0)
    }
}
#[doc = "ADC watchdog higher threshold register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`htr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Htr3Spec;
impl crate::RegisterSpec for Htr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr3::R`](R) reader structure"]
impl crate::Readable for Htr3Spec {}
#[doc = "`write(|w| ..)` method takes [`htr3::W`](W) writer structure"]
impl crate::Writable for Htr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HTR3 to value 0"]
impl crate::Resettable for Htr3Spec {}
