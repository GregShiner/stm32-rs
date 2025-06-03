#[doc = "Register `SWIER2` reader"]
pub type R = crate::R<Swier2Spec>;
#[doc = "Register `SWIER2` writer"]
pub type W = crate::W<Swier2Spec>;
#[doc = "Field `SWIER49` reader - Software interrupt on line x+32"]
pub type Swier49R = crate::BitReader;
#[doc = "Field `SWIER49` writer - Software interrupt on line x+32"]
pub type Swier49W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWIER51` reader - Software interrupt on line x+32"]
pub type Swier51R = crate::BitReader;
#[doc = "Field `SWIER51` writer - Software interrupt on line x+32"]
pub type Swier51W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&self) -> Swier49R {
        Swier49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&self) -> Swier51R {
        Swier51R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 17 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier49(&mut self) -> Swier49W<Swier2Spec> {
        Swier49W::new(self, 17)
    }
    #[doc = "Bit 19 - Software interrupt on line x+32"]
    #[inline(always)]
    pub fn swier51(&mut self) -> Swier51W<Swier2Spec> {
        Swier51W::new(self, 19)
    }
}
#[doc = "EXTI software interrupt event register\n\nYou can [`read`](crate::Reg::read) this register and get [`swier2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`swier2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Swier2Spec;
impl crate::RegisterSpec for Swier2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`swier2::R`](R) reader structure"]
impl crate::Readable for Swier2Spec {}
#[doc = "`write(|w| ..)` method takes [`swier2::W`](W) writer structure"]
impl crate::Writable for Swier2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SWIER2 to value 0"]
impl crate::Resettable for Swier2Spec {}
