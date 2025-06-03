#[doc = "Register `MDIOS_SR` reader"]
pub type R = crate::R<MdiosSrSpec>;
#[doc = "Field `PERF` reader - Preamble error flag"]
pub type PerfR = crate::BitReader;
#[doc = "Field `SERF` reader - Start error flag"]
pub type SerfR = crate::BitReader;
#[doc = "Field `TERF` reader - Turnaround error flag"]
pub type TerfR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Preamble error flag"]
    #[inline(always)]
    pub fn perf(&self) -> PerfR {
        PerfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start error flag"]
    #[inline(always)]
    pub fn serf(&self) -> SerfR {
        SerfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Turnaround error flag"]
    #[inline(always)]
    pub fn terf(&self) -> TerfR {
        TerfR::new(((self.bits >> 2) & 1) != 0)
    }
}
#[doc = "MDIOS status register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosSrSpec;
impl crate::RegisterSpec for MdiosSrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_sr::R`](R) reader structure"]
impl crate::Readable for MdiosSrSpec {}
#[doc = "`reset()` method sets MDIOS_SR to value 0"]
impl crate::Resettable for MdiosSrSpec {}
