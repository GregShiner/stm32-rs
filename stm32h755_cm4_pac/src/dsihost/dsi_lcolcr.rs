#[doc = "Register `DSI_LCOLCR` reader"]
pub type R = crate::R<DsiLcolcrSpec>;
#[doc = "Register `DSI_LCOLCR` writer"]
pub type W = crate::W<DsiLcolcrSpec>;
#[doc = "Field `COLC` reader - COLC"]
pub type ColcR = crate::FieldReader;
#[doc = "Field `COLC` writer - COLC"]
pub type ColcW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LPE` reader - LPE"]
pub type LpeR = crate::BitReader;
#[doc = "Field `LPE` writer - LPE"]
pub type LpeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&self) -> ColcR {
        ColcR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&self) -> LpeR {
        LpeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - COLC"]
    #[inline(always)]
    pub fn colc(&mut self) -> ColcW<DsiLcolcrSpec> {
        ColcW::new(self, 0)
    }
    #[doc = "Bit 8 - LPE"]
    #[inline(always)]
    pub fn lpe(&mut self) -> LpeW<DsiLcolcrSpec> {
        LpeW::new(self, 8)
    }
}
#[doc = "DSI Host LTDC color coding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcolcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_lcolcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLcolcrSpec;
impl crate::RegisterSpec for DsiLcolcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcolcr::R`](R) reader structure"]
impl crate::Readable for DsiLcolcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_lcolcr::W`](W) writer structure"]
impl crate::Writable for DsiLcolcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_LCOLCR to value 0"]
impl crate::Resettable for DsiLcolcrSpec {}
