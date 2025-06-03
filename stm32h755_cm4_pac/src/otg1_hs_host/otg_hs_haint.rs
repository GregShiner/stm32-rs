#[doc = "Register `OTG_HS_HAINT` reader"]
pub type R = crate::R<OtgHsHaintSpec>;
#[doc = "Field `HAINT` reader - Channel interrupts"]
pub type HaintR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Channel interrupts"]
    #[inline(always)]
    pub fn haint(&self) -> HaintR {
        HaintR::new((self.bits & 0xffff) as u16)
    }
}
#[doc = "OTG_HS Host all channels interrupt register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_haint::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHaintSpec;
impl crate::RegisterSpec for OtgHsHaintSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_haint::R`](R) reader structure"]
impl crate::Readable for OtgHsHaintSpec {}
#[doc = "`reset()` method sets OTG_HS_HAINT to value 0"]
impl crate::Resettable for OtgHsHaintSpec {}
