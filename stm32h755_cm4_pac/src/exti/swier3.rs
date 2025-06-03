#[doc = "Register `SWIER3` reader"]
pub type R = crate::R<Swier3Spec>;
#[doc = "Register `SWIER3` writer"]
pub type W = crate::W<Swier3Spec>;
#[doc = "Field `SWIER82` reader - Software interrupt on line x+64"]
pub type Swier82R = crate::BitReader;
#[doc = "Field `SWIER82` writer - Software interrupt on line x+64"]
pub type Swier82W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER84` reader - Software interrupt on line x+64"]
pub type Swier84R = crate::BitReader;
#[doc = "Field `SWIER84` writer - Software interrupt on line x+64"]
pub type Swier84W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER85` reader - Software interrupt on line x+64"]
pub type Swier85R = crate::BitReader;
#[doc = "Field `SWIER85` writer - Software interrupt on line x+64"]
pub type Swier85W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER86` reader - Software interrupt on line x+64"]
pub type Swier86R = crate::BitReader;
#[doc = "Field `SWIER86` writer - Software interrupt on line x+64"]
pub type Swier86W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&self) -> Swier82R {
        Swier82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&self) -> Swier84R {
        Swier84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&self) -> Swier85R {
        Swier85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&self) -> Swier86R {
        Swier86R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier82(&mut self) -> Swier82W<Swier3Spec> {
        Swier82W::new(self, 18)
    }
    #[doc = "Bit 20 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier84(&mut self) -> Swier84W<Swier3Spec> {
        Swier84W::new(self, 20)
    }
    #[doc = "Bit 21 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier85(&mut self) -> Swier85W<Swier3Spec> {
        Swier85W::new(self, 21)
    }
    #[doc = "Bit 22 - Software interrupt on line x+64"]
    #[inline(always)]
    pub fn swier86(&mut self) -> Swier86W<Swier3Spec> {
        Swier86W::new(self, 22)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swier3Spec;
impl crate::RegisterSpec for Swier3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier3::R`](R) reader structure"]
impl crate::Readable for Swier3Spec {}
#[doc = "`write(|w| ..)` method takes [`swier3::W`](W) writer structure"]
impl crate::Writable for Swier3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER3 to value 0"]
impl crate::Resettable for Swier3Spec {}
