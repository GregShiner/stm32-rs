#[doc = "Register `DSI_LVCIDR` reader"]
pub type R = crate::R<DsiLvcidrSpec>;
#[doc = "Register `DSI_LVCIDR` writer"]
pub type W = crate::W<DsiLvcidrSpec>;
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
    pub fn vcid(&mut self) -> VcidW<DsiLvcidrSpec> {
        VcidW::new(self, 0)
    }
}
#[doc = "DSI Host LTDC VCID register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lvcidr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lvcidr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLvcidrSpec;
impl crate::RegisterSpec for DsiLvcidrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lvcidr::R`](R) reader structure"]
impl crate::Readable for DsiLvcidrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lvcidr::W`](W) writer structure"]
impl crate::Writable for DsiLvcidrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LVCIDR to value 0"]
impl crate::Resettable for DsiLvcidrSpec {}
