#[doc = "Register `RTC_BKP21R` reader"]
pub type R = crate::R<RtcBkp21rSpec>;
#[doc = "Register `RTC_BKP21R` writer"]
pub type W = crate::W<RtcBkp21rSpec>;
#[doc = "Field `BKP` reader - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
pub type BkpR = crate::FieldReader<u32>;
#[doc = "Field `BKP` writer - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
pub type BkpW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - The application can write or read data to and from these registers. They are powered-on by VBAT when VDD is switched off, so that they are not reset by System reset, and their contents remain valid when the device operates in low-power mode. This register is reset on a tamper detection event, as long as TAMPxF=1. or when the Flash readout protection is disabled."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<RtcBkp21rSpec> {
        BkpW::new(self, 0)
    }
}
#[doc = "RTC backup registers\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_bkp21r::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_bkp21r::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcBkp21rSpec;
impl crate::RegisterSpec for RtcBkp21rSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_bkp21r::R`](R) reader structure"]
impl crate::Readable for RtcBkp21rSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_bkp21r::W`](W) writer structure"]
impl crate::Writable for RtcBkp21rSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_BKP21R to value 0"]
impl crate::Resettable for RtcBkp21rSpec {}
