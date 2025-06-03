#[doc = "Register `MACDR` reader"]
pub type R = crate::R<MacdrSpec>;
#[doc = "Field `RPESTS` reader - RPESTS"]
pub type RpestsR = crate::BitReader;
#[doc = "Field `RFCFCSTS` reader - RFCFCSTS"]
pub type RfcfcstsR = crate::FieldReader;
#[doc = "Field `TPESTS` reader - TPESTS"]
pub type TpestsR = crate::BitReader;
#[doc = "Field `TFCSTS` reader - TFCSTS"]
pub type TfcstsR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - RPESTS"]
    #[inline(always)]
    pub fn rpests(&self) -> RpestsR {
        RpestsR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - RFCFCSTS"]
    #[inline(always)]
    pub fn rfcfcsts(&self) -> RfcfcstsR {
        RfcfcstsR::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 16 - TPESTS"]
    #[inline(always)]
    pub fn tpests(&self) -> TpestsR {
        TpestsR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:18 - TFCSTS"]
    #[inline(always)]
    pub fn tfcsts(&self) -> TfcstsR {
        TfcstsR::new(((self.bits >> 17) & 3) as u8)
    }
}
#[doc = "Debug register\n\nYou can [`read`](crate::Reg::read) this register and get [`macdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacdrSpec;
impl crate::RegisterSpec for MacdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macdr::R`](R) reader structure"]
impl crate::Readable for MacdrSpec {}
#[doc = "`reset()` method sets MACDR to value 0"]
impl crate::Resettable for MacdrSpec {}
