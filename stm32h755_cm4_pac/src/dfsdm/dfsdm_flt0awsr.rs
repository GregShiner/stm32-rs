#[doc = "Register `DFSDM_FLT0AWSR` reader"]
pub type R = crate::R<DfsdmFlt0awsrSpec>;
#[doc = "Field `AWLTF` reader - Analog watchdog low threshold flag"]
pub type AwltfR = crate::FieldReader;
#[doc = "Field `AWHTF` reader - Analog watchdog high threshold flag"]
pub type AwhtfR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Analog watchdog low threshold flag"]
    #[inline(always)]
    pub fn awltf(&self) -> AwltfR {
        AwltfR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Analog watchdog high threshold flag"]
    #[inline(always)]
    pub fn awhtf(&self) -> AwhtfR {
        AwhtfR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "analog watchdog status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0awsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt0awsrSpec;
impl crate::RegisterSpec for DfsdmFlt0awsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0awsr::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt0awsrSpec {}
#[doc = "`reset()` method sets DFSDM_FLT0AWSR to value 0"]
impl crate::Resettable for DfsdmFlt0awsrSpec {}
