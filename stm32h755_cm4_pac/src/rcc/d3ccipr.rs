#[doc = "Register `D3CCIPR` reader"]
pub type R = crate::R<D3cciprSpec>;
#[doc = "Register `D3CCIPR` writer"]
pub type W = crate::W<D3cciprSpec>;
#[doc = "Field `LPUART1SRC` reader - LPUART1 kernel clock source selection"]
pub type Lpuart1srcR = crate::FieldReader;
#[doc = "Field `LPUART1SRC` writer - LPUART1 kernel clock source selection"]
pub type Lpuart1srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I2C4SRC` reader - I2C4 kernel clock source selection"]
pub type I2c4srcR = crate::FieldReader;
#[doc = "Field `I2C4SRC` writer - I2C4 kernel clock source selection"]
pub type I2c4srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM2SRC` reader - LPTIM2 kernel clock source selection"]
pub type Lptim2srcR = crate::FieldReader;
#[doc = "Field `LPTIM2SRC` writer - LPTIM2 kernel clock source selection"]
pub type Lptim2srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LPTIM345SRC` reader - LPTIM3,4,5 kernel clock source selection"]
pub type Lptim345srcR = crate::FieldReader;
#[doc = "Field `LPTIM345SRC` writer - LPTIM3,4,5 kernel clock source selection"]
pub type Lptim345srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ADCSRC` reader - SAR ADC kernel clock source selection"]
pub type AdcsrcR = crate::FieldReader;
#[doc = "Field `ADCSRC` writer - SAR ADC kernel clock source selection"]
pub type AdcsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SAI4ASRC` reader - Sub-Block A of SAI4 kernel clock source selection"]
pub type Sai4asrcR = crate::FieldReader;
#[doc = "Field `SAI4ASRC` writer - Sub-Block A of SAI4 kernel clock source selection"]
pub type Sai4asrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAI4BSRC` reader - Sub-Block B of SAI4 kernel clock source selection"]
pub type Sai4bsrcR = crate::FieldReader;
#[doc = "Field `SAI4BSRC` writer - Sub-Block B of SAI4 kernel clock source selection"]
pub type Sai4bsrcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI6SRC` reader - SPI6 kernel clock source selection"]
pub type Spi6srcR = crate::FieldReader;
#[doc = "Field `SPI6SRC` writer - SPI6 kernel clock source selection"]
pub type Spi6srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    pub fn lpuart1src(&self) -> Lpuart1srcR {
        Lpuart1srcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4src(&self) -> I2c4srcR {
        I2c4srcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim2src(&self) -> Lptim2srcR {
        Lptim2srcR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim345src(&self) -> Lptim345srcR {
        Lptim345srcR::new(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    pub fn adcsrc(&self) -> AdcsrcR {
        AdcsrcR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4asrc(&self) -> Sai4asrcR {
        Sai4asrcR::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4bsrc(&self) -> Sai4bsrcR {
        Sai4bsrcR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    pub fn spi6src(&self) -> Spi6srcR {
        Spi6srcR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - LPUART1 kernel clock source selection"]
    #[inline(always)]
    pub fn lpuart1src(&mut self) -> Lpuart1srcW<D3cciprSpec> {
        Lpuart1srcW::new(self, 0)
    }
    #[doc = "Bits 8:9 - I2C4 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c4src(&mut self) -> I2c4srcW<D3cciprSpec> {
        I2c4srcW::new(self, 8)
    }
    #[doc = "Bits 10:12 - LPTIM2 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim2src(&mut self) -> Lptim2srcW<D3cciprSpec> {
        Lptim2srcW::new(self, 10)
    }
    #[doc = "Bits 13:15 - LPTIM3,4,5 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim345src(&mut self) -> Lptim345srcW<D3cciprSpec> {
        Lptim345srcW::new(self, 13)
    }
    #[doc = "Bits 16:17 - SAR ADC kernel clock source selection"]
    #[inline(always)]
    pub fn adcsrc(&mut self) -> AdcsrcW<D3cciprSpec> {
        AdcsrcW::new(self, 16)
    }
    #[doc = "Bits 21:23 - Sub-Block A of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4asrc(&mut self) -> Sai4asrcW<D3cciprSpec> {
        Sai4asrcW::new(self, 21)
    }
    #[doc = "Bits 24:26 - Sub-Block B of SAI4 kernel clock source selection"]
    #[inline(always)]
    pub fn sai4bsrc(&mut self) -> Sai4bsrcW<D3cciprSpec> {
        Sai4bsrcW::new(self, 24)
    }
    #[doc = "Bits 28:30 - SPI6 kernel clock source selection"]
    #[inline(always)]
    pub fn spi6src(&mut self) -> Spi6srcW<D3cciprSpec> {
        Spi6srcW::new(self, 28)
    }
}
#[doc = "RCC Domain 3 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3cciprSpec;
impl crate::RegisterSpec for D3cciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3ccipr::R`](R) reader structure"]
impl crate::Readable for D3cciprSpec {}
#[doc = "`write(|w| ..)` method takes [`d3ccipr::W`](W) writer structure"]
impl crate::Writable for D3cciprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3CCIPR to value 0"]
impl crate::Resettable for D3cciprSpec {}
