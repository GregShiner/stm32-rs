#[doc = "Register `DSI_PCONFR` reader"]
pub type R = crate::R<DsiPconfrSpec>;
#[doc = "Register `DSI_PCONFR` writer"]
pub type W = crate::W<DsiPconfrSpec>;
#[doc = "Field `NL` reader - NL"]
pub type NlR = crate::FieldReader;
#[doc = "Field `NL` writer - NL"]
pub type NlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SW_TIME` reader - SW_TIME"]
pub type SwTimeR = crate::FieldReader;
#[doc = "Field `SW_TIME` writer - SW_TIME"]
pub type SwTimeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&self) -> NlR {
        NlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&self) -> SwTimeR {
        SwTimeR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - NL"]
    #[inline(always)]
    pub fn nl(&mut self) -> NlW<DsiPconfrSpec> {
        NlW::new(self, 0)
    }
    #[doc = "Bits 8:15 - SW_TIME"]
    #[inline(always)]
    pub fn sw_time(&mut self) -> SwTimeW<DsiPconfrSpec> {
        SwTimeW::new(self, 8)
    }
}
#[doc = "DSI Host PHY configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pconfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pconfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPconfrSpec;
impl crate::RegisterSpec for DsiPconfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pconfr::R`](R) reader structure"]
impl crate::Readable for DsiPconfrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_pconfr::W`](W) writer structure"]
impl crate::Writable for DsiPconfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PCONFR to value 0x01"]
impl crate::Resettable for DsiPconfrSpec {
    const RESET_VALUE: u32 = 0x01;
}
