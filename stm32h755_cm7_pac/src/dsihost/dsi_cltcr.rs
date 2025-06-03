#[doc = "Register `DSI_CLTCR` reader"]
pub type R = crate::R<DsiCltcrSpec>;
#[doc = "Register `DSI_CLTCR` writer"]
pub type W = crate::W<DsiCltcrSpec>;
#[doc = "Field `LP2HS_TIME` reader - LP2HS_TIME"]
pub type Lp2hsTimeR = crate::FieldReader<u16>;
#[doc = "Field `LP2HS_TIME` writer - LP2HS_TIME"]
pub type Lp2hsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `HS2LP_TIME` reader - HS2LP_TIME"]
pub type Hs2lpTimeR = crate::FieldReader<u16>;
#[doc = "Field `HS2LP_TIME` writer - HS2LP_TIME"]
pub type Hs2lpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> Lp2hsTimeR {
        Lp2hsTimeR::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> Hs2lpTimeR {
        Hs2lpTimeR::new(((self.bits >> 16) & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> Lp2hsTimeW<DsiCltcrSpec> {
        Lp2hsTimeW::new(self, 0)
    }
    #[doc = "Bits 16:25 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> Hs2lpTimeW<DsiCltcrSpec> {
        Hs2lpTimeW::new(self, 16)
    }
}
#[doc = "DSI Host clock lane timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiCltcrSpec;
impl crate::RegisterSpec for DsiCltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cltcr::R`](R) reader structure"]
impl crate::Readable for DsiCltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_cltcr::W`](W) writer structure"]
impl crate::Writable for DsiCltcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CLTCR to value 0"]
impl crate::Resettable for DsiCltcrSpec {}
