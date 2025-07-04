#[doc = "Register `DTCR` reader"]
pub type R = crate::R<DtcrSpec>;
#[doc = "Register `DTCR` writer"]
pub type W = crate::W<DtcrSpec>;
#[doc = "Field `DTRx` reader - Deadtime Rising value"]
pub type DtrxR = crate::FieldReader<u16>;
#[doc = "Field `DTRx` writer - Deadtime Rising value"]
pub type DtrxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTRx` reader - Sign Deadtime Rising value"]
pub type SdtrxR = crate::BitReader;
#[doc = "Field `SDTRx` writer - Sign Deadtime Rising value"]
pub type SdtrxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTPRSC` reader - Deadtime Prescaler"]
pub type DtprscR = crate::FieldReader;
#[doc = "Field `DTPRSC` writer - Deadtime Prescaler"]
pub type DtprscW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DTRSLKx` reader - Deadtime Rising Sign Lock"]
pub type DtrslkxR = crate::BitReader;
#[doc = "Field `DTRSLKx` writer - Deadtime Rising Sign Lock"]
pub type DtrslkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTRLKx` reader - Deadtime Rising Lock"]
pub type DtrlkxR = crate::BitReader;
#[doc = "Field `DTRLKx` writer - Deadtime Rising Lock"]
pub type DtrlkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFx` reader - Deadtime Falling value"]
pub type DtfxR = crate::FieldReader<u16>;
#[doc = "Field `DTFx` writer - Deadtime Falling value"]
pub type DtfxW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `SDTFx` reader - Sign Deadtime Falling value"]
pub type SdtfxR = crate::BitReader;
#[doc = "Field `SDTFx` writer - Sign Deadtime Falling value"]
pub type SdtfxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFSLKx` reader - Deadtime Falling Sign Lock"]
pub type DtfslkxR = crate::BitReader;
#[doc = "Field `DTFSLKx` writer - Deadtime Falling Sign Lock"]
pub type DtfslkxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTFLKx` reader - Deadtime Falling Lock"]
pub type DtflkxR = crate::BitReader;
#[doc = "Field `DTFLKx` writer - Deadtime Falling Lock"]
pub type DtflkxW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&self) -> DtrxR {
        DtrxR::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&self) -> SdtrxR {
        SdtrxR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&self) -> DtprscR {
        DtprscR::new(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&self) -> DtrslkxR {
        DtrslkxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&self) -> DtrlkxR {
        DtrlkxR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&self) -> DtfxR {
        DtfxR::new(((self.bits >> 16) & 0x01ff) as u16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&self) -> SdtfxR {
        SdtfxR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&self) -> DtfslkxR {
        DtfslkxR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&self) -> DtflkxR {
        DtflkxR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - Deadtime Rising value"]
    #[inline(always)]
    pub fn dtrx(&mut self) -> DtrxW<DtcrSpec> {
        DtrxW::new(self, 0)
    }
    #[doc = "Bit 9 - Sign Deadtime Rising value"]
    #[inline(always)]
    pub fn sdtrx(&mut self) -> SdtrxW<DtcrSpec> {
        SdtrxW::new(self, 9)
    }
    #[doc = "Bits 10:12 - Deadtime Prescaler"]
    #[inline(always)]
    pub fn dtprsc(&mut self) -> DtprscW<DtcrSpec> {
        DtprscW::new(self, 10)
    }
    #[doc = "Bit 14 - Deadtime Rising Sign Lock"]
    #[inline(always)]
    pub fn dtrslkx(&mut self) -> DtrslkxW<DtcrSpec> {
        DtrslkxW::new(self, 14)
    }
    #[doc = "Bit 15 - Deadtime Rising Lock"]
    #[inline(always)]
    pub fn dtrlkx(&mut self) -> DtrlkxW<DtcrSpec> {
        DtrlkxW::new(self, 15)
    }
    #[doc = "Bits 16:24 - Deadtime Falling value"]
    #[inline(always)]
    pub fn dtfx(&mut self) -> DtfxW<DtcrSpec> {
        DtfxW::new(self, 16)
    }
    #[doc = "Bit 25 - Sign Deadtime Falling value"]
    #[inline(always)]
    pub fn sdtfx(&mut self) -> SdtfxW<DtcrSpec> {
        SdtfxW::new(self, 25)
    }
    #[doc = "Bit 30 - Deadtime Falling Sign Lock"]
    #[inline(always)]
    pub fn dtfslkx(&mut self) -> DtfslkxW<DtcrSpec> {
        DtfslkxW::new(self, 30)
    }
    #[doc = "Bit 31 - Deadtime Falling Lock"]
    #[inline(always)]
    pub fn dtflkx(&mut self) -> DtflkxW<DtcrSpec> {
        DtflkxW::new(self, 31)
    }
}
#[doc = "Timerx Deadtime Register\n\nYou can [`read`](crate::Reg::read) this register and get [`dtcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DtcrSpec;
impl crate::RegisterSpec for DtcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dtcr::R`](R) reader structure"]
impl crate::Readable for DtcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dtcr::W`](W) writer structure"]
impl crate::Writable for DtcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DTCR to value 0"]
impl crate::Resettable for DtcrSpec {}
