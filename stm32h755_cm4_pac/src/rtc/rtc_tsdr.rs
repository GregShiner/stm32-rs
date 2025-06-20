#[doc = "Register `RTC_TSDR` reader"]
pub type R = crate::R<RtcTsdrSpec>;
#[doc = "Field `DU` reader - Date units in BCD format"]
pub type DuR = crate::FieldReader;
#[doc = "Field `DT` reader - Date tens in BCD format"]
pub type DtR = crate::FieldReader;
#[doc = "Field `MU` reader - Month units in BCD format"]
pub type MuR = crate::FieldReader;
#[doc = "Field `MT` reader - Month tens in BCD format"]
pub type MtR = crate::BitReader;
#[doc = "Field `WDU` reader - Week day units"]
pub type WduR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Date units in BCD format"]
    #[inline(always)]
    pub fn du(&self) -> DuR {
        DuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Date tens in BCD format"]
    #[inline(always)]
    pub fn dt(&self) -> DtR {
        DtR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:11 - Month units in BCD format"]
    #[inline(always)]
    pub fn mu(&self) -> MuR {
        MuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Month tens in BCD format"]
    #[inline(always)]
    pub fn mt(&self) -> MtR {
        MtR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 13:15 - Week day units"]
    #[inline(always)]
    pub fn wdu(&self) -> WduR {
        WduR::new(((self.bits >> 13) & 7) as u8)
    }
}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tsdr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTsdrSpec;
impl crate::RegisterSpec for RtcTsdrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tsdr::R`](R) reader structure"]
impl crate::Readable for RtcTsdrSpec {}
#[doc = "`reset()` method sets RTC_TSDR to value 0"]
impl crate::Resettable for RtcTsdrSpec {}
