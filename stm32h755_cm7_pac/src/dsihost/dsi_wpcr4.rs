#[doc = "Register `DSI_WPCR4` reader"]
pub type R = crate::R<DsiWpcr4Spec>;
#[doc = "Register `DSI_WPCR4` writer"]
pub type W = crate::W<DsiWpcr4Spec>;
#[doc = "Field `TCLKPOST` reader - TCLKPOST"]
pub type TclkpostR = crate::FieldReader;
#[doc = "Field `TCLKPOST` writer - TCLKPOST"]
pub type TclkpostW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TCLKPOST"]
    #[inline(always)]
    pub fn tclkpost(&self) -> TclkpostR {
        TclkpostR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCLKPOST"]
    #[inline(always)]
    pub fn tclkpost(&mut self) -> TclkpostW<DsiWpcr4Spec> {
        TclkpostW::new(self, 0)
    }
}
#[doc = "DSI wrapper PHY configuration register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWpcr4Spec;
impl crate::RegisterSpec for DsiWpcr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr4::R`](R) reader structure"]
impl crate::Readable for DsiWpcr4Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr4::W`](W) writer structure"]
impl crate::Writable for DsiWpcr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WPCR4 to value 0"]
impl crate::Resettable for DsiWpcr4Spec {}
