#[doc = "Register `TX_MULTIPLE_COLLISION_GOOD_PACKETS` reader"]
pub type R = crate::R<TxMultipleCollisionGoodPacketsSpec>;
#[doc = "Field `TXMULTCOLG` reader - TXMULTCOLG"]
pub type TxmultcolgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXMULTCOLG"]
    #[inline(always)]
    pub fn txmultcolg(&self) -> TxmultcolgR {
        TxmultcolgR::new(self.bits)
    }
}
#[doc = "Tx multiple collision good packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_multiple_collision_good_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxMultipleCollisionGoodPacketsSpec;
impl crate::RegisterSpec for TxMultipleCollisionGoodPacketsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_multiple_collision_good_packets::R`](R) reader structure"]
impl crate::Readable for TxMultipleCollisionGoodPacketsSpec {}
#[doc = "`reset()` method sets TX_MULTIPLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TxMultipleCollisionGoodPacketsSpec {}
