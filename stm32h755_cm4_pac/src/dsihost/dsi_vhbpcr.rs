#[doc = "Register `DSI_VHBPCR` reader"]
pub type R = crate::R<DsiVhbpcrSpec>;
#[doc = "Register `DSI_VHBPCR` writer"]
pub type W = crate::W<DsiVhbpcrSpec>;
#[doc = "Field `HBP` reader - HBP"]
pub type HbpR = crate::FieldReader<u16>;
#[doc = "Field `HBP` writer - HBP"]
pub type HbpW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&self) -> HbpR {
        HbpR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HBP"]
    #[inline(always)]
    pub fn hbp(&mut self) -> HbpW<DsiVhbpcrSpec> {
        HbpW::new(self, 0)
    }
}
#[doc = "DSI Host video HBP configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhbpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vhbpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVhbpcrSpec;
impl crate::RegisterSpec for DsiVhbpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhbpcr::R`](R) reader structure"]
impl crate::Readable for DsiVhbpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vhbpcr::W`](W) writer structure"]
impl crate::Writable for DsiVhbpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VHBPCR to value 0"]
impl crate::Resettable for DsiVhbpcrSpec {}
