#[doc = "Register `D2CCIP2R` reader"]
pub type R = crate::R<D2ccip2rSpec>;
#[doc = "Register `D2CCIP2R` writer"]
pub type W = crate::W<D2ccip2rSpec>;
#[doc = "Field `USART234578SRC` reader - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub type Usart234578srcR = crate::FieldReader;
#[doc = "Field `USART234578SRC` writer - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
pub type Usart234578srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `USART16SRC` reader - USART1 and 6 kernel clock source selection"]
pub type Usart16srcR = crate::FieldReader;
#[doc = "Field `USART16SRC` writer - USART1 and 6 kernel clock source selection"]
pub type Usart16srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RNGSRC` reader - RNG kernel clock source selection"]
pub type RngsrcR = crate::FieldReader;
#[doc = "Field `RNGSRC` writer - RNG kernel clock source selection"]
pub type RngsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `I2C123SRC` reader - I2C1,2,3 kernel clock source selection"]
pub type I2c123srcR = crate::FieldReader;
#[doc = "Field `I2C123SRC` writer - I2C1,2,3 kernel clock source selection"]
pub type I2c123srcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USBSRC` reader - USBOTG 1 and 2 kernel clock source selection"]
pub type UsbsrcR = crate::FieldReader;
#[doc = "Field `USBSRC` writer - USBOTG 1 and 2 kernel clock source selection"]
pub type UsbsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CECSRC` reader - HDMI-CEC kernel clock source selection"]
pub type CecsrcR = crate::FieldReader;
#[doc = "Field `CECSRC` writer - HDMI-CEC kernel clock source selection"]
pub type CecsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPTIM1SRC` reader - LPTIM1 kernel clock source selection"]
pub type Lptim1srcR = crate::FieldReader;
#[doc = "Field `LPTIM1SRC` writer - LPTIM1 kernel clock source selection"]
pub type Lptim1srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578src(&self) -> Usart234578srcR {
        Usart234578srcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    pub fn usart16src(&self) -> Usart16srcR {
        Usart16srcR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsrc(&self) -> RngsrcR {
        RngsrcR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c123src(&self) -> I2c123srcR {
        I2c123srcR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    pub fn usbsrc(&self) -> UsbsrcR {
        UsbsrcR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    pub fn cecsrc(&self) -> CecsrcR {
        CecsrcR::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1src(&self) -> Lptim1srcR {
        Lptim1srcR::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - USART2/3, UART4,5, 7/8 (APB1) kernel clock source selection"]
    #[inline(always)]
    pub fn usart234578src(&mut self) -> Usart234578srcW<D2ccip2rSpec> {
        Usart234578srcW::new(self, 0)
    }
    #[doc = "Bits 3:5 - USART1 and 6 kernel clock source selection"]
    #[inline(always)]
    pub fn usart16src(&mut self) -> Usart16srcW<D2ccip2rSpec> {
        Usart16srcW::new(self, 3)
    }
    #[doc = "Bits 8:9 - RNG kernel clock source selection"]
    #[inline(always)]
    pub fn rngsrc(&mut self) -> RngsrcW<D2ccip2rSpec> {
        RngsrcW::new(self, 8)
    }
    #[doc = "Bits 12:13 - I2C1,2,3 kernel clock source selection"]
    #[inline(always)]
    pub fn i2c123src(&mut self) -> I2c123srcW<D2ccip2rSpec> {
        I2c123srcW::new(self, 12)
    }
    #[doc = "Bits 20:21 - USBOTG 1 and 2 kernel clock source selection"]
    #[inline(always)]
    pub fn usbsrc(&mut self) -> UsbsrcW<D2ccip2rSpec> {
        UsbsrcW::new(self, 20)
    }
    #[doc = "Bits 22:23 - HDMI-CEC kernel clock source selection"]
    #[inline(always)]
    pub fn cecsrc(&mut self) -> CecsrcW<D2ccip2rSpec> {
        CecsrcW::new(self, 22)
    }
    #[doc = "Bits 28:30 - LPTIM1 kernel clock source selection"]
    #[inline(always)]
    pub fn lptim1src(&mut self) -> Lptim1srcW<D2ccip2rSpec> {
        Lptim1srcW::new(self, 28)
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2ccip2r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip2r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2ccip2rSpec;
impl crate::RegisterSpec for D2ccip2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2ccip2r::R`](R) reader structure"]
impl crate::Readable for D2ccip2rSpec {}
#[doc = "`write(|w| ..)` method takes [`d2ccip2r::W`](W) writer structure"]
impl crate::Writable for D2ccip2rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2CCIP2R to value 0"]
impl crate::Resettable for D2ccip2rSpec {}
