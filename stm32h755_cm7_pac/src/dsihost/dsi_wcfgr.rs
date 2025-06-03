#[doc = "Register `DSI_WCFGR` reader"]
pub type R = crate::R<DsiWcfgrSpec>;
#[doc = "Register `DSI_WCFGR` writer"]
pub type W = crate::W<DsiWcfgrSpec>;
#[doc = "Field `DSIM` reader - DSIM"]
pub type DsimR = crate::BitReader;
#[doc = "Field `DSIM` writer - DSIM"]
pub type DsimW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COLMUX` reader - COLMUX"]
pub type ColmuxR = crate::FieldReader;
#[doc = "Field `COLMUX` writer - COLMUX"]
pub type ColmuxW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TESRC` reader - TESRC"]
pub type TesrcR = crate::BitReader;
#[doc = "Field `TESRC` writer - TESRC"]
pub type TesrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEPOL` reader - TEPOL"]
pub type TepolR = crate::BitReader;
#[doc = "Field `TEPOL` writer - TEPOL"]
pub type TepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AR` reader - AR"]
pub type ArR = crate::BitReader;
#[doc = "Field `AR` writer - AR"]
pub type ArW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - VSPOL"]
pub type VspolR = crate::BitReader;
#[doc = "Field `VSPOL` writer - VSPOL"]
pub type VspolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DSIM"]
    #[inline(always)]
    pub fn dsim(&self) -> DsimR {
        DsimR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - COLMUX"]
    #[inline(always)]
    pub fn colmux(&self) -> ColmuxR {
        ColmuxR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - TESRC"]
    #[inline(always)]
    pub fn tesrc(&self) -> TesrcR {
        TesrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TEPOL"]
    #[inline(always)]
    pub fn tepol(&self) -> TepolR {
        TepolR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AR"]
    #[inline(always)]
    pub fn ar(&self) -> ArR {
        ArR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&self) -> VspolR {
        VspolR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DSIM"]
    #[inline(always)]
    pub fn dsim(&mut self) -> DsimW<DsiWcfgrSpec> {
        DsimW::new(self, 0)
    }
    #[doc = "Bits 1:3 - COLMUX"]
    #[inline(always)]
    pub fn colmux(&mut self) -> ColmuxW<DsiWcfgrSpec> {
        ColmuxW::new(self, 1)
    }
    #[doc = "Bit 4 - TESRC"]
    #[inline(always)]
    pub fn tesrc(&mut self) -> TesrcW<DsiWcfgrSpec> {
        TesrcW::new(self, 4)
    }
    #[doc = "Bit 5 - TEPOL"]
    #[inline(always)]
    pub fn tepol(&mut self) -> TepolW<DsiWcfgrSpec> {
        TepolW::new(self, 5)
    }
    #[doc = "Bit 6 - AR"]
    #[inline(always)]
    pub fn ar(&mut self) -> ArW<DsiWcfgrSpec> {
        ArW::new(self, 6)
    }
    #[doc = "Bit 7 - VSPOL"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VspolW<DsiWcfgrSpec> {
        VspolW::new(self, 7)
    }
}
#[doc = "DSI wrapper configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWcfgrSpec;
impl crate::RegisterSpec for DsiWcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wcfgr::R`](R) reader structure"]
impl crate::Readable for DsiWcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wcfgr::W`](W) writer structure"]
impl crate::Writable for DsiWcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WCFGR to value 0"]
impl crate::Resettable for DsiWcfgrSpec {}
