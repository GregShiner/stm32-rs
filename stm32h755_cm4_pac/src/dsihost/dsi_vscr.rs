#[doc = "Register `DSI_VSCR` reader"]
pub type R = crate::R<DsiVscrSpec>;
#[doc = "Register `DSI_VSCR` writer"]
pub type W = crate::W<DsiVscrSpec>;
#[doc = "Field `EN` reader - EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UR` reader - UR"]
pub type UrR = crate::BitReader;
#[doc = "Field `UR` writer - UR"]
pub type UrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - UR"]
    #[inline(always)]
    pub fn ur(&self) -> UrR {
        UrR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DsiVscrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 8 - UR"]
    #[inline(always)]
    pub fn ur(&mut self) -> UrW<DsiVscrSpec> {
        UrW::new(self, 8)
    }
}
#[doc = "DSI Host video shadow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVscrSpec;
impl crate::RegisterSpec for DsiVscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vscr::R`](R) reader structure"]
impl crate::Readable for DsiVscrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vscr::W`](W) writer structure"]
impl crate::Writable for DsiVscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VSCR to value 0"]
impl crate::Resettable for DsiVscrSpec {}
