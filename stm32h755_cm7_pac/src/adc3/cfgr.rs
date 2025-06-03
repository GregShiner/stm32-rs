#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `DMNGT` reader - ADC DMA transfer enable"]
pub type DmngtR = crate::FieldReader;
#[doc = "Field `DMNGT` writer - ADC DMA transfer enable"]
pub type DmngtW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RES` reader - ADC data resolution"]
pub type ResR = crate::FieldReader;
#[doc = "Field `RES` writer - ADC data resolution"]
pub type ResW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXTSEL` reader - ADC group regular external trigger source"]
pub type ExtselR = crate::FieldReader;
#[doc = "Field `EXTSEL` writer - ADC group regular external trigger source"]
pub type ExtselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EXTEN` reader - ADC group regular external trigger polarity"]
pub type ExtenR = crate::FieldReader;
#[doc = "Field `EXTEN` writer - ADC group regular external trigger polarity"]
pub type ExtenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `OVRMOD` reader - ADC group regular overrun configuration"]
pub type OvrmodR = crate::BitReader;
#[doc = "Field `OVRMOD` writer - ADC group regular overrun configuration"]
pub type OvrmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CONT` reader - ADC group regular continuous conversion mode"]
pub type ContR = crate::BitReader;
#[doc = "Field `CONT` writer - ADC group regular continuous conversion mode"]
pub type ContW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTDLY` reader - ADC low power auto wait"]
pub type AutdlyR = crate::BitReader;
#[doc = "Field `AUTDLY` writer - ADC low power auto wait"]
pub type AutdlyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCEN` reader - ADC group regular sequencer discontinuous mode"]
pub type DiscenR = crate::BitReader;
#[doc = "Field `DISCEN` writer - ADC group regular sequencer discontinuous mode"]
pub type DiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISCNUM` reader - ADC group regular sequencer discontinuous number of ranks"]
pub type DiscnumR = crate::FieldReader;
#[doc = "Field `DISCNUM` writer - ADC group regular sequencer discontinuous number of ranks"]
pub type DiscnumW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JDISCEN` reader - ADC group injected sequencer discontinuous mode"]
pub type JdiscenR = crate::BitReader;
#[doc = "Field `JDISCEN` writer - ADC group injected sequencer discontinuous mode"]
pub type JdiscenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQM` reader - ADC group injected contexts queue mode"]
pub type JqmR = crate::BitReader;
#[doc = "Field `JQM` writer - ADC group injected contexts queue mode"]
pub type JqmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1SGL` reader - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type Awd1sglR = crate::BitReader;
#[doc = "Field `AWD1SGL` writer - ADC analog watchdog 1 monitoring a single channel or all channels"]
pub type Awd1sglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type Awd1enR = crate::BitReader;
#[doc = "Field `AWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group regular"]
pub type Awd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAWD1EN` reader - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type Jawd1enR = crate::BitReader;
#[doc = "Field `JAWD1EN` writer - ADC analog watchdog 1 enable on scope ADC group injected"]
pub type Jawd1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JAUTO` reader - ADC group injected automatic trigger mode"]
pub type JautoR = crate::BitReader;
#[doc = "Field `JAUTO` writer - ADC group injected automatic trigger mode"]
pub type JautoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWDCH1CH` reader - ADC analog watchdog 1 monitored channel selection"]
pub type Awdch1chR = crate::FieldReader;
#[doc = "Field `AWDCH1CH` writer - ADC analog watchdog 1 monitored channel selection"]
pub type Awdch1chW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JQDIS` reader - ADC group injected contexts queue disable"]
pub type JqdisR = crate::BitReader;
#[doc = "Field `JQDIS` writer - ADC group injected contexts queue disable"]
pub type JqdisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmngt(&self) -> DmngtR {
        DmngtR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&self) -> ResR {
        ResR::new(((self.bits >> 2) & 7) as u8)
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&self) -> ExtselR {
        ExtselR::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&self) -> ExtenR {
        ExtenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&self) -> OvrmodR {
        OvrmodR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&self) -> ContR {
        ContR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&self) -> AutdlyR {
        AutdlyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&self) -> DiscenR {
        DiscenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&self) -> DiscnumR {
        DiscnumR::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&self) -> JdiscenR {
        JdiscenR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&self) -> JqmR {
        JqmR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&self) -> Awd1sglR {
        Awd1sglR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&self) -> Awd1enR {
        Awd1enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&self) -> Jawd1enR {
        Jawd1enR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&self) -> JautoR {
        JautoR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awdch1ch(&self) -> Awdch1chR {
        Awdch1chR::new(((self.bits >> 26) & 0x1f) as u8)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&self) -> JqdisR {
        JqdisR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC DMA transfer enable"]
    #[inline(always)]
    pub fn dmngt(&mut self) -> DmngtW<CfgrSpec> {
        DmngtW::new(self, 0)
    }
    #[doc = "Bits 2:4 - ADC data resolution"]
    #[inline(always)]
    pub fn res(&mut self) -> ResW<CfgrSpec> {
        ResW::new(self, 2)
    }
    #[doc = "Bits 5:9 - ADC group regular external trigger source"]
    #[inline(always)]
    pub fn extsel(&mut self) -> ExtselW<CfgrSpec> {
        ExtselW::new(self, 5)
    }
    #[doc = "Bits 10:11 - ADC group regular external trigger polarity"]
    #[inline(always)]
    pub fn exten(&mut self) -> ExtenW<CfgrSpec> {
        ExtenW::new(self, 10)
    }
    #[doc = "Bit 12 - ADC group regular overrun configuration"]
    #[inline(always)]
    pub fn ovrmod(&mut self) -> OvrmodW<CfgrSpec> {
        OvrmodW::new(self, 12)
    }
    #[doc = "Bit 13 - ADC group regular continuous conversion mode"]
    #[inline(always)]
    pub fn cont(&mut self) -> ContW<CfgrSpec> {
        ContW::new(self, 13)
    }
    #[doc = "Bit 14 - ADC low power auto wait"]
    #[inline(always)]
    pub fn autdly(&mut self) -> AutdlyW<CfgrSpec> {
        AutdlyW::new(self, 14)
    }
    #[doc = "Bit 16 - ADC group regular sequencer discontinuous mode"]
    #[inline(always)]
    pub fn discen(&mut self) -> DiscenW<CfgrSpec> {
        DiscenW::new(self, 16)
    }
    #[doc = "Bits 17:19 - ADC group regular sequencer discontinuous number of ranks"]
    #[inline(always)]
    pub fn discnum(&mut self) -> DiscnumW<CfgrSpec> {
        DiscnumW::new(self, 17)
    }
    #[doc = "Bit 20 - ADC group injected sequencer discontinuous mode"]
    #[inline(always)]
    pub fn jdiscen(&mut self) -> JdiscenW<CfgrSpec> {
        JdiscenW::new(self, 20)
    }
    #[doc = "Bit 21 - ADC group injected contexts queue mode"]
    #[inline(always)]
    pub fn jqm(&mut self) -> JqmW<CfgrSpec> {
        JqmW::new(self, 21)
    }
    #[doc = "Bit 22 - ADC analog watchdog 1 monitoring a single channel or all channels"]
    #[inline(always)]
    pub fn awd1sgl(&mut self) -> Awd1sglW<CfgrSpec> {
        Awd1sglW::new(self, 22)
    }
    #[doc = "Bit 23 - ADC analog watchdog 1 enable on scope ADC group regular"]
    #[inline(always)]
    pub fn awd1en(&mut self) -> Awd1enW<CfgrSpec> {
        Awd1enW::new(self, 23)
    }
    #[doc = "Bit 24 - ADC analog watchdog 1 enable on scope ADC group injected"]
    #[inline(always)]
    pub fn jawd1en(&mut self) -> Jawd1enW<CfgrSpec> {
        Jawd1enW::new(self, 24)
    }
    #[doc = "Bit 25 - ADC group injected automatic trigger mode"]
    #[inline(always)]
    pub fn jauto(&mut self) -> JautoW<CfgrSpec> {
        JautoW::new(self, 25)
    }
    #[doc = "Bits 26:30 - ADC analog watchdog 1 monitored channel selection"]
    #[inline(always)]
    pub fn awdch1ch(&mut self) -> Awdch1chW<CfgrSpec> {
        Awdch1chW::new(self, 26)
    }
    #[doc = "Bit 31 - ADC group injected contexts queue disable"]
    #[inline(always)]
    pub fn jqdis(&mut self) -> JqdisW<CfgrSpec> {
        JqdisW::new(self, 31)
    }
}
#[doc = "ADC configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
