#[doc = "Register `UR16` reader"]
pub type R = crate::R<Ur16Spec>;
#[doc = "Field `FZIWDGSTP` reader - Freeze independent watchdog in Stop mode"]
pub type FziwdgstpR = crate::BitReader;
#[doc = "Field `PKP` reader - Private key programmed"]
pub type PkpR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Freeze independent watchdog in Stop mode"]
    #[inline(always)]
    pub fn fziwdgstp(&self) -> FziwdgstpR {
        FziwdgstpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 16 - Private key programmed"]
    #[inline(always)]
    pub fn pkp(&self) -> PkpR {
        PkpR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`ur16::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur16Spec;
impl crate::RegisterSpec for Ur16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur16::R`](R) reader structure"]
impl crate::Readable for Ur16Spec {}
#[doc = "`reset()` method sets UR16 to value 0"]
impl crate::Resettable for Ur16Spec {}
