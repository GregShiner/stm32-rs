#[doc = "Register `DSI_LCCR` reader"]
pub type R = crate::R<DsiLccrSpec>;
#[doc = "Register `DSI_LCCR` writer"]
pub type W = crate::W<DsiLccrSpec>;
#[doc = "Field `CMDSIZE` reader - CMDSIZE"]
pub type CmdsizeR = crate::FieldReader<u16>;
#[doc = "Field `CMDSIZE` writer - CMDSIZE"]
pub type CmdsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - CMDSIZE"]
    #[inline(always)]
    pub fn cmdsize(&self) -> CmdsizeR {
        CmdsizeR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CMDSIZE"]
    #[inline(always)]
    pub fn cmdsize(&mut self) -> CmdsizeW<DsiLccrSpec> {
        CmdsizeW::new(self, 0)
    }
}
#[doc = "DSI Host LTDC command configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLccrSpec;
impl crate::RegisterSpec for DsiLccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lccr::R`](R) reader structure"]
impl crate::Readable for DsiLccrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lccr::W`](W) writer structure"]
impl crate::Writable for DsiLccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LCCR to value 0"]
impl crate::Resettable for DsiLccrSpec {}
