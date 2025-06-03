#[doc = "Register `DOR` reader"]
pub type R = crate::R<DorSpec>;
#[doc = "Field `DATAOUT` reader - Data Output FIFO Output FIFO data register."]
pub type DataoutR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Data Output FIFO Output FIFO data register."]
    #[inline(always)]
    pub fn dataout(&self) -> DataoutR {
        DataoutR::new(self.bits)
    }
}
#[doc = "JPEG data output register\n\nYou can [`read`](crate::Reg::read) this register and get [`dor::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DorSpec;
impl crate::RegisterSpec for DorSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dor::R`](R) reader structure"]
impl crate::Readable for DorSpec {}
#[doc = "`reset()` method sets DOR to value 0"]
impl crate::Resettable for DorSpec {}
