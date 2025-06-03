#[doc = "Register `CPUPR2` reader"]
pub type R = crate::R<Cpupr2Spec>;
#[doc = "Field `PR49` reader - Configurable event inputs x+32 Pending bit"]
pub type Pr49R = crate::BitReader;
#[doc = "Field `PR51` reader - Configurable event inputs x+32 Pending bit"]
pub type Pr51R = crate::BitReader;
impl R {
    #[doc = "Bit 17 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr49(&self) -> Pr49R {
        Pr49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - Configurable event inputs x+32 Pending bit"]
    #[inline(always)]
    pub fn pr51(&self) -> Pr51R {
        Pr51R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpupr2Spec;
impl crate::RegisterSpec for Cpupr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpupr2::R`](R) reader structure"]
impl crate::Readable for Cpupr2Spec {}
#[doc = "`reset()` method sets CPUPR2 to value 0"]
impl crate::Resettable for Cpupr2Spec {}
