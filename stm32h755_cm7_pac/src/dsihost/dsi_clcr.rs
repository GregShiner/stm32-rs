#[doc = "Register `DSI_CLCR` reader"]
pub type R = crate::R<DsiClcrSpec>;
#[doc = "Register `DSI_CLCR` writer"]
pub type W = crate::W<DsiClcrSpec>;
#[doc = "Field `DPCC` reader - DPCC"]
pub type DpccR = crate::BitReader;
#[doc = "Field `DPCC` writer - DPCC"]
pub type DpccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACR` reader - ACR"]
pub type AcrR = crate::BitReader;
#[doc = "Field `ACR` writer - ACR"]
pub type AcrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DPCC"]
    #[inline(always)]
    pub fn dpcc(&self) -> DpccR {
        DpccR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ACR"]
    #[inline(always)]
    pub fn acr(&self) -> AcrR {
        AcrR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DPCC"]
    #[inline(always)]
    pub fn dpcc(&mut self) -> DpccW<DsiClcrSpec> {
        DpccW::new(self, 0)
    }
    #[doc = "Bit 1 - ACR"]
    #[inline(always)]
    pub fn acr(&mut self) -> AcrW<DsiClcrSpec> {
        AcrW::new(self, 1)
    }
}
#[doc = "DSI Host clock lane configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_clcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_clcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiClcrSpec;
impl crate::RegisterSpec for DsiClcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_clcr::R`](R) reader structure"]
impl crate::Readable for DsiClcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_clcr::W`](W) writer structure"]
impl crate::Writable for DsiClcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CLCR to value 0"]
impl crate::Resettable for DsiClcrSpec {}
