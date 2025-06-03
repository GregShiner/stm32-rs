#[doc = "Register `AHB4LPENR` reader"]
pub type R = crate::R<Ahb4lpenrSpec>;
#[doc = "Register `AHB4LPENR` writer"]
pub type W = crate::W<Ahb4lpenrSpec>;
#[doc = "Field `GPIOALPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioalpenR = crate::BitReader;
#[doc = "Field `GPIOALPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioblpenR = crate::BitReader;
#[doc = "Field `GPIOBLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioblpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioclpenR = crate::BitReader;
#[doc = "Field `GPIOCLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpiodlpenR = crate::BitReader;
#[doc = "Field `GPIODLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpiodlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOELPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioelpenR = crate::BitReader;
#[doc = "Field `GPIOELPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioelpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioflpenR = crate::BitReader;
#[doc = "Field `GPIOFLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioglpenR = crate::BitReader;
#[doc = "Field `GPIOGLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioglpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpiohlpenR = crate::BitReader;
#[doc = "Field `GPIOHLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpiohlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOILPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioilpenR = crate::BitReader;
#[doc = "Field `GPIOILPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioilpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpiojlpenR = crate::BitReader;
#[doc = "Field `GPIOJLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpiojlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKLPEN` reader - GPIO peripheral clock enable during CSleep mode"]
pub type GpioklpenR = crate::BitReader;
#[doc = "Field `GPIOKLPEN` writer - GPIO peripheral clock enable during CSleep mode"]
pub type GpioklpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCLPEN` reader - CRC peripheral clock enable during CSleep mode"]
pub type CrclpenR = crate::BitReader;
#[doc = "Field `CRCLPEN` writer - CRC peripheral clock enable during CSleep mode"]
pub type CrclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDMALPEN` reader - BDMA Clock Enable During CSleep Mode"]
pub type BdmalpenR = crate::BitReader;
#[doc = "Field `BDMALPEN` writer - BDMA Clock Enable During CSleep Mode"]
pub type BdmalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3LPEN` reader - ADC3 Peripheral Clocks Enable During CSleep Mode"]
pub type Adc3lpenR = crate::BitReader;
#[doc = "Field `ADC3LPEN` writer - ADC3 Peripheral Clocks Enable During CSleep Mode"]
pub type Adc3lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPRAMLPEN` reader - Backup RAM Clock Enable During CSleep Mode"]
pub type BkpramlpenR = crate::BitReader;
#[doc = "Field `BKPRAMLPEN` writer - Backup RAM Clock Enable During CSleep Mode"]
pub type BkpramlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4LPEN` reader - SRAM4 Clock Enable During CSleep Mode"]
pub type Sram4lpenR = crate::BitReader;
#[doc = "Field `SRAM4LPEN` writer - SRAM4 Clock Enable During CSleep Mode"]
pub type Sram4lpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&self) -> GpioalpenR {
        GpioalpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&self) -> GpioblpenR {
        GpioblpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&self) -> GpioclpenR {
        GpioclpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&self) -> GpiodlpenR {
        GpiodlpenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&self) -> GpioelpenR {
        GpioelpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&self) -> GpioflpenR {
        GpioflpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&self) -> GpioglpenR {
        GpioglpenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&self) -> GpiohlpenR {
        GpiohlpenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&self) -> GpioilpenR {
        GpioilpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&self) -> GpiojlpenR {
        GpiojlpenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&self) -> GpioklpenR {
        GpioklpenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crclpen(&self) -> CrclpenR {
        CrclpenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bdmalpen(&self) -> BdmalpenR {
        BdmalpenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc3lpen(&self) -> Adc3lpenR {
        Adc3lpenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bkpramlpen(&self) -> BkpramlpenR {
        BkpramlpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram4lpen(&self) -> Sram4lpenR {
        Sram4lpenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioalpen(&mut self) -> GpioalpenW<Ahb4lpenrSpec> {
        GpioalpenW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioblpen(&mut self) -> GpioblpenW<Ahb4lpenrSpec> {
        GpioblpenW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioclpen(&mut self) -> GpioclpenW<Ahb4lpenrSpec> {
        GpioclpenW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiodlpen(&mut self) -> GpiodlpenW<Ahb4lpenrSpec> {
        GpiodlpenW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioelpen(&mut self) -> GpioelpenW<Ahb4lpenrSpec> {
        GpioelpenW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioflpen(&mut self) -> GpioflpenW<Ahb4lpenrSpec> {
        GpioflpenW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioglpen(&mut self) -> GpioglpenW<Ahb4lpenrSpec> {
        GpioglpenW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiohlpen(&mut self) -> GpiohlpenW<Ahb4lpenrSpec> {
        GpiohlpenW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioilpen(&mut self) -> GpioilpenW<Ahb4lpenrSpec> {
        GpioilpenW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpiojlpen(&mut self) -> GpiojlpenW<Ahb4lpenrSpec> {
        GpiojlpenW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn gpioklpen(&mut self) -> GpioklpenW<Ahb4lpenrSpec> {
        GpioklpenW::new(self, 10)
    }
    #[doc = "Bit 19 - CRC peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crclpen(&mut self) -> CrclpenW<Ahb4lpenrSpec> {
        CrclpenW::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bdmalpen(&mut self) -> BdmalpenW<Ahb4lpenrSpec> {
        BdmalpenW::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn adc3lpen(&mut self) -> Adc3lpenW<Ahb4lpenrSpec> {
        Adc3lpenW::new(self, 24)
    }
    #[doc = "Bit 28 - Backup RAM Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn bkpramlpen(&mut self) -> BkpramlpenW<Ahb4lpenrSpec> {
        BkpramlpenW::new(self, 28)
    }
    #[doc = "Bit 29 - SRAM4 Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sram4lpen(&mut self) -> Sram4lpenW<Ahb4lpenrSpec> {
        Sram4lpenW::new(self, 29)
    }
}
#[doc = "RCC AHB4 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb4lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb4lpenrSpec;
impl crate::RegisterSpec for Ahb4lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4lpenr::R`](R) reader structure"]
impl crate::Readable for Ahb4lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb4lpenr::W`](W) writer structure"]
impl crate::Writable for Ahb4lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB4LPENR to value 0"]
impl crate::Resettable for Ahb4lpenrSpec {}
