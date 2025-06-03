#[doc = "Register `RTC_OR` reader"]
pub type R = crate::R<RtcOrSpec>;
#[doc = "Register `RTC_OR` writer"]
pub type W = crate::W<RtcOrSpec>;
#[doc = "Field `RTC_ALARM_TYPE` reader - RTC_ALARM output type on PC13"]
pub type RtcAlarmTypeR = crate::BitReader;
#[doc = "Field `RTC_ALARM_TYPE` writer - RTC_ALARM output type on PC13"]
pub type RtcAlarmTypeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_OUT_RMP` reader - RTC_OUT remap"]
pub type RtcOutRmpR = crate::BitReader;
#[doc = "Field `RTC_OUT_RMP` writer - RTC_OUT remap"]
pub type RtcOutRmpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC_ALARM output type on PC13"]
    #[inline(always)]
    pub fn rtc_alarm_type(&self) -> RtcAlarmTypeR {
        RtcAlarmTypeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - RTC_OUT remap"]
    #[inline(always)]
    pub fn rtc_out_rmp(&self) -> RtcOutRmpR {
        RtcOutRmpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_ALARM output type on PC13"]
    #[inline(always)]
    pub fn rtc_alarm_type(&mut self) -> RtcAlarmTypeW<RtcOrSpec> {
        RtcAlarmTypeW::new(self, 0)
    }
    #[doc = "Bit 1 - RTC_OUT remap"]
    #[inline(always)]
    pub fn rtc_out_rmp(&mut self) -> RtcOutRmpW<RtcOrSpec> {
        RtcOutRmpW::new(self, 1)
    }
}
#[doc = "RTC option register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_or::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_or::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcOrSpec;
impl crate::RegisterSpec for RtcOrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_or::R`](R) reader structure"]
impl crate::Readable for RtcOrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_or::W`](W) writer structure"]
impl crate::Writable for RtcOrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_OR to value 0"]
impl crate::Resettable for RtcOrSpec {}
