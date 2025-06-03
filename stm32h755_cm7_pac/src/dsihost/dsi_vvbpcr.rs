#[doc = "Register `DSI_VVBPCR` reader"]
pub type R = crate::R<DsiVvbpcrSpec>;
#[doc = "Register `DSI_VVBPCR` writer"]
pub type W = crate::W<DsiVvbpcrSpec>;
#[doc = "Field `VBP` reader - VBP"]
pub type VbpR = crate::FieldReader<u16>;
#[doc = "Field `VBP` writer - VBP"]
pub type VbpW<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&self) -> VbpR {
        VbpR::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - VBP"]
    #[inline(always)]
    pub fn vbp(&mut self) -> VbpW<DsiVvbpcrSpec> {
        VbpW::new(self, 0)
    }
}
#[doc = "DSI Host video VBP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vvbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vvbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVvbpcrSpec;
impl crate::RegisterSpec for DsiVvbpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vvbpcr::R`](R) reader structure"]
impl crate::Readable for DsiVvbpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vvbpcr::W`](W) writer structure"]
impl crate::Writable for DsiVvbpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VVBPCR to value 0"]
impl crate::Resettable for DsiVvbpcrSpec {}
