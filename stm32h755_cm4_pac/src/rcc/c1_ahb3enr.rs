#[doc = "Register `C1_AHB3ENR` reader"]
pub type R = crate::R<C1Ahb3enrSpec>;
#[doc = "Register `C1_AHB3ENR` writer"]
pub type W = crate::W<C1Ahb3enrSpec>;
#[doc = "Field `MDMAEN` reader - MDMA Peripheral Clock Enable"]
pub type MdmaenR = crate::BitReader;
#[doc = "Field `MDMAEN` writer - MDMA Peripheral Clock Enable"]
pub type MdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2DEN` reader - DMA2D Peripheral Clock Enable"]
pub type Dma2denR = crate::BitReader;
#[doc = "Field `DMA2DEN` writer - DMA2D Peripheral Clock Enable"]
pub type Dma2denW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JPGDECEN` reader - JPGDEC Peripheral Clock Enable"]
pub type JpgdecenR = crate::BitReader;
#[doc = "Field `JPGDECEN` writer - JPGDEC Peripheral Clock Enable"]
pub type JpgdecenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMCEN` reader - FMC Peripheral Clocks Enable"]
pub type FmcenR = crate::BitReader;
#[doc = "Field `FMCEN` writer - FMC Peripheral Clocks Enable"]
pub type FmcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QSPIEN` reader - QUADSPI and QUADSPI Delay Clock Enable"]
pub type QspienR = crate::BitReader;
#[doc = "Field `QSPIEN` writer - QUADSPI and QUADSPI Delay Clock Enable"]
pub type QspienW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDMMC1EN` reader - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub type Sdmmc1enR = crate::BitReader;
#[doc = "Field `SDMMC1EN` writer - SDMMC1 and SDMMC1 Delay Clock Enable"]
pub type Sdmmc1enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    pub fn mdmaen(&self) -> MdmaenR {
        MdmaenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    pub fn dma2den(&self) -> Dma2denR {
        Dma2denR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    pub fn jpgdecen(&self) -> JpgdecenR {
        JpgdecenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&self) -> FmcenR {
        FmcenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    pub fn qspien(&self) -> QspienR {
        QspienR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&self) -> Sdmmc1enR {
        Sdmmc1enR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MDMA Peripheral Clock Enable"]
    #[inline(always)]
    pub fn mdmaen(&mut self) -> MdmaenW<C1Ahb3enrSpec> {
        MdmaenW::new(self, 0)
    }
    #[doc = "Bit 4 - DMA2D Peripheral Clock Enable"]
    #[inline(always)]
    pub fn dma2den(&mut self) -> Dma2denW<C1Ahb3enrSpec> {
        Dma2denW::new(self, 4)
    }
    #[doc = "Bit 5 - JPGDEC Peripheral Clock Enable"]
    #[inline(always)]
    pub fn jpgdecen(&mut self) -> JpgdecenW<C1Ahb3enrSpec> {
        JpgdecenW::new(self, 5)
    }
    #[doc = "Bit 12 - FMC Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fmcen(&mut self) -> FmcenW<C1Ahb3enrSpec> {
        FmcenW::new(self, 12)
    }
    #[doc = "Bit 14 - QUADSPI and QUADSPI Delay Clock Enable"]
    #[inline(always)]
    pub fn qspien(&mut self) -> QspienW<C1Ahb3enrSpec> {
        QspienW::new(self, 14)
    }
    #[doc = "Bit 16 - SDMMC1 and SDMMC1 Delay Clock Enable"]
    #[inline(always)]
    pub fn sdmmc1en(&mut self) -> Sdmmc1enW<C1Ahb3enrSpec> {
        Sdmmc1enW::new(self, 16)
    }
}
#[doc = "RCC AHB3 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_ahb3enr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_ahb3enr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1Ahb3enrSpec;
impl crate::RegisterSpec for C1Ahb3enrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_ahb3enr::R`](R) reader structure"]
impl crate::Readable for C1Ahb3enrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_ahb3enr::W`](W) writer structure"]
impl crate::Writable for C1Ahb3enrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_AHB3ENR to value 0"]
impl crate::Resettable for C1Ahb3enrSpec {}
