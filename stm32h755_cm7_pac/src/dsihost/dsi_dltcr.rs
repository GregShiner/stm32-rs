#[doc = "Register `DSI_DLTCR` reader"]
pub type R = crate::R<DsiDltcrSpec>;
#[doc = "Register `DSI_DLTCR` writer"]
pub type W = crate::W<DsiDltcrSpec>;
#[doc = "Field `MRD_TIME` reader - Maximum read time"]
pub type MrdTimeR = crate::FieldReader<u16>;
#[doc = "Field `MRD_TIME` writer - Maximum read time"]
pub type MrdTimeW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LP2HS_TIME` reader - LP2HS_TIME"]
pub type Lp2hsTimeR = crate::FieldReader;
#[doc = "Field `LP2HS_TIME` writer - LP2HS_TIME"]
pub type Lp2hsTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HS2LP_TIME` reader - HS2LP_TIME"]
pub type Hs2lpTimeR = crate::FieldReader;
#[doc = "Field `HS2LP_TIME` writer - HS2LP_TIME"]
pub type Hs2lpTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:14 - Maximum read time"]
    #[inline(always)]
    pub fn mrd_time(&self) -> MrdTimeR {
        MrdTimeR::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 16:23 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&self) -> Lp2hsTimeR {
        Lp2hsTimeR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&self) -> Hs2lpTimeR {
        Hs2lpTimeR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Maximum read time"]
    #[inline(always)]
    pub fn mrd_time(&mut self) -> MrdTimeW<DsiDltcrSpec> {
        MrdTimeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - LP2HS_TIME"]
    #[inline(always)]
    pub fn lp2hs_time(&mut self) -> Lp2hsTimeW<DsiDltcrSpec> {
        Lp2hsTimeW::new(self, 16)
    }
    #[doc = "Bits 24:31 - HS2LP_TIME"]
    #[inline(always)]
    pub fn hs2lp_time(&mut self) -> Hs2lpTimeW<DsiDltcrSpec> {
        Hs2lpTimeW::new(self, 24)
    }
}
#[doc = "DSI Host data lane timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_dltcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_dltcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiDltcrSpec;
impl crate::RegisterSpec for DsiDltcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_dltcr::R`](R) reader structure"]
impl crate::Readable for DsiDltcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_dltcr::W`](W) writer structure"]
impl crate::Writable for DsiDltcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_DLTCR to value 0"]
impl crate::Resettable for DsiDltcrSpec {}
