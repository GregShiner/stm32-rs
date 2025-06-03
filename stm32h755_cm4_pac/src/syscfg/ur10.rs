#[doc = "Register `UR10` reader"]
pub type R = crate::R<Ur10Spec>;
#[doc = "Field `PA_END_2` reader - Protected area end address for bank 2"]
pub type PaEnd2R = crate::FieldReader<u16>;
#[doc = "Field `SA_BEG_2` reader - Secured area start address for bank 2"]
pub type SaBeg2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Protected area end address for bank 2"]
    #[inline(always)]
    pub fn pa_end_2(&self) -> PaEnd2R {
        PaEnd2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area start address for bank 2"]
    #[inline(always)]
    pub fn sa_beg_2(&self) -> SaBeg2R {
        SaBeg2R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`ur10::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur10Spec;
impl crate::RegisterSpec for Ur10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur10::R`](R) reader structure"]
impl crate::Readable for Ur10Spec {}
#[doc = "`reset()` method sets UR10 to value 0"]
impl crate::Resettable for Ur10Spec {}
