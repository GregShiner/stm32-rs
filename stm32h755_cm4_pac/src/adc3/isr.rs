#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `ADRDY` reader - ADC ready flag"]
pub type AdrdyR = crate::BitReader;
#[doc = "Field `ADRDY` writer - ADC ready flag"]
pub type AdrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMP` reader - ADC group regular end of sampling flag"]
pub type EosmpR = crate::BitReader;
#[doc = "Field `EOSMP` writer - ADC group regular end of sampling flag"]
pub type EosmpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOC` reader - ADC group regular end of unitary conversion flag"]
pub type EocR = crate::BitReader;
#[doc = "Field `EOC` writer - ADC group regular end of unitary conversion flag"]
pub type EocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOS` reader - ADC group regular end of sequence conversions flag"]
pub type EosR = crate::BitReader;
#[doc = "Field `EOS` writer - ADC group regular end of sequence conversions flag"]
pub type EosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVR` reader - ADC group regular overrun flag"]
pub type OvrR = crate::BitReader;
#[doc = "Field `OVR` writer - ADC group regular overrun flag"]
pub type OvrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOC` reader - ADC group injected end of unitary conversion flag"]
pub type JeocR = crate::BitReader;
#[doc = "Field `JEOC` writer - ADC group injected end of unitary conversion flag"]
pub type JeocW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOS` reader - ADC group injected end of sequence conversions flag"]
pub type JeosR = crate::BitReader;
#[doc = "Field `JEOS` writer - ADC group injected end of sequence conversions flag"]
pub type JeosW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1` reader - ADC analog watchdog 1 flag"]
pub type Awd1R = crate::BitReader;
#[doc = "Field `AWD1` writer - ADC analog watchdog 1 flag"]
pub type Awd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2` reader - ADC analog watchdog 2 flag"]
pub type Awd2R = crate::BitReader;
#[doc = "Field `AWD2` writer - ADC analog watchdog 2 flag"]
pub type Awd2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3` reader - ADC analog watchdog 3 flag"]
pub type Awd3R = crate::BitReader;
#[doc = "Field `AWD3` writer - ADC analog watchdog 3 flag"]
pub type Awd3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVF` reader - ADC group injected contexts queue overflow flag"]
pub type JqovfR = crate::BitReader;
#[doc = "Field `JQOVF` writer - ADC group injected contexts queue overflow flag"]
pub type JqovfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready flag"]
    #[inline(always)]
    pub fn adrdy(&self) -> AdrdyR {
        AdrdyR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&self) -> EosmpR {
        EosmpR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&self) -> EocR {
        EocR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eos(&self) -> EosR {
        EosR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OvrR {
        OvrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion flag"]
    #[inline(always)]
    pub fn jeoc(&self) -> JeocR {
        JeocR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions flag"]
    #[inline(always)]
    pub fn jeos(&self) -> JeosR {
        JeosR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 flag"]
    #[inline(always)]
    pub fn awd1(&self) -> Awd1R {
        Awd1R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 flag"]
    #[inline(always)]
    pub fn awd2(&self) -> Awd2R {
        Awd2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 flag"]
    #[inline(always)]
    pub fn awd3(&self) -> Awd3R {
        Awd3R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow flag"]
    #[inline(always)]
    pub fn jqovf(&self) -> JqovfR {
        JqovfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready flag"]
    #[inline(always)]
    pub fn adrdy(&mut self) -> AdrdyW<IsrSpec> {
        AdrdyW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling flag"]
    #[inline(always)]
    pub fn eosmp(&mut self) -> EosmpW<IsrSpec> {
        EosmpW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion flag"]
    #[inline(always)]
    pub fn eoc(&mut self) -> EocW<IsrSpec> {
        EocW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions flag"]
    #[inline(always)]
    pub fn eos(&mut self) -> EosW<IsrSpec> {
        EosW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun flag"]
    #[inline(always)]
    pub fn ovr(&mut self) -> OvrW<IsrSpec> {
        OvrW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion flag"]
    #[inline(always)]
    pub fn jeoc(&mut self) -> JeocW<IsrSpec> {
        JeocW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions flag"]
    #[inline(always)]
    pub fn jeos(&mut self) -> JeosW<IsrSpec> {
        JeosW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 flag"]
    #[inline(always)]
    pub fn awd1(&mut self) -> Awd1W<IsrSpec> {
        Awd1W::new(self, 7)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 flag"]
    #[inline(always)]
    pub fn awd2(&mut self) -> Awd2W<IsrSpec> {
        Awd2W::new(self, 8)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 flag"]
    #[inline(always)]
    pub fn awd3(&mut self) -> Awd3W<IsrSpec> {
        Awd3W::new(self, 9)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow flag"]
    #[inline(always)]
    pub fn jqovf(&mut self) -> JqovfW<IsrSpec> {
        JqovfW::new(self, 10)
    }
}
#[doc = "ADC interrupt and status register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
