#[doc = "Register `MACSTSR` reader"]
pub type R = crate::R<MacstsrSpec>;
#[doc = "Field `TSS` reader - TSS"]
pub type TssR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TSS"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new(self.bits)
    }
}
#[doc = "System time seconds register\n\nYou can [`read`](crate::Reg::read) this register and get [`macstsr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacstsrSpec;
impl crate::RegisterSpec for MacstsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macstsr::R`](R) reader structure"]
impl crate::Readable for MacstsrSpec {}
#[doc = "`reset()` method sets MACSTSR to value 0"]
impl crate::Resettable for MacstsrSpec {}
