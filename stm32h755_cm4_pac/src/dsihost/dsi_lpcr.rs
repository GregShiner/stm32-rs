#[doc = "Register `DSI_LPCR` reader"]
pub type R = crate::R<DsiLpcrSpec>;
#[doc = "Register `DSI_LPCR` writer"]
pub type W = crate::W<DsiLpcrSpec>;
#[doc = "Field `DEP` reader - DEP"]
pub type DepR = crate::BitReader;
#[doc = "Field `DEP` writer - DEP"]
pub type DepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSP` reader - VSP"]
pub type VspR = crate::BitReader;
#[doc = "Field `VSP` writer - VSP"]
pub type VspW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSP` reader - HSP"]
pub type HspR = crate::BitReader;
#[doc = "Field `HSP` writer - HSP"]
pub type HspW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DEP"]
    #[inline(always)]
    pub fn dep(&self) -> DepR {
        DepR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - VSP"]
    #[inline(always)]
    pub fn vsp(&self) -> VspR {
        VspR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSP"]
    #[inline(always)]
    pub fn hsp(&self) -> HspR {
        HspR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DEP"]
    #[inline(always)]
    pub fn dep(&mut self) -> DepW<DsiLpcrSpec> {
        DepW::new(self, 0)
    }
    #[doc = "Bit 1 - VSP"]
    #[inline(always)]
    pub fn vsp(&mut self) -> VspW<DsiLpcrSpec> {
        VspW::new(self, 1)
    }
    #[doc = "Bit 2 - HSP"]
    #[inline(always)]
    pub fn hsp(&mut self) -> HspW<DsiLpcrSpec> {
        HspW::new(self, 2)
    }
}
#[doc = "DSI Host LTDC polarity configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLpcrSpec;
impl crate::RegisterSpec for DsiLpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpcr::R`](R) reader structure"]
impl crate::Readable for DsiLpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpcr::W`](W) writer structure"]
impl crate::Writable for DsiLpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LPCR to value 0"]
impl crate::Resettable for DsiLpcrSpec {}
