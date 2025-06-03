#[doc = "Register `DSI_TCCR5` reader"]
pub type R = crate::R<DsiTccr5Spec>;
#[doc = "Register `DSI_TCCR5` writer"]
pub type W = crate::W<DsiTccr5Spec>;
#[doc = "Field `BTA_TOCNT` reader - BTA_TOCNT"]
pub type BtaTocntR = crate::FieldReader<u16>;
#[doc = "Field `BTA_TOCNT` writer - BTA_TOCNT"]
pub type BtaTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - BTA_TOCNT"]
    #[inline(always)]
    pub fn bta_tocnt(&self) -> BtaTocntR {
        BtaTocntR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - BTA_TOCNT"]
    #[inline(always)]
    pub fn bta_tocnt(&mut self) -> BtaTocntW<DsiTccr5Spec> {
        BtaTocntW::new(self, 0)
    }
}
#[doc = "DSI Host timeout counter configuration register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr5Spec;
impl crate::RegisterSpec for DsiTccr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr5::R`](R) reader structure"]
impl crate::Readable for DsiTccr5Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr5::W`](W) writer structure"]
impl crate::Writable for DsiTccr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR5 to value 0"]
impl crate::Resettable for DsiTccr5Spec {}
