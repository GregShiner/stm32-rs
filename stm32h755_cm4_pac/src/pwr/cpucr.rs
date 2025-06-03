#[doc = "Register `CPUCR` reader"]
pub type R = crate::R<CpucrSpec>;
#[doc = "Register `CPUCR` writer"]
pub type W = crate::W<CpucrSpec>;
#[doc = "Field `PDDS_D1` reader - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub type PddsD1R = crate::BitReader;
#[doc = "Field `PDDS_D1` writer - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
pub type PddsD1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS_D2` reader - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub type PddsD2R = crate::BitReader;
#[doc = "Field `PDDS_D2` writer - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
pub type PddsD2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDDS_D3` reader - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub type PddsD3R = crate::BitReader;
#[doc = "Field `PDDS_D3` writer - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
pub type PddsD3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPF` reader - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
pub type StopfR = crate::BitReader;
#[doc = "Field `SBF` reader - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
pub type SbfR = crate::BitReader;
#[doc = "Field `SBF_D1` reader - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
pub type SbfD1R = crate::BitReader;
#[doc = "Field `SBF_D2` reader - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
pub type SbfD2R = crate::BitReader;
#[doc = "Field `CSSF` reader - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CssfR = crate::BitReader;
#[doc = "Field `CSSF` writer - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
pub type CssfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RUN_D3` reader - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub type RunD3R = crate::BitReader;
#[doc = "Field `RUN_D3` writer - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
pub type RunD3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&self) -> PddsD1R {
        PddsD1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&self) -> PddsD2R {
        PddsD2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&self) -> PddsD3R {
        PddsD3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP flag This bit is set by hardware and cleared only by any reset or by setting the CPU1 CSSF bit."]
    #[inline(always)]
    pub fn stopf(&self) -> StopfR {
        StopfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - System Standby flag This bit is set by hardware and cleared only by a POR (Power-on Reset) or by setting the CPU1 CSSF bit"]
    #[inline(always)]
    pub fn sbf(&self) -> SbfR {
        SbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - D1 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D1 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d1(&self) -> SbfD1R {
        SbfD1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - D2 domain DStandby flag This bit is set by hardware and cleared by any system reset or by setting the CPU1 CSSF bit. Once set, this bit can be cleared only when the D2 domain is no longer in DStandby mode."]
    #[inline(always)]
    pub fn sbf_d2(&self) -> SbfD2R {
        SbfD2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&self) -> CssfR {
        CssfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&self) -> RunD3R {
        RunD3R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - D1 domain Power Down Deepsleep selection. This bit allows CPU1 to define the Deepsleep mode for D1 domain."]
    #[inline(always)]
    pub fn pdds_d1(&mut self) -> PddsD1W<CpucrSpec> {
        PddsD1W::new(self, 0)
    }
    #[doc = "Bit 1 - D2 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for D2 domain."]
    #[inline(always)]
    pub fn pdds_d2(&mut self) -> PddsD2W<CpucrSpec> {
        PddsD2W::new(self, 1)
    }
    #[doc = "Bit 2 - System D3 domain Power Down Deepsleep. This bit allows CPU1 to define the Deepsleep mode for System D3 domain."]
    #[inline(always)]
    pub fn pdds_d3(&mut self) -> PddsD3W<CpucrSpec> {
        PddsD3W::new(self, 2)
    }
    #[doc = "Bit 9 - Clear D1 domain CPU1 Standby, Stop and HOLD flags (always read as 0) This bit is cleared to 0 by hardware."]
    #[inline(always)]
    pub fn cssf(&mut self) -> CssfW<CpucrSpec> {
        CssfW::new(self, 9)
    }
    #[doc = "Bit 11 - Keep system D3 domain in Run mode regardless of the CPU sub-systems modes"]
    #[inline(always)]
    pub fn run_d3(&mut self) -> RunD3W<CpucrSpec> {
        RunD3W::new(self, 11)
    }
}
#[doc = "This register allows controlling CPU1 power.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CpucrSpec;
impl crate::RegisterSpec for CpucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpucr::R`](R) reader structure"]
impl crate::Readable for CpucrSpec {}
#[doc = "`write(|w| ..)` method takes [`cpucr::W`](W) writer structure"]
impl crate::Writable for CpucrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUCR to value 0"]
impl crate::Resettable for CpucrSpec {}
