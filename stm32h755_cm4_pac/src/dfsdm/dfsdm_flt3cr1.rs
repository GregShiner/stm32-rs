#[doc = "Register `DFSDM_FLT3CR1` reader"]
pub type R = crate::R<DfsdmFlt3cr1Spec>;
#[doc = "Register `DFSDM_FLT3CR1` writer"]
pub type W = crate::W<DfsdmFlt3cr1Spec>;
#[doc = "Field `DFEN` reader - DFSDM enable"]
pub type DfenR = crate::BitReader;
#[doc = "Field `DFEN` writer - DFSDM enable"]
pub type DfenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSWSTART` reader - Start a conversion of the injected group of channels"]
pub type JswstartR = crate::BitReader;
#[doc = "Field `JSWSTART` writer - Start a conversion of the injected group of channels"]
pub type JswstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSYNC` reader - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JsyncR = crate::BitReader;
#[doc = "Field `JSYNC` writer - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
pub type JsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JSCAN` reader - Scanning conversion mode for injected conversions"]
pub type JscanR = crate::BitReader;
#[doc = "Field `JSCAN` writer - Scanning conversion mode for injected conversions"]
pub type JscanW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JDMAEN` reader - DMA channel enabled to read data for the injected channel group"]
pub type JdmaenR = crate::BitReader;
#[doc = "Field `JDMAEN` writer - DMA channel enabled to read data for the injected channel group"]
pub type JdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEXTSEL` reader - Trigger signal selection for launching injected conversions"]
pub type JextselR = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - Trigger signal selection for launching injected conversions"]
pub type JextselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `JEXTEN` reader - Trigger enable and trigger edge selection for injected conversions"]
pub type JextenR = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - Trigger enable and trigger edge selection for injected conversions"]
pub type JextenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RSWSTART` reader - Software start of a conversion on the regular channel"]
pub type RswstartR = crate::BitReader;
#[doc = "Field `RSWSTART` writer - Software start of a conversion on the regular channel"]
pub type RswstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCONT` reader - Continuous mode selection for regular conversions"]
pub type RcontR = crate::BitReader;
#[doc = "Field `RCONT` writer - Continuous mode selection for regular conversions"]
pub type RcontW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSYNC` reader - Launch regular conversion synchronously with DFSDM0"]
pub type RsyncR = crate::BitReader;
#[doc = "Field `RSYNC` writer - Launch regular conversion synchronously with DFSDM0"]
pub type RsyncW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RDMAEN` reader - DMA channel enabled to read data for the regular conversion"]
pub type RdmaenR = crate::BitReader;
#[doc = "Field `RDMAEN` writer - DMA channel enabled to read data for the regular conversion"]
pub type RdmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RCH` reader - Regular channel selection"]
pub type RchR = crate::FieldReader;
#[doc = "Field `RCH` writer - Regular channel selection"]
pub type RchW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FAST` reader - Fast conversion mode selection for regular conversions"]
pub type FastR = crate::BitReader;
#[doc = "Field `FAST` writer - Fast conversion mode selection for regular conversions"]
pub type FastW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWFSEL` reader - Analog watchdog fast mode select"]
pub type AwfselR = crate::BitReader;
#[doc = "Field `AWFSEL` writer - Analog watchdog fast mode select"]
pub type AwfselW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&self) -> DfenR {
        DfenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&self) -> JswstartR {
        JswstartR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&self) -> JsyncR {
        JsyncR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&self) -> JscanR {
        JscanR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&self) -> JdmaenR {
        JdmaenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&self) -> JextselR {
        JextselR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&self) -> JextenR {
        JextenR::new(((self.bits >> 13) & 3) as u8)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&self) -> RswstartR {
        RswstartR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&self) -> RcontR {
        RcontR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&self) -> RsyncR {
        RsyncR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&self) -> RdmaenR {
        RdmaenR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&self) -> RchR {
        RchR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&self) -> FastR {
        FastR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&self) -> AwfselR {
        AwfselR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - DFSDM enable"]
    #[inline(always)]
    pub fn dfen(&mut self) -> DfenW<DfsdmFlt3cr1Spec> {
        DfenW::new(self, 0)
    }
    #[doc = "Bit 1 - Start a conversion of the injected group of channels"]
    #[inline(always)]
    pub fn jswstart(&mut self) -> JswstartW<DfsdmFlt3cr1Spec> {
        JswstartW::new(self, 1)
    }
    #[doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger"]
    #[inline(always)]
    pub fn jsync(&mut self) -> JsyncW<DfsdmFlt3cr1Spec> {
        JsyncW::new(self, 3)
    }
    #[doc = "Bit 4 - Scanning conversion mode for injected conversions"]
    #[inline(always)]
    pub fn jscan(&mut self) -> JscanW<DfsdmFlt3cr1Spec> {
        JscanW::new(self, 4)
    }
    #[doc = "Bit 5 - DMA channel enabled to read data for the injected channel group"]
    #[inline(always)]
    pub fn jdmaen(&mut self) -> JdmaenW<DfsdmFlt3cr1Spec> {
        JdmaenW::new(self, 5)
    }
    #[doc = "Bits 8:10 - Trigger signal selection for launching injected conversions"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JextselW<DfsdmFlt3cr1Spec> {
        JextselW::new(self, 8)
    }
    #[doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JextenW<DfsdmFlt3cr1Spec> {
        JextenW::new(self, 13)
    }
    #[doc = "Bit 17 - Software start of a conversion on the regular channel"]
    #[inline(always)]
    pub fn rswstart(&mut self) -> RswstartW<DfsdmFlt3cr1Spec> {
        RswstartW::new(self, 17)
    }
    #[doc = "Bit 18 - Continuous mode selection for regular conversions"]
    #[inline(always)]
    pub fn rcont(&mut self) -> RcontW<DfsdmFlt3cr1Spec> {
        RcontW::new(self, 18)
    }
    #[doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0"]
    #[inline(always)]
    pub fn rsync(&mut self) -> RsyncW<DfsdmFlt3cr1Spec> {
        RsyncW::new(self, 19)
    }
    #[doc = "Bit 21 - DMA channel enabled to read data for the regular conversion"]
    #[inline(always)]
    pub fn rdmaen(&mut self) -> RdmaenW<DfsdmFlt3cr1Spec> {
        RdmaenW::new(self, 21)
    }
    #[doc = "Bits 24:26 - Regular channel selection"]
    #[inline(always)]
    pub fn rch(&mut self) -> RchW<DfsdmFlt3cr1Spec> {
        RchW::new(self, 24)
    }
    #[doc = "Bit 29 - Fast conversion mode selection for regular conversions"]
    #[inline(always)]
    pub fn fast(&mut self) -> FastW<DfsdmFlt3cr1Spec> {
        FastW::new(self, 29)
    }
    #[doc = "Bit 30 - Analog watchdog fast mode select"]
    #[inline(always)]
    pub fn awfsel(&mut self) -> AwfselW<DfsdmFlt3cr1Spec> {
        AwfselW::new(self, 30)
    }
}
#[doc = "control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`dfsdm_flt3cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfsdm_flt3cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DfsdmFlt3cr1Spec;
impl crate::RegisterSpec for DfsdmFlt3cr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dfsdm_flt3cr1::R`](R) reader structure"]
impl crate::Readable for DfsdmFlt3cr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dfsdm_flt3cr1::W`](W) writer structure"]
impl crate::Writable for DfsdmFlt3cr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DFSDM_FLT3CR1 to value 0"]
impl crate::Resettable for DfsdmFlt3cr1Spec {}
