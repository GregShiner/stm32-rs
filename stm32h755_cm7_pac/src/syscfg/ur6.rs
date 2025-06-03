#[doc = "Register `UR6` reader"]
pub type R = crate::R<Ur6Spec>;
#[doc = "Field `PA_BEG_1` reader - Protected area start address for bank 1"]
pub type PaBeg1R = crate::FieldReader<u16>;
#[doc = "Field `PA_END_1` reader - Protected area end address for bank 1"]
pub type PaEnd1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area start address for bank 1"]
    #[inline(always)]
    pub fn pa_beg_1(&self) -> PaBeg1R {
        PaBeg1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Protected area end address for bank 1"]
    #[inline(always)]
    pub fn pa_end_1(&self) -> PaEnd1R {
        PaEnd1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 6\n\nYou can [`read`](crate::Reg::read) this register and get [`ur6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur6Spec;
impl crate::RegisterSpec for Ur6Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur6::R`](R) reader structure"]
impl crate::Readable for Ur6Spec {}
#[doc = "`reset()` method sets UR6 to value 0"]
impl crate::Resettable for Ur6Spec {}
