#[doc = "Register `DSI_LPMCR` reader"]
pub type R = crate::R<DsiLpmcrSpec>;
#[doc = "Register `DSI_LPMCR` writer"]
pub type W = crate::W<DsiLpmcrSpec>;
#[doc = "Field `VLPSIZE` reader - VLPSIZE"]
pub type VlpsizeR = crate::FieldReader;
#[doc = "Field `VLPSIZE` writer - VLPSIZE"]
pub type VlpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPSIZE` reader - LPSIZE"]
pub type LpsizeR = crate::FieldReader;
#[doc = "Field `LPSIZE` writer - LPSIZE"]
pub type LpsizeW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&self) -> VlpsizeR {
        VlpsizeR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&self) -> LpsizeR {
        LpsizeR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - VLPSIZE"]
    #[inline(always)]
    pub fn vlpsize(&mut self) -> VlpsizeW<DsiLpmcrSpec> {
        VlpsizeW::new(self, 0)
    }
    #[doc = "Bits 16:23 - LPSIZE"]
    #[inline(always)]
    pub fn lpsize(&mut self) -> LpsizeW<DsiLpmcrSpec> {
        LpsizeW::new(self, 16)
    }
}
#[doc = "DSI Host low-power mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lpmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lpmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLpmcrSpec;
impl crate::RegisterSpec for DsiLpmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lpmcr::R`](R) reader structure"]
impl crate::Readable for DsiLpmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lpmcr::W`](W) writer structure"]
impl crate::Writable for DsiLpmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LPMCR to value 0"]
impl crate::Resettable for DsiLpmcrSpec {}
