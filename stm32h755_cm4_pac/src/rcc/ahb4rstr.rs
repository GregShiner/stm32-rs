#[doc = "Register `AHB4RSTR` reader"]
pub type R = crate::R<Ahb4rstrSpec>;
#[doc = "Register `AHB4RSTR` writer"]
pub type W = crate::W<Ahb4rstrSpec>;
#[doc = "Field `GPIOARST` reader - GPIO block reset"]
pub type GpioarstR = crate::BitReader;
#[doc = "Field `GPIOARST` writer - GPIO block reset"]
pub type GpioarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOBRST` reader - GPIO block reset"]
pub type GpiobrstR = crate::BitReader;
#[doc = "Field `GPIOBRST` writer - GPIO block reset"]
pub type GpiobrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOCRST` reader - GPIO block reset"]
pub type GpiocrstR = crate::BitReader;
#[doc = "Field `GPIOCRST` writer - GPIO block reset"]
pub type GpiocrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIODRST` reader - GPIO block reset"]
pub type GpiodrstR = crate::BitReader;
#[doc = "Field `GPIODRST` writer - GPIO block reset"]
pub type GpiodrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOERST` reader - GPIO block reset"]
pub type GpioerstR = crate::BitReader;
#[doc = "Field `GPIOERST` writer - GPIO block reset"]
pub type GpioerstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOFRST` reader - GPIO block reset"]
pub type GpiofrstR = crate::BitReader;
#[doc = "Field `GPIOFRST` writer - GPIO block reset"]
pub type GpiofrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOGRST` reader - GPIO block reset"]
pub type GpiogrstR = crate::BitReader;
#[doc = "Field `GPIOGRST` writer - GPIO block reset"]
pub type GpiogrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOHRST` reader - GPIO block reset"]
pub type GpiohrstR = crate::BitReader;
#[doc = "Field `GPIOHRST` writer - GPIO block reset"]
pub type GpiohrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOIRST` reader - GPIO block reset"]
pub type GpioirstR = crate::BitReader;
#[doc = "Field `GPIOIRST` writer - GPIO block reset"]
pub type GpioirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOJRST` reader - GPIO block reset"]
pub type GpiojrstR = crate::BitReader;
#[doc = "Field `GPIOJRST` writer - GPIO block reset"]
pub type GpiojrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPIOKRST` reader - GPIO block reset"]
pub type GpiokrstR = crate::BitReader;
#[doc = "Field `GPIOKRST` writer - GPIO block reset"]
pub type GpiokrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCRST` reader - CRC block reset"]
pub type CrcrstR = crate::BitReader;
#[doc = "Field `CRCRST` writer - CRC block reset"]
pub type CrcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BDMARST` reader - BDMA block reset"]
pub type BdmarstR = crate::BitReader;
#[doc = "Field `BDMARST` writer - BDMA block reset"]
pub type BdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3RST` reader - ADC3 block reset"]
pub type Adc3rstR = crate::BitReader;
#[doc = "Field `ADC3RST` writer - ADC3 block reset"]
pub type Adc3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEMRST` reader - HSEM block reset"]
pub type HsemrstR = crate::BitReader;
#[doc = "Field `HSEMRST` writer - HSEM block reset"]
pub type HsemrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioarst(&self) -> GpioarstR {
        GpioarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiobrst(&self) -> GpiobrstR {
        GpiobrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiocrst(&self) -> GpiocrstR {
        GpiocrstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiodrst(&self) -> GpiodrstR {
        GpiodrstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioerst(&self) -> GpioerstR {
        GpioerstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiofrst(&self) -> GpiofrstR {
        GpiofrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiogrst(&self) -> GpiogrstR {
        GpiogrstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiohrst(&self) -> GpiohrstR {
        GpiohrstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioirst(&self) -> GpioirstR {
        GpioirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiojrst(&self) -> GpiojrstR {
        GpiojrstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiokrst(&self) -> GpiokrstR {
        GpiokrstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    pub fn crcrst(&self) -> CrcrstR {
        CrcrstR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    pub fn bdmarst(&self) -> BdmarstR {
        BdmarstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    pub fn adc3rst(&self) -> Adc3rstR {
        Adc3rstR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    pub fn hsemrst(&self) -> HsemrstR {
        HsemrstR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioarst(&mut self) -> GpioarstW<Ahb4rstrSpec> {
        GpioarstW::new(self, 0)
    }
    #[doc = "Bit 1 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiobrst(&mut self) -> GpiobrstW<Ahb4rstrSpec> {
        GpiobrstW::new(self, 1)
    }
    #[doc = "Bit 2 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiocrst(&mut self) -> GpiocrstW<Ahb4rstrSpec> {
        GpiocrstW::new(self, 2)
    }
    #[doc = "Bit 3 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiodrst(&mut self) -> GpiodrstW<Ahb4rstrSpec> {
        GpiodrstW::new(self, 3)
    }
    #[doc = "Bit 4 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioerst(&mut self) -> GpioerstW<Ahb4rstrSpec> {
        GpioerstW::new(self, 4)
    }
    #[doc = "Bit 5 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiofrst(&mut self) -> GpiofrstW<Ahb4rstrSpec> {
        GpiofrstW::new(self, 5)
    }
    #[doc = "Bit 6 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiogrst(&mut self) -> GpiogrstW<Ahb4rstrSpec> {
        GpiogrstW::new(self, 6)
    }
    #[doc = "Bit 7 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiohrst(&mut self) -> GpiohrstW<Ahb4rstrSpec> {
        GpiohrstW::new(self, 7)
    }
    #[doc = "Bit 8 - GPIO block reset"]
    #[inline(always)]
    pub fn gpioirst(&mut self) -> GpioirstW<Ahb4rstrSpec> {
        GpioirstW::new(self, 8)
    }
    #[doc = "Bit 9 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiojrst(&mut self) -> GpiojrstW<Ahb4rstrSpec> {
        GpiojrstW::new(self, 9)
    }
    #[doc = "Bit 10 - GPIO block reset"]
    #[inline(always)]
    pub fn gpiokrst(&mut self) -> GpiokrstW<Ahb4rstrSpec> {
        GpiokrstW::new(self, 10)
    }
    #[doc = "Bit 19 - CRC block reset"]
    #[inline(always)]
    pub fn crcrst(&mut self) -> CrcrstW<Ahb4rstrSpec> {
        CrcrstW::new(self, 19)
    }
    #[doc = "Bit 21 - BDMA block reset"]
    #[inline(always)]
    pub fn bdmarst(&mut self) -> BdmarstW<Ahb4rstrSpec> {
        BdmarstW::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 block reset"]
    #[inline(always)]
    pub fn adc3rst(&mut self) -> Adc3rstW<Ahb4rstrSpec> {
        Adc3rstW::new(self, 24)
    }
    #[doc = "Bit 25 - HSEM block reset"]
    #[inline(always)]
    pub fn hsemrst(&mut self) -> HsemrstW<Ahb4rstrSpec> {
        HsemrstW::new(self, 25)
    }
}
#[doc = "RCC AHB4 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb4rstrSpec;
impl crate::RegisterSpec for Ahb4rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb4rstr::R`](R) reader structure"]
impl crate::Readable for Ahb4rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb4rstr::W`](W) writer structure"]
impl crate::Writable for Ahb4rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB4RSTR to value 0"]
impl crate::Resettable for Ahb4rstrSpec {}
