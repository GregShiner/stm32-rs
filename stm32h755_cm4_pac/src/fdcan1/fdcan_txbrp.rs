#[doc = "Register `FDCAN_TXBRP` reader"]
pub type R = crate::R<FdcanTxbrpSpec>;
#[doc = "Field `TRP` reader - Transmission Request Pending"]
pub type TrpR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmission Request Pending"]
    #[inline(always)]
    pub fn trp(&self) -> TrpR {
        TrpR::new(self.bits)
    }
}
#[doc = "FDCAN Tx Buffer Request Pending Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txbrp::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxbrpSpec;
impl crate::RegisterSpec for FdcanTxbrpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txbrp::R`](R) reader structure"]
impl crate::Readable for FdcanTxbrpSpec {}
#[doc = "`reset()` method sets FDCAN_TXBRP to value 0"]
impl crate::Resettable for FdcanTxbrpSpec {}
