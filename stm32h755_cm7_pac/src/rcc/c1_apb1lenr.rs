#[doc = "Register `C1_APB1LENR` reader"]
pub type R = crate::R<C1Apb1lenrSpec>;
#[doc = "Register `C1_APB1LENR` writer"]
pub type W = crate::W<C1Apb1lenrSpec>;
#[doc = "Field `TIM2EN` reader - TIM peripheral clock enable"]
pub type Tim2enR = crate::BitReader;
#[doc = "Field `TIM2EN` writer - TIM peripheral clock enable"]
pub type Tim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM3EN` reader - TIM peripheral clock enable"]
pub type Tim3enR = crate::BitReader;
#[doc = "Field `TIM3EN` writer - TIM peripheral clock enable"]
pub type Tim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM4EN` reader - TIM peripheral clock enable"]
pub type Tim4enR = crate::BitReader;
#[doc = "Field `TIM4EN` writer - TIM peripheral clock enable"]
pub type Tim4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM5EN` reader - TIM peripheral clock enable"]
pub type Tim5enR = crate::BitReader;
#[doc = "Field `TIM5EN` writer - TIM peripheral clock enable"]
pub type Tim5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM6EN` reader - TIM peripheral clock enable"]
pub type Tim6enR = crate::BitReader;
#[doc = "Field `TIM6EN` writer - TIM peripheral clock enable"]
pub type Tim6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM7EN` reader - TIM peripheral clock enable"]
pub type Tim7enR = crate::BitReader;
#[doc = "Field `TIM7EN` writer - TIM peripheral clock enable"]
pub type Tim7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM12EN` reader - TIM peripheral clock enable"]
pub type Tim12enR = crate::BitReader;
#[doc = "Field `TIM12EN` writer - TIM peripheral clock enable"]
pub type Tim12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM13EN` reader - TIM peripheral clock enable"]
pub type Tim13enR = crate::BitReader;
#[doc = "Field `TIM13EN` writer - TIM peripheral clock enable"]
pub type Tim13enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIM14EN` reader - TIM peripheral clock enable"]
pub type Tim14enR = crate::BitReader;
#[doc = "Field `TIM14EN` writer - TIM peripheral clock enable"]
pub type Tim14enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM1EN` reader - LPTIM1 Peripheral Clocks Enable"]
pub type Lptim1enR = crate::BitReader;
#[doc = "Field `LPTIM1EN` writer - LPTIM1 Peripheral Clocks Enable"]
pub type Lptim1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI2EN` reader - SPI2 Peripheral Clocks Enable"]
pub type Spi2enR = crate::BitReader;
#[doc = "Field `SPI2EN` writer - SPI2 Peripheral Clocks Enable"]
pub type Spi2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI3EN` reader - SPI3 Peripheral Clocks Enable"]
pub type Spi3enR = crate::BitReader;
#[doc = "Field `SPI3EN` writer - SPI3 Peripheral Clocks Enable"]
pub type Spi3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPDIFRXEN` reader - SPDIFRX Peripheral Clocks Enable"]
pub type SpdifrxenR = crate::BitReader;
#[doc = "Field `SPDIFRXEN` writer - SPDIFRX Peripheral Clocks Enable"]
pub type SpdifrxenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART2EN` reader - USART2 Peripheral Clocks Enable"]
pub type Usart2enR = crate::BitReader;
#[doc = "Field `USART2EN` writer - USART2 Peripheral Clocks Enable"]
pub type Usart2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART3EN` reader - USART3 Peripheral Clocks Enable"]
pub type Usart3enR = crate::BitReader;
#[doc = "Field `USART3EN` writer - USART3 Peripheral Clocks Enable"]
pub type Usart3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART4EN` reader - UART4 Peripheral Clocks Enable"]
pub type Uart4enR = crate::BitReader;
#[doc = "Field `UART4EN` writer - UART4 Peripheral Clocks Enable"]
pub type Uart4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UART5EN` reader - UART5 Peripheral Clocks Enable"]
pub type Uart5enR = crate::BitReader;
#[doc = "Field `UART5EN` writer - UART5 Peripheral Clocks Enable"]
pub type Uart5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C1EN` reader - I2C1 Peripheral Clocks Enable"]
pub type I2c1enR = crate::BitReader;
#[doc = "Field `I2C1EN` writer - I2C1 Peripheral Clocks Enable"]
pub type I2c1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C2EN` reader - I2C2 Peripheral Clocks Enable"]
pub type I2c2enR = crate::BitReader;
#[doc = "Field `I2C2EN` writer - I2C2 Peripheral Clocks Enable"]
pub type I2c2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C3EN` reader - I2C3 Peripheral Clocks Enable"]
pub type I2c3enR = crate::BitReader;
#[doc = "Field `I2C3EN` writer - I2C3 Peripheral Clocks Enable"]
pub type I2c3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HDMICECEN` reader - HDMI-CEC peripheral clock enable"]
pub type HdmicecenR = crate::BitReader;
#[doc = "Field `HDMICECEN` writer - HDMI-CEC peripheral clock enable"]
pub type HdmicecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAC12EN` reader - DAC1&2 peripheral clock enable"]
pub type Dac12enR = crate::BitReader;
#[doc = "Field `DAC12EN` writer - DAC1&2 peripheral clock enable"]
pub type Dac12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART7EN` reader - USART7 Peripheral Clocks Enable"]
pub type Usart7enR = crate::BitReader;
#[doc = "Field `USART7EN` writer - USART7 Peripheral Clocks Enable"]
pub type Usart7enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USART8EN` reader - USART8 Peripheral Clocks Enable"]
pub type Usart8enR = crate::BitReader;
#[doc = "Field `USART8EN` writer - USART8 Peripheral Clocks Enable"]
pub type Usart8enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim2en(&self) -> Tim2enR {
        Tim2enR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim3en(&self) -> Tim3enR {
        Tim3enR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim4en(&self) -> Tim4enR {
        Tim4enR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim5en(&self) -> Tim5enR {
        Tim5enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim6en(&self) -> Tim6enR {
        Tim6enR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim7en(&self) -> Tim7enR {
        Tim7enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim12en(&self) -> Tim12enR {
        Tim12enR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim13en(&self) -> Tim13enR {
        Tim13enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim14en(&self) -> Tim14enR {
        Tim14enR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim1en(&self) -> Lptim1enR {
        Lptim1enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi2en(&self) -> Spi2enR {
        Spi2enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi3en(&self) -> Spi3enR {
        Spi3enR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spdifrxen(&self) -> SpdifrxenR {
        SpdifrxenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart2en(&self) -> Usart2enR {
        Usart2enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart3en(&self) -> Usart3enR {
        Usart3enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart4en(&self) -> Uart4enR {
        Uart4enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart5en(&self) -> Uart5enR {
        Uart5enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c1en(&self) -> I2c1enR {
        I2c1enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c2en(&self) -> I2c2enR {
        I2c2enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c3en(&self) -> I2c3enR {
        I2c3enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    pub fn hdmicecen(&self) -> HdmicecenR {
        HdmicecenR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 29 - DAC1&2 peripheral clock enable"]
    #[inline(always)]
    pub fn dac12en(&self) -> Dac12enR {
        Dac12enR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - USART7 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart7en(&self) -> Usart7enR {
        Usart7enR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - USART8 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart8en(&self) -> Usart8enR {
        Usart8enR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim2en(&mut self) -> Tim2enW<C1Apb1lenrSpec> {
        Tim2enW::new(self, 0)
    }
    #[doc = "Bit 1 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim3en(&mut self) -> Tim3enW<C1Apb1lenrSpec> {
        Tim3enW::new(self, 1)
    }
    #[doc = "Bit 2 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim4en(&mut self) -> Tim4enW<C1Apb1lenrSpec> {
        Tim4enW::new(self, 2)
    }
    #[doc = "Bit 3 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim5en(&mut self) -> Tim5enW<C1Apb1lenrSpec> {
        Tim5enW::new(self, 3)
    }
    #[doc = "Bit 4 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim6en(&mut self) -> Tim6enW<C1Apb1lenrSpec> {
        Tim6enW::new(self, 4)
    }
    #[doc = "Bit 5 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim7en(&mut self) -> Tim7enW<C1Apb1lenrSpec> {
        Tim7enW::new(self, 5)
    }
    #[doc = "Bit 6 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim12en(&mut self) -> Tim12enW<C1Apb1lenrSpec> {
        Tim12enW::new(self, 6)
    }
    #[doc = "Bit 7 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim13en(&mut self) -> Tim13enW<C1Apb1lenrSpec> {
        Tim13enW::new(self, 7)
    }
    #[doc = "Bit 8 - TIM peripheral clock enable"]
    #[inline(always)]
    pub fn tim14en(&mut self) -> Tim14enW<C1Apb1lenrSpec> {
        Tim14enW::new(self, 8)
    }
    #[doc = "Bit 9 - LPTIM1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim1en(&mut self) -> Lptim1enW<C1Apb1lenrSpec> {
        Lptim1enW::new(self, 9)
    }
    #[doc = "Bit 14 - SPI2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi2en(&mut self) -> Spi2enW<C1Apb1lenrSpec> {
        Spi2enW::new(self, 14)
    }
    #[doc = "Bit 15 - SPI3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi3en(&mut self) -> Spi3enW<C1Apb1lenrSpec> {
        Spi3enW::new(self, 15)
    }
    #[doc = "Bit 16 - SPDIFRX Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spdifrxen(&mut self) -> SpdifrxenW<C1Apb1lenrSpec> {
        SpdifrxenW::new(self, 16)
    }
    #[doc = "Bit 17 - USART2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart2en(&mut self) -> Usart2enW<C1Apb1lenrSpec> {
        Usart2enW::new(self, 17)
    }
    #[doc = "Bit 18 - USART3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart3en(&mut self) -> Usart3enW<C1Apb1lenrSpec> {
        Usart3enW::new(self, 18)
    }
    #[doc = "Bit 19 - UART4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart4en(&mut self) -> Uart4enW<C1Apb1lenrSpec> {
        Uart4enW::new(self, 19)
    }
    #[doc = "Bit 20 - UART5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn uart5en(&mut self) -> Uart5enW<C1Apb1lenrSpec> {
        Uart5enW::new(self, 20)
    }
    #[doc = "Bit 21 - I2C1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c1en(&mut self) -> I2c1enW<C1Apb1lenrSpec> {
        I2c1enW::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c2en(&mut self) -> I2c2enW<C1Apb1lenrSpec> {
        I2c2enW::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c3en(&mut self) -> I2c3enW<C1Apb1lenrSpec> {
        I2c3enW::new(self, 23)
    }
    #[doc = "Bit 27 - HDMI-CEC peripheral clock enable"]
    #[inline(always)]
    pub fn hdmicecen(&mut self) -> HdmicecenW<C1Apb1lenrSpec> {
        HdmicecenW::new(self, 27)
    }
    #[doc = "Bit 29 - DAC1&2 peripheral clock enable"]
    #[inline(always)]
    pub fn dac12en(&mut self) -> Dac12enW<C1Apb1lenrSpec> {
        Dac12enW::new(self, 29)
    }
    #[doc = "Bit 30 - USART7 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart7en(&mut self) -> Usart7enW<C1Apb1lenrSpec> {
        Usart7enW::new(self, 30)
    }
    #[doc = "Bit 31 - USART8 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn usart8en(&mut self) -> Usart8enW<C1Apb1lenrSpec> {
        Usart8enW::new(self, 31)
    }
}
#[doc = "RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_apb1lenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_apb1lenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Apb1lenrSpec;
impl crate::RegisterSpec for C1Apb1lenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_apb1lenr::R`](R) reader structure"]
impl crate::Readable for C1Apb1lenrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_apb1lenr::W`](W) writer structure"]
impl crate::Writable for C1Apb1lenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_APB1LENR to value 0"]
impl crate::Resettable for C1Apb1lenrSpec {}
