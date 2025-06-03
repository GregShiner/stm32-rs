#[doc = "Register `DSI_VLCR` reader"]
pub type R = crate::R<DsiVlcrSpec>;
#[doc = "Register `DSI_VLCR` writer"]
pub type W = crate::W<DsiVlcrSpec>;
#[doc = "Field `HLINE` reader - HLINE"]
pub type HlineR = crate::FieldReader<u16>;
#[doc = "Field `HLINE` writer - HLINE"]
pub type HlineW<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
impl R {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&self) -> HlineR {
        HlineR::new((self.bits & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:14 - HLINE"]
    #[inline(always)]
    pub fn hline(&mut self) -> HlineW<DsiVlcrSpec> {
        HlineW::new(self, 0)
    }
}
#[doc = "DSI Host video line configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_vlcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_vlcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiVlcrSpec;
impl crate::RegisterSpec for DsiVlcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_vlcr::R`](R) reader structure"]
impl crate::Readable for DsiVlcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_vlcr::W`](W) writer structure"]
impl crate::Writable for DsiVlcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_VLCR to value 0"]
impl crate::Resettable for DsiVlcrSpec {}
