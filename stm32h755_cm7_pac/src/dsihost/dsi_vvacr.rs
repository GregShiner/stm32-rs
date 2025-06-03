#[doc = "Register `DSI_VVACR` reader"]
pub type R = crate::R<DsiVvacrSpec>;
#[doc = "Register `DSI_VVACR` writer"]
pub type W = crate::W<DsiVvacrSpec>;
#[doc = "Field `VA` reader - VA"]
pub type VaR = crate::FieldReader<u16>;
#[doc = "Field `VA` writer - VA"]
pub type VaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&self) -> VaR {
        VaR::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - VA"]
    #[inline(always)]
    pub fn va(&mut self) -> VaW<DsiVvacrSpec> {
        VaW::new(self, 0)
    }
}
#[doc = "DSI Host video VA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvacrSpec;
impl crate::RegisterSpec for DsiVvacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvacr::R`](R) reader structure"]
impl crate::Readable for DsiVvacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvacr::W`](W) writer structure"]
impl crate::Writable for DsiVvacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VVACR to value 0"]
impl crate::Resettable for DsiVvacrSpec {}
