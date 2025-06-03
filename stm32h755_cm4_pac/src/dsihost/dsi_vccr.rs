#[doc = "Register `DSI_VCCR` reader"]
pub type R = crate::R<DsiVccrSpec>;
#[doc = "Register `DSI_VCCR` writer"]
pub type W = crate::W<DsiVccrSpec>;
#[doc = "Field `NUMC` reader - NUMC"]
pub type NumcR = crate::FieldReader<u16>;
#[doc = "Field `NUMC` writer - NUMC"]
pub type NumcW<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&self) -> NumcR {
        NumcR::new((self.bits & 0x1fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:12 - NUMC"]
    #[inline(always)]
    pub fn numc(&mut self) -> NumcW<DsiVccrSpec> {
        NumcW::new(self, 0)
    }
}
#[doc = "DSI Host video chunks configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVccrSpec;
impl crate::RegisterSpec for DsiVccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vccr::R`](R) reader structure"]
impl crate::Readable for DsiVccrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vccr::W`](W) writer structure"]
impl crate::Writable for DsiVccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VCCR to value 0"]
impl crate::Resettable for DsiVccrSpec {}
