#[doc = "Register `AHB2ENR` reader"]
pub type R = crate::R<Ahb2enrSpec>;
#[doc = "Register `AHB2ENR` writer"]
pub type W = crate::W<Ahb2enrSpec>;
#[doc = "Field `CAMITFEN` reader - CAMITF peripheral clock enable"]
pub type CamitfenR = crate::BitReader;
#[doc = "Field `CAMITFEN` writer - CAMITF peripheral clock enable"]
pub type CamitfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTEN` reader - CRYPT peripheral clock enable"]
pub type CryptenR = crate::BitReader;
#[doc = "Field `CRYPTEN` writer - CRYPT peripheral clock enable"]
pub type CryptenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HASHEN` reader - HASH peripheral clock enable"]
pub type HashenR = crate::BitReader;
#[doc = "Field `HASHEN` writer - HASH peripheral clock enable"]
pub type HashenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RNGEN` reader - RNG peripheral clocks enable"]
pub type RngenR = crate::BitReader;
#[doc = "Field `RNGEN` writer - RNG peripheral clocks enable"]
pub type RngenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC2EN` reader - SDMMC2 and SDMMC2 delay clock enable"]
pub type Sdmmc2enR = crate::BitReader;
#[doc = "Field `SDMMC2EN` writer - SDMMC2 and SDMMC2 delay clock enable"]
pub type Sdmmc2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1EN` reader - SRAM1 block enable"]
pub type Sram1enR = crate::BitReader;
#[doc = "Field `SRAM1EN` writer - SRAM1 block enable"]
pub type Sram1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2EN` reader - SRAM2 block enable"]
pub type Sram2enR = crate::BitReader;
#[doc = "Field `SRAM2EN` writer - SRAM2 block enable"]
pub type Sram2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3EN` reader - SRAM3 block enable"]
pub type Sram3enR = crate::BitReader;
#[doc = "Field `SRAM3EN` writer - SRAM3 block enable"]
pub type Sram3enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CAMITF peripheral clock enable"]
    #[inline(always)]
    pub fn camitfen(&self) -> CamitfenR {
        CamitfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&self) -> CryptenR {
        CryptenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&self) -> HashenR {
        HashenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&self) -> RngenR {
        RngenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&self) -> Sdmmc2enR {
        Sdmmc2enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&self) -> Sram1enR {
        Sram1enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&self) -> Sram2enR {
        Sram2enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&self) -> Sram3enR {
        Sram3enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CAMITF peripheral clock enable"]
    #[inline(always)]
    pub fn camitfen(&mut self) -> CamitfenW<Ahb2enrSpec> {
        CamitfenW::new(self, 0)
    }
    #[doc = "Bit 4 - CRYPT peripheral clock enable"]
    #[inline(always)]
    pub fn crypten(&mut self) -> CryptenW<Ahb2enrSpec> {
        CryptenW::new(self, 4)
    }
    #[doc = "Bit 5 - HASH peripheral clock enable"]
    #[inline(always)]
    pub fn hashen(&mut self) -> HashenW<Ahb2enrSpec> {
        HashenW::new(self, 5)
    }
    #[doc = "Bit 6 - RNG peripheral clocks enable"]
    #[inline(always)]
    pub fn rngen(&mut self) -> RngenW<Ahb2enrSpec> {
        RngenW::new(self, 6)
    }
    #[doc = "Bit 9 - SDMMC2 and SDMMC2 delay clock enable"]
    #[inline(always)]
    pub fn sdmmc2en(&mut self) -> Sdmmc2enW<Ahb2enrSpec> {
        Sdmmc2enW::new(self, 9)
    }
    #[doc = "Bit 29 - SRAM1 block enable"]
    #[inline(always)]
    pub fn sram1en(&mut self) -> Sram1enW<Ahb2enrSpec> {
        Sram1enW::new(self, 29)
    }
    #[doc = "Bit 30 - SRAM2 block enable"]
    #[inline(always)]
    pub fn sram2en(&mut self) -> Sram2enW<Ahb2enrSpec> {
        Sram2enW::new(self, 30)
    }
    #[doc = "Bit 31 - SRAM3 block enable"]
    #[inline(always)]
    pub fn sram3en(&mut self) -> Sram3enW<Ahb2enrSpec> {
        Sram3enW::new(self, 31)
    }
}
#[doc = "RCC AHB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb2enrSpec;
impl crate::RegisterSpec for Ahb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb2enr::R`](R) reader structure"]
impl crate::Readable for Ahb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb2enr::W`](W) writer structure"]
impl crate::Writable for Ahb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB2ENR to value 0"]
impl crate::Resettable for Ahb2enrSpec {}
