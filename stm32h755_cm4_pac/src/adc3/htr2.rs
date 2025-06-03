#[doc = "Register `HTR2` reader"]
pub type R = crate::R<Htr2Spec>;
#[doc = "Register `HTR2` writer"]
pub type W = crate::W<Htr2Spec>;
#[doc = "Field `HTR2` reader - Analog watchdog 2 higher threshold"]
pub type Htr2R = crate::FieldReader<u32>;
#[doc = "Field `HTR2` writer - Analog watchdog 2 higher threshold"]
pub type Htr2W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn htr2(&self) -> Htr2R {
        Htr2R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - Analog watchdog 2 higher threshold"]
    #[inline(always)]
    pub fn htr2(&mut self) -> Htr2W<Htr2Spec> {
        Htr2W::new(self, 0)
    }
}
#[doc = "ADC watchdog higher threshold register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`htr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`htr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Htr2Spec;
impl crate::RegisterSpec for Htr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`htr2::R`](R) reader structure"]
impl crate::Readable for Htr2Spec {}
#[doc = "`write(|w| ..)` method takes [`htr2::W`](W) writer structure"]
impl crate::Writable for Htr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HTR2 to value 0"]
impl crate::Resettable for Htr2Spec {}
