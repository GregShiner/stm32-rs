#[doc = "Register `FTSR2` reader"]
pub type R = crate::R<Ftsr2Spec>;
#[doc = "Register `FTSR2` writer"]
pub type W = crate::W<Ftsr2Spec>;
#[doc = "Field `TR49` reader - Falling trigger event configuration bit of Configurable Event input x+32"]
pub type Tr49R = crate::BitReader;
#[doc = "Field `TR49` writer - Falling trigger event configuration bit of Configurable Event input x+32"]
pub type Tr49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR51` reader - Falling trigger event configuration bit of Configurable Event input x+32"]
pub type Tr51R = crate::BitReader;
#[doc = "Field `TR51` writer - Falling trigger event configuration bit of Configurable Event input x+32"]
pub type Tr51W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - Falling trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&self) -> Tr49R {
        Tr49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Falling trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&self) -> Tr51R {
        Tr51R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Falling trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&mut self) -> Tr49W<Ftsr2Spec> {
        Tr49W::new(self, 17)
    }
    #[doc = "Bit 19 - Falling trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&mut self) -> Tr51W<Ftsr2Spec> {
        Tr51W::new(self, 19)
    }
}
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftsr2Spec;
impl crate::RegisterSpec for Ftsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr2::R`](R) reader structure"]
impl crate::Readable for Ftsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`ftsr2::W`](W) writer structure"]
impl crate::Writable for Ftsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR2 to value 0"]
impl crate::Resettable for Ftsr2Spec {}
