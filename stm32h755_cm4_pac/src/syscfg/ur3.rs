#[doc = "Register `UR3` reader"]
pub type R = crate::R<Ur3Spec>;
#[doc = "Register `UR3` writer"]
pub type W = crate::W<Ur3Spec>;
#[doc = "Field `BCM4_ADD1` reader - Cortex-M4 Boot Address 0"]
pub type Bcm4Add1R = crate::FieldReader<u16>;
#[doc = "Field `BCM4_ADD1` writer - Cortex-M4 Boot Address 0"]
pub type Bcm4Add1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `BCM7_ADD1` reader - Cortex-M7 Boot Address 1"]
pub type Bcm7Add1R = crate::FieldReader<u16>;
#[doc = "Field `BCM7_ADD1` writer - Cortex-M7 Boot Address 1"]
pub type Bcm7Add1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    pub fn bcm4_add1(&self) -> Bcm4Add1R {
        Bcm4Add1R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    pub fn bcm7_add1(&self) -> Bcm7Add1R {
        Bcm7Add1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Cortex-M4 Boot Address 0"]
    #[inline(always)]
    pub fn bcm4_add1(&mut self) -> Bcm4Add1W<Ur3Spec> {
        Bcm4Add1W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 1"]
    #[inline(always)]
    pub fn bcm7_add1(&mut self) -> Bcm7Add1W<Ur3Spec> {
        Bcm7Add1W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`ur3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur3Spec;
impl crate::RegisterSpec for Ur3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur3::R`](R) reader structure"]
impl crate::Readable for Ur3Spec {}
#[doc = "`write(|w| ..)` method takes [`ur3::W`](W) writer structure"]
impl crate::Writable for Ur3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR3 to value 0"]
impl crate::Resettable for Ur3Spec {}
