#[doc = "Register `DSI_WCR` reader"]
pub type R = crate::R<DsiWcrSpec>;
#[doc = "Register `DSI_WCR` writer"]
pub type W = crate::W<DsiWcrSpec>;
#[doc = "Field `COLM` reader - COLM"]
pub type ColmR = crate::BitReader;
#[doc = "Field `COLM` writer - COLM"]
pub type ColmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SHTDN` reader - SHTDN"]
pub type ShtdnR = crate::BitReader;
#[doc = "Field `SHTDN` writer - SHTDN"]
pub type ShtdnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LTDCEN` reader - LTDCEN"]
pub type LtdcenR = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LTDCEN"]
pub type LtdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSIEN` reader - DSIEN"]
pub type DsienR = crate::BitReader;
#[doc = "Field `DSIEN` writer - DSIEN"]
pub type DsienW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COLM"]
    #[inline(always)]
    pub fn colm(&self) -> ColmR {
        ColmR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SHTDN"]
    #[inline(always)]
    pub fn shtdn(&self) -> ShtdnR {
        ShtdnR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LtdcenR {
        LtdcenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&self) -> DsienR {
        DsienR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COLM"]
    #[inline(always)]
    pub fn colm(&mut self) -> ColmW<DsiWcrSpec> {
        ColmW::new(self, 0)
    }
    #[doc = "Bit 1 - SHTDN"]
    #[inline(always)]
    pub fn shtdn(&mut self) -> ShtdnW<DsiWcrSpec> {
        ShtdnW::new(self, 1)
    }
    #[doc = "Bit 2 - LTDCEN"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LtdcenW<DsiWcrSpec> {
        LtdcenW::new(self, 2)
    }
    #[doc = "Bit 3 - DSIEN"]
    #[inline(always)]
    pub fn dsien(&mut self) -> DsienW<DsiWcrSpec> {
        DsienW::new(self, 3)
    }
}
#[doc = "DSI wrapper control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWcrSpec;
impl crate::RegisterSpec for DsiWcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wcr::R`](R) reader structure"]
impl crate::Readable for DsiWcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wcr::W`](W) writer structure"]
impl crate::Writable for DsiWcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WCR to value 0"]
impl crate::Resettable for DsiWcrSpec {}
