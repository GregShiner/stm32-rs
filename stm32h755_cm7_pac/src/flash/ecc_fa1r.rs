#[doc = "Register `ECC_FA1R` reader"]
pub type R = crate::R<EccFa1rSpec>;
#[doc = "Field `FAIL_ECC_ADDR1` reader - Bank 1 ECC error address"]
pub type FailEccAddr1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 1 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr1(&self) -> FailEccAddr1R {
        FailEccAddr1R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "FLASH ECC fail address for bank 1\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_fa1r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccFa1rSpec;
impl crate::RegisterSpec for EccFa1rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_fa1r::R`](R) reader structure"]
impl crate::Readable for EccFa1rSpec {}
#[doc = "`reset()` method sets ECC_FA1R to value 0"]
impl crate::Resettable for EccFa1rSpec {}
