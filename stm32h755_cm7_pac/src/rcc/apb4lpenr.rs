#[doc = "Register `APB4LPENR` reader"]
pub type R = crate::R<Apb4lpenrSpec>;
#[doc = "Register `APB4LPENR` writer"]
pub type W = crate::W<Apb4lpenrSpec>;
#[doc = "Field `SYSCFGLPEN` reader - SYSCFG peripheral clock enable during CSleep mode"]
pub type SyscfglpenR = crate::BitReader;
#[doc = "Field `SYSCFGLPEN` writer - SYSCFG peripheral clock enable during CSleep mode"]
pub type SyscfglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1LPEN` reader - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type Lpuart1lpenR = crate::BitReader;
#[doc = "Field `LPUART1LPEN` writer - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
pub type Lpuart1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6LPEN` reader - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi6lpenR = crate::BitReader;
#[doc = "Field `SPI6LPEN` writer - SPI6 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi6lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4LPEN` reader - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2c4lpenR = crate::BitReader;
#[doc = "Field `I2C4LPEN` writer - I2C4 Peripheral Clocks Enable During CSleep Mode"]
pub type I2c4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2LPEN` reader - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim2lpenR = crate::BitReader;
#[doc = "Field `LPTIM2LPEN` writer - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3LPEN` reader - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim3lpenR = crate::BitReader;
#[doc = "Field `LPTIM3LPEN` writer - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4LPEN` reader - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim4lpenR = crate::BitReader;
#[doc = "Field `LPTIM4LPEN` writer - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5LPEN` reader - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim5lpenR = crate::BitReader;
#[doc = "Field `LPTIM5LPEN` writer - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
pub type Lptim5lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP12LPEN` reader - COMP1/2 peripheral clock enable during CSleep mode"]
pub type Comp12lpenR = crate::BitReader;
#[doc = "Field `COMP12LPEN` writer - COMP1/2 peripheral clock enable during CSleep mode"]
pub type Comp12lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFLPEN` reader - VREF peripheral clock enable during CSleep mode"]
pub type VreflpenR = crate::BitReader;
#[doc = "Field `VREFLPEN` writer - VREF peripheral clock enable during CSleep mode"]
pub type VreflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBLPEN` reader - RTC APB Clock Enable During CSleep Mode"]
pub type RtcapblpenR = crate::BitReader;
#[doc = "Field `RTCAPBLPEN` writer - RTC APB Clock Enable During CSleep Mode"]
pub type RtcapblpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4LPEN` reader - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai4lpenR = crate::BitReader;
#[doc = "Field `SAI4LPEN` writer - SAI4 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&self) -> SyscfglpenR {
        SyscfglpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&self) -> Lpuart1lpenR {
        Lpuart1lpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&self) -> Spi6lpenR {
        Spi6lpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&self) -> I2c4lpenR {
        I2c4lpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&self) -> Lptim2lpenR {
        Lptim2lpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&self) -> Lptim3lpenR {
        Lptim3lpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&self) -> Lptim4lpenR {
        Lptim4lpenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&self) -> Lptim5lpenR {
        Lptim5lpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&self) -> Comp12lpenR {
        Comp12lpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&self) -> VreflpenR {
        VreflpenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&self) -> RtcapblpenR {
        RtcapblpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&self) -> Sai4lpenR {
        Sai4lpenR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn syscfglpen(&mut self) -> SyscfglpenW<Apb4lpenrSpec> {
        SyscfglpenW::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lpuart1lpen(&mut self) -> Lpuart1lpenW<Apb4lpenrSpec> {
        Lpuart1lpenW::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi6lpen(&mut self) -> Spi6lpenW<Apb4lpenrSpec> {
        Spi6lpenW::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn i2c4lpen(&mut self) -> I2c4lpenW<Apb4lpenrSpec> {
        I2c4lpenW::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim2lpen(&mut self) -> Lptim2lpenW<Apb4lpenrSpec> {
        Lptim2lpenW::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim3lpen(&mut self) -> Lptim3lpenW<Apb4lpenrSpec> {
        Lptim3lpenW::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim4lpen(&mut self) -> Lptim4lpenW<Apb4lpenrSpec> {
        Lptim4lpenW::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn lptim5lpen(&mut self) -> Lptim5lpenW<Apb4lpenrSpec> {
        Lptim5lpenW::new(self, 12)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn comp12lpen(&mut self) -> Comp12lpenW<Apb4lpenrSpec> {
        Comp12lpenW::new(self, 14)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn vreflpen(&mut self) -> VreflpenW<Apb4lpenrSpec> {
        VreflpenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn rtcapblpen(&mut self) -> RtcapblpenW<Apb4lpenrSpec> {
        RtcapblpenW::new(self, 16)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai4lpen(&mut self) -> Sai4lpenW<Apb4lpenrSpec> {
        Sai4lpenW::new(self, 21)
    }
}
#[doc = "RCC APB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb4lpenrSpec;
impl crate::RegisterSpec for Apb4lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4lpenr::R`](R) reader structure"]
impl crate::Readable for Apb4lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb4lpenr::W`](W) writer structure"]
impl crate::Writable for Apb4lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB4LPENR to value 0"]
impl crate::Resettable for Apb4lpenrSpec {}
