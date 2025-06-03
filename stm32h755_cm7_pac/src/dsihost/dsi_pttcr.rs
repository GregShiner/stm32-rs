#[doc = "Register `DSI_PTTCR` reader"]
pub type R = crate::R<DsiPttcrSpec>;
#[doc = "Register `DSI_PTTCR` writer"]
pub type W = crate::W<DsiPttcrSpec>;
#[doc = "Field `TX_TRIG` reader - TX_TRIG"]
pub type TxTrigR = crate::FieldReader;
#[doc = "Field `TX_TRIG` writer - TX_TRIG"]
pub type TxTrigW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - TX_TRIG"]
    #[inline(always)]
    pub fn tx_trig(&self) -> TxTrigR {
        TxTrigR::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - TX_TRIG"]
    #[inline(always)]
    pub fn tx_trig(&mut self) -> TxTrigW<DsiPttcrSpec> {
        TxTrigW::new(self, 0)
    }
}
#[doc = "DSI Host PHY TX triggers configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pttcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pttcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPttcrSpec;
impl crate::RegisterSpec for DsiPttcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pttcr::R`](R) reader structure"]
impl crate::Readable for DsiPttcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_pttcr::W`](W) writer structure"]
impl crate::Writable for DsiPttcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PTTCR to value 0"]
impl crate::Resettable for DsiPttcrSpec {}
