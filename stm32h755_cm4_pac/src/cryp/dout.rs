#[doc = "Register `DOUT` reader"]
pub type R = crate::R<DoutSpec>;
#[doc = "Field `DATAOUT` reader - Data output"]
pub type DataoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data output"]
    #[inline(always)]
    pub fn dataout(&self) -> DataoutR {
        DataoutR::new(self.bits)
    }
}
#[doc = "data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dout::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DoutSpec;
impl crate::RegisterSpec for DoutSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dout::R`](R) reader structure"]
impl crate::Readable for DoutSpec {}
#[doc = "`reset()` method sets DOUT to value 0"]
impl crate::Resettable for DoutSpec {}
