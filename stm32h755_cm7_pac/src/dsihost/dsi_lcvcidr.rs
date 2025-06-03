#[doc = "Register `DSI_LCVCIDR` reader"]
pub type R = crate::R<DsiLcvcidrSpec>;
#[doc = "Register `DSI_LCVCIDR` writer"]
pub type W = crate::W<DsiLcvcidrSpec>;
#[doc = "Field `VCID` reader - VCID"]
pub type VcidR = crate::FieldReader;
#[doc = "Field `VCID` writer - VCID"]
pub type VcidW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&self) -> VcidR {
        VcidR::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - VCID"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VcidW<DsiLcvcidrSpec> {
        VcidW::new(self, 0)
    }
}
#[doc = "DSI Host LTDC current VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lcvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLcvcidrSpec;
impl crate::RegisterSpec for DsiLcvcidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcvcidr::R`](R) reader structure"]
impl crate::Readable for DsiLcvcidrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lcvcidr::W`](W) writer structure"]
impl crate::Writable for DsiLcvcidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LCVCIDR to value 0"]
impl crate::Resettable for DsiLcvcidrSpec {}
