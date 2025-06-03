#[doc = "Register `DFSDM_FLT2ISR` reader"]
pub type R = crate::R<DfsdmFlt2isrSpec>;
#[doc = "Field `JEOCF` reader - End of injected conversion flag"]
pub type JeocfR = crate::BitReader;
#[doc = "Field `REOCF` reader - End of regular conversion flag"]
pub type ReocfR = crate::BitReader;
#[doc = "Field `JOVRF` reader - Injected conversion overrun flag"]
pub type JovrfR = crate::BitReader;
#[doc = "Field `ROVRF` reader - Regular conversion overrun flag"]
pub type RovrfR = crate::BitReader;
#[doc = "Field `AWDF` reader - Analog watchdog"]
pub type AwdfR = crate::BitReader;
#[doc = "Field `JCIP` reader - Injected conversion in progress status"]
pub type JcipR = crate::BitReader;
#[doc = "Field `RCIP` reader - Regular conversion in progress status"]
pub type RcipR = crate::BitReader;
#[doc = "Field `CKABF` reader - Clock absence flag"]
pub type CkabfR = crate::FieldReader;
#[doc = "Field `SCDF` reader - short-circuit detector flag"]
pub type ScdfR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - End of injected conversion flag"]
    #[inline(always)]
    pub fn jeocf(&self) -> JeocfR {
        JeocfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - End of regular conversion flag"]
    #[inline(always)]
    pub fn reocf(&self) -> ReocfR {
        ReocfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Injected conversion overrun flag"]
    #[inline(always)]
    pub fn jovrf(&self) -> JovrfR {
        JovrfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Regular conversion overrun flag"]
    #[inline(always)]
    pub fn rovrf(&self) -> RovrfR {
        RovrfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Analog watchdog"]
    #[inline(always)]
    pub fn awdf(&self) -> AwdfR {
        AwdfR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Injected conversion in progress status"]
    #[inline(always)]
    pub fn jcip(&self) -> JcipR {
        JcipR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Regular conversion in progress status"]
    #[inline(always)]
    pub fn rcip(&self) -> RcipR {
        RcipR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Clock absence flag"]
    #[inline(always)]
    pub fn ckabf(&self) -> CkabfR {
        CkabfR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - short-circuit detector flag"]
    #[inline(always)]
    pub fn scdf(&self) -> ScdfR {
        ScdfR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt2isr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt2isrSpec;
impl crate::RegisterSpec for DfsdmFlt2isrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt2isr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt2isrSpec {}
#[doc = "`reset()` method sets DFSDM_FLT2ISR to value 0x00ff_0000"]
impl crate::Resettable for DfsdmFlt2isrSpec {
    const RESET_VALUE: u32 = 0x00ff_0000;
}
