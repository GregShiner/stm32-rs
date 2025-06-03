#[doc = "Register `GCR` reader"]
pub type R = crate::R<GcrSpec>;
#[doc = "Register `GCR` writer"]
pub type W = crate::W<GcrSpec>;
#[doc = "Field `LTDCEN` reader - LCD-TFT controller enable bit"]
pub type LtdcenR = crate::BitReader;
#[doc = "Field `LTDCEN` writer - LCD-TFT controller enable bit"]
pub type LtdcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBW` reader - Dither Blue Width"]
pub type DbwR = crate::FieldReader;
#[doc = "Field `DGW` reader - Dither Green Width"]
pub type DgwR = crate::FieldReader;
#[doc = "Field `DRW` reader - Dither Red Width"]
pub type DrwR = crate::FieldReader;
#[doc = "Field `DEN` reader - Dither Enable"]
pub type DenR = crate::BitReader;
#[doc = "Field `DEN` writer - Dither Enable"]
pub type DenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCPOL` reader - Pixel Clock Polarity"]
pub type PcpolR = crate::BitReader;
#[doc = "Field `PCPOL` writer - Pixel Clock Polarity"]
pub type PcpolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEPOL` reader - Data Enable Polarity"]
pub type DepolR = crate::BitReader;
#[doc = "Field `DEPOL` writer - Data Enable Polarity"]
pub type DepolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSPOL` reader - Vertical Synchronization Polarity"]
pub type VspolR = crate::BitReader;
#[doc = "Field `VSPOL` writer - Vertical Synchronization Polarity"]
pub type VspolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSPOL` reader - Horizontal Synchronization Polarity"]
pub type HspolR = crate::BitReader;
#[doc = "Field `HSPOL` writer - Horizontal Synchronization Polarity"]
pub type HspolW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&self) -> LtdcenR {
        LtdcenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:6 - Dither Blue Width"]
    #[inline(always)]
    pub fn dbw(&self) -> DbwR {
        DbwR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Dither Green Width"]
    #[inline(always)]
    pub fn dgw(&self) -> DgwR {
        DgwR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:14 - Dither Red Width"]
    #[inline(always)]
    pub fn drw(&self) -> DrwR {
        DrwR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&self) -> DenR {
        DenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&self) -> PcpolR {
        PcpolR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&self) -> DepolR {
        DepolR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&self) -> VspolR {
        VspolR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&self) -> HspolR {
        HspolR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LCD-TFT controller enable bit"]
    #[inline(always)]
    pub fn ltdcen(&mut self) -> LtdcenW<GcrSpec> {
        LtdcenW::new(self, 0)
    }
    #[doc = "Bit 16 - Dither Enable"]
    #[inline(always)]
    pub fn den(&mut self) -> DenW<GcrSpec> {
        DenW::new(self, 16)
    }
    #[doc = "Bit 28 - Pixel Clock Polarity"]
    #[inline(always)]
    pub fn pcpol(&mut self) -> PcpolW<GcrSpec> {
        PcpolW::new(self, 28)
    }
    #[doc = "Bit 29 - Data Enable Polarity"]
    #[inline(always)]
    pub fn depol(&mut self) -> DepolW<GcrSpec> {
        DepolW::new(self, 29)
    }
    #[doc = "Bit 30 - Vertical Synchronization Polarity"]
    #[inline(always)]
    pub fn vspol(&mut self) -> VspolW<GcrSpec> {
        VspolW::new(self, 30)
    }
    #[doc = "Bit 31 - Horizontal Synchronization Polarity"]
    #[inline(always)]
    pub fn hspol(&mut self) -> HspolW<GcrSpec> {
        HspolW::new(self, 31)
    }
}
#[doc = "Global Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`gcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GcrSpec;
impl crate::RegisterSpec for GcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gcr::R`](R) reader structure"]
impl crate::Readable for GcrSpec {}
#[doc = "`write(|w| ..)` method takes [`gcr::W`](W) writer structure"]
impl crate::Writable for GcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GCR to value 0x2220"]
impl crate::Resettable for GcrSpec {
    const RESET_VALUE: u32 = 0x2220;
}
