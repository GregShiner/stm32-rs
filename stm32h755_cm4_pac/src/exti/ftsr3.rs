#[doc = "Register `FTSR3` reader"]
pub type R = crate::R<Ftsr3Spec>;
#[doc = "Register `FTSR3` writer"]
pub type W = crate::W<Ftsr3Spec>;
#[doc = "Field `TR82` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr82R = crate::BitReader;
#[doc = "Field `TR82` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr82W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR84` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr84R = crate::BitReader;
#[doc = "Field `TR84` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR85` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr85R = crate::BitReader;
#[doc = "Field `TR85` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr85W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TR86` reader - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr86R = crate::BitReader;
#[doc = "Field `TR86` writer - Falling trigger event configuration bit of Configurable Event input x+64"]
pub type Tr86W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&self) -> Tr82R {
        Tr82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&self) -> Tr84R {
        Tr84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&self) -> Tr85R {
        Tr85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&self) -> Tr86R {
        Tr86R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr82(&mut self) -> Tr82W<Ftsr3Spec> {
        Tr82W::new(self, 18)
    }
    #[doc = "Bit 20 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr84(&mut self) -> Tr84W<Ftsr3Spec> {
        Tr84W::new(self, 20)
    }
    #[doc = "Bit 21 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr85(&mut self) -> Tr85W<Ftsr3Spec> {
        Tr85W::new(self, 21)
    }
    #[doc = "Bit 22 - Falling trigger event configuration bit of Configurable Event input x+64"]
    #[inline(always)]
    pub fn tr86(&mut self) -> Tr86W<Ftsr3Spec> {
        Tr86W::new(self, 22)
    }
}
#[doc = "EXTI falling trigger selection register\n\nYou can [`read`](crate::Reg::read) this register and get [`ftsr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftsr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ftsr3Spec;
impl crate::RegisterSpec for Ftsr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ftsr3::R`](R) reader structure"]
impl crate::Readable for Ftsr3Spec {}
#[doc = "`write(|w| ..)` method takes [`ftsr3::W`](W) writer structure"]
impl crate::Writable for Ftsr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FTSR3 to value 0"]
impl crate::Resettable for Ftsr3Spec {}
