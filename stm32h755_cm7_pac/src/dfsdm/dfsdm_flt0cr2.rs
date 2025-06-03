#[doc = "Register `DFSDM_FLT0CR2` reader"]
pub type R = crate::R<DfsdmFlt0cr2Spec>;
#[doc = "Register `DFSDM_FLT0CR2` writer"]
pub type W = crate::W<DfsdmFlt0cr2Spec>;
#[doc = "Field `JEOCIE` reader - Injected end of conversion interrupt enable"]
pub type JeocieR = crate::BitReader;
#[doc = "Field `JEOCIE` writer - Injected end of conversion interrupt enable"]
pub type JeocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REOCIE` reader - Regular end of conversion interrupt enable"]
pub type ReocieR = crate::BitReader;
#[doc = "Field `REOCIE` writer - Regular end of conversion interrupt enable"]
pub type ReocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JOVRIE` reader - Injected data overrun interrupt enable"]
pub type JovrieR = crate::BitReader;
#[doc = "Field `JOVRIE` writer - Injected data overrun interrupt enable"]
pub type JovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ROVRIE` reader - Regular data overrun interrupt enable"]
pub type RovrieR = crate::BitReader;
#[doc = "Field `ROVRIE` writer - Regular data overrun interrupt enable"]
pub type RovrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDIE` reader - Analog watchdog interrupt enable"]
pub type AwdieR = crate::BitReader;
#[doc = "Field `AWDIE` writer - Analog watchdog interrupt enable"]
pub type AwdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCDIE` reader - Short-circuit detector interrupt enable"]
pub type ScdieR = crate::BitReader;
#[doc = "Field `SCDIE` writer - Short-circuit detector interrupt enable"]
pub type ScdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKABIE` reader - Clock absence interrupt enable"]
pub type CkabieR = crate::BitReader;
#[doc = "Field `CKABIE` writer - Clock absence interrupt enable"]
pub type CkabieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXCH` reader - Extremes detector channel selection"]
pub type ExchR = crate::FieldReader;
#[doc = "Field `EXCH` writer - Extremes detector channel selection"]
pub type ExchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AWDCH` reader - Analog watchdog channel selection"]
pub type AwdchR = crate::FieldReader;
#[doc = "Field `AWDCH` writer - Analog watchdog channel selection"]
pub type AwdchW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&self) -> JeocieR {
        JeocieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&self) -> ReocieR {
        ReocieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&self) -> JovrieR {
        JovrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&self) -> RovrieR {
        RovrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&self) -> AwdieR {
        AwdieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&self) -> ScdieR {
        ScdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&self) -> CkabieR {
        CkabieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&self) -> ExchR {
        ExchR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&self) -> AwdchR {
        AwdchR::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Injected end of conversion interrupt enable"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JeocieW<DfsdmFlt0cr2Spec> {
        JeocieW::new(self, 0)
    }
    #[doc = "Bit 1 - Regular end of conversion interrupt enable"]
    #[inline(always)]
    pub fn reocie(&mut self) -> ReocieW<DfsdmFlt0cr2Spec> {
        ReocieW::new(self, 1)
    }
    #[doc = "Bit 2 - Injected data overrun interrupt enable"]
    #[inline(always)]
    pub fn jovrie(&mut self) -> JovrieW<DfsdmFlt0cr2Spec> {
        JovrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Regular data overrun interrupt enable"]
    #[inline(always)]
    pub fn rovrie(&mut self) -> RovrieW<DfsdmFlt0cr2Spec> {
        RovrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Analog watchdog interrupt enable"]
    #[inline(always)]
    pub fn awdie(&mut self) -> AwdieW<DfsdmFlt0cr2Spec> {
        AwdieW::new(self, 4)
    }
    #[doc = "Bit 5 - Short-circuit detector interrupt enable"]
    #[inline(always)]
    pub fn scdie(&mut self) -> ScdieW<DfsdmFlt0cr2Spec> {
        ScdieW::new(self, 5)
    }
    #[doc = "Bit 6 - Clock absence interrupt enable"]
    #[inline(always)]
    pub fn ckabie(&mut self) -> CkabieW<DfsdmFlt0cr2Spec> {
        CkabieW::new(self, 6)
    }
    #[doc = "Bits 8:15 - Extremes detector channel selection"]
    #[inline(always)]
    pub fn exch(&mut self) -> ExchW<DfsdmFlt0cr2Spec> {
        ExchW::new(self, 8)
    }
    #[doc = "Bits 16:23 - Analog watchdog channel selection"]
    #[inline(always)]
    pub fn awdch(&mut self) -> AwdchW<DfsdmFlt0cr2Spec> {
        AwdchW::new(self, 16)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt0cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt0cr2Spec;
impl crate::RegisterSpec for DfsdmFlt0cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0cr2::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt0cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt0cr2::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt0cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT0CR2 to value 0"]
impl crate::Resettable for DfsdmFlt0cr2Spec {}
