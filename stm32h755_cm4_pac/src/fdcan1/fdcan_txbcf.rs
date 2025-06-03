#[doc = "Register `FDCAN_TXBCF` reader"]
pub type R = crate::R<FdcanTxbcfSpec>;
#[doc = "Field `CF` reader - Cancellation Finished"]
pub type CfR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Cancellation Finished"]
    #[inline(always)]
    pub fn cf(&self) -> CfR {
        CfR::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Cancellation Finished Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbcf::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbcfSpec;
impl crate::RegisterSpec for FdcanTxbcfSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbcf::R`](R) reader structure"]
impl crate::Readable for FdcanTxbcfSpec {}
#[doc = "`reset()` method sets FDCAN_TXBCF to value 0"]
impl crate::Resettable for FdcanTxbcfSpec {}
