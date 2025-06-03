#[doc = "Register `RTC_TSTR` reader"]
pub type R = crate::R<RtcTstrSpec>;
#[doc = "Field `SU` reader - Second units in BCD format."]
pub type SuR = crate::FieldReader;
#[doc = "Field `ST` reader - Second tens in BCD format."]
pub type StR = crate::FieldReader;
#[doc = "Field `MNU` reader - Minute units in BCD format."]
pub type MnuR = crate::FieldReader;
#[doc = "Field `MNT` reader - Minute tens in BCD format."]
pub type MntR = crate::FieldReader;
#[doc = "Field `HU` reader - Hour units in BCD format."]
pub type HuR = crate::FieldReader;
#[doc = "Field `HT` reader - Hour tens in BCD format."]
pub type HtR = crate::FieldReader;
#[doc = "Field `PM` reader - AM/PM notation"]
pub type PmR = crate::BitReader;
impl R {
    #[doc = "Bits 0:3 - Second units in BCD format."]
    #[inline(always)]
    pub fn su(&self) -> SuR {
        SuR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Second tens in BCD format."]
    #[inline(always)]
    pub fn st(&self) -> StR {
        StR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:11 - Minute units in BCD format."]
    #[inline(always)]
    pub fn mnu(&self) -> MnuR {
        MnuR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:14 - Minute tens in BCD format."]
    #[inline(always)]
    pub fn mnt(&self) -> MntR {
        MntR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 16:19 - Hour units in BCD format."]
    #[inline(always)]
    pub fn hu(&self) -> HuR {
        HuR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:21 - Hour tens in BCD format."]
    #[inline(always)]
    pub fn ht(&self) -> HtR {
        HtR::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - AM/PM notation"]
    #[inline(always)]
    pub fn pm(&self) -> PmR {
        PmR::new(((self.bits >> 22) & 1) != 0)
    }
}
#[doc = "The content of this register is valid only when TSF is set to 1 in RTC_ISR. It is cleared when TSF bit is reset.\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tstr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTstrSpec;
impl crate::RegisterSpec for RtcTstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tstr::R`](R) reader structure"]
impl crate::Readable for RtcTstrSpec {}
#[doc = "`reset()` method sets RTC_TSTR to value 0"]
impl crate::Resettable for RtcTstrSpec {}
