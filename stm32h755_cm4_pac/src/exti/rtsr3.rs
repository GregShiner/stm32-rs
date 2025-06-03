#[doc = "Register `RTSR3` reader"]
pub type R = crate::R<Rtsr3Spec>;
#[doc = "Register `RTSR3` writer"]
pub type W = crate::W<Rtsr3Spec>;
#[doc = "Field `TR82` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr82R = crate::BitReader;
#[doc = "Field `TR82` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr82W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR84` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr84R = crate::BitReader;
#[doc = "Field `TR84` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR85` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr85R = crate::BitReader;
#[doc = "Field `TR85` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr85W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR86` reader - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr86R = crate::BitReader;
#[doc = "Field `TR86` writer - Rising trigger event configuration bit of Configurable Event input x+64"]
pub type Tr86W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> Tr82R {
        Tr82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> Tr84R {
        Tr84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> Tr85R {
        Tr85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> Tr86R {
        Tr86R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&mut self) -> Tr82W<Rtsr3Spec> {
        Tr82W::new(self, 18)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&mut self) -> Tr84W<Rtsr3Spec> {
        Tr84W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&mut self) -> Tr85W<Rtsr3Spec> {
        Tr85W::new(self, 21)
    }
    #[doc = "Bit 22 - Rising trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&mut self) -> Tr86W<Rtsr3Spec> {
        Tr86W::new(self, 22)
    }
}
#[doc = "EXTI rising trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Rtsr3Spec;
impl crate::RegisterSpec for Rtsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtsr3::R`](R) reader structure"]
impl crate::Readable for Rtsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`rtsr3::W`](W) writer structure"]
impl crate::Writable for Rtsr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTSR3 to value 0"]
impl crate::Resettable for Rtsr3Spec {}
