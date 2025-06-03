#[doc = "Register `DSI_LCCCR` reader"]
pub type R = crate::R<DsiLcccrSpec>;
#[doc = "Field `COLC` reader - COLC"]
pub type ColcR = crate::FieldReader;
#[doc = "Field `LPE` reader - LPE"]
pub type LpeR = crate::BitReader;
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
#[doc = "DSI Host LTDC current color coding register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_lcccr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiLcccrSpec;
impl crate::RegisterSpec for DsiLcccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_lcccr::R`](R) reader structure"]
impl crate::Readable for DsiLcccrSpec {}
#[doc = "`reset()` method sets DSI_LCCCR to value 0"]
impl crate::Resettable for DsiLcccrSpec {}
