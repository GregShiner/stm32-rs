#[doc = "Register `FCR` reader"]
pub type R = crate::R<FcrSpec>;
#[doc = "Register `FCR` writer"]
pub type W = crate::W<FcrSpec>;
#[doc = "Field `CTEF` reader - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
pub type CtefR = crate::BitReader;
#[doc = "Field `CTEF` writer - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
pub type CtefW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCF` reader - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
pub type CtcfR = crate::BitReader;
#[doc = "Field `CTCF` writer - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
pub type CtcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSMF` reader - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
pub type CsmfR = crate::BitReader;
#[doc = "Field `CSMF` writer - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
pub type CsmfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTOF` reader - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
pub type CtofR = crate::BitReader;
#[doc = "Field `CTOF` writer - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
pub type CtofW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctef(&self) -> CtefR {
        CtefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctcf(&self) -> CtcfR {
        CtcfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn csmf(&self) -> CsmfR {
        CsmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctof(&self) -> CtofR {
        CtofR::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear transfer error flag Writing 1 clears the TEF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctef(&mut self) -> CtefW<FcrSpec> {
        CtefW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear transfer complete flag Writing 1 clears the TCF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctcf(&mut self) -> CtcfW<FcrSpec> {
        CtcfW::new(self, 1)
    }
    #[doc = "Bit 3 - Clear status match flag Writing 1 clears the SMF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn csmf(&mut self) -> CsmfW<FcrSpec> {
        CsmfW::new(self, 3)
    }
    #[doc = "Bit 4 - Clear timeout flag Writing 1 clears the TOF flag in the QUADSPI_SR register"]
    #[inline(always)]
    pub fn ctof(&mut self) -> CtofW<FcrSpec> {
        CtofW::new(self, 4)
    }
}
#[doc = "QUADSPI flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FcrSpec;
impl crate::RegisterSpec for FcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fcr::R`](R) reader structure"]
impl crate::Readable for FcrSpec {}
#[doc = "`write(|w| ..)` method takes [`fcr::W`](W) writer structure"]
impl crate::Writable for FcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FCR to value 0"]
impl crate::Resettable for FcrSpec {}
