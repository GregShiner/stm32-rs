#[doc = "Register `RTC_TAMPCR` reader"]
pub type R = crate::R<RtcTampcrSpec>;
#[doc = "Register `RTC_TAMPCR` writer"]
pub type W = crate::W<RtcTampcrSpec>;
#[doc = "Field `TAMP1E` reader - RTC_TAMP1 input detection enable"]
pub type Tamp1eR = crate::BitReader;
#[doc = "Field `TAMP1E` writer - RTC_TAMP1 input detection enable"]
pub type Tamp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1TRG` reader - Active level for RTC_TAMP1 input If TAMPFLT != 00 if TAMPFLT = 00:"]
pub type Tamp1trgR = crate::BitReader;
#[doc = "Field `TAMP1TRG` writer - Active level for RTC_TAMP1 input If TAMPFLT != 00 if TAMPFLT = 00:"]
pub type Tamp1trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPIE` reader - Tamper interrupt enable"]
pub type TampieR = crate::BitReader;
#[doc = "Field `TAMPIE` writer - Tamper interrupt enable"]
pub type TampieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2E` reader - RTC_TAMP2 input detection enable"]
pub type Tamp2eR = crate::BitReader;
#[doc = "Field `TAMP2E` writer - RTC_TAMP2 input detection enable"]
pub type Tamp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2TRG` reader - Active level for RTC_TAMP2 input if TAMPFLT != 00: if TAMPFLT = 00:"]
pub type Tamp2trgR = crate::BitReader;
#[doc = "Field `TAMP2TRG` writer - Active level for RTC_TAMP2 input if TAMPFLT != 00: if TAMPFLT = 00:"]
pub type Tamp2trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3E` reader - RTC_TAMP3 detection enable"]
pub type Tamp3eR = crate::BitReader;
#[doc = "Field `TAMP3E` writer - RTC_TAMP3 detection enable"]
pub type Tamp3eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3TRG` reader - Active level for RTC_TAMP3 input if TAMPFLT != 00: if TAMPFLT = 00:"]
pub type Tamp3trgR = crate::BitReader;
#[doc = "Field `TAMP3TRG` writer - Active level for RTC_TAMP3 input if TAMPFLT != 00: if TAMPFLT = 00:"]
pub type Tamp3trgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPTS` reader - Activate timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
pub type TamptsR = crate::BitReader;
#[doc = "Field `TAMPTS` writer - Activate timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
pub type TamptsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMPFREQ` reader - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled."]
pub type TampfreqR = crate::FieldReader;
#[doc = "Field `TAMPFREQ` writer - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled."]
pub type TampfreqW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TAMPFLT` reader - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs."]
pub type TampfltR = crate::FieldReader;
#[doc = "Field `TAMPFLT` writer - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs."]
pub type TampfltW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPRCH` reader - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs."]
pub type TampprchR = crate::FieldReader;
#[doc = "Field `TAMPPRCH` writer - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs."]
pub type TampprchW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TAMPPUDIS` reader - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample."]
pub type TamppudisR = crate::BitReader;
#[doc = "Field `TAMPPUDIS` writer - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample."]
pub type TamppudisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1IE` reader - Tamper 1 interrupt enable"]
pub type Tamp1ieR = crate::BitReader;
#[doc = "Field `TAMP1IE` writer - Tamper 1 interrupt enable"]
pub type Tamp1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1NOERASE` reader - Tamper 1 no erase"]
pub type Tamp1noeraseR = crate::BitReader;
#[doc = "Field `TAMP1NOERASE` writer - Tamper 1 no erase"]
pub type Tamp1noeraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP1MF` reader - Tamper 1 mask flag"]
pub type Tamp1mfR = crate::BitReader;
#[doc = "Field `TAMP1MF` writer - Tamper 1 mask flag"]
pub type Tamp1mfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2IE` reader - Tamper 2 interrupt enable"]
pub type Tamp2ieR = crate::BitReader;
#[doc = "Field `TAMP2IE` writer - Tamper 2 interrupt enable"]
pub type Tamp2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2NOERASE` reader - Tamper 2 no erase"]
pub type Tamp2noeraseR = crate::BitReader;
#[doc = "Field `TAMP2NOERASE` writer - Tamper 2 no erase"]
pub type Tamp2noeraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP2MF` reader - Tamper 2 mask flag"]
pub type Tamp2mfR = crate::BitReader;
#[doc = "Field `TAMP2MF` writer - Tamper 2 mask flag"]
pub type Tamp2mfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3IE` reader - Tamper 3 interrupt enable"]
pub type Tamp3ieR = crate::BitReader;
#[doc = "Field `TAMP3IE` writer - Tamper 3 interrupt enable"]
pub type Tamp3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3NOERASE` reader - Tamper 3 no erase"]
pub type Tamp3noeraseR = crate::BitReader;
#[doc = "Field `TAMP3NOERASE` writer - Tamper 3 no erase"]
pub type Tamp3noeraseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TAMP3MF` reader - Tamper 3 mask flag"]
pub type Tamp3mfR = crate::BitReader;
#[doc = "Field `TAMP3MF` writer - Tamper 3 mask flag"]
pub type Tamp3mfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&self) -> Tamp1eR {
        Tamp1eR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp1trg(&self) -> Tamp1trgR {
        Tamp1trgR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&self) -> TampieR {
        TampieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&self) -> Tamp2eR {
        Tamp2eR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input if TAMPFLT != 00: if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp2trg(&self) -> Tamp2trgR {
        Tamp2trgR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&self) -> Tamp3eR {
        Tamp3eR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input if TAMPFLT != 00: if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp3trg(&self) -> Tamp3trgR {
        Tamp3trgR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
    #[inline(always)]
    pub fn tampts(&self) -> TamptsR {
        TamptsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled."]
    #[inline(always)]
    pub fn tampfreq(&self) -> TampfreqR {
        TampfreqR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs."]
    #[inline(always)]
    pub fn tampflt(&self) -> TampfltR {
        TampfltR::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs."]
    #[inline(always)]
    pub fn tampprch(&self) -> TampprchR {
        TampprchR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample."]
    #[inline(always)]
    pub fn tamppudis(&self) -> TamppudisR {
        TamppudisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&self) -> Tamp1ieR {
        Tamp1ieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&self) -> Tamp1noeraseR {
        Tamp1noeraseR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&self) -> Tamp1mfR {
        Tamp1mfR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&self) -> Tamp2ieR {
        Tamp2ieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&self) -> Tamp2noeraseR {
        Tamp2noeraseR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&self) -> Tamp2mfR {
        Tamp2mfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&self) -> Tamp3ieR {
        Tamp3ieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&self) -> Tamp3noeraseR {
        Tamp3noeraseR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&self) -> Tamp3mfR {
        Tamp3mfR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RTC_TAMP1 input detection enable"]
    #[inline(always)]
    pub fn tamp1e(&mut self) -> Tamp1eW<RtcTampcrSpec> {
        Tamp1eW::new(self, 0)
    }
    #[doc = "Bit 1 - Active level for RTC_TAMP1 input If TAMPFLT != 00 if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp1trg(&mut self) -> Tamp1trgW<RtcTampcrSpec> {
        Tamp1trgW::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper interrupt enable"]
    #[inline(always)]
    pub fn tampie(&mut self) -> TampieW<RtcTampcrSpec> {
        TampieW::new(self, 2)
    }
    #[doc = "Bit 3 - RTC_TAMP2 input detection enable"]
    #[inline(always)]
    pub fn tamp2e(&mut self) -> Tamp2eW<RtcTampcrSpec> {
        Tamp2eW::new(self, 3)
    }
    #[doc = "Bit 4 - Active level for RTC_TAMP2 input if TAMPFLT != 00: if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp2trg(&mut self) -> Tamp2trgW<RtcTampcrSpec> {
        Tamp2trgW::new(self, 4)
    }
    #[doc = "Bit 5 - RTC_TAMP3 detection enable"]
    #[inline(always)]
    pub fn tamp3e(&mut self) -> Tamp3eW<RtcTampcrSpec> {
        Tamp3eW::new(self, 5)
    }
    #[doc = "Bit 6 - Active level for RTC_TAMP3 input if TAMPFLT != 00: if TAMPFLT = 00:"]
    #[inline(always)]
    pub fn tamp3trg(&mut self) -> Tamp3trgW<RtcTampcrSpec> {
        Tamp3trgW::new(self, 6)
    }
    #[doc = "Bit 7 - Activate timestamp on tamper detection event TAMPTS is valid even if TSE=0 in the RTC_CR register."]
    #[inline(always)]
    pub fn tampts(&mut self) -> TamptsW<RtcTampcrSpec> {
        TamptsW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Tamper sampling frequency Determines the frequency at which each of the RTC_TAMPx inputs are sampled."]
    #[inline(always)]
    pub fn tampfreq(&mut self) -> TampfreqW<RtcTampcrSpec> {
        TampfreqW::new(self, 8)
    }
    #[doc = "Bits 11:12 - RTC_TAMPx filter count These bits determines the number of consecutive samples at the specified level (TAMP*TRG) needed to activate a Tamper event. TAMPFLT is valid for each of the RTC_TAMPx inputs."]
    #[inline(always)]
    pub fn tampflt(&mut self) -> TampfltW<RtcTampcrSpec> {
        TampfltW::new(self, 11)
    }
    #[doc = "Bits 13:14 - RTC_TAMPx precharge duration These bit determines the duration of time during which the pull-up/is activated before each sample. TAMPPRCH is valid for each of the RTC_TAMPx inputs."]
    #[inline(always)]
    pub fn tampprch(&mut self) -> TampprchW<RtcTampcrSpec> {
        TampprchW::new(self, 13)
    }
    #[doc = "Bit 15 - RTC_TAMPx pull-up disable This bit determines if each of the RTC_TAMPx pins are pre-charged before each sample."]
    #[inline(always)]
    pub fn tamppudis(&mut self) -> TamppudisW<RtcTampcrSpec> {
        TamppudisW::new(self, 15)
    }
    #[doc = "Bit 16 - Tamper 1 interrupt enable"]
    #[inline(always)]
    pub fn tamp1ie(&mut self) -> Tamp1ieW<RtcTampcrSpec> {
        Tamp1ieW::new(self, 16)
    }
    #[doc = "Bit 17 - Tamper 1 no erase"]
    #[inline(always)]
    pub fn tamp1noerase(&mut self) -> Tamp1noeraseW<RtcTampcrSpec> {
        Tamp1noeraseW::new(self, 17)
    }
    #[doc = "Bit 18 - Tamper 1 mask flag"]
    #[inline(always)]
    pub fn tamp1mf(&mut self) -> Tamp1mfW<RtcTampcrSpec> {
        Tamp1mfW::new(self, 18)
    }
    #[doc = "Bit 19 - Tamper 2 interrupt enable"]
    #[inline(always)]
    pub fn tamp2ie(&mut self) -> Tamp2ieW<RtcTampcrSpec> {
        Tamp2ieW::new(self, 19)
    }
    #[doc = "Bit 20 - Tamper 2 no erase"]
    #[inline(always)]
    pub fn tamp2noerase(&mut self) -> Tamp2noeraseW<RtcTampcrSpec> {
        Tamp2noeraseW::new(self, 20)
    }
    #[doc = "Bit 21 - Tamper 2 mask flag"]
    #[inline(always)]
    pub fn tamp2mf(&mut self) -> Tamp2mfW<RtcTampcrSpec> {
        Tamp2mfW::new(self, 21)
    }
    #[doc = "Bit 22 - Tamper 3 interrupt enable"]
    #[inline(always)]
    pub fn tamp3ie(&mut self) -> Tamp3ieW<RtcTampcrSpec> {
        Tamp3ieW::new(self, 22)
    }
    #[doc = "Bit 23 - Tamper 3 no erase"]
    #[inline(always)]
    pub fn tamp3noerase(&mut self) -> Tamp3noeraseW<RtcTampcrSpec> {
        Tamp3noeraseW::new(self, 23)
    }
    #[doc = "Bit 24 - Tamper 3 mask flag"]
    #[inline(always)]
    pub fn tamp3mf(&mut self) -> Tamp3mfW<RtcTampcrSpec> {
        Tamp3mfW::new(self, 24)
    }
}
#[doc = "RTC tamper and alternate function configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`rtc_tampcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rtc_tampcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RtcTampcrSpec;
impl crate::RegisterSpec for RtcTampcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rtc_tampcr::R`](R) reader structure"]
impl crate::Readable for RtcTampcrSpec {}
#[doc = "`write(|w| ..)` method takes [`rtc_tampcr::W`](W) writer structure"]
impl crate::Writable for RtcTampcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RTC_TAMPCR to value 0"]
impl crate::Resettable for RtcTampcrSpec {}
