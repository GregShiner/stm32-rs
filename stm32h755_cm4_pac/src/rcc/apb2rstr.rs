#[doc = "Register `APB2RSTR` reader"]
pub type R = crate::R<Apb2rstrSpec>;
#[doc = "Register `APB2RSTR` writer"]
pub type W = crate::W<Apb2rstrSpec>;
#[doc = "Field `TIM1RST` reader - TIM1 block reset"]
pub type Tim1rstR = crate::BitReader;
#[doc = "Field `TIM1RST` writer - TIM1 block reset"]
pub type Tim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM8RST` reader - TIM8 block reset"]
pub type Tim8rstR = crate::BitReader;
#[doc = "Field `TIM8RST` writer - TIM8 block reset"]
pub type Tim8rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART1RST` reader - USART1 block reset"]
pub type Usart1rstR = crate::BitReader;
#[doc = "Field `USART1RST` writer - USART1 block reset"]
pub type Usart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART6RST` reader - USART6 block reset"]
pub type Usart6rstR = crate::BitReader;
#[doc = "Field `USART6RST` writer - USART6 block reset"]
pub type Usart6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI1RST` reader - SPI1 block reset"]
pub type Spi1rstR = crate::BitReader;
#[doc = "Field `SPI1RST` writer - SPI1 block reset"]
pub type Spi1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI4RST` reader - SPI4 block reset"]
pub type Spi4rstR = crate::BitReader;
#[doc = "Field `SPI4RST` writer - SPI4 block reset"]
pub type Spi4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM15RST` reader - TIM15 block reset"]
pub type Tim15rstR = crate::BitReader;
#[doc = "Field `TIM15RST` writer - TIM15 block reset"]
pub type Tim15rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM16RST` reader - TIM16 block reset"]
pub type Tim16rstR = crate::BitReader;
#[doc = "Field `TIM16RST` writer - TIM16 block reset"]
pub type Tim16rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM17RST` reader - TIM17 block reset"]
pub type Tim17rstR = crate::BitReader;
#[doc = "Field `TIM17RST` writer - TIM17 block reset"]
pub type Tim17rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI5RST` reader - SPI5 block reset"]
pub type Spi5rstR = crate::BitReader;
#[doc = "Field `SPI5RST` writer - SPI5 block reset"]
pub type Spi5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI1RST` reader - SAI1 block reset"]
pub type Sai1rstR = crate::BitReader;
#[doc = "Field `SAI1RST` writer - SAI1 block reset"]
pub type Sai1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI2RST` reader - SAI2 block reset"]
pub type Sai2rstR = crate::BitReader;
#[doc = "Field `SAI2RST` writer - SAI2 block reset"]
pub type Sai2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI3RST` reader - SAI3 block reset"]
pub type Sai3rstR = crate::BitReader;
#[doc = "Field `SAI3RST` writer - SAI3 block reset"]
pub type Sai3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFSDM1RST` reader - DFSDM1 block reset"]
pub type Dfsdm1rstR = crate::BitReader;
#[doc = "Field `DFSDM1RST` writer - DFSDM1 block reset"]
pub type Dfsdm1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HRTIMRST` reader - HRTIM block reset"]
pub type HrtimrstR = crate::BitReader;
#[doc = "Field `HRTIMRST` writer - HRTIM block reset"]
pub type HrtimrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    pub fn tim1rst(&self) -> Tim1rstR {
        Tim1rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    pub fn tim8rst(&self) -> Tim8rstR {
        Tim8rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    pub fn usart1rst(&self) -> Usart1rstR {
        Usart1rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    pub fn usart6rst(&self) -> Usart6rstR {
        Usart6rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    pub fn spi1rst(&self) -> Spi1rstR {
        Spi1rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    pub fn spi4rst(&self) -> Spi4rstR {
        Spi4rstR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    pub fn tim15rst(&self) -> Tim15rstR {
        Tim15rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    pub fn tim16rst(&self) -> Tim16rstR {
        Tim16rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    pub fn tim17rst(&self) -> Tim17rstR {
        Tim17rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    pub fn spi5rst(&self) -> Spi5rstR {
        Spi5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    pub fn sai1rst(&self) -> Sai1rstR {
        Sai1rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    pub fn sai2rst(&self) -> Sai2rstR {
        Sai2rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    pub fn sai3rst(&self) -> Sai3rstR {
        Sai3rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&self) -> Dfsdm1rstR {
        Dfsdm1rstR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM block reset"]
    #[inline(always)]
    pub fn hrtimrst(&self) -> HrtimrstR {
        HrtimrstR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 block reset"]
    #[inline(always)]
    pub fn tim1rst(&mut self) -> Tim1rstW<Apb2rstrSpec> {
        Tim1rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 block reset"]
    #[inline(always)]
    pub fn tim8rst(&mut self) -> Tim8rstW<Apb2rstrSpec> {
        Tim8rstW::new(self, 1)
    }
    #[doc = "Bit 4 - USART1 block reset"]
    #[inline(always)]
    pub fn usart1rst(&mut self) -> Usart1rstW<Apb2rstrSpec> {
        Usart1rstW::new(self, 4)
    }
    #[doc = "Bit 5 - USART6 block reset"]
    #[inline(always)]
    pub fn usart6rst(&mut self) -> Usart6rstW<Apb2rstrSpec> {
        Usart6rstW::new(self, 5)
    }
    #[doc = "Bit 12 - SPI1 block reset"]
    #[inline(always)]
    pub fn spi1rst(&mut self) -> Spi1rstW<Apb2rstrSpec> {
        Spi1rstW::new(self, 12)
    }
    #[doc = "Bit 13 - SPI4 block reset"]
    #[inline(always)]
    pub fn spi4rst(&mut self) -> Spi4rstW<Apb2rstrSpec> {
        Spi4rstW::new(self, 13)
    }
    #[doc = "Bit 16 - TIM15 block reset"]
    #[inline(always)]
    pub fn tim15rst(&mut self) -> Tim15rstW<Apb2rstrSpec> {
        Tim15rstW::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 block reset"]
    #[inline(always)]
    pub fn tim16rst(&mut self) -> Tim16rstW<Apb2rstrSpec> {
        Tim16rstW::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 block reset"]
    #[inline(always)]
    pub fn tim17rst(&mut self) -> Tim17rstW<Apb2rstrSpec> {
        Tim17rstW::new(self, 18)
    }
    #[doc = "Bit 20 - SPI5 block reset"]
    #[inline(always)]
    pub fn spi5rst(&mut self) -> Spi5rstW<Apb2rstrSpec> {
        Spi5rstW::new(self, 20)
    }
    #[doc = "Bit 22 - SAI1 block reset"]
    #[inline(always)]
    pub fn sai1rst(&mut self) -> Sai1rstW<Apb2rstrSpec> {
        Sai1rstW::new(self, 22)
    }
    #[doc = "Bit 23 - SAI2 block reset"]
    #[inline(always)]
    pub fn sai2rst(&mut self) -> Sai2rstW<Apb2rstrSpec> {
        Sai2rstW::new(self, 23)
    }
    #[doc = "Bit 24 - SAI3 block reset"]
    #[inline(always)]
    pub fn sai3rst(&mut self) -> Sai3rstW<Apb2rstrSpec> {
        Sai3rstW::new(self, 24)
    }
    #[doc = "Bit 28 - DFSDM1 block reset"]
    #[inline(always)]
    pub fn dfsdm1rst(&mut self) -> Dfsdm1rstW<Apb2rstrSpec> {
        Dfsdm1rstW::new(self, 28)
    }
    #[doc = "Bit 29 - HRTIM block reset"]
    #[inline(always)]
    pub fn hrtimrst(&mut self) -> HrtimrstW<Apb2rstrSpec> {
        HrtimrstW::new(self, 29)
    }
}
#[doc = "RCC APB2 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2rstrSpec;
impl crate::RegisterSpec for Apb2rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2rstr::R`](R) reader structure"]
impl crate::Readable for Apb2rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb2rstr::W`](W) writer structure"]
impl crate::Writable for Apb2rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2RSTR to value 0"]
impl crate::Resettable for Apb2rstrSpec {}
