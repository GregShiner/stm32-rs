#[doc = "Register `RX_ALIGNMENT_ERROR_PACKETS` reader"]
pub type R = crate::R<RxAlignmentErrorPacketsSpec>;
#[doc = "Field `RXALGNERR` reader - RXALGNERR"]
pub type RxalgnerrR = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - RXALGNERR"]
    #[inline(always)]
    pub fn rxalgnerr(&self) -> RxalgnerrR {
        RxalgnerrR::new(self.bits)
    }
}
#[doc = "Rx alignment error packets register\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_alignment_error_packets::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RxAlignmentErrorPacketsSpec;
impl crate::RegisterSpec for RxAlignmentErrorPacketsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_alignment_error_packets::R`](R) reader structure"]
impl crate::Readable for RxAlignmentErrorPacketsSpec {}
#[doc = "`reset()` method sets RX_ALIGNMENT_ERROR_PACKETS to value 0"]
impl crate::Resettable for RxAlignmentErrorPacketsSpec {}
