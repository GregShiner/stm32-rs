#[doc = "Register `DSI_WIFCR` writer"]
pub type W = crate::W<DsiWifcrSpec>;
#[doc = "Field `CTEIF` writer - CTEIF"]
pub type CteifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERIF` writer - CERIF"]
pub type CerifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLLIF` writer - CPLLLIF"]
pub type CplllifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPLLUIF` writer - CPLLUIF"]
pub type CplluifW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRRIF` writer - CRRIF"]
pub type CrrifW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - CTEIF"]
    #[inline(always)]
    pub fn cteif(&mut self) -> CteifW<DsiWifcrSpec> {
        CteifW::new(self, 0)
    }
    #[doc = "Bit 1 - CERIF"]
    #[inline(always)]
    pub fn cerif(&mut self) -> CerifW<DsiWifcrSpec> {
        CerifW::new(self, 1)
    }
    #[doc = "Bit 9 - CPLLLIF"]
    #[inline(always)]
    pub fn cplllif(&mut self) -> CplllifW<DsiWifcrSpec> {
        CplllifW::new(self, 9)
    }
    #[doc = "Bit 10 - CPLLUIF"]
    #[inline(always)]
    pub fn cplluif(&mut self) -> CplluifW<DsiWifcrSpec> {
        CplluifW::new(self, 10)
    }
    #[doc = "Bit 13 - CRRIF"]
    #[inline(always)]
    pub fn crrif(&mut self) -> CrrifW<DsiWifcrSpec> {
        CrrifW::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt flag clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wifcr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWifcrSpec;
impl crate::RegisterSpec for DsiWifcrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_wifcr::W`](W) writer structure"]
impl crate::Writable for DsiWifcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WIFCR to value 0"]
impl crate::Resettable for DsiWifcrSpec {}
