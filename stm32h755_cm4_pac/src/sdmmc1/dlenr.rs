#[doc = "Register `DLENR` reader"]
pub type R = crate::R<DlenrSpec>;
#[doc = "Register `DLENR` writer"]
pub type W = crate::W<DlenrSpec>;
#[doc = "Field `DATALENGTH` reader - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
pub type DatalengthR = crate::FieldReader<u32>;
#[doc = "Field `DATALENGTH` writer - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
pub type DatalengthW<'a, REG> = crate::FieldWriter<'a, REG, 25, u32>;
impl R {
    #[doc = "Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
    #[inline(always)]
    pub fn datalength(&self) -> DatalengthR {
        DatalengthR::new(self.bits & 0x01ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:24 - Data length value This register can only be written by firmware when DPSM is inactive (DPSMACT = 0). Number of data bytes to be transferred. When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not transfered) When DATALENGTH = 0 no data will be transfered, when requested by a CPSMEN and CMDTRANS = 1 also no command will be transfered. DTEN and CPSMEN are cleared to 0."]
    #[inline(always)]
    pub fn datalength(&mut self) -> DatalengthW<DlenrSpec> {
        DatalengthW::new(self, 0)
    }
}
#[doc = "The SDMMC_DLENR register contains the number of data bytes to be transferred. The value is loaded into the data counter when data transfer starts.\n\nYou can [`read`](crate::Reg::read) this register and get [`dlenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dlenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DlenrSpec;
impl crate::RegisterSpec for DlenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dlenr::R`](R) reader structure"]
impl crate::Readable for DlenrSpec {}
#[doc = "`write(|w| ..)` method takes [`dlenr::W`](W) writer structure"]
impl crate::Writable for DlenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DLENR to value 0"]
impl crate::Resettable for DlenrSpec {}
