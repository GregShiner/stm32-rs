#[doc = "Register `CGFR` reader"]
pub type R = crate::R<CgfrSpec>;
#[doc = "Register `CGFR` writer"]
pub type W = crate::W<CgfrSpec>;
#[doc = "Field `I2SMOD` reader - I2S mode selection"]
pub type I2smodR = crate::BitReader;
#[doc = "Field `I2SMOD` writer - I2S mode selection"]
pub type I2smodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SCFG` reader - I2S configuration mode"]
pub type I2scfgR = crate::FieldReader;
#[doc = "Field `I2SCFG` writer - I2S configuration mode"]
pub type I2scfgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `I2SSTD` reader - I2S standard selection"]
pub type I2sstdR = crate::FieldReader;
#[doc = "Field `I2SSTD` writer - I2S standard selection"]
pub type I2sstdW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PCMSYNC` reader - PCM frame synchronization"]
pub type PcmsyncR = crate::BitReader;
#[doc = "Field `PCMSYNC` writer - PCM frame synchronization"]
pub type PcmsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATLEN` reader - Data length to be transferred"]
pub type DatlenR = crate::FieldReader;
#[doc = "Field `DATLEN` writer - Data length to be transferred"]
pub type DatlenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CHLEN` reader - Channel length (number of bits per audio channel)"]
pub type ChlenR = crate::BitReader;
#[doc = "Field `CHLEN` writer - Channel length (number of bits per audio channel)"]
pub type ChlenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPOL` reader - Serial audio clock polarity"]
pub type CkpolR = crate::BitReader;
#[doc = "Field `CKPOL` writer - Serial audio clock polarity"]
pub type CkpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIXCH` reader - Word select inversion"]
pub type FixchR = crate::BitReader;
#[doc = "Field `FIXCH` writer - Word select inversion"]
pub type FixchW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WSINV` reader - Fixed channel length in SLAVE"]
pub type WsinvR = crate::BitReader;
#[doc = "Field `WSINV` writer - Fixed channel length in SLAVE"]
pub type WsinvW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATFMT` reader - Data format"]
pub type DatfmtR = crate::BitReader;
#[doc = "Field `DATFMT` writer - Data format"]
pub type DatfmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2SDIV` reader - I2S linear prescaler"]
pub type I2sdivR = crate::FieldReader;
#[doc = "Field `I2SDIV` writer - I2S linear prescaler"]
pub type I2sdivW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `ODD` reader - Odd factor for the prescaler"]
pub type OddR = crate::BitReader;
#[doc = "Field `ODD` writer - Odd factor for the prescaler"]
pub type OddW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKOE` reader - Master clock output enable"]
pub type MckoeR = crate::BitReader;
#[doc = "Field `MCKOE` writer - Master clock output enable"]
pub type MckoeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&self) -> I2smodR {
        I2smodR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&self) -> I2scfgR {
        I2scfgR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&self) -> I2sstdR {
        I2sstdR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&self) -> PcmsyncR {
        PcmsyncR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&self) -> DatlenR {
        DatlenR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&self) -> ChlenR {
        ChlenR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&self) -> CkpolR {
        CkpolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    pub fn fixch(&self) -> FixchR {
        FixchR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    pub fn wsinv(&self) -> WsinvR {
        WsinvR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    pub fn datfmt(&self) -> DatfmtR {
        DatfmtR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&self) -> I2sdivR {
        I2sdivR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&self) -> OddR {
        OddR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&self) -> MckoeR {
        MckoeR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - I2S mode selection"]
    #[inline(always)]
    pub fn i2smod(&mut self) -> I2smodW<CgfrSpec> {
        I2smodW::new(self, 0)
    }
    #[doc = "Bits 1:3 - I2S configuration mode"]
    #[inline(always)]
    pub fn i2scfg(&mut self) -> I2scfgW<CgfrSpec> {
        I2scfgW::new(self, 1)
    }
    #[doc = "Bits 4:5 - I2S standard selection"]
    #[inline(always)]
    pub fn i2sstd(&mut self) -> I2sstdW<CgfrSpec> {
        I2sstdW::new(self, 4)
    }
    #[doc = "Bit 7 - PCM frame synchronization"]
    #[inline(always)]
    pub fn pcmsync(&mut self) -> PcmsyncW<CgfrSpec> {
        PcmsyncW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Data length to be transferred"]
    #[inline(always)]
    pub fn datlen(&mut self) -> DatlenW<CgfrSpec> {
        DatlenW::new(self, 8)
    }
    #[doc = "Bit 10 - Channel length (number of bits per audio channel)"]
    #[inline(always)]
    pub fn chlen(&mut self) -> ChlenW<CgfrSpec> {
        ChlenW::new(self, 10)
    }
    #[doc = "Bit 11 - Serial audio clock polarity"]
    #[inline(always)]
    pub fn ckpol(&mut self) -> CkpolW<CgfrSpec> {
        CkpolW::new(self, 11)
    }
    #[doc = "Bit 12 - Word select inversion"]
    #[inline(always)]
    pub fn fixch(&mut self) -> FixchW<CgfrSpec> {
        FixchW::new(self, 12)
    }
    #[doc = "Bit 13 - Fixed channel length in SLAVE"]
    #[inline(always)]
    pub fn wsinv(&mut self) -> WsinvW<CgfrSpec> {
        WsinvW::new(self, 13)
    }
    #[doc = "Bit 14 - Data format"]
    #[inline(always)]
    pub fn datfmt(&mut self) -> DatfmtW<CgfrSpec> {
        DatfmtW::new(self, 14)
    }
    #[doc = "Bits 16:23 - I2S linear prescaler"]
    #[inline(always)]
    pub fn i2sdiv(&mut self) -> I2sdivW<CgfrSpec> {
        I2sdivW::new(self, 16)
    }
    #[doc = "Bit 24 - Odd factor for the prescaler"]
    #[inline(always)]
    pub fn odd(&mut self) -> OddW<CgfrSpec> {
        OddW::new(self, 24)
    }
    #[doc = "Bit 25 - Master clock output enable"]
    #[inline(always)]
    pub fn mckoe(&mut self) -> MckoeW<CgfrSpec> {
        MckoeW::new(self, 25)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cgfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CgfrSpec;
impl crate::RegisterSpec for CgfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cgfr::R`](R) reader structure"]
impl crate::Readable for CgfrSpec {}
#[doc = "`write(|w| ..)` method takes [`cgfr::W`](W) writer structure"]
impl crate::Writable for CgfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CGFR to value 0"]
impl crate::Resettable for CgfrSpec {}
