#[doc = "Register `DSI_GHCR` reader"]
pub type R = crate::R<DsiGhcrSpec>;
#[doc = "Register `DSI_GHCR` writer"]
pub type W = crate::W<DsiGhcrSpec>;
#[doc = "Field `DT` reader - DT"]
pub type DtR = crate::FieldReader;
#[doc = "Field `DT` writer - DT"]
pub type DtW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `VCID` reader - VCID"]
pub type VcidR = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VcidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `WCLSB` reader - WCLSB"]
pub type WclsbR = crate::FieldReader;
#[doc = "Field `WCLSB` writer - WCLSB"]
pub type WclsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WCMSB` reader - WCMSB"]
pub type WcmsbR = crate::FieldReader;
#[doc = "Field `WCMSB` writer - WCMSB"]
pub type WcmsbW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VcidR {
        VcidR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    pub fn wclsb(&self) -> WclsbR {
        WclsbR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    pub fn wcmsb(&self) -> WcmsbR {
        WcmsbR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - DT"]
    #[inline(always)]
    pub fn dt(&mut self) -> DtW<DsiGhcrSpec> {
        DtW::new(self, 0)
    }
    #[doc = "Bits 6:7 - VCID"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VcidW<DsiGhcrSpec> {
        VcidW::new(self, 6)
    }
    #[doc = "Bits 8:15 - WCLSB"]
    #[inline(always)]
    pub fn wclsb(&mut self) -> WclsbW<DsiGhcrSpec> {
        WclsbW::new(self, 8)
    }
    #[doc = "Bits 16:23 - WCMSB"]
    #[inline(always)]
    pub fn wcmsb(&mut self) -> WcmsbW<DsiGhcrSpec> {
        WcmsbW::new(self, 16)
    }
}
#[doc = "DSI Host generic header configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ghcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ghcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiGhcrSpec;
impl crate::RegisterSpec for DsiGhcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ghcr::R`](R) reader structure"]
impl crate::Readable for DsiGhcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_ghcr::W`](W) writer structure"]
impl crate::Writable for DsiGhcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_GHCR to value 0"]
impl crate::Resettable for DsiGhcrSpec {}
