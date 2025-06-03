#[doc = "Register `AHB3RSTR` reader"]
pub type R = crate::R<Ahb3rstrSpec>;
#[doc = "Register `AHB3RSTR` writer"]
pub type W = crate::W<Ahb3rstrSpec>;
#[doc = "Field `MDMARST` reader - MDMA block reset"]
pub type MdmarstR = crate::BitReader;
#[doc = "Field `MDMARST` writer - MDMA block reset"]
pub type MdmarstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DRST` reader - DMA2D block reset"]
pub type Dma2drstR = crate::BitReader;
#[doc = "Field `DMA2DRST` writer - DMA2D block reset"]
pub type Dma2drstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPGDECRST` reader - JPGDEC block reset"]
pub type JpgdecrstR = crate::BitReader;
#[doc = "Field `JPGDECRST` writer - JPGDEC block reset"]
pub type JpgdecrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCRST` reader - FMC block reset"]
pub type FmcrstR = crate::BitReader;
#[doc = "Field `FMCRST` writer - FMC block reset"]
pub type FmcrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIRST` reader - QUADSPI and QUADSPI delay block reset"]
pub type QspirstR = crate::BitReader;
#[doc = "Field `QSPIRST` writer - QUADSPI and QUADSPI delay block reset"]
pub type QspirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1RST` reader - SDMMC1 and SDMMC1 delay block reset"]
pub type Sdmmc1rstR = crate::BitReader;
#[doc = "Field `SDMMC1RST` writer - SDMMC1 and SDMMC1 delay block reset"]
pub type Sdmmc1rstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPURST` reader - CPU reset"]
pub type CpurstR = crate::BitReader;
#[doc = "Field `CPURST` writer - CPU reset"]
pub type CpurstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    pub fn mdmarst(&self) -> MdmarstR {
        MdmarstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    pub fn dma2drst(&self) -> Dma2drstR {
        Dma2drstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    pub fn jpgdecrst(&self) -> JpgdecrstR {
        JpgdecrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    pub fn fmcrst(&self) -> FmcrstR {
        FmcrstR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    pub fn qspirst(&self) -> QspirstR {
        QspirstR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&self) -> Sdmmc1rstR {
        Sdmmc1rstR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    pub fn cpurst(&self) -> CpurstR {
        CpurstR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA block reset"]
    #[inline(always)]
    pub fn mdmarst(&mut self) -> MdmarstW<Ahb3rstrSpec> {
        MdmarstW::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D block reset"]
    #[inline(always)]
    pub fn dma2drst(&mut self) -> Dma2drstW<Ahb3rstrSpec> {
        Dma2drstW::new(self, 4)
    }
    #[doc = "Bit 5 - JPGDEC block reset"]
    #[inline(always)]
    pub fn jpgdecrst(&mut self) -> JpgdecrstW<Ahb3rstrSpec> {
        JpgdecrstW::new(self, 5)
    }
    #[doc = "Bit 12 - FMC block reset"]
    #[inline(always)]
    pub fn fmcrst(&mut self) -> FmcrstW<Ahb3rstrSpec> {
        FmcrstW::new(self, 12)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI delay block reset"]
    #[inline(always)]
    pub fn qspirst(&mut self) -> QspirstW<Ahb3rstrSpec> {
        QspirstW::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 delay block reset"]
    #[inline(always)]
    pub fn sdmmc1rst(&mut self) -> Sdmmc1rstW<Ahb3rstrSpec> {
        Sdmmc1rstW::new(self, 16)
    }
    #[doc = "Bit 31 - CPU reset"]
    #[inline(always)]
    pub fn cpurst(&mut self) -> CpurstW<Ahb3rstrSpec> {
        CpurstW::new(self, 31)
    }
}
#[doc = "RCC AHB3 Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3rstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3rstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3rstrSpec;
impl crate::RegisterSpec for Ahb3rstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3rstr::R`](R) reader structure"]
impl crate::Readable for Ahb3rstrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3rstr::W`](W) writer structure"]
impl crate::Writable for Ahb3rstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3RSTR to value 0"]
impl crate::Resettable for Ahb3rstrSpec {}
