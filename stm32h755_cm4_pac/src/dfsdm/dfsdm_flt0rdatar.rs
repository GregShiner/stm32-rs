#[doc = "Register `DFSDM_FLT0RDATAR` reader"]
pub type R = crate::R<DfsdmFlt0rdatarSpec>;
#[doc = "Field `RDATACH` reader - Regular channel most recently converted"]
pub type RdatachR = crate::FieldReader;
#[doc = "Field `RPEND` reader - Regular channel pending data"]
pub type RpendR = crate::BitReader;
#[doc = "Field `RDATA` reader - Regular channel conversion data"]
pub type RdataR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:2 - Regular channel most recently converted"]
    #[inline(always)]
    pub fn rdatach(&self) -> RdatachR {
        RdatachR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Regular channel pending data"]
    #[inline(always)]
    pub fn rpend(&self) -> RpendR {
        RpendR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 8:31 - Regular channel conversion data"]
    #[inline(always)]
    pub fn rdata(&self) -> RdataR {
        RdataR::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[doc = "data register for the regular channel\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt0rdatar::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt0rdatarSpec;
impl crate::RegisterSpec for DfsdmFlt0rdatarSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt0rdatar::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt0rdatarSpec {}
#[doc = "`reset()` method sets DFSDM_FLT0RDATAR to value 0"]
impl crate::Resettable for DfsdmFlt0rdatarSpec {}
