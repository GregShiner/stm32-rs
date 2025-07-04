#[doc = "Register `C1_APB2ENR` reader"]
pub type R = crate::R<C1Apb2enrSpec>;
#[doc = "Register `C1_APB2ENR` writer"]
pub type W = crate::W<C1Apb2enrSpec>;
#[doc = "Field `TIM1EN` reader - TIM1 peripheral clock enable"]
pub type Tim1enR = crate::BitReader;
#[doc = "Field `TIM1EN` writer - TIM1 peripheral clock enable"]
pub type Tim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8EN` reader - TIM8 peripheral clock enable"]
pub type Tim8enR = crate::BitReader;
#[doc = "Field `TIM8EN` writer - TIM8 peripheral clock enable"]
pub type Tim8enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1EN` reader - USART1 Peripheral Clocks Enable"]
pub type Usart1enR = crate::BitReader;
#[doc = "Field `USART1EN` writer - USART1 Peripheral Clocks Enable"]
pub type Usart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6EN` reader - USART6 Peripheral Clocks Enable"]
pub type Usart6enR = crate::BitReader;
#[doc = "Field `USART6EN` writer - USART6 Peripheral Clocks Enable"]
pub type Usart6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1EN` reader - SPI1 Peripheral Clocks Enable"]
pub type Spi1enR = crate::BitReader;
#[doc = "Field `SPI1EN` writer - SPI1 Peripheral Clocks Enable"]
pub type Spi1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4EN` reader - SPI4 Peripheral Clocks Enable"]
pub type Spi4enR = crate::BitReader;
#[doc = "Field `SPI4EN` writer - SPI4 Peripheral Clocks Enable"]
pub type Spi4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15EN` reader - TIM15 peripheral clock enable"]
pub type Tim15enR = crate::BitReader;
#[doc = "Field `TIM15EN` writer - TIM15 peripheral clock enable"]
pub type Tim15enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16EN` reader - TIM16 peripheral clock enable"]
pub type Tim16enR = crate::BitReader;
#[doc = "Field `TIM16EN` writer - TIM16 peripheral clock enable"]
pub type Tim16enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17EN` reader - TIM17 peripheral clock enable"]
pub type Tim17enR = crate::BitReader;
#[doc = "Field `TIM17EN` writer - TIM17 peripheral clock enable"]
pub type Tim17enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5EN` reader - SPI5 Peripheral Clocks Enable"]
pub type Spi5enR = crate::BitReader;
#[doc = "Field `SPI5EN` writer - SPI5 Peripheral Clocks Enable"]
pub type Spi5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1EN` reader - SAI1 Peripheral Clocks Enable"]
pub type Sai1enR = crate::BitReader;
#[doc = "Field `SAI1EN` writer - SAI1 Peripheral Clocks Enable"]
pub type Sai1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2EN` reader - SAI2 Peripheral Clocks Enable"]
pub type Sai2enR = crate::BitReader;
#[doc = "Field `SAI2EN` writer - SAI2 Peripheral Clocks Enable"]
pub type Sai2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3EN` reader - SAI3 Peripheral Clocks Enable"]
pub type Sai3enR = crate::BitReader;
#[doc = "Field `SAI3EN` writer - SAI3 Peripheral Clocks Enable"]
pub type Sai3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDM1EN` reader - DFSDM1 Peripheral Clocks Enable"]
pub type Dfsdm1enR = crate::BitReader;
#[doc = "Field `DFSDM1EN` writer - DFSDM1 Peripheral Clocks Enable"]
pub type Dfsdm1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRTIMEN` reader - HRTIM peripheral clock enable"]
pub type HrtimenR = crate::BitReader;
#[doc = "Field `HRTIMEN` writer - HRTIM peripheral clock enable"]
pub type HrtimenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    pub fn tim1en(&self) -> Tim1enR {
        Tim1enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    pub fn tim8en(&self) -> Tim8enR {
        Tim8enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart1en(&self) -> Usart1enR {
        Usart1enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart6en(&self) -> Usart6enR {
        Usart6enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi1en(&self) -> Spi1enR {
        Spi1enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi4en(&self) -> Spi4enR {
        Spi4enR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    pub fn tim15en(&self) -> Tim15enR {
        Tim15enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    pub fn tim16en(&self) -> Tim16enR {
        Tim16enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    pub fn tim17en(&self) -> Tim17enR {
        Tim17enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi5en(&self) -> Spi5enR {
        Spi5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai1en(&self) -> Sai1enR {
        Sai1enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai2en(&self) -> Sai2enR {
        Sai2enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai3en(&self) -> Sai3enR {
        Sai3enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn dfsdm1en(&self) -> Dfsdm1enR {
        Dfsdm1enR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    pub fn hrtimen(&self) -> HrtimenR {
        HrtimenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 peripheral clock enable"]
    #[inline(always)]
    pub fn tim1en(&mut self) -> Tim1enW<C1Apb2enrSpec> {
        Tim1enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 peripheral clock enable"]
    #[inline(always)]
    pub fn tim8en(&mut self) -> Tim8enW<C1Apb2enrSpec> {
        Tim8enW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart1en(&mut self) -> Usart1enW<C1Apb2enrSpec> {
        Usart1enW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart6en(&mut self) -> Usart6enW<C1Apb2enrSpec> {
        Usart6enW::new(self, 5)
    }
    #[doc = "Bit 12 - SPI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi1en(&mut self) -> Spi1enW<C1Apb2enrSpec> {
        Spi1enW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi4en(&mut self) -> Spi4enW<C1Apb2enrSpec> {
        Spi4enW::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 peripheral clock enable"]
    #[inline(always)]
    pub fn tim15en(&mut self) -> Tim15enW<C1Apb2enrSpec> {
        Tim15enW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 peripheral clock enable"]
    #[inline(always)]
    pub fn tim16en(&mut self) -> Tim16enW<C1Apb2enrSpec> {
        Tim16enW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 peripheral clock enable"]
    #[inline(always)]
    pub fn tim17en(&mut self) -> Tim17enW<C1Apb2enrSpec> {
        Tim17enW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi5en(&mut self) -> Spi5enW<C1Apb2enrSpec> {
        Spi5enW::new(self, 20)
    }
    #[doc = "Bit 22 - SAI1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai1en(&mut self) -> Sai1enW<C1Apb2enrSpec> {
        Sai1enW::new(self, 22)
    }
    #[doc = "Bit 23 - SAI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai2en(&mut self) -> Sai2enW<C1Apb2enrSpec> {
        Sai2enW::new(self, 23)
    }
    #[doc = "Bit 24 - SAI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai3en(&mut self) -> Sai3enW<C1Apb2enrSpec> {
        Sai3enW::new(self, 24)
    }
    #[doc = "Bit 28 - DFSDM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn dfsdm1en(&mut self) -> Dfsdm1enW<C1Apb2enrSpec> {
        Dfsdm1enW::new(self, 28)
    }
    #[doc = "Bit 29 - HRTIM peripheral clock enable"]
    #[inline(always)]
    pub fn hrtimen(&mut self) -> HrtimenW<C1Apb2enrSpec> {
        HrtimenW::new(self, 29)
    }
}
#[doc = "RCC APB2 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb2enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb2enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Apb2enrSpec;
impl crate::RegisterSpec for C1Apb2enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb2enr::R`](R) reader structure"]
impl crate::Readable for C1Apb2enrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_apb2enr::W`](W) writer structure"]
impl crate::Writable for C1Apb2enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_APB2ENR to value 0"]
impl crate::Resettable for C1Apb2enrSpec {}
