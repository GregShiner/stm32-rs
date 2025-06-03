#[doc = "Register `DSI_CCR` reader"]
pub type R = crate::R<DsiCcrSpec>;
#[doc = "Register `DSI_CCR` writer"]
pub type W = crate::W<DsiCcrSpec>;
#[doc = "Field `TXECKDIV` reader - TXECKDIV"]
pub type TxeckdivR = crate::FieldReader;
#[doc = "Field `TXECKDIV` writer - TXECKDIV"]
pub type TxeckdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOCKDIV` reader - TOCKDIV"]
pub type TockdivR = crate::FieldReader;
#[doc = "Field `TOCKDIV` writer - TOCKDIV"]
pub type TockdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&self) -> TxeckdivR {
        TxeckdivR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&self) -> TockdivR {
        TockdivR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXECKDIV"]
    #[inline(always)]
    pub fn txeckdiv(&mut self) -> TxeckdivW<DsiCcrSpec> {
        TxeckdivW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TOCKDIV"]
    #[inline(always)]
    pub fn tockdiv(&mut self) -> TockdivW<DsiCcrSpec> {
        TockdivW::new(self, 8)
    }
}
#[doc = "DSI Host clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiCcrSpec;
impl crate::RegisterSpec for DsiCcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ccr::R`](R) reader structure"]
impl crate::Readable for DsiCcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_ccr::W`](W) writer structure"]
impl crate::Writable for DsiCcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CCR to value 0"]
impl crate::Resettable for DsiCcrSpec {}
