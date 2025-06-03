#[doc = "Register `LTR1` reader"]
pub type R = crate::R<Ltr1Spec>;
#[doc = "Register `LTR1` writer"]
pub type W = crate::W<Ltr1Spec>;
#[doc = "Field `LTR1` reader - ADC analog watchdog 1 threshold low"]
pub type Ltr1R = crate::FieldReader<u32>;
#[doc = "Field `LTR1` writer - ADC analog watchdog 1 threshold low"]
pub type Ltr1W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn ltr1(&self) -> Ltr1R {
        Ltr1R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 1 threshold low"]
    #[inline(always)]
    pub fn ltr1(&mut self) -> Ltr1W<Ltr1Spec> {
        Ltr1W::new(self, 0)
    }
}
#[doc = "ADC analog watchdog 1 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`ltr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ltr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ltr1Spec;
impl crate::RegisterSpec for Ltr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ltr1::R`](R) reader structure"]
impl crate::Readable for Ltr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ltr1::W`](W) writer structure"]
impl crate::Writable for Ltr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LTR1 to value 0x0fff_0000"]
impl crate::Resettable for Ltr1Spec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
