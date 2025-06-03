#[doc = "Register `MDIOS_CLRFR` reader"]
pub type R = crate::R<MdiosClrfrSpec>;
#[doc = "Register `MDIOS_CLRFR` writer"]
pub type W = crate::W<MdiosClrfrSpec>;
#[doc = "Field `CPERF` reader - Clear the preamble error flag"]
pub type CperfR = crate::BitReader;
#[doc = "Field `CPERF` writer - Clear the preamble error flag"]
pub type CperfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSERF` reader - Clear the start error flag"]
pub type CserfR = crate::BitReader;
#[doc = "Field `CSERF` writer - Clear the start error flag"]
pub type CserfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTERF` reader - Clear the turnaround error flag"]
pub type CterfR = crate::BitReader;
#[doc = "Field `CTERF` writer - Clear the turnaround error flag"]
pub type CterfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&self) -> CperfR {
        CperfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&self) -> CserfR {
        CserfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&self) -> CterfR {
        CterfR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clear the preamble error flag"]
    #[inline(always)]
    pub fn cperf(&mut self) -> CperfW<MdiosClrfrSpec> {
        CperfW::new(self, 0)
    }
    #[doc = "Bit 1 - Clear the start error flag"]
    #[inline(always)]
    pub fn cserf(&mut self) -> CserfW<MdiosClrfrSpec> {
        CserfW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear the turnaround error flag"]
    #[inline(always)]
    pub fn cterf(&mut self) -> CterfW<MdiosClrfrSpec> {
        CterfW::new(self, 2)
    }
}
#[doc = "MDIOS clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_clrfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_clrfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosClrfrSpec;
impl crate::RegisterSpec for MdiosClrfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_clrfr::R`](R) reader structure"]
impl crate::Readable for MdiosClrfrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdios_clrfr::W`](W) writer structure"]
impl crate::Writable for MdiosClrfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_CLRFR to value 0"]
impl crate::Resettable for MdiosClrfrSpec {}
