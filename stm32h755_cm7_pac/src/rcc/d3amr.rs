#[doc = "Register `D3AMR` reader"]
pub type R = crate::R<D3amrSpec>;
#[doc = "Register `D3AMR` writer"]
pub type W = crate::W<D3amrSpec>;
#[doc = "Field `BDMAAMEN` reader - BDMA and DMAMUX Autonomous mode enable"]
pub type BdmaamenR = crate::BitReader;
#[doc = "Field `BDMAAMEN` writer - BDMA and DMAMUX Autonomous mode enable"]
pub type BdmaamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPUART1AMEN` reader - LPUART1 Autonomous mode enable"]
pub type Lpuart1amenR = crate::BitReader;
#[doc = "Field `LPUART1AMEN` writer - LPUART1 Autonomous mode enable"]
pub type Lpuart1amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI6AMEN` reader - SPI6 Autonomous mode enable"]
pub type Spi6amenR = crate::BitReader;
#[doc = "Field `SPI6AMEN` writer - SPI6 Autonomous mode enable"]
pub type Spi6amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2C4AMEN` reader - I2C4 Autonomous mode enable"]
pub type I2c4amenR = crate::BitReader;
#[doc = "Field `I2C4AMEN` writer - I2C4 Autonomous mode enable"]
pub type I2c4amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM2AMEN` reader - LPTIM2 Autonomous mode enable"]
pub type Lptim2amenR = crate::BitReader;
#[doc = "Field `LPTIM2AMEN` writer - LPTIM2 Autonomous mode enable"]
pub type Lptim2amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM3AMEN` reader - LPTIM3 Autonomous mode enable"]
pub type Lptim3amenR = crate::BitReader;
#[doc = "Field `LPTIM3AMEN` writer - LPTIM3 Autonomous mode enable"]
pub type Lptim3amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM4AMEN` reader - LPTIM4 Autonomous mode enable"]
pub type Lptim4amenR = crate::BitReader;
#[doc = "Field `LPTIM4AMEN` writer - LPTIM4 Autonomous mode enable"]
pub type Lptim4amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPTIM5AMEN` reader - LPTIM5 Autonomous mode enable"]
pub type Lptim5amenR = crate::BitReader;
#[doc = "Field `LPTIM5AMEN` writer - LPTIM5 Autonomous mode enable"]
pub type Lptim5amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP12AMEN` reader - COMP12 Autonomous mode enable"]
pub type Comp12amenR = crate::BitReader;
#[doc = "Field `COMP12AMEN` writer - COMP12 Autonomous mode enable"]
pub type Comp12amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VREFAMEN` reader - VREF Autonomous mode enable"]
pub type VrefamenR = crate::BitReader;
#[doc = "Field `VREFAMEN` writer - VREF Autonomous mode enable"]
pub type VrefamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCAMEN` reader - RTC Autonomous mode enable"]
pub type RtcamenR = crate::BitReader;
#[doc = "Field `RTCAMEN` writer - RTC Autonomous mode enable"]
pub type RtcamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRCAMEN` reader - CRC Autonomous mode enable"]
pub type CrcamenR = crate::BitReader;
#[doc = "Field `CRCAMEN` writer - CRC Autonomous mode enable"]
pub type CrcamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAI4AMEN` reader - SAI4 Autonomous mode enable"]
pub type Sai4amenR = crate::BitReader;
#[doc = "Field `SAI4AMEN` writer - SAI4 Autonomous mode enable"]
pub type Sai4amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC3AMEN` reader - ADC3 Autonomous mode enable"]
pub type Adc3amenR = crate::BitReader;
#[doc = "Field `ADC3AMEN` writer - ADC3 Autonomous mode enable"]
pub type Adc3amenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKPSRAMAMEN` reader - Backup RAM Autonomous mode enable"]
pub type BkpsramamenR = crate::BitReader;
#[doc = "Field `BKPSRAMAMEN` writer - Backup RAM Autonomous mode enable"]
pub type BkpsramamenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4AMEN` reader - SRAM4 Autonomous mode enable"]
pub type Sram4amenR = crate::BitReader;
#[doc = "Field `SRAM4AMEN` writer - SRAM4 Autonomous mode enable"]
pub type Sram4amenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&self) -> BdmaamenR {
        BdmaamenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&self) -> Lpuart1amenR {
        Lpuart1amenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&self) -> Spi6amenR {
        Spi6amenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&self) -> I2c4amenR {
        I2c4amenR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&self) -> Lptim2amenR {
        Lptim2amenR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&self) -> Lptim3amenR {
        Lptim3amenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&self) -> Lptim4amenR {
        Lptim4amenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&self) -> Lptim5amenR {
        Lptim5amenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&self) -> Comp12amenR {
        Comp12amenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&self) -> VrefamenR {
        VrefamenR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&self) -> RtcamenR {
        RtcamenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&self) -> CrcamenR {
        CrcamenR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&self) -> Sai4amenR {
        Sai4amenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&self) -> Adc3amenR {
        Adc3amenR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpsramamen(&self) -> BkpsramamenR {
        BkpsramamenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&self) -> Sram4amenR {
        Sram4amenR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BDMA and DMAMUX Autonomous mode enable"]
    #[inline(always)]
    pub fn bdmaamen(&mut self) -> BdmaamenW<D3amrSpec> {
        BdmaamenW::new(self, 0)
    }
    #[doc = "Bit 3 - LPUART1 Autonomous mode enable"]
    #[inline(always)]
    pub fn lpuart1amen(&mut self) -> Lpuart1amenW<D3amrSpec> {
        Lpuart1amenW::new(self, 3)
    }
    #[doc = "Bit 5 - SPI6 Autonomous mode enable"]
    #[inline(always)]
    pub fn spi6amen(&mut self) -> Spi6amenW<D3amrSpec> {
        Spi6amenW::new(self, 5)
    }
    #[doc = "Bit 7 - I2C4 Autonomous mode enable"]
    #[inline(always)]
    pub fn i2c4amen(&mut self) -> I2c4amenW<D3amrSpec> {
        I2c4amenW::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim2amen(&mut self) -> Lptim2amenW<D3amrSpec> {
        Lptim2amenW::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM3 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim3amen(&mut self) -> Lptim3amenW<D3amrSpec> {
        Lptim3amenW::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim4amen(&mut self) -> Lptim4amenW<D3amrSpec> {
        Lptim4amenW::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 Autonomous mode enable"]
    #[inline(always)]
    pub fn lptim5amen(&mut self) -> Lptim5amenW<D3amrSpec> {
        Lptim5amenW::new(self, 12)
    }
    #[doc = "Bit 14 - COMP12 Autonomous mode enable"]
    #[inline(always)]
    pub fn comp12amen(&mut self) -> Comp12amenW<D3amrSpec> {
        Comp12amenW::new(self, 14)
    }
    #[doc = "Bit 15 - VREF Autonomous mode enable"]
    #[inline(always)]
    pub fn vrefamen(&mut self) -> VrefamenW<D3amrSpec> {
        VrefamenW::new(self, 15)
    }
    #[doc = "Bit 16 - RTC Autonomous mode enable"]
    #[inline(always)]
    pub fn rtcamen(&mut self) -> RtcamenW<D3amrSpec> {
        RtcamenW::new(self, 16)
    }
    #[doc = "Bit 19 - CRC Autonomous mode enable"]
    #[inline(always)]
    pub fn crcamen(&mut self) -> CrcamenW<D3amrSpec> {
        CrcamenW::new(self, 19)
    }
    #[doc = "Bit 21 - SAI4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sai4amen(&mut self) -> Sai4amenW<D3amrSpec> {
        Sai4amenW::new(self, 21)
    }
    #[doc = "Bit 24 - ADC3 Autonomous mode enable"]
    #[inline(always)]
    pub fn adc3amen(&mut self) -> Adc3amenW<D3amrSpec> {
        Adc3amenW::new(self, 24)
    }
    #[doc = "Bit 28 - Backup RAM Autonomous mode enable"]
    #[inline(always)]
    pub fn bkpsramamen(&mut self) -> BkpsramamenW<D3amrSpec> {
        BkpsramamenW::new(self, 28)
    }
    #[doc = "Bit 29 - SRAM4 Autonomous mode enable"]
    #[inline(always)]
    pub fn sram4amen(&mut self) -> Sram4amenW<D3amrSpec> {
        Sram4amenW::new(self, 29)
    }
}
#[doc = "RCC D3 Autonomous mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3amr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3amr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3amrSpec;
impl crate::RegisterSpec for D3amrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3amr::R`](R) reader structure"]
impl crate::Readable for D3amrSpec {}
#[doc = "`write(|w| ..)` method takes [`d3amr::W`](W) writer structure"]
impl crate::Writable for D3amrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3AMR to value 0"]
impl crate::Resettable for D3amrSpec {}
