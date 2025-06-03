#[doc = "Register `UR12` reader"]
pub type R = crate::R<Ur12Spec>;
#[doc = "Field `IWDG2M` reader - Independent Watchdog 2 mode"]
pub type Iwdg2mR = crate::BitReader;
#[doc = "Field `SECURE` reader - Secure mode"]
pub type SecureR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Independent Watchdog 2 mode"]
    #[inline(always)]
    pub fn iwdg2m(&self) -> Iwdg2mR {
        Iwdg2mR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Secure mode"]
    #[inline(always)]
    pub fn secure(&self) -> SecureR {
        SecureR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`ur12::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur12Spec;
impl crate::RegisterSpec for Ur12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur12::R`](R) reader structure"]
impl crate::Readable for Ur12Spec {}
#[doc = "`reset()` method sets UR12 to value 0"]
impl crate::Resettable for Ur12Spec {}
