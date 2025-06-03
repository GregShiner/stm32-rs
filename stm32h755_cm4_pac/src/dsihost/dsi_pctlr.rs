#[doc = "Register `DSI_PCTLR` reader"]
pub type R = crate::R<DsiPctlrSpec>;
#[doc = "Register `DSI_PCTLR` writer"]
pub type W = crate::W<DsiPctlrSpec>;
#[doc = "Field `DEN` reader - DEN"]
pub type DenR = crate::BitReader;
#[doc = "Field `DEN` writer - DEN"]
pub type DenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKE` reader - CKE"]
pub type CkeR = crate::BitReader;
#[doc = "Field `CKE` writer - CKE"]
pub type CkeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        DenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    pub fn cke(&self) -> CkeR {
        CkeR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - DEN"]
    #[inline(always)]
    pub fn den(&mut self) -> DenW<DsiPctlrSpec> {
        DenW::new(self, 1)
    }
    #[doc = "Bit 2 - CKE"]
    #[inline(always)]
    pub fn cke(&mut self) -> CkeW<DsiPctlrSpec> {
        CkeW::new(self, 2)
    }
}
#[doc = "DSI Host PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pctlr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pctlr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPctlrSpec;
impl crate::RegisterSpec for DsiPctlrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pctlr::R`](R) reader structure"]
impl crate::Readable for DsiPctlrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_pctlr::W`](W) writer structure"]
impl crate::Writable for DsiPctlrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PCTLR to value 0"]
impl crate::Resettable for DsiPctlrSpec {}
