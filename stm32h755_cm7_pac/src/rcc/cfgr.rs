#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `SW` reader - System clock switch"]
pub type SwR = crate::FieldReader;
#[doc = "Field `SW` writer - System clock switch"]
pub type SwW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SWS` reader - System clock switch status"]
pub type SwsR = crate::FieldReader;
#[doc = "Field `SWS` writer - System clock switch status"]
pub type SwsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STOPWUCK` reader - System clock selection after a wake up from system Stop"]
pub type StopwuckR = crate::BitReader;
#[doc = "Field `STOPWUCK` writer - System clock selection after a wake up from system Stop"]
pub type StopwuckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STOPKERWUCK` reader - Kernel clock selection after a wake up from system Stop"]
pub type StopkerwuckR = crate::BitReader;
#[doc = "Field `STOPKERWUCK` writer - Kernel clock selection after a wake up from system Stop"]
pub type StopkerwuckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTCPRE` reader - HSE division factor for RTC clock"]
pub type RtcpreR = crate::FieldReader;
#[doc = "Field `RTCPRE` writer - HSE division factor for RTC clock"]
pub type RtcpreW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `HRTIMSEL` reader - High Resolution Timer clock prescaler selection"]
pub type HrtimselR = crate::BitReader;
#[doc = "Field `HRTIMSEL` writer - High Resolution Timer clock prescaler selection"]
pub type HrtimselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMPRE` reader - Timers clocks prescaler selection"]
pub type TimpreR = crate::BitReader;
#[doc = "Field `TIMPRE` writer - Timers clocks prescaler selection"]
pub type TimpreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCO1PRE` reader - MCO1 prescaler"]
pub type Mco1preR = crate::FieldReader;
#[doc = "Field `MCO1PRE` writer - MCO1 prescaler"]
pub type Mco1preW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO1SEL` reader - Micro-controller clock output 1"]
pub type Mco1selR = crate::FieldReader;
#[doc = "Field `MCO1SEL` writer - Micro-controller clock output 1"]
pub type Mco1selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MCO2PRE` reader - MCO2 prescaler"]
pub type Mco2preR = crate::FieldReader;
#[doc = "Field `MCO2PRE` writer - MCO2 prescaler"]
pub type Mco2preW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `MCO2SEL` reader - Micro-controller clock output 2"]
pub type Mco2selR = crate::FieldReader;
#[doc = "Field `MCO2SEL` writer - Micro-controller clock output 2"]
pub type Mco2selW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&self) -> SwR {
        SwR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - System clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopwuck(&self) -> StopwuckR {
        StopwuckR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Kernel clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopkerwuck(&self) -> StopkerwuckR {
        StopkerwuckR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&self) -> RtcpreR {
        RtcpreR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 14 - High Resolution Timer clock prescaler selection"]
    #[inline(always)]
    pub fn hrtimsel(&self) -> HrtimselR {
        HrtimselR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Timers clocks prescaler selection"]
    #[inline(always)]
    pub fn timpre(&self) -> TimpreR {
        TimpreR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&self) -> Mco1preR {
        Mco1preR::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:24 - Micro-controller clock output 1"]
    #[inline(always)]
    pub fn mco1sel(&self) -> Mco1selR {
        Mco1selR::new(((self.bits >> 22) & 7) as u8)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&self) -> Mco2preR {
        Mco2preR::new(((self.bits >> 25) & 0x0f) as u8)
    }
    #[doc = "Bits 29:31 - Micro-controller clock output 2"]
    #[inline(always)]
    pub fn mco2sel(&self) -> Mco2selR {
        Mco2selR::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - System clock switch"]
    #[inline(always)]
    pub fn sw(&mut self) -> SwW<CfgrSpec> {
        SwW::new(self, 0)
    }
    #[doc = "Bits 3:5 - System clock switch status"]
    #[inline(always)]
    pub fn sws(&mut self) -> SwsW<CfgrSpec> {
        SwsW::new(self, 3)
    }
    #[doc = "Bit 6 - System clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopwuck(&mut self) -> StopwuckW<CfgrSpec> {
        StopwuckW::new(self, 6)
    }
    #[doc = "Bit 7 - Kernel clock selection after a wake up from system Stop"]
    #[inline(always)]
    pub fn stopkerwuck(&mut self) -> StopkerwuckW<CfgrSpec> {
        StopkerwuckW::new(self, 7)
    }
    #[doc = "Bits 8:13 - HSE division factor for RTC clock"]
    #[inline(always)]
    pub fn rtcpre(&mut self) -> RtcpreW<CfgrSpec> {
        RtcpreW::new(self, 8)
    }
    #[doc = "Bit 14 - High Resolution Timer clock prescaler selection"]
    #[inline(always)]
    pub fn hrtimsel(&mut self) -> HrtimselW<CfgrSpec> {
        HrtimselW::new(self, 14)
    }
    #[doc = "Bit 15 - Timers clocks prescaler selection"]
    #[inline(always)]
    pub fn timpre(&mut self) -> TimpreW<CfgrSpec> {
        TimpreW::new(self, 15)
    }
    #[doc = "Bits 18:21 - MCO1 prescaler"]
    #[inline(always)]
    pub fn mco1pre(&mut self) -> Mco1preW<CfgrSpec> {
        Mco1preW::new(self, 18)
    }
    #[doc = "Bits 22:24 - Micro-controller clock output 1"]
    #[inline(always)]
    pub fn mco1sel(&mut self) -> Mco1selW<CfgrSpec> {
        Mco1selW::new(self, 22)
    }
    #[doc = "Bits 25:28 - MCO2 prescaler"]
    #[inline(always)]
    pub fn mco2pre(&mut self) -> Mco2preW<CfgrSpec> {
        Mco2preW::new(self, 25)
    }
    #[doc = "Bits 29:31 - Micro-controller clock output 2"]
    #[inline(always)]
    pub fn mco2sel(&mut self) -> Mco2selW<CfgrSpec> {
        Mco2selW::new(self, 29)
    }
}
#[doc = "RCC Clock Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
