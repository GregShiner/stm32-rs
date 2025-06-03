#[doc = "Register `DSI_VVSACR` reader"]
pub type R = crate::R<DsiVvsacrSpec>;
#[doc = "Register `DSI_VVSACR` writer"]
pub type W = crate::W<DsiVvsacrSpec>;
#[doc = "Field `VSA` reader - VSA"]
pub type VsaR = crate::FieldReader<u16>;
#[doc = "Field `VSA` writer - VSA"]
pub type VsaW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&self) -> VsaR {
        VsaR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VSA"]
    #[inline(always)]
    pub fn vsa(&mut self) -> VsaW<DsiVvsacrSpec> {
        VsaW::new(self, 0)
    }
}
#[doc = "DSI Host video VSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvsacrSpec;
impl crate::RegisterSpec for DsiVvsacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvsacr::R`](R) reader structure"]
impl crate::Readable for DsiVvsacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvsacr::W`](W) writer structure"]
impl crate::Writable for DsiVvsacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VVSACR to value 0"]
impl crate::Resettable for DsiVvsacrSpec {}
