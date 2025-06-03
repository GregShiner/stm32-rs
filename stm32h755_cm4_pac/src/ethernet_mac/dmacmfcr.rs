#[doc = "Register `DMACMFCR` reader"]
pub type R = crate::R<DmacmfcrSpec>;
#[doc = "Field `MFC` reader - Dropped Packet Counters"]
pub type MfcR = crate::FieldReader<u16>;
#[doc = "Field `MFCO` reader - Overflow status of the MFC Counter"]
pub type MfcoR = crate::BitReader;
impl R {
    #[doc = "Bits 0:10 - Dropped Packet Counters"]
    #[inline(always)]
    pub fn mfc(&self) -> MfcR {
        MfcR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 15 - Overflow status of the MFC Counter"]
    #[inline(always)]
    pub fn mfco(&self) -> MfcoR {
        MfcoR::new(((self.bits >> 15) & 1) != 0)
    }
}
#[doc = "Channel missed frame count register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacmfcr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacmfcrSpec;
impl crate::RegisterSpec for DmacmfcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacmfcr::R`](R) reader structure"]
impl crate::Readable for DmacmfcrSpec {}
#[doc = "`reset()` method sets DMACMFCR to value 0"]
impl crate::Resettable for DmacmfcrSpec {}
