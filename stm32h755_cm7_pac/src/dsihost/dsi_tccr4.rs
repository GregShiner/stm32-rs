#[doc = "Register `DSI_TCCR4` reader"]
pub type R = crate::R<DsiTccr4Spec>;
#[doc = "Register `DSI_TCCR4` writer"]
pub type W = crate::W<DsiTccr4Spec>;
#[doc = "Field `LPWR_TOCNT` reader - LPWR_TOCNT"]
pub type LpwrTocntR = crate::FieldReader<u16>;
#[doc = "Field `LPWR_TOCNT` writer - LPWR_TOCNT"]
pub type LpwrTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    pub fn lpwr_tocnt(&self) -> LpwrTocntR {
        LpwrTocntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPWR_TOCNT"]
    #[inline(always)]
    pub fn lpwr_tocnt(&mut self) -> LpwrTocntW<DsiTccr4Spec> {
        LpwrTocntW::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr4Spec;
impl crate::RegisterSpec for DsiTccr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr4::R`](R) reader structure"]
impl crate::Readable for DsiTccr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr4::W`](W) writer structure"]
impl crate::Writable for DsiTccr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR4 to value 0"]
impl crate::Resettable for DsiTccr4Spec {}
