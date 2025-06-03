#[doc = "Register `OTG_HS_HFNUM` reader"]
pub type R = crate::R<OtgHsHfnumSpec>;
#[doc = "Field `FRNUM` reader - Frame number"]
pub type FrnumR = crate::FieldReader<u16>;
#[doc = "Field `FTREM` reader - Frame time remaining"]
pub type FtremR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Frame number"]
    #[inline(always)]
    pub fn frnum(&self) -> FrnumR {
        FrnumR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Frame time remaining"]
    #[inline(always)]
    pub fn ftrem(&self) -> FtremR {
        FtremR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_HS host frame number/frame time remaining register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hfnum::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHfnumSpec;
impl crate::RegisterSpec for OtgHsHfnumSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hfnum::R`](R) reader structure"]
impl crate::Readable for OtgHsHfnumSpec {}
#[doc = "`reset()` method sets OTG_HS_HFNUM to value 0x3fff"]
impl crate::Resettable for OtgHsHfnumSpec {
    const RESET_VALUE: u32 = 0x3fff;
}
