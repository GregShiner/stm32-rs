#[doc = "Register `DSI_WPCR2` reader"]
pub type R = crate::R<DsiWpcr2Spec>;
#[doc = "Register `DSI_WPCR2` writer"]
pub type W = crate::W<DsiWpcr2Spec>;
#[doc = "Field `TCLKPREP` reader - TCLKPREP"]
pub type TclkprepR = crate::FieldReader;
#[doc = "Field `TCLKPREP` writer - TCLKPREP"]
pub type TclkprepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TCLKZERO` reader - TCLKZERO"]
pub type TclkzeroR = crate::FieldReader;
#[doc = "Field `TCLKZERO` writer - TCLKZERO"]
pub type TclkzeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSPREP` reader - THSPREP"]
pub type ThsprepR = crate::FieldReader;
#[doc = "Field `THSPREP` writer - THSPREP"]
pub type ThsprepW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSTRAIL` reader - THSTRAIL"]
pub type ThstrailR = crate::FieldReader;
#[doc = "Field `THSTRAIL` writer - THSTRAIL"]
pub type ThstrailW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TCLKPREP"]
    #[inline(always)]
    pub fn tclkprep(&self) -> TclkprepR {
        TclkprepR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TCLKZERO"]
    #[inline(always)]
    pub fn tclkzero(&self) -> TclkzeroR {
        TclkzeroR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - THSPREP"]
    #[inline(always)]
    pub fn thsprep(&self) -> ThsprepR {
        ThsprepR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - THSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&self) -> ThstrailR {
        ThstrailR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TCLKPREP"]
    #[inline(always)]
    pub fn tclkprep(&mut self) -> TclkprepW<DsiWpcr2Spec> {
        TclkprepW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TCLKZERO"]
    #[inline(always)]
    pub fn tclkzero(&mut self) -> TclkzeroW<DsiWpcr2Spec> {
        TclkzeroW::new(self, 8)
    }
    #[doc = "Bits 16:23 - THSPREP"]
    #[inline(always)]
    pub fn thsprep(&mut self) -> ThsprepW<DsiWpcr2Spec> {
        ThsprepW::new(self, 16)
    }
    #[doc = "Bits 24:31 - THSTRAIL"]
    #[inline(always)]
    pub fn thstrail(&mut self) -> ThstrailW<DsiWpcr2Spec> {
        ThstrailW::new(self, 24)
    }
}
#[doc = "DSI wrapper PHY configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWpcr2Spec;
impl crate::RegisterSpec for DsiWpcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr2::R`](R) reader structure"]
impl crate::Readable for DsiWpcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr2::W`](W) writer structure"]
impl crate::Writable for DsiWpcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WPCR2 to value 0"]
impl crate::Resettable for DsiWpcr2Spec {}
