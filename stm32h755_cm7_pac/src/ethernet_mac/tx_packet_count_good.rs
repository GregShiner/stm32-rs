#[doc = "Register `TX_PACKET_COUNT_GOOD` reader"]
pub type R = crate::R<TxPacketCountGoodSpec>;
#[doc = "Field `TXPKTG` reader - TXPKTG"]
pub type TxpktgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - TXPKTG"]
    #[inline(always)]
    pub fn txpktg(&self) -> TxpktgR {
        TxpktgR::new(self.bits)
    }
}
#[doc = "Tx packet count good register\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_packet_count_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxPacketCountGoodSpec;
impl crate::RegisterSpec for TxPacketCountGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_packet_count_good::R`](R) reader structure"]
impl crate::Readable for TxPacketCountGoodSpec {}
#[doc = "`reset()` method sets TX_PACKET_COUNT_GOOD to value 0"]
impl crate::Resettable for TxPacketCountGoodSpec {}
