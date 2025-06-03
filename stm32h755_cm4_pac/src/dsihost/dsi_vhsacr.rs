#[doc = "Register `DSI_VHSACR` reader"]
pub type R = crate::R<DsiVhsacrSpec>;
#[doc = "Register `DSI_VHSACR` writer"]
pub type W = crate::W<DsiVhsacrSpec>;
#[doc = "Field `HSA` reader - HSA"]
pub type HsaR = crate::FieldReader<u16>;
#[doc = "Field `HSA` writer - HSA"]
pub type HsaW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&self) -> HsaR {
        HsaR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - HSA"]
    #[inline(always)]
    pub fn hsa(&mut self) -> HsaW<DsiVhsacrSpec> {
        HsaW::new(self, 0)
    }
}
#[doc = "DSI Host video HSA configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vhsacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vhsacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVhsacrSpec;
impl crate::RegisterSpec for DsiVhsacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vhsacr::R`](R) reader structure"]
impl crate::Readable for DsiVhsacrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vhsacr::W`](W) writer structure"]
impl crate::Writable for DsiVhsacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VHSACR to value 0"]
impl crate::Resettable for DsiVhsacrSpec {}
