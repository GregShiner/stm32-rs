#[doc = "Register `LHTR1` reader"]
pub type R = crate::R<Lhtr1Spec>;
#[doc = "Register `LHTR1` writer"]
pub type W = crate::W<Lhtr1Spec>;
#[doc = "Field `LHTR1` reader - ADC analog watchdog 2 threshold low"]
pub type Lhtr1R = crate::FieldReader<u32>;
#[doc = "Field `LHTR1` writer - ADC analog watchdog 2 threshold low"]
pub type Lhtr1W<'a, REG> = crate::FieldWriter<'a, REG, 26, u32>;
impl R {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lhtr1(&self) -> Lhtr1R {
        Lhtr1R::new(self.bits & 0x03ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:25 - ADC analog watchdog 2 threshold low"]
    #[inline(always)]
    pub fn lhtr1(&mut self) -> Lhtr1W<Lhtr1Spec> {
        Lhtr1W::new(self, 0)
    }
}
#[doc = "ADC analog watchdog 2 threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`lhtr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lhtr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Lhtr1Spec;
impl crate::RegisterSpec for Lhtr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lhtr1::R`](R) reader structure"]
impl crate::Readable for Lhtr1Spec {}
#[doc = "`write(|w| ..)` method takes [`lhtr1::W`](W) writer structure"]
impl crate::Writable for Lhtr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LHTR1 to value 0x0fff_0000"]
impl crate::Resettable for Lhtr1Spec {
    const RESET_VALUE: u32 = 0x0fff_0000;
}
