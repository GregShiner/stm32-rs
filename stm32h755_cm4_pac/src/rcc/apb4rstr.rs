#[doc = "Register `APB4RSTR` reader"]
pub type R = crate::R<Apb4rstrSpec>;
#[doc = "Register `APB4RSTR` writer"]
pub type W = crate::W<Apb4rstrSpec>;
#[doc = "Field `SYSCFGRST` reader - SYSCFG block reset"]
pub type SyscfgrstR = crate::BitReader;
#[doc = "Field `SYSCFGRST` writer - SYSCFG block reset"]
pub type SyscfgrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1RST` reader - LPUART1 block reset"]
pub type Lpuart1rstR = crate::BitReader;
#[doc = "Field `LPUART1RST` writer - LPUART1 block reset"]
pub type Lpuart1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6RST` reader - SPI6 block reset"]
pub type Spi6rstR = crate::BitReader;
#[doc = "Field `SPI6RST` writer - SPI6 block reset"]
pub type Spi6rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4RST` reader - I2C4 block reset"]
pub type I2c4rstR = crate::BitReader;
#[doc = "Field `I2C4RST` writer - I2C4 block reset"]
pub type I2c4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2RST` reader - LPTIM2 block reset"]
pub type Lptim2rstR = crate::BitReader;
#[doc = "Field `LPTIM2RST` writer - LPTIM2 block reset"]
pub type Lptim2rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3RST` reader - LPTIM3 block reset"]
pub type Lptim3rstR = crate::BitReader;
#[doc = "Field `LPTIM3RST` writer - LPTIM3 block reset"]
pub type Lptim3rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4RST` reader - LPTIM4 block reset"]
pub type Lptim4rstR = crate::BitReader;
#[doc = "Field `LPTIM4RST` writer - LPTIM4 block reset"]
pub type Lptim4rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5RST` reader - LPTIM5 block reset"]
pub type Lptim5rstR = crate::BitReader;
#[doc = "Field `LPTIM5RST` writer - LPTIM5 block reset"]
pub type Lptim5rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP12RST` reader - COMP12 Blocks Reset"]
pub type Comp12rstR = crate::BitReader;
#[doc = "Field `COMP12RST` writer - COMP12 Blocks Reset"]
pub type Comp12rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFRST` reader - VREF block reset"]
pub type VrefrstR = crate::BitReader;
#[doc = "Field `VREFRST` writer - VREF block reset"]
pub type VrefrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4RST` reader - SAI4 block reset"]
pub type Sai4rstR = crate::BitReader;
#[doc = "Field `SAI4RST` writer - SAI4 block reset"]
pub type Sai4rstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    pub fn syscfgrst(&self) -> SyscfgrstR {
        SyscfgrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    pub fn lpuart1rst(&self) -> Lpuart1rstR {
        Lpuart1rstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    pub fn spi6rst(&self) -> Spi6rstR {
        Spi6rstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    pub fn i2c4rst(&self) -> I2c4rstR {
        I2c4rstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    pub fn lptim2rst(&self) -> Lptim2rstR {
        Lptim2rstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    pub fn lptim3rst(&self) -> Lptim3rstR {
        Lptim3rstR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    pub fn lptim4rst(&self) -> Lptim4rstR {
        Lptim4rstR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    pub fn lptim5rst(&self) -> Lptim5rstR {
        Lptim5rstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    pub fn comp12rst(&self) -> Comp12rstR {
        Comp12rstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    pub fn vrefrst(&self) -> VrefrstR {
        VrefrstR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    pub fn sai4rst(&self) -> Sai4rstR {
        Sai4rstR::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - SYSCFG block reset"]
    #[inline(always)]
    pub fn syscfgrst(&mut self) -> SyscfgrstW<Apb4rstrSpec> {
        SyscfgrstW::new(self, 1)
    }
    #[doc = "Bit 3 - LPUART1 block reset"]
    #[inline(always)]
    pub fn lpuart1rst(&mut self) -> Lpuart1rstW<Apb4rstrSpec> {
        Lpuart1rstW::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 block reset"]
    #[inline(always)]
    pub fn spi6rst(&mut self) -> Spi6rstW<Apb4rstrSpec> {
        Spi6rstW::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 block reset"]
    #[inline(always)]
    pub fn i2c4rst(&mut self) -> I2c4rstW<Apb4rstrSpec> {
        I2c4rstW::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 block reset"]
    #[inline(always)]
    pub fn lptim2rst(&mut self) -> Lptim2rstW<Apb4rstrSpec> {
        Lptim2rstW::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 block reset"]
    #[inline(always)]
    pub fn lptim3rst(&mut self) -> Lptim3rstW<Apb4rstrSpec> {
        Lptim3rstW::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 block reset"]
    #[inline(always)]
    pub fn lptim4rst(&mut self) -> Lptim4rstW<Apb4rstrSpec> {
        Lptim4rstW::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 block reset"]
    #[inline(always)]
    pub fn lptim5rst(&mut self) -> Lptim5rstW<Apb4rstrSpec> {
        Lptim5rstW::new(self, 12)
    }
    #[doc = "Bit 14 - COMP12 Blocks Reset"]
    #[inline(always)]
    pub fn comp12rst(&mut self) -> Comp12rstW<Apb4rstrSpec> {
        Comp12rstW::new(self, 14)
    }
    #[doc = "Bit 15 - VREF block reset"]
    #[inline(always)]
    pub fn vrefrst(&mut self) -> VrefrstW<Apb4rstrSpec> {
        VrefrstW::new(self, 15)
    }
    #[doc = "Bit 21 - SAI4 block reset"]
    #[inline(always)]
    pub fn sai4rst(&mut self) -> Sai4rstW<Apb4rstrSpec> {
        Sai4rstW::new(self, 21)
    }
}
#[doc = "RCC APB4 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb4rstrSpec;
impl crate::RegisterSpec for Apb4rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4rstr::R`](R) reader structure"]
impl crate::Readable for Apb4rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb4rstr::W`](W) writer structure"]
impl crate::Writable for Apb4rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB4RSTR to value 0"]
impl crate::Resettable for Apb4rstrSpec {}
