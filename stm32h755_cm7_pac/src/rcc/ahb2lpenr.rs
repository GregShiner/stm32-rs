#[doc = "Register `AHB2LPENR` reader"]
pub type R = crate::R<Ahb2lpenrSpec>;
#[doc = "Register `AHB2LPENR` writer"]
pub type W = crate::W<Ahb2lpenrSpec>;
#[doc = "Field `CAMITFLPEN` reader - CAMITF peripheral clock enable during CSleep mode"]
pub type CamitflpenR = crate::BitReader;
#[doc = "Field `CAMITFLPEN` writer - CAMITF peripheral clock enable during CSleep mode"]
pub type CamitflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTLPEN` reader - CRYPT peripheral clock enable during CSleep mode"]
pub type CryptlpenR = crate::BitReader;
#[doc = "Field `CRYPTLPEN` writer - CRYPT peripheral clock enable during CSleep mode"]
pub type CryptlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHLPEN` reader - HASH peripheral clock enable during CSleep mode"]
pub type HashlpenR = crate::BitReader;
#[doc = "Field `HASHLPEN` writer - HASH peripheral clock enable during CSleep mode"]
pub type HashlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGLPEN` reader - RNG peripheral clock enable during CSleep mode"]
pub type RnglpenR = crate::BitReader;
#[doc = "Field `RNGLPEN` writer - RNG peripheral clock enable during CSleep mode"]
pub type RnglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2LPEN` reader - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type Sdmmc2lpenR = crate::BitReader;
#[doc = "Field `SDMMC2LPEN` writer - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
pub type Sdmmc2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1LPEN` reader - SRAM1 Clock Enable During CSleep Mode"]
pub type Sram1lpenR = crate::BitReader;
#[doc = "Field `SRAM1LPEN` writer - SRAM1 Clock Enable During CSleep Mode"]
pub type Sram1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2LPEN` reader - SRAM2 Clock Enable During CSleep Mode"]
pub type Sram2lpenR = crate::BitReader;
#[doc = "Field `SRAM2LPEN` writer - SRAM2 Clock Enable During CSleep Mode"]
pub type Sram2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3LPEN` reader - SRAM3 Clock Enable During CSleep Mode"]
pub type Sram3lpenR = crate::BitReader;
#[doc = "Field `SRAM3LPEN` writer - SRAM3 Clock Enable During CSleep Mode"]
pub type Sram3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CAMITF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn camitflpen(&self) -> CamitflpenR {
        CamitflpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&self) -> CryptlpenR {
        CryptlpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&self) -> HashlpenR {
        HashlpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&self) -> RnglpenR {
        RnglpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&self) -> Sdmmc2lpenR {
        Sdmmc2lpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&self) -> Sram1lpenR {
        Sram1lpenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&self) -> Sram2lpenR {
        Sram2lpenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram3lpen(&self) -> Sram3lpenR {
        Sram3lpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn camitflpen(&mut self) -> CamitflpenW<Ahb2lpenrSpec> {
        CamitflpenW::new(self, 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn cryptlpen(&mut self) -> CryptlpenW<Ahb2lpenrSpec> {
        CryptlpenW::new(self, 4)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hashlpen(&mut self) -> HashlpenW<Ahb2lpenrSpec> {
        HashlpenW::new(self, 5)
    }
    #[doc = "Bit 6 - RNG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn rnglpen(&mut self) -> RnglpenW<Ahb2lpenrSpec> {
        RnglpenW::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc2lpen(&mut self) -> Sdmmc2lpenW<Ahb2lpenrSpec> {
        Sdmmc2lpenW::new(self, 9)
    }
    #[doc = "Bit 29 - SRAM1 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram1lpen(&mut self) -> Sram1lpenW<Ahb2lpenrSpec> {
        Sram1lpenW::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM2 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram2lpen(&mut self) -> Sram2lpenW<Ahb2lpenrSpec> {
        Sram2lpenW::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM3 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram3lpen(&mut self) -> Sram3lpenW<Ahb2lpenrSpec> {
        Sram3lpenW::new(self, 31)
    }
}
#[doc = "RCC AHB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2lpenrSpec;
impl crate::RegisterSpec for Ahb2lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2lpenr::R`](R) reader structure"]
impl crate::Readable for Ahb2lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2lpenr::W`](W) writer structure"]
impl crate::Writable for Ahb2lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2LPENR to value 0"]
impl crate::Resettable for Ahb2lpenrSpec {}
