#[doc = "Register `ECC_FA2R` reader"]
pub type R = crate::R<EccFa2rSpec>;
#[doc = "Field `FAIL_ECC_ADDR2` reader - Bank 2 ECC error address"]
pub type FailEccAddr2R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:14 - Bank 2 ECC error address"]
    #[inline(always)]
    pub fn fail_ecc_addr2(&self) -> FailEccAddr2R {
        FailEccAddr2R::new((self.bits & 0x7fff) as u16)
    }
}
#[doc = "FLASH ECC fail address for bank 2\n\nYou can [`read`](crate::Reg::read) this register and get [`ecc_fa2r::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EccFa2rSpec;
impl crate::RegisterSpec for EccFa2rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ecc_fa2r::R`](R) reader structure"]
impl crate::Readable for EccFa2rSpec {}
#[doc = "`reset()` method sets ECC_FA2R to value 0"]
impl crate::Resettable for EccFa2rSpec {}
