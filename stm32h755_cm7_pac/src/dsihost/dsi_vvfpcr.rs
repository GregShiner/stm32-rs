#[doc = "Register `DSI_VVFPCR` reader"]
pub type R = crate::R<DsiVvfpcrSpec>;
#[doc = "Register `DSI_VVFPCR` writer"]
pub type W = crate::W<DsiVvfpcrSpec>;
#[doc = "Field `VFP` reader - VFP"]
pub type VfpR = crate::FieldReader<u16>;
#[doc = "Field `VFP` writer - VFP"]
pub type VfpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&self) -> VfpR {
        VfpR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VFP"]
    #[inline(always)]
    pub fn vfp(&mut self) -> VfpW<DsiVvfpcrSpec> {
        VfpW::new(self, 0)
    }
}
#[doc = "DSI Host video VFP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvfpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvfpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvfpcrSpec;
impl crate::RegisterSpec for DsiVvfpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvfpcr::R`](R) reader structure"]
impl crate::Readable for DsiVvfpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvfpcr::W`](W) writer structure"]
impl crate::Writable for DsiVvfpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VVFPCR to value 0"]
impl crate::Resettable for DsiVvfpcrSpec {}
