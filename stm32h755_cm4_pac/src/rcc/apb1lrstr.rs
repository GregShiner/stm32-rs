#[doc = "Register `APB1LRSTR` reader"]
pub type R = crate::R<Apb1lrstrSpec>;
#[doc = "Register `APB1LRSTR` writer"]
pub type W = crate::W<Apb1lrstrSpec>;
#[doc = "Field `TIM2RST` reader - TIM block reset"]
pub type Tim2rstR = crate::BitReader;
#[doc = "Field `TIM2RST` writer - TIM block reset"]
pub type Tim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3RST` reader - TIM block reset"]
pub type Tim3rstR = crate::BitReader;
#[doc = "Field `TIM3RST` writer - TIM block reset"]
pub type Tim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4RST` reader - TIM block reset"]
pub type Tim4rstR = crate::BitReader;
#[doc = "Field `TIM4RST` writer - TIM block reset"]
pub type Tim4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5RST` reader - TIM block reset"]
pub type Tim5rstR = crate::BitReader;
#[doc = "Field `TIM5RST` writer - TIM block reset"]
pub type Tim5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6RST` reader - TIM block reset"]
pub type Tim6rstR = crate::BitReader;
#[doc = "Field `TIM6RST` writer - TIM block reset"]
pub type Tim6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7RST` reader - TIM block reset"]
pub type Tim7rstR = crate::BitReader;
#[doc = "Field `TIM7RST` writer - TIM block reset"]
pub type Tim7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12RST` reader - TIM block reset"]
pub type Tim12rstR = crate::BitReader;
#[doc = "Field `TIM12RST` writer - TIM block reset"]
pub type Tim12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13RST` reader - TIM block reset"]
pub type Tim13rstR = crate::BitReader;
#[doc = "Field `TIM13RST` writer - TIM block reset"]
pub type Tim13rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14RST` reader - TIM block reset"]
pub type Tim14rstR = crate::BitReader;
#[doc = "Field `TIM14RST` writer - TIM block reset"]
pub type Tim14rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1RST` reader - TIM block reset"]
pub type Lptim1rstR = crate::BitReader;
#[doc = "Field `LPTIM1RST` writer - TIM block reset"]
pub type Lptim1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2RST` reader - SPI2 block reset"]
pub type Spi2rstR = crate::BitReader;
#[doc = "Field `SPI2RST` writer - SPI2 block reset"]
pub type Spi2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3RST` reader - SPI3 block reset"]
pub type Spi3rstR = crate::BitReader;
#[doc = "Field `SPI3RST` writer - SPI3 block reset"]
pub type Spi3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPDIFRXRST` reader - SPDIFRX block reset"]
pub type SpdifrxrstR = crate::BitReader;
#[doc = "Field `SPDIFRXRST` writer - SPDIFRX block reset"]
pub type SpdifrxrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2RST` reader - USART2 block reset"]
pub type Usart2rstR = crate::BitReader;
#[doc = "Field `USART2RST` writer - USART2 block reset"]
pub type Usart2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3RST` reader - USART3 block reset"]
pub type Usart3rstR = crate::BitReader;
#[doc = "Field `USART3RST` writer - USART3 block reset"]
pub type Usart3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4RST` reader - UART4 block reset"]
pub type Uart4rstR = crate::BitReader;
#[doc = "Field `UART4RST` writer - UART4 block reset"]
pub type Uart4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5RST` reader - UART5 block reset"]
pub type Uart5rstR = crate::BitReader;
#[doc = "Field `UART5RST` writer - UART5 block reset"]
pub type Uart5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1RST` reader - I2C1 block reset"]
pub type I2c1rstR = crate::BitReader;
#[doc = "Field `I2C1RST` writer - I2C1 block reset"]
pub type I2c1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2RST` reader - I2C2 block reset"]
pub type I2c2rstR = crate::BitReader;
#[doc = "Field `I2C2RST` writer - I2C2 block reset"]
pub type I2c2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3RST` reader - I2C3 block reset"]
pub type I2c3rstR = crate::BitReader;
#[doc = "Field `I2C3RST` writer - I2C3 block reset"]
pub type I2c3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDMICECRST` reader - HDMI-CEC block reset"]
pub type HdmicecrstR = crate::BitReader;
#[doc = "Field `HDMICECRST` writer - HDMI-CEC block reset"]
pub type HdmicecrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12RST` reader - DAC1 and 2 Blocks Reset"]
pub type Dac12rstR = crate::BitReader;
#[doc = "Field `DAC12RST` writer - DAC1 and 2 Blocks Reset"]
pub type Dac12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART7RST` reader - USART7 block reset"]
pub type Usart7rstR = crate::BitReader;
#[doc = "Field `USART7RST` writer - USART7 block reset"]
pub type Usart7rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART8RST` reader - USART8 block reset"]
pub type Usart8rstR = crate::BitReader;
#[doc = "Field `USART8RST` writer - USART8 block reset"]
pub type Usart8rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM block reset"]
    #[inline(always)]
    pub fn tim2rst(&self) -> Tim2rstR {
        Tim2rstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM block reset"]
    #[inline(always)]
    pub fn tim3rst(&self) -> Tim3rstR {
        Tim3rstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM block reset"]
    #[inline(always)]
    pub fn tim4rst(&self) -> Tim4rstR {
        Tim4rstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM block reset"]
    #[inline(always)]
    pub fn tim5rst(&self) -> Tim5rstR {
        Tim5rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM block reset"]
    #[inline(always)]
    pub fn tim6rst(&self) -> Tim6rstR {
        Tim6rstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM block reset"]
    #[inline(always)]
    pub fn tim7rst(&self) -> Tim7rstR {
        Tim7rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM block reset"]
    #[inline(always)]
    pub fn tim12rst(&self) -> Tim12rstR {
        Tim12rstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM block reset"]
    #[inline(always)]
    pub fn tim13rst(&self) -> Tim13rstR {
        Tim13rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM block reset"]
    #[inline(always)]
    pub fn tim14rst(&self) -> Tim14rstR {
        Tim14rstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TIM block reset"]
    #[inline(always)]
    pub fn lptim1rst(&self) -> Lptim1rstR {
        Lptim1rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 block reset"]
    #[inline(always)]
    pub fn spi2rst(&self) -> Spi2rstR {
        Spi2rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 block reset"]
    #[inline(always)]
    pub fn spi3rst(&self) -> Spi3rstR {
        Spi3rstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX block reset"]
    #[inline(always)]
    pub fn spdifrxrst(&self) -> SpdifrxrstR {
        SpdifrxrstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 block reset"]
    #[inline(always)]
    pub fn usart2rst(&self) -> Usart2rstR {
        Usart2rstR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 block reset"]
    #[inline(always)]
    pub fn usart3rst(&self) -> Usart3rstR {
        Usart3rstR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 block reset"]
    #[inline(always)]
    pub fn uart4rst(&self) -> Uart4rstR {
        Uart4rstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 block reset"]
    #[inline(always)]
    pub fn uart5rst(&self) -> Uart5rstR {
        Uart5rstR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 block reset"]
    #[inline(always)]
    pub fn i2c1rst(&self) -> I2c1rstR {
        I2c1rstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 block reset"]
    #[inline(always)]
    pub fn i2c2rst(&self) -> I2c2rstR {
        I2c2rstR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 block reset"]
    #[inline(always)]
    pub fn i2c3rst(&self) -> I2c3rstR {
        I2c3rstR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC block reset"]
    #[inline(always)]
    pub fn hdmicecrst(&self) -> HdmicecrstR {
        HdmicecrstR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1 and 2 Blocks Reset"]
    #[inline(always)]
    pub fn dac12rst(&self) -> Dac12rstR {
        Dac12rstR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USART7 block reset"]
    #[inline(always)]
    pub fn usart7rst(&self) -> Usart7rstR {
        Usart7rstR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USART8 block reset"]
    #[inline(always)]
    pub fn usart8rst(&self) -> Usart8rstR {
        Usart8rstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM block reset"]
    #[inline(always)]
    pub fn tim2rst(&mut self) -> Tim2rstW<Apb1lrstrSpec> {
        Tim2rstW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM block reset"]
    #[inline(always)]
    pub fn tim3rst(&mut self) -> Tim3rstW<Apb1lrstrSpec> {
        Tim3rstW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM block reset"]
    #[inline(always)]
    pub fn tim4rst(&mut self) -> Tim4rstW<Apb1lrstrSpec> {
        Tim4rstW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM block reset"]
    #[inline(always)]
    pub fn tim5rst(&mut self) -> Tim5rstW<Apb1lrstrSpec> {
        Tim5rstW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM block reset"]
    #[inline(always)]
    pub fn tim6rst(&mut self) -> Tim6rstW<Apb1lrstrSpec> {
        Tim6rstW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM block reset"]
    #[inline(always)]
    pub fn tim7rst(&mut self) -> Tim7rstW<Apb1lrstrSpec> {
        Tim7rstW::new(self, 5)
    }
    #[doc = "Bit 6 - TIM block reset"]
    #[inline(always)]
    pub fn tim12rst(&mut self) -> Tim12rstW<Apb1lrstrSpec> {
        Tim12rstW::new(self, 6)
    }
    #[doc = "Bit 7 - TIM block reset"]
    #[inline(always)]
    pub fn tim13rst(&mut self) -> Tim13rstW<Apb1lrstrSpec> {
        Tim13rstW::new(self, 7)
    }
    #[doc = "Bit 8 - TIM block reset"]
    #[inline(always)]
    pub fn tim14rst(&mut self) -> Tim14rstW<Apb1lrstrSpec> {
        Tim14rstW::new(self, 8)
    }
    #[doc = "Bit 9 - TIM block reset"]
    #[inline(always)]
    pub fn lptim1rst(&mut self) -> Lptim1rstW<Apb1lrstrSpec> {
        Lptim1rstW::new(self, 9)
    }
    #[doc = "Bit 14 - SPI2 block reset"]
    #[inline(always)]
    pub fn spi2rst(&mut self) -> Spi2rstW<Apb1lrstrSpec> {
        Spi2rstW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 block reset"]
    #[inline(always)]
    pub fn spi3rst(&mut self) -> Spi3rstW<Apb1lrstrSpec> {
        Spi3rstW::new(self, 15)
    }
    #[doc = "Bit 16 - SPDIFRX block reset"]
    #[inline(always)]
    pub fn spdifrxrst(&mut self) -> SpdifrxrstW<Apb1lrstrSpec> {
        SpdifrxrstW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 block reset"]
    #[inline(always)]
    pub fn usart2rst(&mut self) -> Usart2rstW<Apb1lrstrSpec> {
        Usart2rstW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 block reset"]
    #[inline(always)]
    pub fn usart3rst(&mut self) -> Usart3rstW<Apb1lrstrSpec> {
        Usart3rstW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 block reset"]
    #[inline(always)]
    pub fn uart4rst(&mut self) -> Uart4rstW<Apb1lrstrSpec> {
        Uart4rstW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 block reset"]
    #[inline(always)]
    pub fn uart5rst(&mut self) -> Uart5rstW<Apb1lrstrSpec> {
        Uart5rstW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 block reset"]
    #[inline(always)]
    pub fn i2c1rst(&mut self) -> I2c1rstW<Apb1lrstrSpec> {
        I2c1rstW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 block reset"]
    #[inline(always)]
    pub fn i2c2rst(&mut self) -> I2c2rstW<Apb1lrstrSpec> {
        I2c2rstW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 block reset"]
    #[inline(always)]
    pub fn i2c3rst(&mut self) -> I2c3rstW<Apb1lrstrSpec> {
        I2c3rstW::new(self, 23)
    }
    #[doc = "Bit 27 - HDMI-CEC block reset"]
    #[inline(always)]
    pub fn hdmicecrst(&mut self) -> HdmicecrstW<Apb1lrstrSpec> {
        HdmicecrstW::new(self, 27)
    }
    #[doc = "Bit 29 - DAC1 and 2 Blocks Reset"]
    #[inline(always)]
    pub fn dac12rst(&mut self) -> Dac12rstW<Apb1lrstrSpec> {
        Dac12rstW::new(self, 29)
    }
    #[doc = "Bit 30 - USART7 block reset"]
    #[inline(always)]
    pub fn usart7rst(&mut self) -> Usart7rstW<Apb1lrstrSpec> {
        Usart7rstW::new(self, 30)
    }
    #[doc = "Bit 31 - USART8 block reset"]
    #[inline(always)]
    pub fn usart8rst(&mut self) -> Usart8rstW<Apb1lrstrSpec> {
        Usart8rstW::new(self, 31)
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1lrstrSpec;
impl crate::RegisterSpec for Apb1lrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lrstr::R`](R) reader structure"]
impl crate::Readable for Apb1lrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1lrstr::W`](W) writer structure"]
impl crate::Writable for Apb1lrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1LRSTR to value 0"]
impl crate::Resettable for Apb1lrstrSpec {}
