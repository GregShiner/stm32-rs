#[doc = "Register `DFSDM_FLT1ICR` reader"]
pub type R = crate::R<DfsdmFlt1icrSpec>;
#[doc = "Register `DFSDM_FLT1ICR` writer"]
pub type W = crate::W<DfsdmFlt1icrSpec>;
#[doc = "Field `CLRJOVRF` reader - Clear the injected conversion overrun flag"]
pub type ClrjovrfR = crate::BitReader;
#[doc = "Field `CLRJOVRF` writer - Clear the injected conversion overrun flag"]
pub type ClrjovrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRROVRF` reader - Clear the regular conversion overrun flag"]
pub type ClrrovrfR = crate::BitReader;
#[doc = "Field `CLRROVRF` writer - Clear the regular conversion overrun flag"]
pub type ClrrovrfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLRCKABF` reader - Clear the clock absence flag"]
pub type ClrckabfR = crate::FieldReader;
#[doc = "Field `CLRCKABF` writer - Clear the clock absence flag"]
pub type ClrckabfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CLRSCDF` reader - Clear the short-circuit detector flag"]
pub type ClrscdfR = crate::FieldReader;
#[doc = "Field `CLRSCDF` writer - Clear the short-circuit detector flag"]
pub type ClrscdfW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&self) -> ClrjovrfR {
        ClrjovrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&self) -> ClrrovrfR {
        ClrrovrfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    pub fn clrckabf(&self) -> ClrckabfR {
        ClrckabfR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    pub fn clrscdf(&self) -> ClrscdfR {
        ClrscdfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - Clear the injected conversion overrun flag"]
    #[inline(always)]
    pub fn clrjovrf(&mut self) -> ClrjovrfW<DfsdmFlt1icrSpec> {
        ClrjovrfW::new(self, 2)
    }
    #[doc = "Bit 3 - Clear the regular conversion overrun flag"]
    #[inline(always)]
    pub fn clrrovrf(&mut self) -> ClrrovrfW<DfsdmFlt1icrSpec> {
        ClrrovrfW::new(self, 3)
    }
    #[doc = "Bits 16:23 - Clear the clock absence flag"]
    #[inline(always)]
    pub fn clrckabf(&mut self) -> ClrckabfW<DfsdmFlt1icrSpec> {
        ClrckabfW::new(self, 16)
    }
    #[doc = "Bits 24:31 - Clear the short-circuit detector flag"]
    #[inline(always)]
    pub fn clrscdf(&mut self) -> ClrscdfW<DfsdmFlt1icrSpec> {
        ClrscdfW::new(self, 24)
    }
}
#[doc = "interrupt flag clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt1icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt1icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt1icrSpec;
impl crate::RegisterSpec for DfsdmFlt1icrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt1icr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt1icrSpec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt1icr::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt1icrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT1ICR to value 0"]
impl crate::Resettable for DfsdmFlt1icrSpec {}
