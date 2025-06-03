#[doc = "Register `DSI_TCCR3` reader"]
pub type R = crate::R<DsiTccr3Spec>;
#[doc = "Register `DSI_TCCR3` writer"]
pub type W = crate::W<DsiTccr3Spec>;
#[doc = "Field `HSWR_TOCNT` reader - HSWR_TOCNT"]
pub type HswrTocntR = crate::FieldReader<u16>;
#[doc = "Field `HSWR_TOCNT` writer - HSWR_TOCNT"]
pub type HswrTocntW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `PM` reader - PM"]
pub type PmR = crate::BitReader;
#[doc = "Field `PM` writer - PM"]
pub type PmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    pub fn hswr_tocnt(&self) -> HswrTocntR {
        HswrTocntR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:15 - HSWR_TOCNT"]
    #[inline(always)]
    pub fn hswr_tocnt(&mut self) -> HswrTocntW<DsiTccr3Spec> {
        HswrTocntW::new(self, 0)
    }
    #[doc = "Bit 24 - PM"]
    #[inline(always)]
    pub fn pm(&mut self) -> PmW<DsiTccr3Spec> {
        PmW::new(self, 24)
    }
}
#[doc = "DSI Host timeout counter configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_tccr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_tccr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiTccr3Spec;
impl crate::RegisterSpec for DsiTccr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_tccr3::R`](R) reader structure"]
impl crate::Readable for DsiTccr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_tccr3::W`](W) writer structure"]
impl crate::Writable for DsiTccr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_TCCR3 to value 0"]
impl crate::Resettable for DsiTccr3Spec {}
