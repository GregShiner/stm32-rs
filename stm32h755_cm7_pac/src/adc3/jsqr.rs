#[doc = "Register `JSQR` reader"]
pub type R = crate::R<JsqrSpec>;
#[doc = "Register `JSQR` writer"]
pub type W = crate::W<JsqrSpec>;
#[doc = "Field `JL` reader - ADC group injected sequencer scan length"]
pub type JlR = crate::FieldReader;
#[doc = "Field `JL` writer - ADC group injected sequencer scan length"]
pub type JlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JEXTSEL` reader - ADC group injected external trigger source"]
pub type JextselR = crate::FieldReader;
#[doc = "Field `JEXTSEL` writer - ADC group injected external trigger source"]
pub type JextselW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JEXTEN` reader - ADC group injected external trigger polarity"]
pub type JextenR = crate::FieldReader;
#[doc = "Field `JEXTEN` writer - ADC group injected external trigger polarity"]
pub type JextenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `JSQ1` reader - ADC group injected sequencer rank 1"]
pub type Jsq1R = crate::FieldReader;
#[doc = "Field `JSQ1` writer - ADC group injected sequencer rank 1"]
pub type Jsq1W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ2` reader - ADC group injected sequencer rank 2"]
pub type Jsq2R = crate::FieldReader;
#[doc = "Field `JSQ2` writer - ADC group injected sequencer rank 2"]
pub type Jsq2W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ3` reader - ADC group injected sequencer rank 3"]
pub type Jsq3R = crate::FieldReader;
#[doc = "Field `JSQ3` writer - ADC group injected sequencer rank 3"]
pub type Jsq3W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `JSQ4` reader - ADC group injected sequencer rank 4"]
pub type Jsq4R = crate::FieldReader;
#[doc = "Field `JSQ4` writer - ADC group injected sequencer rank 4"]
pub type Jsq4W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    pub fn jl(&self) -> JlR {
        JlR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:6 - ADC group injected external trigger source"]
    #[inline(always)]
    pub fn jextsel(&self) -> JextselR {
        JextselR::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:8 - ADC group injected external trigger polarity"]
    #[inline(always)]
    pub fn jexten(&self) -> JextenR {
        JextenR::new(((self.bits >> 7) & 3) as u8)
    }
    #[doc = "Bits 9:13 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub fn jsq1(&self) -> Jsq1R {
        Jsq1R::new(((self.bits >> 9) & 0x1f) as u8)
    }
    #[doc = "Bits 15:19 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub fn jsq2(&self) -> Jsq2R {
        Jsq2R::new(((self.bits >> 15) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub fn jsq3(&self) -> Jsq3R {
        Jsq3R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bits 27:31 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub fn jsq4(&self) -> Jsq4R {
        Jsq4R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - ADC group injected sequencer scan length"]
    #[inline(always)]
    pub fn jl(&mut self) -> JlW<JsqrSpec> {
        JlW::new(self, 0)
    }
    #[doc = "Bits 2:6 - ADC group injected external trigger source"]
    #[inline(always)]
    pub fn jextsel(&mut self) -> JextselW<JsqrSpec> {
        JextselW::new(self, 2)
    }
    #[doc = "Bits 7:8 - ADC group injected external trigger polarity"]
    #[inline(always)]
    pub fn jexten(&mut self) -> JextenW<JsqrSpec> {
        JextenW::new(self, 7)
    }
    #[doc = "Bits 9:13 - ADC group injected sequencer rank 1"]
    #[inline(always)]
    pub fn jsq1(&mut self) -> Jsq1W<JsqrSpec> {
        Jsq1W::new(self, 9)
    }
    #[doc = "Bits 15:19 - ADC group injected sequencer rank 2"]
    #[inline(always)]
    pub fn jsq2(&mut self) -> Jsq2W<JsqrSpec> {
        Jsq2W::new(self, 15)
    }
    #[doc = "Bits 21:25 - ADC group injected sequencer rank 3"]
    #[inline(always)]
    pub fn jsq3(&mut self) -> Jsq3W<JsqrSpec> {
        Jsq3W::new(self, 21)
    }
    #[doc = "Bits 27:31 - ADC group injected sequencer rank 4"]
    #[inline(always)]
    pub fn jsq4(&mut self) -> Jsq4W<JsqrSpec> {
        Jsq4W::new(self, 27)
    }
}
#[doc = "ADC group injected sequencer register\n\nYou can [`read`](crate::Reg::read) this register and get [`jsqr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jsqr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct JsqrSpec;
impl crate::RegisterSpec for JsqrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jsqr::R`](R) reader structure"]
impl crate::Readable for JsqrSpec {}
#[doc = "`write(|w| ..)` method takes [`jsqr::W`](W) writer structure"]
impl crate::Writable for JsqrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JSQR to value 0"]
impl crate::Resettable for JsqrSpec {}
