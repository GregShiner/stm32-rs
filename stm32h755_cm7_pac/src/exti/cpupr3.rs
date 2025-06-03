#[doc = "Register `CPUPR3` reader"]
pub type R = crate::R<Cpupr3Spec>;
#[doc = "Field `PR82` reader - Configurable event inputs x+64 Pending bit"]
pub type Pr82R = crate::BitReader;
#[doc = "Field `PR84` reader - Configurable event inputs x+64 Pending bit"]
pub type Pr84R = crate::BitReader;
#[doc = "Field `PR85` reader - Configurable event inputs x+64 Pending bit"]
pub type Pr85R = crate::BitReader;
#[doc = "Field `PR86` reader - Configurable event inputs x+64 Pending bit"]
pub type Pr86R = crate::BitReader;
impl R {
    #[doc = "Bit 18 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr82(&self) -> Pr82R {
        Pr82R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr84(&self) -> Pr84R {
        Pr84R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr85(&self) -> Pr85R {
        Pr85R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Configurable event inputs x+64 Pending bit"]
    #[inline(always)]
    pub fn pr86(&self) -> Pr86R {
        Pr86R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpupr3Spec;
impl crate::RegisterSpec for Cpupr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpupr3::R`](R) reader structure"]
impl crate::Readable for Cpupr3Spec {}
#[doc = "`reset()` method sets CPUPR3 to value 0"]
impl crate::Resettable for Cpupr3Spec {}
