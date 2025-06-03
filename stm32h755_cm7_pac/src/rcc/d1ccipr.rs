#[doc = "Register `D1CCIPR` reader"]
pub type R = crate::R<D1cciprSpec>;
#[doc = "Register `D1CCIPR` writer"]
pub type W = crate::W<D1cciprSpec>;
#[doc = "Field `FMCSRC` reader - FMC kernel clock source selection"]
pub type FmcsrcR = crate::FieldReader;
#[doc = "Field `FMCSRC` writer - FMC kernel clock source selection"]
pub type FmcsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `QSPISRC` reader - QUADSPI kernel clock source selection"]
pub type QspisrcR = crate::FieldReader;
#[doc = "Field `QSPISRC` writer - QUADSPI kernel clock source selection"]
pub type QspisrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDMMCSRC` reader - SDMMC kernel clock source selection"]
pub type SdmmcsrcR = crate::BitReader;
#[doc = "Field `SDMMCSRC` writer - SDMMC kernel clock source selection"]
pub type SdmmcsrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKPERSRC` reader - per_ck clock source selection"]
pub type CkpersrcR = crate::FieldReader;
#[doc = "Field `CKPERSRC` writer - per_ck clock source selection"]
pub type CkpersrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsrc(&self) -> FmcsrcR {
        FmcsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn qspisrc(&self) -> QspisrcR {
        QspisrcR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsrc(&self) -> SdmmcsrcR {
        SdmmcsrcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersrc(&self) -> CkpersrcR {
        CkpersrcR::new(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - FMC kernel clock source selection"]
    #[inline(always)]
    pub fn fmcsrc(&mut self) -> FmcsrcW<D1cciprSpec> {
        FmcsrcW::new(self, 0)
    }
    #[doc = "Bits 4:5 - QUADSPI kernel clock source selection"]
    #[inline(always)]
    pub fn qspisrc(&mut self) -> QspisrcW<D1cciprSpec> {
        QspisrcW::new(self, 4)
    }
    #[doc = "Bit 16 - SDMMC kernel clock source selection"]
    #[inline(always)]
    pub fn sdmmcsrc(&mut self) -> SdmmcsrcW<D1cciprSpec> {
        SdmmcsrcW::new(self, 16)
    }
    #[doc = "Bits 28:29 - per_ck clock source selection"]
    #[inline(always)]
    pub fn ckpersrc(&mut self) -> CkpersrcW<D1cciprSpec> {
        CkpersrcW::new(self, 28)
    }
}
#[doc = "RCC Domain 1 Kernel Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`d1ccipr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1ccipr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D1cciprSpec;
impl crate::RegisterSpec for D1cciprSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d1ccipr::R`](R) reader structure"]
impl crate::Readable for D1cciprSpec {}
#[doc = "`write(|w| ..)` method takes [`d1ccipr::W`](W) writer structure"]
impl crate::Writable for D1cciprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D1CCIPR to value 0"]
impl crate::Resettable for D1cciprSpec {}
