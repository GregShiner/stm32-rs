#[doc = "Register `DSI_WPCR3` reader"]
pub type R = crate::R<DsiWpcr3Spec>;
#[doc = "Register `DSI_WPCR3` writer"]
pub type W = crate::W<DsiWpcr3Spec>;
#[doc = "Field `THSZERO` reader - THSZERO"]
pub type ThszeroR = crate::FieldReader;
#[doc = "Field `THSZERO` writer - THSZERO"]
pub type ThszeroW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLPXD` reader - TLPXD"]
pub type TlpxdR = crate::FieldReader;
#[doc = "Field `TLPXD` writer - TLPXD"]
pub type TlpxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `THSEXIT` reader - THSEXIT"]
pub type ThsexitR = crate::FieldReader;
#[doc = "Field `THSEXIT` writer - THSEXIT"]
pub type ThsexitW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TLPXC` reader - TLPXC"]
pub type TlpxcR = crate::FieldReader;
#[doc = "Field `TLPXC` writer - TLPXC"]
pub type TlpxcW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - THSZERO"]
    #[inline(always)]
    pub fn thszero(&self) -> ThszeroR {
        ThszeroR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - TLPXD"]
    #[inline(always)]
    pub fn tlpxd(&self) -> TlpxdR {
        TlpxdR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - THSEXIT"]
    #[inline(always)]
    pub fn thsexit(&self) -> ThsexitR {
        ThsexitR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - TLPXC"]
    #[inline(always)]
    pub fn tlpxc(&self) -> TlpxcR {
        TlpxcR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - THSZERO"]
    #[inline(always)]
    pub fn thszero(&mut self) -> ThszeroW<DsiWpcr3Spec> {
        ThszeroW::new(self, 0)
    }
    #[doc = "Bits 8:15 - TLPXD"]
    #[inline(always)]
    pub fn tlpxd(&mut self) -> TlpxdW<DsiWpcr3Spec> {
        TlpxdW::new(self, 8)
    }
    #[doc = "Bits 16:23 - THSEXIT"]
    #[inline(always)]
    pub fn thsexit(&mut self) -> ThsexitW<DsiWpcr3Spec> {
        ThsexitW::new(self, 16)
    }
    #[doc = "Bits 24:31 - TLPXC"]
    #[inline(always)]
    pub fn tlpxc(&mut self) -> TlpxcW<DsiWpcr3Spec> {
        TlpxcW::new(self, 24)
    }
}
#[doc = "DSI wrapper PHY configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWpcr3Spec;
impl crate::RegisterSpec for DsiWpcr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr3::R`](R) reader structure"]
impl crate::Readable for DsiWpcr3Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr3::W`](W) writer structure"]
impl crate::Writable for DsiWpcr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WPCR3 to value 0"]
impl crate::Resettable for DsiWpcr3Spec {}
