#[doc = "Register `OPTCCR_` writer"]
pub type W = crate::W<Optccr_Spec>;
#[doc = "Field `CLR_OPTCHANGEERR` writer - OPTCHANGEERR reset bit"]
pub type ClrOptchangeerrW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 30 - OPTCHANGEERR reset bit"]
    #[inline(always)]
    pub fn clr_optchangeerr(&mut self) -> ClrOptchangeerrW<Optccr_Spec> {
        ClrOptchangeerrW::new(self, 30)
    }
}
#[doc = "FLASH option clear control register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`optccr_::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Optccr_Spec;
impl crate::RegisterSpec for Optccr_Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`optccr_::W`](W) writer structure"]
impl crate::Writable for Optccr_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OPTCCR_ to value 0"]
impl crate::Resettable for Optccr_Spec {}
