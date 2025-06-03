#[doc = "Register `APB2LPENR` reader"]
pub type R = crate::R<Apb2lpenrSpec>;
#[doc = "Register `APB2LPENR` writer"]
pub type W = crate::W<Apb2lpenrSpec>;
#[doc = "Field `TIM1LPEN` reader - TIM1 peripheral clock enable during CSleep mode"]
pub type Tim1lpenR = crate::BitReader;
#[doc = "Field `TIM1LPEN` writer - TIM1 peripheral clock enable during CSleep mode"]
pub type Tim1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8LPEN` reader - TIM8 peripheral clock enable during CSleep mode"]
pub type Tim8lpenR = crate::BitReader;
#[doc = "Field `TIM8LPEN` writer - TIM8 peripheral clock enable during CSleep mode"]
pub type Tim8lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1LPEN` reader - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub type Usart1lpenR = crate::BitReader;
#[doc = "Field `USART1LPEN` writer - USART1 Peripheral Clocks Enable During CSleep Mode"]
pub type Usart1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6LPEN` reader - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub type Usart6lpenR = crate::BitReader;
#[doc = "Field `USART6LPEN` writer - USART6 Peripheral Clocks Enable During CSleep Mode"]
pub type Usart6lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1LPEN` reader - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi1lpenR = crate::BitReader;
#[doc = "Field `SPI1LPEN` writer - SPI1 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4LPEN` reader - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi4lpenR = crate::BitReader;
#[doc = "Field `SPI4LPEN` writer - SPI4 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15LPEN` reader - TIM15 peripheral clock enable during CSleep mode"]
pub type Tim15lpenR = crate::BitReader;
#[doc = "Field `TIM15LPEN` writer - TIM15 peripheral clock enable during CSleep mode"]
pub type Tim15lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16LPEN` reader - TIM16 peripheral clock enable during CSleep mode"]
pub type Tim16lpenR = crate::BitReader;
#[doc = "Field `TIM16LPEN` writer - TIM16 peripheral clock enable during CSleep mode"]
pub type Tim16lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17LPEN` reader - TIM17 peripheral clock enable during CSleep mode"]
pub type Tim17lpenR = crate::BitReader;
#[doc = "Field `TIM17LPEN` writer - TIM17 peripheral clock enable during CSleep mode"]
pub type Tim17lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5LPEN` reader - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi5lpenR = crate::BitReader;
#[doc = "Field `SPI5LPEN` writer - SPI5 Peripheral Clocks Enable During CSleep Mode"]
pub type Spi5lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1LPEN` reader - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai1lpenR = crate::BitReader;
#[doc = "Field `SAI1LPEN` writer - SAI1 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2LPEN` reader - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai2lpenR = crate::BitReader;
#[doc = "Field `SAI2LPEN` writer - SAI2 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3LPEN` reader - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai3lpenR = crate::BitReader;
#[doc = "Field `SAI3LPEN` writer - SAI3 Peripheral Clocks Enable During CSleep Mode"]
pub type Sai3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDM1LPEN` reader - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub type Dfsdm1lpenR = crate::BitReader;
#[doc = "Field `DFSDM1LPEN` writer - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
pub type Dfsdm1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRTIMLPEN` reader - HRTIM peripheral clock enable during CSleep mode"]
pub type HrtimlpenR = crate::BitReader;
#[doc = "Field `HRTIMLPEN` writer - HRTIM peripheral clock enable during CSleep mode"]
pub type HrtimlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&self) -> Tim1lpenR {
        Tim1lpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&self) -> Tim8lpenR {
        Tim8lpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart1lpen(&self) -> Usart1lpenR {
        Usart1lpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart6lpen(&self) -> Usart6lpenR {
        Usart6lpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi1lpen(&self) -> Spi1lpenR {
        Spi1lpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi4lpen(&self) -> Spi4lpenR {
        Spi4lpenR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim15lpen(&self) -> Tim15lpenR {
        Tim15lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim16lpen(&self) -> Tim16lpenR {
        Tim16lpenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim17lpen(&self) -> Tim17lpenR {
        Tim17lpenR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi5lpen(&self) -> Spi5lpenR {
        Spi5lpenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai1lpen(&self) -> Sai1lpenR {
        Sai1lpenR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai2lpen(&self) -> Sai2lpenR {
        Sai2lpenR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai3lpen(&self) -> Sai3lpenR {
        Sai3lpenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dfsdm1lpen(&self) -> Dfsdm1lpenR {
        Dfsdm1lpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hrtimlpen(&self) -> HrtimlpenR {
        HrtimlpenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim1lpen(&mut self) -> Tim1lpenW<Apb2lpenrSpec> {
        Tim1lpenW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim8lpen(&mut self) -> Tim8lpenW<Apb2lpenrSpec> {
        Tim8lpenW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart1lpen(&mut self) -> Usart1lpenW<Apb2lpenrSpec> {
        Usart1lpenW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn usart6lpen(&mut self) -> Usart6lpenW<Apb2lpenrSpec> {
        Usart6lpenW::new(self, 5)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi1lpen(&mut self) -> Spi1lpenW<Apb2lpenrSpec> {
        Spi1lpenW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi4lpen(&mut self) -> Spi4lpenW<Apb2lpenrSpec> {
        Spi4lpenW::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim15lpen(&mut self) -> Tim15lpenW<Apb2lpenrSpec> {
        Tim15lpenW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim16lpen(&mut self) -> Tim16lpenW<Apb2lpenrSpec> {
        Tim16lpenW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn tim17lpen(&mut self) -> Tim17lpenW<Apb2lpenrSpec> {
        Tim17lpenW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn spi5lpen(&mut self) -> Spi5lpenW<Apb2lpenrSpec> {
        Spi5lpenW::new(self, 20)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai1lpen(&mut self) -> Sai1lpenW<Apb2lpenrSpec> {
        Sai1lpenW::new(self, 22)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai2lpen(&mut self) -> Sai2lpenW<Apb2lpenrSpec> {
        Sai2lpenW::new(self, 23)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sai3lpen(&mut self) -> Sai3lpenW<Apb2lpenrSpec> {
        Sai3lpenW::new(self, 24)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dfsdm1lpen(&mut self) -> Dfsdm1lpenW<Apb2lpenrSpec> {
        Dfsdm1lpenW::new(self, 28)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn hrtimlpen(&mut self) -> HrtimlpenW<Apb2lpenrSpec> {
        HrtimlpenW::new(self, 29)
    }
}
#[doc = "RCC APB2 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2lpenrSpec;
impl crate::RegisterSpec for Apb2lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2lpenr::R`](R) reader structure"]
impl crate::Readable for Apb2lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2lpenr::W`](W) writer structure"]
impl crate::Writable for Apb2lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2LPENR to value 0"]
impl crate::Resettable for Apb2lpenrSpec {}
