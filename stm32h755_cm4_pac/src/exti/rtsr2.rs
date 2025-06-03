#[doc = "Register `RTSR2` reader"]
pub type R = crate::R<Rtsr2Spec>;
#[doc = "Register `RTSR2` writer"]
pub type W = crate::W<Rtsr2Spec>;
#[doc = "Field `TR49` reader - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type Tr49R = crate::BitReader;
#[doc = "Field `TR49` writer - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type Tr49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR51` reader - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type Tr51R = crate::BitReader;
#[doc = "Field `TR51` writer - Rising trigger event configuration bit of Configurable Event input x+32"]
pub type Tr51W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&self) -> Tr49R {
        Tr49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&self) -> Tr51R {
        Tr51R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr49(&mut self) -> Tr49W<Rtsr2Spec> {
        Tr49W::new(self, 17)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input x+32"]
    #[inline(always)]
    pub fn tr51(&mut self) -> Tr51W<Rtsr2Spec> {
        Tr51W::new(self, 19)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr2Spec;
impl crate::RegisterSpec for Rtsr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr2::R`](R) reader structure"]
impl crate::Readable for Rtsr2Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr2::W`](W) writer structure"]
impl crate::Writable for Rtsr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR2 to value 0"]
impl crate::Resettable for Rtsr2Spec {}
