#[doc = "Register `UR2` reader"]
pub type R = crate::R<Ur2Spec>;
#[doc = "Register `UR2` writer"]
pub type W = crate::W<Ur2Spec>;
#[doc = "Field `BORH` reader - BOR_LVL Brownout Reset Threshold Level"]
pub type BorhR = crate::FieldReader;
#[doc = "Field `BCM7_ADD0` reader - Cortex-M7 Boot Address 0"]
pub type Bcm7Add0R = crate::FieldReader<u16>;
#[doc = "Field `BCM7_ADD0` writer - Cortex-M7 Boot Address 0"]
pub type Bcm7Add0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:1 - BOR_LVL Brownout Reset Threshold Level"]
    #[inline(always)]
    pub fn borh(&self) -> BorhR {
        BorhR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 0"]
    #[inline(always)]
    pub fn bcm7_add0(&self) -> Bcm7Add0R {
        Bcm7Add0R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - Cortex-M7 Boot Address 0"]
    #[inline(always)]
    pub fn bcm7_add0(&mut self) -> Bcm7Add0W<Ur2Spec> {
        Bcm7Add0W::new(self, 16)
    }
}
#[doc = "SYSCFG user register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ur2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ur2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur2Spec;
impl crate::RegisterSpec for Ur2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur2::R`](R) reader structure"]
impl crate::Readable for Ur2Spec {}
#[doc = "`write(|w| ..)` method takes [`ur2::W`](W) writer structure"]
impl crate::Writable for Ur2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UR2 to value 0"]
impl crate::Resettable for Ur2Spec {}
