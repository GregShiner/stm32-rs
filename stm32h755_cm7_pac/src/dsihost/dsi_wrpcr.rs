#[doc = "Register `DSI_WRPCR` reader"]
pub type R = crate::R<DsiWrpcrSpec>;
#[doc = "Register `DSI_WRPCR` writer"]
pub type W = crate::W<DsiWrpcrSpec>;
#[doc = "Field `PLLEN` reader - PLLEN"]
pub type PllenR = crate::BitReader;
#[doc = "Field `PLLEN` writer - PLLEN"]
pub type PllenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NDIV` reader - NDIV"]
pub type NdivR = crate::FieldReader;
#[doc = "Field `NDIV` writer - NDIV"]
pub type NdivW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `IDF` reader - IDF"]
pub type IdfR = crate::FieldReader;
#[doc = "Field `IDF` writer - IDF"]
pub type IdfW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ODF` reader - ODF"]
pub type OdfR = crate::FieldReader;
#[doc = "Field `ODF` writer - ODF"]
pub type OdfW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `REGEN` reader - REGEN"]
pub type RegenR = crate::BitReader;
#[doc = "Field `REGEN` writer - REGEN"]
pub type RegenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&self) -> PllenR {
        PllenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&self) -> NdivR {
        NdivR::new(((self.bits >> 2) & 0x7f) as u8)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&self) -> IdfR {
        IdfR::new(((self.bits >> 11) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&self) -> OdfR {
        OdfR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&self) -> RegenR {
        RegenR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLLEN"]
    #[inline(always)]
    pub fn pllen(&mut self) -> PllenW<DsiWrpcrSpec> {
        PllenW::new(self, 0)
    }
    #[doc = "Bits 2:8 - NDIV"]
    #[inline(always)]
    pub fn ndiv(&mut self) -> NdivW<DsiWrpcrSpec> {
        NdivW::new(self, 2)
    }
    #[doc = "Bits 11:14 - IDF"]
    #[inline(always)]
    pub fn idf(&mut self) -> IdfW<DsiWrpcrSpec> {
        IdfW::new(self, 11)
    }
    #[doc = "Bits 16:17 - ODF"]
    #[inline(always)]
    pub fn odf(&mut self) -> OdfW<DsiWrpcrSpec> {
        OdfW::new(self, 16)
    }
    #[doc = "Bit 24 - REGEN"]
    #[inline(always)]
    pub fn regen(&mut self) -> RegenW<DsiWrpcrSpec> {
        RegenW::new(self, 24)
    }
}
#[doc = "DSI wrapper regulator and PLL control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wrpcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wrpcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWrpcrSpec;
impl crate::RegisterSpec for DsiWrpcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wrpcr::R`](R) reader structure"]
impl crate::Readable for DsiWrpcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wrpcr::W`](W) writer structure"]
impl crate::Writable for DsiWrpcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WRPCR to value 0"]
impl crate::Resettable for DsiWrpcrSpec {}
