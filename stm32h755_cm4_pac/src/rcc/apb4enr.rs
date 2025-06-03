#[doc = "Register `APB4ENR` reader"]
pub type R = crate::R<Apb4enrSpec>;
#[doc = "Register `APB4ENR` writer"]
pub type W = crate::W<Apb4enrSpec>;
#[doc = "Field `SYSCFGEN` reader - SYSCFG peripheral clock enable"]
pub type SyscfgenR = crate::BitReader;
#[doc = "Field `SYSCFGEN` writer - SYSCFG peripheral clock enable"]
pub type SyscfgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1EN` reader - LPUART1 Peripheral Clocks Enable"]
pub type Lpuart1enR = crate::BitReader;
#[doc = "Field `LPUART1EN` writer - LPUART1 Peripheral Clocks Enable"]
pub type Lpuart1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6EN` reader - SPI6 Peripheral Clocks Enable"]
pub type Spi6enR = crate::BitReader;
#[doc = "Field `SPI6EN` writer - SPI6 Peripheral Clocks Enable"]
pub type Spi6enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4EN` reader - I2C4 Peripheral Clocks Enable"]
pub type I2c4enR = crate::BitReader;
#[doc = "Field `I2C4EN` writer - I2C4 Peripheral Clocks Enable"]
pub type I2c4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2EN` reader - LPTIM2 Peripheral Clocks Enable"]
pub type Lptim2enR = crate::BitReader;
#[doc = "Field `LPTIM2EN` writer - LPTIM2 Peripheral Clocks Enable"]
pub type Lptim2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3EN` reader - LPTIM3 Peripheral Clocks Enable"]
pub type Lptim3enR = crate::BitReader;
#[doc = "Field `LPTIM3EN` writer - LPTIM3 Peripheral Clocks Enable"]
pub type Lptim3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4EN` reader - LPTIM4 Peripheral Clocks Enable"]
pub type Lptim4enR = crate::BitReader;
#[doc = "Field `LPTIM4EN` writer - LPTIM4 Peripheral Clocks Enable"]
pub type Lptim4enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5EN` reader - LPTIM5 Peripheral Clocks Enable"]
pub type Lptim5enR = crate::BitReader;
#[doc = "Field `LPTIM5EN` writer - LPTIM5 Peripheral Clocks Enable"]
pub type Lptim5enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP12EN` reader - COMP1/2 peripheral clock enable"]
pub type Comp12enR = crate::BitReader;
#[doc = "Field `COMP12EN` writer - COMP1/2 peripheral clock enable"]
pub type Comp12enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFEN` reader - VREF peripheral clock enable"]
pub type VrefenR = crate::BitReader;
#[doc = "Field `VREFEN` writer - VREF peripheral clock enable"]
pub type VrefenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAPBEN` reader - RTC APB Clock Enable"]
pub type RtcapbenR = crate::BitReader;
#[doc = "Field `RTCAPBEN` writer - RTC APB Clock Enable"]
pub type RtcapbenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4EN` reader - SAI4 Peripheral Clocks Enable"]
pub type Sai4enR = crate::BitReader;
#[doc = "Field `SAI4EN` writer - SAI4 Peripheral Clocks Enable"]
pub type Sai4enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&self) -> SyscfgenR {
        SyscfgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&self) -> Lpuart1enR {
        Lpuart1enR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&self) -> Spi6enR {
        Spi6enR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&self) -> I2c4enR {
        I2c4enR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&self) -> Lptim2enR {
        Lptim2enR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&self) -> Lptim3enR {
        Lptim3enR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&self) -> Lptim4enR {
        Lptim4enR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&self) -> Lptim5enR {
        Lptim5enR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&self) -> Comp12enR {
        Comp12enR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VrefenR {
        VrefenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&self) -> RtcapbenR {
        RtcapbenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&self) -> Sai4enR {
        Sai4enR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG peripheral clock enable"]
    #[inline(always)]
    pub fn syscfgen(&mut self) -> SyscfgenW<Apb4enrSpec> {
        SyscfgenW::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lpuart1en(&mut self) -> Lpuart1enW<Apb4enrSpec> {
        Lpuart1enW::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn spi6en(&mut self) -> Spi6enW<Apb4enrSpec> {
        Spi6enW::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn i2c4en(&mut self) -> I2c4enW<Apb4enrSpec> {
        I2c4enW::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim2en(&mut self) -> Lptim2enW<Apb4enrSpec> {
        Lptim2enW::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim3en(&mut self) -> Lptim3enW<Apb4enrSpec> {
        Lptim3enW::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim4en(&mut self) -> Lptim4enW<Apb4enrSpec> {
        Lptim4enW::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn lptim5en(&mut self) -> Lptim5enW<Apb4enrSpec> {
        Lptim5enW::new(self, 12)
    }
    #[doc = "Bit 14 - COMP1/2 peripheral clock enable"]
    #[inline(always)]
    pub fn comp12en(&mut self) -> Comp12enW<Apb4enrSpec> {
        Comp12enW::new(self, 14)
    }
    #[doc = "Bit 15 - VREF peripheral clock enable"]
    #[inline(always)]
    pub fn vrefen(&mut self) -> VrefenW<Apb4enrSpec> {
        VrefenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC APB Clock Enable"]
    #[inline(always)]
    pub fn rtcapben(&mut self) -> RtcapbenW<Apb4enrSpec> {
        RtcapbenW::new(self, 16)
    }
    #[doc = "Bit 21 - SAI4 Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn sai4en(&mut self) -> Sai4enW<Apb4enrSpec> {
        Sai4enW::new(self, 21)
    }
}
#[doc = "RCC APB4 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb4enrSpec;
impl crate::RegisterSpec for Apb4enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4enr::R`](R) reader structure"]
impl crate::Readable for Apb4enrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb4enr::W`](W) writer structure"]
impl crate::Writable for Apb4enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB4ENR to value 0"]
impl crate::Resettable for Apb4enrSpec {}
