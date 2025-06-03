#[doc = "Register `TX_SINGLE_COLLISION_GOOD_PACKETS` reader"]
pub type R = crate::R<TxSingleCollisionGoodPacketsSpec>;
#[doc = "Field `TXSNGLCOLG` reader - TXSNGLCOLG"]
pub type TxsnglcolgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXSNGLCOLG"]
    #[inline(always)]
    pub fn txsnglcolg(&self) -> TxsnglcolgR {
        TxsnglcolgR::new(self.bits)
    }
}
#[doc = "Tx single collision good packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_single_collision_good_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxSingleCollisionGoodPacketsSpec;
impl crate::RegisterSpec for TxSingleCollisionGoodPacketsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_single_collision_good_packets::R`](R) reader structure"]
impl crate::Readable for TxSingleCollisionGoodPacketsSpec {}
#[doc = "`reset()` method sets TX_SINGLE_COLLISION_GOOD_PACKETS to value 0"]
impl crate::Resettable for TxSingleCollisionGoodPacketsSpec {}
