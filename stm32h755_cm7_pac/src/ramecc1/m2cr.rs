#[doc = "Register `M2CR` reader"]
pub type R = crate::R<M2crSpec>;
#[doc = "Register `M2CR` writer"]
pub type W = crate::W<M2crSpec>;
#[doc = "Field `SEDCF` reader - ECC single error detected and corrected flag"]
pub type SedcfR = crate::BitReader;
#[doc = "Field `SEDCF` writer - ECC single error detected and corrected flag"]
pub type SedcfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEDF` reader - ECC double error detected flag"]
pub type DedfR = crate::BitReader;
#[doc = "Field `DEDF` writer - ECC double error detected flag"]
pub type DedfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DEBWDF` reader - ECC double error on byte write (BW) detected flag"]
pub type DebwdfR = crate::BitReader;
#[doc = "Field `DEBWDF` writer - ECC double error on byte write (BW) detected flag"]
pub type DebwdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&self) -> SedcfR {
        SedcfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&self) -> DedfR {
        DedfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&self) -> DebwdfR {
        DebwdfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ECC single error detected and corrected flag"]
    #[inline(always)]
    pub fn sedcf(&mut self) -> SedcfW<M2crSpec> {
        SedcfW::new(self, 0)
    }
    #[doc = "Bit 1 - ECC double error detected flag"]
    #[inline(always)]
    pub fn dedf(&mut self) -> DedfW<M2crSpec> {
        DedfW::new(self, 1)
    }
    #[doc = "Bit 2 - ECC double error on byte write (BW) detected flag"]
    #[inline(always)]
    pub fn debwdf(&mut self) -> DebwdfW<M2crSpec> {
        DebwdfW::new(self, 2)
    }
}
#[doc = "RAMECC monitor x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`m2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`m2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M2crSpec;
impl crate::RegisterSpec for M2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m2cr::R`](R) reader structure"]
impl crate::Readable for M2crSpec {}
#[doc = "`write(|w| ..)` method takes [`m2cr::W`](W) writer structure"]
impl crate::Writable for M2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets M2CR to value 0"]
impl crate::Resettable for M2crSpec {}
