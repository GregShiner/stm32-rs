#[doc = "Register `OTG_HS_DAINT` reader"]
pub type R = crate::R<OtgHsDaintSpec>;
#[doc = "Field `IEPINT` reader - IN endpoint interrupt bits"]
pub type IepintR = crate::FieldReader<u16>;
#[doc = "Field `OEPINT` reader - OUT endpoint interrupt bits"]
pub type OepintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - IN endpoint interrupt bits"]
    #[inline(always)]
    pub fn iepint(&self) -> IepintR {
        IepintR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - OUT endpoint interrupt bits"]
    #[inline(always)]
    pub fn oepint(&self) -> OepintR {
        OepintR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "OTG_HS device all endpoints interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_daint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsDaintSpec;
impl crate::RegisterSpec for OtgHsDaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_daint::R`](R) reader structure"]
impl crate::Readable for OtgHsDaintSpec {}
#[doc = "`reset()` method sets OTG_HS_DAINT to value 0"]
impl crate::Resettable for OtgHsDaintSpec {}
