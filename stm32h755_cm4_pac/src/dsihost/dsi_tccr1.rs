#[doc = "Register `DSI_TCCR1` reader"]
pub type R = crate::R<DsiTccr1Spec>;
#[doc = "Register `DSI_TCCR1` writer"]
pub type W = crate::W<DsiTccr1Spec>;
#[doc = "Field `HSRD_TOCNT` reader - HSRD_TOCNT"]
pub type HsrdTocntR = crate::FieldReader<u16>;
#[doc = "Field `HSRD_TOCNT` writer - HSRD_TOCNT"]
pub type HsrdTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - HSRD_TOCNT"]
    #[inline(always)]
    pub fn hsrd_tocnt(&self) -> HsrdTocntR {
        HsrdTocntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSRD_TOCNT"]
    #[inline(always)]
    pub fn hsrd_tocnt(&mut self) -> HsrdTocntW<DsiTccr1Spec> {
        HsrdTocntW::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr1Spec;
impl crate::RegisterSpec for DsiTccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr1::R`](R) reader structure"]
impl crate::Readable for DsiTccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr1::W`](W) writer structure"]
impl crate::Writable for DsiTccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR1 to value 0"]
impl crate::Resettable for DsiTccr1Spec {}
