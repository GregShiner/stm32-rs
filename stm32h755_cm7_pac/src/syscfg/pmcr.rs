#[doc = "Register `PMCR` reader"]
pub type R = crate::R<PmcrSpec>;
#[doc = "Register `PMCR` writer"]
pub type W = crate::W<PmcrSpec>;
#[doc = "Field `I2C1FMP` reader - I2C1 Fm+"]
pub type I2c1fmpR = crate::BitReader;
#[doc = "Field `I2C1FMP` writer - I2C1 Fm+"]
pub type I2c1fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2FMP` reader - I2C2 Fm+"]
pub type I2c2fmpR = crate::BitReader;
#[doc = "Field `I2C2FMP` writer - I2C2 Fm+"]
pub type I2c2fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3FMP` reader - I2C3 Fm+"]
pub type I2c3fmpR = crate::BitReader;
#[doc = "Field `I2C3FMP` writer - I2C3 Fm+"]
pub type I2c3fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4FMP` reader - I2C4 Fm+"]
pub type I2c4fmpR = crate::BitReader;
#[doc = "Field `I2C4FMP` writer - I2C4 Fm+"]
pub type I2c4fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB6FMP` reader - PB(6) Fm+"]
pub type Pb6fmpR = crate::BitReader;
#[doc = "Field `PB6FMP` writer - PB(6) Fm+"]
pub type Pb6fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB7FMP` reader - PB(7) Fast Mode Plus"]
pub type Pb7fmpR = crate::BitReader;
#[doc = "Field `PB7FMP` writer - PB(7) Fast Mode Plus"]
pub type Pb7fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB8FMP` reader - PB(8) Fast Mode Plus"]
pub type Pb8fmpR = crate::BitReader;
#[doc = "Field `PB8FMP` writer - PB(8) Fast Mode Plus"]
pub type Pb8fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PB9FMP` reader - PB(9) Fm+"]
pub type Pb9fmpR = crate::BitReader;
#[doc = "Field `PB9FMP` writer - PB(9) Fm+"]
pub type Pb9fmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTE` reader - Booster Enable"]
pub type BoosteR = crate::BitReader;
#[doc = "Field `BOOSTE` writer - Booster Enable"]
pub type BoosteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOSTVDDSEL` reader - Analog switch supply voltage selection"]
pub type BoostvddselR = crate::BitReader;
#[doc = "Field `BOOSTVDDSEL` writer - Analog switch supply voltage selection"]
pub type BoostvddselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPIS` reader - Ethernet PHY Interface Selection"]
pub type EpisR = crate::FieldReader;
#[doc = "Field `EPIS` writer - Ethernet PHY Interface Selection"]
pub type EpisW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PA0SO` reader - PA0 Switch Open"]
pub type Pa0soR = crate::BitReader;
#[doc = "Field `PA0SO` writer - PA0 Switch Open"]
pub type Pa0soW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PA1SO` reader - PA1 Switch Open"]
pub type Pa1soR = crate::BitReader;
#[doc = "Field `PA1SO` writer - PA1 Switch Open"]
pub type Pa1soW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC2SO` reader - PC2 Switch Open"]
pub type Pc2soR = crate::BitReader;
#[doc = "Field `PC2SO` writer - PC2 Switch Open"]
pub type Pc2soW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PC3SO` reader - PC3 Switch Open"]
pub type Pc3soR = crate::BitReader;
#[doc = "Field `PC3SO` writer - PC3 Switch Open"]
pub type Pc3soW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&self) -> I2c1fmpR {
        I2c1fmpR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&self) -> I2c2fmpR {
        I2c2fmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&self) -> I2c3fmpR {
        I2c3fmpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&self) -> I2c4fmpR {
        I2c4fmpR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&self) -> Pb6fmpR {
        Pb6fmpR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&self) -> Pb7fmpR {
        Pb7fmpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&self) -> Pb8fmpR {
        Pb8fmpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&self) -> Pb9fmpR {
        Pb9fmpR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&self) -> BoosteR {
        BoosteR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    pub fn boostvddsel(&self) -> BoostvddselR {
        BoostvddselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&self) -> EpisR {
        EpisR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&self) -> Pa0soR {
        Pa0soR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&self) -> Pa1soR {
        Pa1soR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&self) -> Pc2soR {
        Pc2soR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&self) -> Pc3soR {
        Pc3soR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2C1 Fm+"]
    #[inline(always)]
    pub fn i2c1fmp(&mut self) -> I2c1fmpW<PmcrSpec> {
        I2c1fmpW::new(self, 0)
    }
    #[doc = "Bit 1 - I2C2 Fm+"]
    #[inline(always)]
    pub fn i2c2fmp(&mut self) -> I2c2fmpW<PmcrSpec> {
        I2c2fmpW::new(self, 1)
    }
    #[doc = "Bit 2 - I2C3 Fm+"]
    #[inline(always)]
    pub fn i2c3fmp(&mut self) -> I2c3fmpW<PmcrSpec> {
        I2c3fmpW::new(self, 2)
    }
    #[doc = "Bit 3 - I2C4 Fm+"]
    #[inline(always)]
    pub fn i2c4fmp(&mut self) -> I2c4fmpW<PmcrSpec> {
        I2c4fmpW::new(self, 3)
    }
    #[doc = "Bit 4 - PB(6) Fm+"]
    #[inline(always)]
    pub fn pb6fmp(&mut self) -> Pb6fmpW<PmcrSpec> {
        Pb6fmpW::new(self, 4)
    }
    #[doc = "Bit 5 - PB(7) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb7fmp(&mut self) -> Pb7fmpW<PmcrSpec> {
        Pb7fmpW::new(self, 5)
    }
    #[doc = "Bit 6 - PB(8) Fast Mode Plus"]
    #[inline(always)]
    pub fn pb8fmp(&mut self) -> Pb8fmpW<PmcrSpec> {
        Pb8fmpW::new(self, 6)
    }
    #[doc = "Bit 7 - PB(9) Fm+"]
    #[inline(always)]
    pub fn pb9fmp(&mut self) -> Pb9fmpW<PmcrSpec> {
        Pb9fmpW::new(self, 7)
    }
    #[doc = "Bit 8 - Booster Enable"]
    #[inline(always)]
    pub fn booste(&mut self) -> BoosteW<PmcrSpec> {
        BoosteW::new(self, 8)
    }
    #[doc = "Bit 9 - Analog switch supply voltage selection"]
    #[inline(always)]
    pub fn boostvddsel(&mut self) -> BoostvddselW<PmcrSpec> {
        BoostvddselW::new(self, 9)
    }
    #[doc = "Bits 21:23 - Ethernet PHY Interface Selection"]
    #[inline(always)]
    pub fn epis(&mut self) -> EpisW<PmcrSpec> {
        EpisW::new(self, 21)
    }
    #[doc = "Bit 24 - PA0 Switch Open"]
    #[inline(always)]
    pub fn pa0so(&mut self) -> Pa0soW<PmcrSpec> {
        Pa0soW::new(self, 24)
    }
    #[doc = "Bit 25 - PA1 Switch Open"]
    #[inline(always)]
    pub fn pa1so(&mut self) -> Pa1soW<PmcrSpec> {
        Pa1soW::new(self, 25)
    }
    #[doc = "Bit 26 - PC2 Switch Open"]
    #[inline(always)]
    pub fn pc2so(&mut self) -> Pc2soW<PmcrSpec> {
        Pc2soW::new(self, 26)
    }
    #[doc = "Bit 27 - PC3 Switch Open"]
    #[inline(always)]
    pub fn pc3so(&mut self) -> Pc3soW<PmcrSpec> {
        Pc3soW::new(self, 27)
    }
}
#[doc = "peripheral mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`pmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PmcrSpec;
impl crate::RegisterSpec for PmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pmcr::R`](R) reader structure"]
impl crate::Readable for PmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`pmcr::W`](W) writer structure"]
impl crate::Writable for PmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PMCR to value 0"]
impl crate::Resettable for PmcrSpec {}
