#[doc = "Register `DSI_IER1` reader"]
pub type R = crate::R<DsiIer1Spec>;
#[doc = "Register `DSI_IER1` writer"]
pub type W = crate::W<DsiIer1Spec>;
#[doc = "Field `TOHSTXIE` reader - TOHSTXIE"]
pub type TohstxieR = crate::BitReader;
#[doc = "Field `TOHSTXIE` writer - TOHSTXIE"]
pub type TohstxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOLPRXIE` reader - TOLPRXIE"]
pub type TolprxieR = crate::BitReader;
#[doc = "Field `TOLPRXIE` writer - TOLPRXIE"]
pub type TolprxieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCSEIE` reader - ECCSEIE"]
pub type EccseieR = crate::BitReader;
#[doc = "Field `ECCSEIE` writer - ECCSEIE"]
pub type EccseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECCMEIE` reader - ECCMEIE"]
pub type EccmeieR = crate::BitReader;
#[doc = "Field `ECCMEIE` writer - ECCMEIE"]
pub type EccmeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCEIE` reader - CRCEIE"]
pub type CrceieR = crate::BitReader;
#[doc = "Field `CRCEIE` writer - CRCEIE"]
pub type CrceieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSEIE` reader - PSEIE"]
pub type PseieR = crate::BitReader;
#[doc = "Field `PSEIE` writer - PSEIE"]
pub type PseieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOTPEIE` reader - EOTPEIE"]
pub type EotpeieR = crate::BitReader;
#[doc = "Field `EOTPEIE` writer - EOTPEIE"]
pub type EotpeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWREIE` reader - LPWREIE"]
pub type LpwreieR = crate::BitReader;
#[doc = "Field `LPWREIE` writer - LPWREIE"]
pub type LpwreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCWREIE` reader - GCWREIE"]
pub type GcwreieR = crate::BitReader;
#[doc = "Field `GCWREIE` writer - GCWREIE"]
pub type GcwreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPWREIE` reader - GPWREIE"]
pub type GpwreieR = crate::BitReader;
#[doc = "Field `GPWREIE` writer - GPWREIE"]
pub type GpwreieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPTXEIE` reader - GPTXEIE"]
pub type GptxeieR = crate::BitReader;
#[doc = "Field `GPTXEIE` writer - GPTXEIE"]
pub type GptxeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPRDEIE` reader - GPRDEIE"]
pub type GprdeieR = crate::BitReader;
#[doc = "Field `GPRDEIE` writer - GPRDEIE"]
pub type GprdeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPRXEIE` reader - GPRXEIE"]
pub type GprxeieR = crate::BitReader;
#[doc = "Field `GPRXEIE` writer - GPRXEIE"]
pub type GprxeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TOHSTXIE"]
    #[inline(always)]
    pub fn tohstxie(&self) -> TohstxieR {
        TohstxieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TOLPRXIE"]
    #[inline(always)]
    pub fn tolprxie(&self) -> TolprxieR {
        TolprxieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECCSEIE"]
    #[inline(always)]
    pub fn eccseie(&self) -> EccseieR {
        EccseieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ECCMEIE"]
    #[inline(always)]
    pub fn eccmeie(&self) -> EccmeieR {
        EccmeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&self) -> CrceieR {
        CrceieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PSEIE"]
    #[inline(always)]
    pub fn pseie(&self) -> PseieR {
        PseieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - EOTPEIE"]
    #[inline(always)]
    pub fn eotpeie(&self) -> EotpeieR {
        EotpeieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - LPWREIE"]
    #[inline(always)]
    pub fn lpwreie(&self) -> LpwreieR {
        LpwreieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GCWREIE"]
    #[inline(always)]
    pub fn gcwreie(&self) -> GcwreieR {
        GcwreieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPWREIE"]
    #[inline(always)]
    pub fn gpwreie(&self) -> GpwreieR {
        GpwreieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPTXEIE"]
    #[inline(always)]
    pub fn gptxeie(&self) -> GptxeieR {
        GptxeieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GPRDEIE"]
    #[inline(always)]
    pub fn gprdeie(&self) -> GprdeieR {
        GprdeieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GPRXEIE"]
    #[inline(always)]
    pub fn gprxeie(&self) -> GprxeieR {
        GprxeieR::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TOHSTXIE"]
    #[inline(always)]
    pub fn tohstxie(&mut self) -> TohstxieW<DsiIer1Spec> {
        TohstxieW::new(self, 0)
    }
    #[doc = "Bit 1 - TOLPRXIE"]
    #[inline(always)]
    pub fn tolprxie(&mut self) -> TolprxieW<DsiIer1Spec> {
        TolprxieW::new(self, 1)
    }
    #[doc = "Bit 2 - ECCSEIE"]
    #[inline(always)]
    pub fn eccseie(&mut self) -> EccseieW<DsiIer1Spec> {
        EccseieW::new(self, 2)
    }
    #[doc = "Bit 3 - ECCMEIE"]
    #[inline(always)]
    pub fn eccmeie(&mut self) -> EccmeieW<DsiIer1Spec> {
        EccmeieW::new(self, 3)
    }
    #[doc = "Bit 4 - CRCEIE"]
    #[inline(always)]
    pub fn crceie(&mut self) -> CrceieW<DsiIer1Spec> {
        CrceieW::new(self, 4)
    }
    #[doc = "Bit 5 - PSEIE"]
    #[inline(always)]
    pub fn pseie(&mut self) -> PseieW<DsiIer1Spec> {
        PseieW::new(self, 5)
    }
    #[doc = "Bit 6 - EOTPEIE"]
    #[inline(always)]
    pub fn eotpeie(&mut self) -> EotpeieW<DsiIer1Spec> {
        EotpeieW::new(self, 6)
    }
    #[doc = "Bit 7 - LPWREIE"]
    #[inline(always)]
    pub fn lpwreie(&mut self) -> LpwreieW<DsiIer1Spec> {
        LpwreieW::new(self, 7)
    }
    #[doc = "Bit 8 - GCWREIE"]
    #[inline(always)]
    pub fn gcwreie(&mut self) -> GcwreieW<DsiIer1Spec> {
        GcwreieW::new(self, 8)
    }
    #[doc = "Bit 9 - GPWREIE"]
    #[inline(always)]
    pub fn gpwreie(&mut self) -> GpwreieW<DsiIer1Spec> {
        GpwreieW::new(self, 9)
    }
    #[doc = "Bit 10 - GPTXEIE"]
    #[inline(always)]
    pub fn gptxeie(&mut self) -> GptxeieW<DsiIer1Spec> {
        GptxeieW::new(self, 10)
    }
    #[doc = "Bit 11 - GPRDEIE"]
    #[inline(always)]
    pub fn gprdeie(&mut self) -> GprdeieW<DsiIer1Spec> {
        GprdeieW::new(self, 11)
    }
    #[doc = "Bit 12 - GPRXEIE"]
    #[inline(always)]
    pub fn gprxeie(&mut self) -> GprxeieW<DsiIer1Spec> {
        GprxeieW::new(self, 12)
    }
}
#[doc = "DSI Host interrupt enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_ier1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_ier1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIer1Spec;
impl crate::RegisterSpec for DsiIer1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_ier1::R`](R) reader structure"]
impl crate::Readable for DsiIer1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_ier1::W`](W) writer structure"]
impl crate::Writable for DsiIer1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_IER1 to value 0"]
impl crate::Resettable for DsiIer1Spec {}
