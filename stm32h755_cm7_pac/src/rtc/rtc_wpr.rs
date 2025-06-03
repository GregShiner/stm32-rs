#[doc = "Register `RTC_WPR` writer"]
pub type W = crate::W<RtcWprSpec>;
#[doc = "Field `KEY` writer - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection."]
pub type KeyW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - Write protection key This byte is written by software. Reading this byte always returns 0x00. Refer to RTC register write protection for a description of how to unlock RTC register write protection."]
    #[inline(always)]
    pub fn key(&mut self) -> KeyW<RtcWprSpec> {
        KeyW::new(self, 0)
    }
}
#[doc = "RTC write protection register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_wpr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcWprSpec;
impl crate::RegisterSpec for RtcWprSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`rtc_wpr::W`](W) writer structure"]
impl crate::Writable for RtcWprSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_WPR to value 0"]
impl crate::Resettable for RtcWprSpec {}
