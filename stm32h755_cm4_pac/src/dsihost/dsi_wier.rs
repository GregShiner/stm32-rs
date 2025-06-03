#[doc = "Register `DSI_WIER` reader"]
pub type R = crate::R<DsiWierSpec>;
#[doc = "Register `DSI_WIER` writer"]
pub type W = crate::W<DsiWierSpec>;
#[doc = "Field `TEIE` reader - TEIE"]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - TEIE"]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - ERIE"]
pub type ErieR = crate::BitReader;
#[doc = "Field `ERIE` writer - ERIE"]
pub type ErieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLLIE` reader - PLLLIE"]
pub type PlllieR = crate::BitReader;
#[doc = "Field `PLLLIE` writer - PLLLIE"]
pub type PlllieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLLUIE` reader - PLLUIE"]
pub type PlluieR = crate::BitReader;
#[doc = "Field `PLLUIE` writer - PLLUIE"]
pub type PlluieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIE` reader - RRIE"]
pub type RrieR = crate::BitReader;
#[doc = "Field `RRIE` writer - RRIE"]
pub type RrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TEIE"]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ERIE"]
    #[inline(always)]
    pub fn erie(&self) -> ErieR {
        ErieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 9 - PLLLIE"]
    #[inline(always)]
    pub fn plllie(&self) -> PlllieR {
        PlllieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - PLLUIE"]
    #[inline(always)]
    pub fn plluie(&self) -> PlluieR {
        PlluieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - RRIE"]
    #[inline(always)]
    pub fn rrie(&self) -> RrieR {
        RrieR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEIE"]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<DsiWierSpec> {
        TeieW::new(self, 0)
    }
    #[doc = "Bit 1 - ERIE"]
    #[inline(always)]
    pub fn erie(&mut self) -> ErieW<DsiWierSpec> {
        ErieW::new(self, 1)
    }
    #[doc = "Bit 9 - PLLLIE"]
    #[inline(always)]
    pub fn plllie(&mut self) -> PlllieW<DsiWierSpec> {
        PlllieW::new(self, 9)
    }
    #[doc = "Bit 10 - PLLUIE"]
    #[inline(always)]
    pub fn plluie(&mut self) -> PlluieW<DsiWierSpec> {
        PlluieW::new(self, 10)
    }
    #[doc = "Bit 13 - RRIE"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RrieW<DsiWierSpec> {
        RrieW::new(self, 13)
    }
}
#[doc = "DSI wrapper interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWierSpec;
impl crate::RegisterSpec for DsiWierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wier::R`](R) reader structure"]
impl crate::Readable for DsiWierSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wier::W`](W) writer structure"]
impl crate::Writable for DsiWierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WIER to value 0"]
impl crate::Resettable for DsiWierSpec {}
