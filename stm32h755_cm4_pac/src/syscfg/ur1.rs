#[doc = "Register `UR1` reader"]
pub type R = crate::R<Ur1Spec>;
#[doc = "Register `UR1` writer"]
pub type W = crate::W<Ur1Spec>;
#[doc = "Field `BCM4` reader - Boot Cortex-M4"]
pub type Bcm4R = crate::BitReader;
#[doc = "Field `BCM4` writer - Boot Cortex-M4"]
pub type Bcm4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BCM7` reader - Boot Cortex-M7"]
pub type Bcm7R = crate::BitReader;
#[doc = "Field `BCM7` writer - Boot Cortex-M7"]
pub type Bcm7W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&self) -> Bcm4R {
        Bcm4R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&self) -> Bcm7R {
        Bcm7R::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Boot Cortex-M4"]
    #[inline(always)]
    pub fn bcm4(&mut self) -> Bcm4W<Ur1Spec> {
        Bcm4W::new(self, 0)
    }
    #[doc = "Bit 16 - Boot Cortex-M7"]
    #[inline(always)]
    pub fn bcm7(&mut self) -> Bcm7W<Ur1Spec> {
        Bcm7W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ur1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur1Spec;
impl crate::RegisterSpec for Ur1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur1::R`](R) reader structure"]
impl crate::Readable for Ur1Spec {}
#[doc = "`write(|w| ..)` method takes [`ur1::W`](W) writer structure"]
impl crate::Writable for Ur1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR1 to value 0"]
impl crate::Resettable for Ur1Spec {}
