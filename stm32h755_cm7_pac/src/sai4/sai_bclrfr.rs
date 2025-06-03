#[doc = "Register `SAI_BCLRFR` writer"]
pub type W = crate::W<SaiBclrfrSpec>;
#[doc = "Field `COVRUDR` writer - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type CovrudrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMUTEDET` writer - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
pub type CmutedetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CWCKCFG` writer - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CwckcfgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCNRDY` writer - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
pub type CcnrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAFSDET` writer - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
pub type CafsdetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CLFSDET` writer - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
pub type ClfsdetW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Clear overrun / underrun. This bit is write only. Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn covrudr(&mut self) -> CovrudrW<SaiBclrfrSpec> {
        CovrudrW::new(self, 0)
    }
    #[doc = "Bit 1 - Mute detection flag. This bit is write only. Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cmutedet(&mut self) -> CmutedetW<SaiBclrfrSpec> {
        CmutedetW::new(self, 1)
    }
    #[doc = "Bit 2 - Clear wrong clock configuration flag. This bit is write only. Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register. This bit is used only when the audio block is set as master (MODE\\[1\\] = 0) and NODIV = 0 in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cwckcfg(&mut self) -> CwckcfgW<SaiBclrfrSpec> {
        CwckcfgW::new(self, 2)
    }
    #[doc = "Bit 4 - Clear Codec not ready flag. This bit is write only. Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register. This bit is used only when the AC97 audio protocol is selected in the SAI_xCR1 register. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn ccnrdy(&mut self) -> CcnrdyW<SaiBclrfrSpec> {
        CcnrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Clear anticipated frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register. It is not used in AC97or SPDIF mode. Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn cafsdet(&mut self) -> CafsdetW<SaiBclrfrSpec> {
        CafsdetW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear late frame synchronization detection flag. This bit is write only. Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register. This bit is not used in AC97or SPDIF mode Reading this bit always returns the value 0."]
    #[inline(always)]
    pub fn clfsdet(&mut self) -> ClfsdetW<SaiBclrfrSpec> {
        ClfsdetW::new(self, 6)
    }
}
#[doc = "Clear flag register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bclrfr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiBclrfrSpec;
impl crate::RegisterSpec for SaiBclrfrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`sai_bclrfr::W`](W) writer structure"]
impl crate::Writable for SaiBclrfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_BCLRFR to value 0"]
impl crate::Resettable for SaiBclrfrSpec {}
