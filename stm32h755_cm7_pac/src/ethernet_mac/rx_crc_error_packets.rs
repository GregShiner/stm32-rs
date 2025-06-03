#[doc = "Register `RX_CRC_ERROR_PACKETS` reader"]
pub type R = crate::R<RxCrcErrorPacketsSpec>;
#[doc = "Field `RXCRCERR` reader - RXCRCERR"]
pub type RxcrcerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXCRCERR"]
    #[inline(always)]
    pub fn rxcrcerr(&self) -> RxcrcerrR {
        RxcrcerrR::new(self.bits)
    }
}
#[doc = "Rx CRC error packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_crc_error_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxCrcErrorPacketsSpec;
impl crate::RegisterSpec for RxCrcErrorPacketsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_crc_error_packets::R`](R) reader structure"]
impl crate::Readable for RxCrcErrorPacketsSpec {}
#[doc = "`reset()` method sets RX_CRC_ERROR_PACKETS to value 0"]
impl crate::Resettable for RxCrcErrorPacketsSpec {}
