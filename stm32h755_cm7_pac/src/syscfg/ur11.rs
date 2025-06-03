#[doc = "Register `UR11` reader"]
pub type R = crate::R<Ur11Spec>;
#[doc = "Field `SA_END_2` reader - Secured area end address for bank 2"]
pub type SaEnd2R = crate::FieldReader<u16>;
#[doc = "Field `IWDG1M` reader - Independent Watchdog 1 mode"]
pub type Iwdg1mR = crate::BitReader;
impl R {
    #[doc = "Bits 0:11 - Secured area end address for bank 2"]
    #[inline(always)]
    pub fn sa_end_2(&self) -> SaEnd2R {
        SaEnd2R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bit 16 - Independent Watchdog 1 mode"]
    #[inline(always)]
    pub fn iwdg1m(&self) -> Iwdg1mR {
        Iwdg1mR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "SYSCFG user register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`ur11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ur11Spec;
impl crate::RegisterSpec for Ur11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ur11::R`](R) reader structure"]
impl crate::Readable for Ur11Spec {}
#[doc = "`reset()` method sets UR11 to value 0"]
impl crate::Resettable for Ur11Spec {}
