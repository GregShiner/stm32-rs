#[doc = "Register `AHB3LPENR` reader"]
pub type R = crate::R<Ahb3lpenrSpec>;
#[doc = "Register `AHB3LPENR` writer"]
pub type W = crate::W<Ahb3lpenrSpec>;
#[doc = "Field `MDMALPEN` reader - MDMA Clock Enable During CSleep Mode"]
pub type MdmalpenR = crate::BitReader;
#[doc = "Field `MDMALPEN` writer - MDMA Clock Enable During CSleep Mode"]
pub type MdmalpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DLPEN` reader - DMA2D Clock Enable During CSleep Mode"]
pub type Dma2dlpenR = crate::BitReader;
#[doc = "Field `DMA2DLPEN` writer - DMA2D Clock Enable During CSleep Mode"]
pub type Dma2dlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPGDECLPEN` reader - JPGDEC Clock Enable During CSleep Mode"]
pub type JpgdeclpenR = crate::BitReader;
#[doc = "Field `JPGDECLPEN` writer - JPGDEC Clock Enable During CSleep Mode"]
pub type JpgdeclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLITFLPEN` reader - FLITF Clock Enable During CSleep Mode"]
pub type FlitflpenR = crate::BitReader;
#[doc = "Field `FLITFLPEN` writer - FLITF Clock Enable During CSleep Mode"]
pub type FlitflpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCLPEN` reader - FMC Peripheral Clocks Enable During CSleep Mode"]
pub type FmclpenR = crate::BitReader;
#[doc = "Field `FMCLPEN` writer - FMC Peripheral Clocks Enable During CSleep Mode"]
pub type FmclpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPILPEN` reader - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub type QspilpenR = crate::BitReader;
#[doc = "Field `QSPILPEN` writer - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
pub type QspilpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1LPEN` reader - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub type Sdmmc1lpenR = crate::BitReader;
#[doc = "Field `SDMMC1LPEN` writer - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
pub type Sdmmc1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1DTCM1LPEN` reader - D1DTCM1 Block Clock Enable During CSleep mode"]
pub type D1dtcm1lpenR = crate::BitReader;
#[doc = "Field `D1DTCM1LPEN` writer - D1DTCM1 Block Clock Enable During CSleep mode"]
pub type D1dtcm1lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCM2LPEN` reader - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub type Dtcm2lpenR = crate::BitReader;
#[doc = "Field `DTCM2LPEN` writer - D1 DTCM2 Block Clock Enable During CSleep mode"]
pub type Dtcm2lpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCMLPEN` reader - D1ITCM Block Clock Enable During CSleep mode"]
pub type ItcmlpenR = crate::BitReader;
#[doc = "Field `ITCMLPEN` writer - D1ITCM Block Clock Enable During CSleep mode"]
pub type ItcmlpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXISRAMLPEN` reader - AXISRAM Block Clock Enable During CSleep mode"]
pub type AxisramlpenR = crate::BitReader;
#[doc = "Field `AXISRAMLPEN` writer - AXISRAM Block Clock Enable During CSleep mode"]
pub type AxisramlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&self) -> MdmalpenR {
        MdmalpenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&self) -> Dma2dlpenR {
        Dma2dlpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn jpgdeclpen(&self) -> JpgdeclpenR {
        JpgdeclpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn flitflpen(&self) -> FlitflpenR {
        FlitflpenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&self) -> FmclpenR {
        FmclpenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn qspilpen(&self) -> QspilpenR {
        QspilpenR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&self) -> Sdmmc1lpenR {
        Sdmmc1lpenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&self) -> D1dtcm1lpenR {
        D1dtcm1lpenR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&self) -> Dtcm2lpenR {
        Dtcm2lpenR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&self) -> ItcmlpenR {
        ItcmlpenR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&self) -> AxisramlpenR {
        AxisramlpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn mdmalpen(&mut self) -> MdmalpenW<Ahb3lpenrSpec> {
        MdmalpenW::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn dma2dlpen(&mut self) -> Dma2dlpenW<Ahb3lpenrSpec> {
        Dma2dlpenW::new(self, 4)
    }
    #[doc = "Bit 5 - JPGDEC Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn jpgdeclpen(&mut self) -> JpgdeclpenW<Ahb3lpenrSpec> {
        JpgdeclpenW::new(self, 5)
    }
    #[doc = "Bit 8 - FLITF Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn flitflpen(&mut self) -> FlitflpenW<Ahb3lpenrSpec> {
        FlitflpenW::new(self, 8)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fmclpen(&mut self) -> FmclpenW<Ahb3lpenrSpec> {
        FmclpenW::new(self, 12)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn qspilpen(&mut self) -> QspilpenW<Ahb3lpenrSpec> {
        QspilpenW::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable During CSleep Mode"]
    #[inline(always)]
    pub fn sdmmc1lpen(&mut self) -> Sdmmc1lpenW<Ahb3lpenrSpec> {
        Sdmmc1lpenW::new(self, 16)
    }
    #[doc = "Bit 28 - D1DTCM1 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn d1dtcm1lpen(&mut self) -> D1dtcm1lpenW<Ahb3lpenrSpec> {
        D1dtcm1lpenW::new(self, 28)
    }
    #[doc = "Bit 29 - D1 DTCM2 Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn dtcm2lpen(&mut self) -> Dtcm2lpenW<Ahb3lpenrSpec> {
        Dtcm2lpenW::new(self, 29)
    }
    #[doc = "Bit 30 - D1ITCM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn itcmlpen(&mut self) -> ItcmlpenW<Ahb3lpenrSpec> {
        ItcmlpenW::new(self, 30)
    }
    #[doc = "Bit 31 - AXISRAM Block Clock Enable During CSleep mode"]
    #[inline(always)]
    pub fn axisramlpen(&mut self) -> AxisramlpenW<Ahb3lpenrSpec> {
        AxisramlpenW::new(self, 31)
    }
}
#[doc = "RCC AHB3 Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb3lpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb3lpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ahb3lpenrSpec;
impl crate::RegisterSpec for Ahb3lpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb3lpenr::R`](R) reader structure"]
impl crate::Readable for Ahb3lpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`ahb3lpenr::W`](W) writer structure"]
impl crate::Writable for Ahb3lpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB3LPENR to value 0"]
impl crate::Resettable for Ahb3lpenrSpec {}
