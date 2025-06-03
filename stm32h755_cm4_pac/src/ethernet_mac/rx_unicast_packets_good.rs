#[doc = "Register `RX_UNICAST_PACKETS_GOOD` reader"]
pub type R = crate::R<RxUnicastPacketsGoodSpec>;
#[doc = "Field `RXUCASTG` reader - RXUCASTG"]
pub type RxucastgR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXUCASTG"]
    #[inline(always)]
    pub fn rxucastg(&self) -> RxucastgR {
        RxucastgR::new(self.bits)
    }
}
#[doc = "Rx unicast packets good register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_unicast_packets_good::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxUnicastPacketsGoodSpec;
impl crate::RegisterSpec for RxUnicastPacketsGoodSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_unicast_packets_good::R`](R) reader structure"]
impl crate::Readable for RxUnicastPacketsGoodSpec {}
#[doc = "`reset()` method sets RX_UNICAST_PACKETS_GOOD to value 0"]
impl crate::Resettable for RxUnicastPacketsGoodSpec {}
