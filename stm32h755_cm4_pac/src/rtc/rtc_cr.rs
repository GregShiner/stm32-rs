#[doc = "Register `RTC_CR` reader"]
pub type R = crate::R<RtcCrSpec>;
#[doc = "Register `RTC_CR` writer"]
pub type W = crate::W<RtcCrSpec>;
#[doc = "Field `WUCKSEL` reader - Wakeup clock selection"]
pub type WuckselR = crate::FieldReader;
#[doc = "Field `WUCKSEL` writer - Wakeup clock selection"]
pub type WuckselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TSEDGE` reader - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeR = crate::BitReader;
#[doc = "Field `TSEDGE` writer - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
pub type TsedgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REFCKON` reader - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type RefckonR = crate::BitReader;
#[doc = "Field `REFCKON` writer - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
pub type RefckonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BYPSHAD` reader - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BypshadR = crate::BitReader;
#[doc = "Field `BYPSHAD` writer - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
pub type BypshadW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FMT` reader - Hour format"]
pub type FmtR = crate::BitReader;
#[doc = "Field `FMT` writer - Hour format"]
pub type FmtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAE` reader - Alarm A enable"]
pub type AlraeR = crate::BitReader;
#[doc = "Field `ALRAE` writer - Alarm A enable"]
pub type AlraeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBE` reader - Alarm B enable"]
pub type AlrbeR = crate::BitReader;
#[doc = "Field `ALRBE` writer - Alarm B enable"]
pub type AlrbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTE` reader - Wakeup timer enable"]
pub type WuteR = crate::BitReader;
#[doc = "Field `WUTE` writer - Wakeup timer enable"]
pub type WuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSE` reader - timestamp enable"]
pub type TseR = crate::BitReader;
#[doc = "Field `TSE` writer - timestamp enable"]
pub type TseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRAIE` reader - Alarm A interrupt enable"]
pub type AlraieR = crate::BitReader;
#[doc = "Field `ALRAIE` writer - Alarm A interrupt enable"]
pub type AlraieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ALRBIE` reader - Alarm B interrupt enable"]
pub type AlrbieR = crate::BitReader;
#[doc = "Field `ALRBIE` writer - Alarm B interrupt enable"]
pub type AlrbieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WUTIE` reader - Wakeup timer interrupt enable"]
pub type WutieR = crate::BitReader;
#[doc = "Field `WUTIE` writer - Wakeup timer interrupt enable"]
pub type WutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIE` reader - Time-stamp interrupt enable"]
pub type TsieR = crate::BitReader;
#[doc = "Field `TSIE` writer - Time-stamp interrupt enable"]
pub type TsieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADD1H` writer - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
pub type Add1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SUB1H` writer - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
pub type Sub1hW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKP` reader - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpR = crate::BitReader;
#[doc = "Field `BKP` writer - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
pub type BkpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COSEL` reader - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output"]
pub type CoselR = crate::BitReader;
#[doc = "Field `COSEL` writer - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output"]
pub type CoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POL` reader - Output polarity This bit is used to configure the polarity of RTC_ALARM output"]
pub type PolR = crate::BitReader;
#[doc = "Field `POL` writer - Output polarity This bit is used to configure the polarity of RTC_ALARM output"]
pub type PolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OSEL` reader - Output selection These bits are used to select the flag to be routed to RTC_ALARM output"]
pub type OselR = crate::FieldReader;
#[doc = "Field `OSEL` writer - Output selection These bits are used to select the flag to be routed to RTC_ALARM output"]
pub type OselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `COE` reader - Calibration output enable This bit enables the RTC_CALIB output"]
pub type CoeR = crate::BitReader;
#[doc = "Field `COE` writer - Calibration output enable This bit enables the RTC_CALIB output"]
pub type CoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITSE` reader - timestamp on internal event enable"]
pub type ItseR = crate::BitReader;
#[doc = "Field `ITSE` writer - timestamp on internal event enable"]
pub type ItseW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&self) -> WuckselR {
        WuckselR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&self) -> TsedgeR {
        TsedgeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&self) -> RefckonR {
        RefckonR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&self) -> BypshadR {
        BypshadR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&self) -> FmtR {
        FmtR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&self) -> AlraeR {
        AlraeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&self) -> AlrbeR {
        AlrbeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&self) -> WuteR {
        WuteR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&self) -> TseR {
        TseR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&self) -> AlraieR {
        AlraieR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&self) -> AlrbieR {
        AlrbieR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&self) -> WutieR {
        WutieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&self) -> TsieR {
        TsieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&self) -> BkpR {
        BkpR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output"]
    #[inline(always)]
    pub fn cosel(&self) -> CoselR {
        CoselR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output"]
    #[inline(always)]
    pub fn pol(&self) -> PolR {
        PolR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output"]
    #[inline(always)]
    pub fn osel(&self) -> OselR {
        OselR::new(((self.bits >> 21) & 3) as u8)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the RTC_CALIB output"]
    #[inline(always)]
    pub fn coe(&self) -> CoeR {
        CoeR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&self) -> ItseR {
        ItseR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Wakeup clock selection"]
    #[inline(always)]
    pub fn wucksel(&mut self) -> WuckselW<RtcCrSpec> {
        WuckselW::new(self, 0)
    }
    #[doc = "Bit 3 - Time-stamp event active edge TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting."]
    #[inline(always)]
    pub fn tsedge(&mut self) -> TsedgeW<RtcCrSpec> {
        TsedgeW::new(self, 3)
    }
    #[doc = "Bit 4 - RTC_REFIN reference clock detection enable (50 or 60Hz) Note: PREDIV_S must be 0x00FF."]
    #[inline(always)]
    pub fn refckon(&mut self) -> RefckonW<RtcCrSpec> {
        RefckonW::new(self, 4)
    }
    #[doc = "Bit 5 - Bypass the shadow registers Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK, BYPSHAD must be set to 1."]
    #[inline(always)]
    pub fn bypshad(&mut self) -> BypshadW<RtcCrSpec> {
        BypshadW::new(self, 5)
    }
    #[doc = "Bit 6 - Hour format"]
    #[inline(always)]
    pub fn fmt(&mut self) -> FmtW<RtcCrSpec> {
        FmtW::new(self, 6)
    }
    #[doc = "Bit 8 - Alarm A enable"]
    #[inline(always)]
    pub fn alrae(&mut self) -> AlraeW<RtcCrSpec> {
        AlraeW::new(self, 8)
    }
    #[doc = "Bit 9 - Alarm B enable"]
    #[inline(always)]
    pub fn alrbe(&mut self) -> AlrbeW<RtcCrSpec> {
        AlrbeW::new(self, 9)
    }
    #[doc = "Bit 10 - Wakeup timer enable"]
    #[inline(always)]
    pub fn wute(&mut self) -> WuteW<RtcCrSpec> {
        WuteW::new(self, 10)
    }
    #[doc = "Bit 11 - timestamp enable"]
    #[inline(always)]
    pub fn tse(&mut self) -> TseW<RtcCrSpec> {
        TseW::new(self, 11)
    }
    #[doc = "Bit 12 - Alarm A interrupt enable"]
    #[inline(always)]
    pub fn alraie(&mut self) -> AlraieW<RtcCrSpec> {
        AlraieW::new(self, 12)
    }
    #[doc = "Bit 13 - Alarm B interrupt enable"]
    #[inline(always)]
    pub fn alrbie(&mut self) -> AlrbieW<RtcCrSpec> {
        AlrbieW::new(self, 13)
    }
    #[doc = "Bit 14 - Wakeup timer interrupt enable"]
    #[inline(always)]
    pub fn wutie(&mut self) -> WutieW<RtcCrSpec> {
        WutieW::new(self, 14)
    }
    #[doc = "Bit 15 - Time-stamp interrupt enable"]
    #[inline(always)]
    pub fn tsie(&mut self) -> TsieW<RtcCrSpec> {
        TsieW::new(self, 15)
    }
    #[doc = "Bit 16 - Add 1 hour (summer time change) When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit is always read as 0."]
    #[inline(always)]
    pub fn add1h(&mut self) -> Add1hW<RtcCrSpec> {
        Add1hW::new(self, 16)
    }
    #[doc = "Bit 17 - Subtract 1 hour (winter time change) When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the current hour is not 0. This bit is always read as 0. Setting this bit has no effect when current hour is 0."]
    #[inline(always)]
    pub fn sub1h(&mut self) -> Sub1hW<RtcCrSpec> {
        Sub1hW::new(self, 17)
    }
    #[doc = "Bit 18 - Backup This bit can be written by the user to memorize whether the daylight saving time change has been performed or not."]
    #[inline(always)]
    pub fn bkp(&mut self) -> BkpW<RtcCrSpec> {
        BkpW::new(self, 18)
    }
    #[doc = "Bit 19 - Calibration output selection When COE=1, this bit selects which signal is output on RTC_CALIB. These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values (PREDIV_A=127 and PREDIV_S=255). Refer to Section24.3.15: Calibration clock output"]
    #[inline(always)]
    pub fn cosel(&mut self) -> CoselW<RtcCrSpec> {
        CoselW::new(self, 19)
    }
    #[doc = "Bit 20 - Output polarity This bit is used to configure the polarity of RTC_ALARM output"]
    #[inline(always)]
    pub fn pol(&mut self) -> PolW<RtcCrSpec> {
        PolW::new(self, 20)
    }
    #[doc = "Bits 21:22 - Output selection These bits are used to select the flag to be routed to RTC_ALARM output"]
    #[inline(always)]
    pub fn osel(&mut self) -> OselW<RtcCrSpec> {
        OselW::new(self, 21)
    }
    #[doc = "Bit 23 - Calibration output enable This bit enables the RTC_CALIB output"]
    #[inline(always)]
    pub fn coe(&mut self) -> CoeW<RtcCrSpec> {
        CoeW::new(self, 23)
    }
    #[doc = "Bit 24 - timestamp on internal event enable"]
    #[inline(always)]
    pub fn itse(&mut self) -> ItseW<RtcCrSpec> {
        ItseW::new(self, 24)
    }
}
#[doc = "RTC control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcCrSpec;
impl crate::RegisterSpec for RtcCrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_cr::R`](R) reader structure"]
impl crate::Readable for RtcCrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_cr::W`](W) writer structure"]
impl crate::Writable for RtcCrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_CR to value 0"]
impl crate::Resettable for RtcCrSpec {}
