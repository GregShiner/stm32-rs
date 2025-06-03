#[doc = "Register `SAI_PDMCR` reader"]
pub type R = crate::R<SaiPdmcrSpec>;
#[doc = "Register `SAI_PDMCR` writer"]
pub type W = crate::W<SaiPdmcrSpec>;
#[doc = "Field `PDMEN` reader - PDM enable"]
pub type PdmenR = crate::BitReader;
#[doc = "Field `PDMEN` writer - PDM enable"]
pub type PdmenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MICNBR` reader - Number of microphones"]
pub type MicnbrR = crate::FieldReader;
#[doc = "Field `MICNBR` writer - Number of microphones"]
pub type MicnbrW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CKEN1` reader - Clock enable of bitstream clock number 1"]
pub type Cken1R = crate::BitReader;
#[doc = "Field `CKEN1` writer - Clock enable of bitstream clock number 1"]
pub type Cken1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN2` reader - Clock enable of bitstream clock number 2"]
pub type Cken2R = crate::BitReader;
#[doc = "Field `CKEN2` writer - Clock enable of bitstream clock number 2"]
pub type Cken2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN3` reader - Clock enable of bitstream clock number 3"]
pub type Cken3R = crate::BitReader;
#[doc = "Field `CKEN3` writer - Clock enable of bitstream clock number 3"]
pub type Cken3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKEN4` reader - Clock enable of bitstream clock number 4"]
pub type Cken4R = crate::BitReader;
#[doc = "Field `CKEN4` writer - Clock enable of bitstream clock number 4"]
pub type Cken4W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&self) -> PdmenR {
        PdmenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - Number of microphones"]
    #[inline(always)]
    pub fn micnbr(&self) -> MicnbrR {
        MicnbrR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&self) -> Cken1R {
        Cken1R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock enable of bitstream clock number 2"]
    #[inline(always)]
    pub fn cken2(&self) -> Cken2R {
        Cken2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Clock enable of bitstream clock number 3"]
    #[inline(always)]
    pub fn cken3(&self) -> Cken3R {
        Cken3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Clock enable of bitstream clock number 4"]
    #[inline(always)]
    pub fn cken4(&self) -> Cken4R {
        Cken4R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PDM enable"]
    #[inline(always)]
    pub fn pdmen(&mut self) -> PdmenW<SaiPdmcrSpec> {
        PdmenW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Number of microphones"]
    #[inline(always)]
    pub fn micnbr(&mut self) -> MicnbrW<SaiPdmcrSpec> {
        MicnbrW::new(self, 4)
    }
    #[doc = "Bit 8 - Clock enable of bitstream clock number 1"]
    #[inline(always)]
    pub fn cken1(&mut self) -> Cken1W<SaiPdmcrSpec> {
        Cken1W::new(self, 8)
    }
    #[doc = "Bit 9 - Clock enable of bitstream clock number 2"]
    #[inline(always)]
    pub fn cken2(&mut self) -> Cken2W<SaiPdmcrSpec> {
        Cken2W::new(self, 9)
    }
    #[doc = "Bit 10 - Clock enable of bitstream clock number 3"]
    #[inline(always)]
    pub fn cken3(&mut self) -> Cken3W<SaiPdmcrSpec> {
        Cken3W::new(self, 10)
    }
    #[doc = "Bit 11 - Clock enable of bitstream clock number 4"]
    #[inline(always)]
    pub fn cken4(&mut self) -> Cken4W<SaiPdmcrSpec> {
        Cken4W::new(self, 11)
    }
}
#[doc = "PDM control register\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_pdmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_pdmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiPdmcrSpec;
impl crate::RegisterSpec for SaiPdmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_pdmcr::R`](R) reader structure"]
impl crate::Readable for SaiPdmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`sai_pdmcr::W`](W) writer structure"]
impl crate::Writable for SaiPdmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_PDMCR to value 0"]
impl crate::Resettable for SaiPdmcrSpec {}
