#[doc = "Register `DFSDM_FLT3EXMAX` reader"]
pub type R = crate::R<DfsdmFlt3exmaxSpec>;
#[doc = "Field `EXMAXCH` reader - Extremes detector maximum data channel"]
pub type ExmaxchR = crate::FieldReader;
#[doc = "Field `EXMAX` reader - Extremes detector maximum value"]
pub type ExmaxR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Extremes detector maximum data channel"]
    #[inline(always)]
    pub fn exmaxch(&self) -> ExmaxchR {
        ExmaxchR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 8:31 - Extremes detector maximum value"]
    #[inline(always)]
    pub fn exmax(&self) -> ExmaxR {
        ExmaxR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "Extremes detector maximum register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3exmax::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt3exmaxSpec;
impl crate::RegisterSpec for DfsdmFlt3exmaxSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3exmax::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt3exmaxSpec {}
#[doc = "`reset()` method sets DFSDM_FLT3EXMAX to value 0x8000_0000"]
impl crate::Resettable for DfsdmFlt3exmaxSpec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
