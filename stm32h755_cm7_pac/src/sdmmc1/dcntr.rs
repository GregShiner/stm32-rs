#[doc = "Register `DCNTR` reader"]
pub type R = crate::R<DcntrSpec>;
#[doc = "Field `DATACOUNT` reader - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
pub type DatacountR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:24 - Data count value When read, the number of remaining data bytes to be transferred is returned. Write has no effect."]
    #[inline(always)]
    pub fn datacount(&self) -> DatacountR {
        DatacountR::new(self.bits & 0x01ff_ffff)
    }
}
#[doc = "The SDMMC_DCNTR register loads the value from the data length register (see SDMMC_DLENR) when the DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and when there has been no error, the data status end flag (DATAEND) is set.\n\nYou can [`read`](crate::Reg::read) this register and get [`dcntr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcntrSpec;
impl crate::RegisterSpec for DcntrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcntr::R`](R) reader structure"]
impl crate::Readable for DcntrSpec {}
#[doc = "`reset()` method sets DCNTR to value 0"]
impl crate::Resettable for DcntrSpec {}
