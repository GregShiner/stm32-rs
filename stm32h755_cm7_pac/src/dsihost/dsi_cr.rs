#[doc = "Register `DSI_CR` reader"]
pub type R = crate::R<DsiCrSpec>;
#[doc = "Register `DSI_CR` writer"]
pub type W = crate::W<DsiCrSpec>;
#[doc = "Field `EN` reader - EN"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - EN"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - EN"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<DsiCrSpec> {
        EnW::new(self, 0)
    }
}
#[doc = "DSI Host control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiCrSpec;
impl crate::RegisterSpec for DsiCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cr::R`](R) reader structure"]
impl crate::Readable for DsiCrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_cr::W`](W) writer structure"]
impl crate::Writable for DsiCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CR to value 0"]
impl crate::Resettable for DsiCrSpec {}
