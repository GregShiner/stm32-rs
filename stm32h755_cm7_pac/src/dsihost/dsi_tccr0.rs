#[doc = "Register `DSI_TCCR0` reader"]
pub type R = crate::R<DsiTccr0Spec>;
#[doc = "Register `DSI_TCCR0` writer"]
pub type W = crate::W<DsiTccr0Spec>;
#[doc = "Field `LPRX_TOCNT` reader - LPRX_TOCNT"]
pub type LprxTocntR = crate::FieldReader<u16>;
#[doc = "Field `LPRX_TOCNT` writer - LPRX_TOCNT"]
pub type LprxTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `HSTX_TOCNT` reader - HSTX_TOCNT"]
pub type HstxTocntR = crate::FieldReader<u16>;
#[doc = "Field `HSTX_TOCNT` writer - HSTX_TOCNT"]
pub type HstxTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - LPRX_TOCNT"]
    #[inline(always)]
    pub fn lprx_tocnt(&self) -> LprxTocntR {
        LprxTocntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - HSTX_TOCNT"]
    #[inline(always)]
    pub fn hstx_tocnt(&self) -> HstxTocntR {
        HstxTocntR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - LPRX_TOCNT"]
    #[inline(always)]
    pub fn lprx_tocnt(&mut self) -> LprxTocntW<DsiTccr0Spec> {
        LprxTocntW::new(self, 0)
    }
    #[doc = "Bits 16:31 - HSTX_TOCNT"]
    #[inline(always)]
    pub fn hstx_tocnt(&mut self) -> HstxTocntW<DsiTccr0Spec> {
        HstxTocntW::new(self, 16)
    }
}
#[doc = "DSI Host timeout counter configuration register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr0Spec;
impl crate::RegisterSpec for DsiTccr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr0::R`](R) reader structure"]
impl crate::Readable for DsiTccr0Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr0::W`](W) writer structure"]
impl crate::Writable for DsiTccr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR0 to value 0"]
impl crate::Resettable for DsiTccr0Spec {}
