#[doc = "Register `D2CCIP1R` reader"]
pub type R = crate::R<D2ccip1rSpec>;
#[doc = "Register `D2CCIP1R` writer"]
pub type W = crate::W<D2ccip1rSpec>;
#[doc = "Field `SAI1SRC` reader - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type Sai1srcR = crate::FieldReader;
#[doc = "Field `SAI1SRC` writer - SAI1 and DFSDM1 kernel Aclk clock source selection"]
pub type Sai1srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SAI23SRC` reader - SAI2 and SAI3 kernel clock source selection"]
pub type Sai23srcR = crate::FieldReader;
#[doc = "Field `SAI23SRC` writer - SAI2 and SAI3 kernel clock source selection"]
pub type Sai23srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI123SRC` reader - SPI/I2S1,2 and 3 kernel clock source selection"]
pub type Spi123srcR = crate::FieldReader;
#[doc = "Field `SPI123SRC` writer - SPI/I2S1,2 and 3 kernel clock source selection"]
pub type Spi123srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPI45SRC` reader - SPI4 and 5 kernel clock source selection"]
pub type Spi45srcR = crate::FieldReader;
#[doc = "Field `SPI45SRC` writer - SPI4 and 5 kernel clock source selection"]
pub type Spi45srcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SPDIFSRC` reader - SPDIFRX kernel clock source selection"]
pub type SpdifsrcR = crate::FieldReader;
#[doc = "Field `SPDIFSRC` writer - SPDIFRX kernel clock source selection"]
pub type SpdifsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DFSDM1SRC` reader - DFSDM1 kernel Clk clock source selection"]
pub type Dfsdm1srcR = crate::BitReader;
#[doc = "Field `DFSDM1SRC` writer - DFSDM1 kernel Clk clock source selection"]
pub type Dfsdm1srcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANSRC` reader - FDCAN kernel clock source selection"]
pub type FdcansrcR = crate::FieldReader;
#[doc = "Field `FDCANSRC` writer - FDCAN kernel clock source selection"]
pub type FdcansrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWPSRC` reader - SWPMI kernel clock source selection"]
pub type SwpsrcR = crate::BitReader;
#[doc = "Field `SWPSRC` writer - SWPMI kernel clock source selection"]
pub type SwpsrcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1src(&self) -> Sai1srcR {
        Sai1srcR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23src(&self) -> Sai23srcR {
        Sai23srcR::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123src(&self) -> Spi123srcR {
        Spi123srcR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45src(&self) -> Spi45srcR {
        Spi45srcR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsrc(&self) -> SpdifsrcR {
        SpdifsrcR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1src(&self) -> Dfsdm1srcR {
        Dfsdm1srcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansrc(&self) -> FdcansrcR {
        FdcansrcR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsrc(&self) -> SwpsrcR {
        SwpsrcR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - SAI1 and DFSDM1 kernel Aclk clock source selection"]
    #[inline(always)]
    pub fn sai1src(&mut self) -> Sai1srcW<D2ccip1rSpec> {
        Sai1srcW::new(self, 0)
    }
    #[doc = "Bits 6:8 - SAI2 and SAI3 kernel clock source selection"]
    #[inline(always)]
    pub fn sai23src(&mut self) -> Sai23srcW<D2ccip1rSpec> {
        Sai23srcW::new(self, 6)
    }
    #[doc = "Bits 12:14 - SPI/I2S1,2 and 3 kernel clock source selection"]
    #[inline(always)]
    pub fn spi123src(&mut self) -> Spi123srcW<D2ccip1rSpec> {
        Spi123srcW::new(self, 12)
    }
    #[doc = "Bits 16:18 - SPI4 and 5 kernel clock source selection"]
    #[inline(always)]
    pub fn spi45src(&mut self) -> Spi45srcW<D2ccip1rSpec> {
        Spi45srcW::new(self, 16)
    }
    #[doc = "Bits 20:21 - SPDIFRX kernel clock source selection"]
    #[inline(always)]
    pub fn spdifsrc(&mut self) -> SpdifsrcW<D2ccip1rSpec> {
        SpdifsrcW::new(self, 20)
    }
    #[doc = "Bit 24 - DFSDM1 kernel Clk clock source selection"]
    #[inline(always)]
    pub fn dfsdm1src(&mut self) -> Dfsdm1srcW<D2ccip1rSpec> {
        Dfsdm1srcW::new(self, 24)
    }
    #[doc = "Bits 28:29 - FDCAN kernel clock source selection"]
    #[inline(always)]
    pub fn fdcansrc(&mut self) -> FdcansrcW<D2ccip1rSpec> {
        FdcansrcW::new(self, 28)
    }
    #[doc = "Bit 31 - SWPMI kernel clock source selection"]
    #[inline(always)]
    pub fn swpsrc(&mut self) -> SwpsrcW<D2ccip1rSpec> {
        SwpsrcW::new(self, 31)
    }
}
#[doc = "RCC Domain 2 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d2ccip1r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d2ccip1r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D2ccip1rSpec;
impl crate::RegisterSpec for D2ccip1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d2ccip1r::R`](R) reader structure"]
impl crate::Readable for D2ccip1rSpec {}
#[doc = "`write(|w| ..)` method takes [`d2ccip1r::W`](W) writer structure"]
impl crate::Writable for D2ccip1rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D2CCIP1R to value 0"]
impl crate::Resettable for D2ccip1rSpec {}
