#[doc = "Register `DSI_TCCR2` reader"]
pub type R = crate::R<DsiTccr2Spec>;
#[doc = "Register `DSI_TCCR2` writer"]
pub type W = crate::W<DsiTccr2Spec>;
#[doc = "Field `LPRD_TOCNT` reader - LPRD_TOCNT"]
pub type LprdTocntR = crate::FieldReader<u16>;
#[doc = "Field `LPRD_TOCNT` writer - LPRD_TOCNT"]
pub type LprdTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LPRD_TOCNT"]
    #[inline(always)]
    pub fn lprd_tocnt(&self) -> LprdTocntR {
        LprdTocntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPRD_TOCNT"]
    #[inline(always)]
    pub fn lprd_tocnt(&mut self) -> LprdTocntW<DsiTccr2Spec> {
        LprdTocntW::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr2Spec;
impl crate::RegisterSpec for DsiTccr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr2::R`](R) reader structure"]
impl crate::Readable for DsiTccr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr2::W`](W) writer structure"]
impl crate::Writable for DsiTccr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR2 to value 0"]
impl crate::Resettable for DsiTccr2Spec {}
