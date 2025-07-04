#[doc = "Register `OTG_HS_GRXSTSR_Host` reader"]
pub type R = crate::R<OtgHsGrxstsrHostSpec>;
#[doc = "Field `CHNUM` reader - Channel number"]
pub type ChnumR = crate::FieldReader;
#[doc = "Field `BCNT` reader - Byte count"]
pub type BcntR = crate::FieldReader<u16>;
#[doc = "Field `DPID` reader - Data PID"]
pub type DpidR = crate::FieldReader;
#[doc = "Field `PKTSTS` reader - Packet status"]
pub type PktstsR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Channel number"]
    #[inline(always)]
    pub fn chnum(&self) -> ChnumR {
        ChnumR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:14 - Byte count"]
    #[inline(always)]
    pub fn bcnt(&self) -> BcntR {
        BcntR::new(((self.bits >> 4) & 0x07ff) as u16)
    }
    #[doc = "Bits 15:16 - Data PID"]
    #[inline(always)]
    pub fn dpid(&self) -> DpidR {
        DpidR::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bits 17:20 - Packet status"]
    #[inline(always)]
    pub fn pktsts(&self) -> PktstsR {
        PktstsR::new(((self.bits >> 17) & 0x0f) as u8)
    }
}
#[doc = "OTG_HS Receive status debug read register (host mode)\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_grxstsr_host::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGrxstsrHostSpec;
impl crate::RegisterSpec for OtgHsGrxstsrHostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_grxstsr_host::R`](R) reader structure"]
impl crate::Readable for OtgHsGrxstsrHostSpec {}
#[doc = "`reset()` method sets OTG_HS_GRXSTSR_Host to value 0"]
impl crate::Resettable for OtgHsGrxstsrHostSpec {}
