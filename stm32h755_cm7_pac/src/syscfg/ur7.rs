#[doc = "Register `UR7` reader"]
pub type R = crate::R<Ur7Spec>;
#[doc = "Field `SA_BEG_1` reader - Secured area start address for bank 1"]
pub type SaBeg1R = crate::FieldReader<u16>;
#[doc = "Field `SA_END_1` reader - Secured area end address for bank 1"]
pub type SaEnd1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:11 - Secured area start address for bank 1"]
    #[inline(always)]
    pub fn sa_beg_1(&self) -> SaBeg1R {
        SaBeg1R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:27 - Secured area end address for bank 1"]
    #[inline(always)]
    pub fn sa_end_1(&self) -> SaEnd1R {
        SaEnd1R::new(((self.bits >> 16) & 0x0fff) as u16)
    }
}
#[doc = "SYSCFG user register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`ur7::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur7Spec;
impl crate::RegisterSpec for Ur7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur7::R`](R) reader structure"]
impl crate::Readable for Ur7Spec {}
#[doc = "`reset()` method sets UR7 to value 0"]
impl crate::Resettable for Ur7Spec {}
