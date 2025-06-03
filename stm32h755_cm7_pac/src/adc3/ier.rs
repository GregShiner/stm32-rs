#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `ADRDYIE` reader - ADC ready interrupt"]
pub type AdrdyieR = crate::BitReader;
#[doc = "Field `ADRDYIE` writer - ADC ready interrupt"]
pub type AdrdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSMPIE` reader - ADC group regular end of sampling interrupt"]
pub type EosmpieR = crate::BitReader;
#[doc = "Field `EOSMPIE` writer - ADC group regular end of sampling interrupt"]
pub type EosmpieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOCIE` reader - ADC group regular end of unitary conversion interrupt"]
pub type EocieR = crate::BitReader;
#[doc = "Field `EOCIE` writer - ADC group regular end of unitary conversion interrupt"]
pub type EocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EOSIE` reader - ADC group regular end of sequence conversions interrupt"]
pub type EosieR = crate::BitReader;
#[doc = "Field `EOSIE` writer - ADC group regular end of sequence conversions interrupt"]
pub type EosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - ADC group regular overrun interrupt"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - ADC group regular overrun interrupt"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOCIE` reader - ADC group injected end of unitary conversion interrupt"]
pub type JeocieR = crate::BitReader;
#[doc = "Field `JEOCIE` writer - ADC group injected end of unitary conversion interrupt"]
pub type JeocieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JEOSIE` reader - ADC group injected end of sequence conversions interrupt"]
pub type JeosieR = crate::BitReader;
#[doc = "Field `JEOSIE` writer - ADC group injected end of sequence conversions interrupt"]
pub type JeosieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD1IE` reader - ADC analog watchdog 1 interrupt"]
pub type Awd1ieR = crate::BitReader;
#[doc = "Field `AWD1IE` writer - ADC analog watchdog 1 interrupt"]
pub type Awd1ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD2IE` reader - ADC analog watchdog 2 interrupt"]
pub type Awd2ieR = crate::BitReader;
#[doc = "Field `AWD2IE` writer - ADC analog watchdog 2 interrupt"]
pub type Awd2ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWD3IE` reader - ADC analog watchdog 3 interrupt"]
pub type Awd3ieR = crate::BitReader;
#[doc = "Field `AWD3IE` writer - ADC analog watchdog 3 interrupt"]
pub type Awd3ieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JQOVFIE` reader - ADC group injected contexts queue overflow interrupt"]
pub type JqovfieR = crate::BitReader;
#[doc = "Field `JQOVFIE` writer - ADC group injected contexts queue overflow interrupt"]
pub type JqovfieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&self) -> AdrdyieR {
        AdrdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&self) -> EosmpieR {
        EosmpieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&self) -> EocieR {
        EocieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&self) -> EosieR {
        EosieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn jeocie(&self) -> JeocieR {
        JeocieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn jeosie(&self) -> JeosieR {
        JeosieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&self) -> Awd1ieR {
        Awd1ieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&self) -> Awd2ieR {
        Awd2ieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&self) -> Awd3ieR {
        Awd3ieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    pub fn jqovfie(&self) -> JqovfieR {
        JqovfieR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADC ready interrupt"]
    #[inline(always)]
    pub fn adrdyie(&mut self) -> AdrdyieW<IerSpec> {
        AdrdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - ADC group regular end of sampling interrupt"]
    #[inline(always)]
    pub fn eosmpie(&mut self) -> EosmpieW<IerSpec> {
        EosmpieW::new(self, 1)
    }
    #[doc = "Bit 2 - ADC group regular end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn eocie(&mut self) -> EocieW<IerSpec> {
        EocieW::new(self, 2)
    }
    #[doc = "Bit 3 - ADC group regular end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn eosie(&mut self) -> EosieW<IerSpec> {
        EosieW::new(self, 3)
    }
    #[doc = "Bit 4 - ADC group regular overrun interrupt"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<IerSpec> {
        OvrieW::new(self, 4)
    }
    #[doc = "Bit 5 - ADC group injected end of unitary conversion interrupt"]
    #[inline(always)]
    pub fn jeocie(&mut self) -> JeocieW<IerSpec> {
        JeocieW::new(self, 5)
    }
    #[doc = "Bit 6 - ADC group injected end of sequence conversions interrupt"]
    #[inline(always)]
    pub fn jeosie(&mut self) -> JeosieW<IerSpec> {
        JeosieW::new(self, 6)
    }
    #[doc = "Bit 7 - ADC analog watchdog 1 interrupt"]
    #[inline(always)]
    pub fn awd1ie(&mut self) -> Awd1ieW<IerSpec> {
        Awd1ieW::new(self, 7)
    }
    #[doc = "Bit 8 - ADC analog watchdog 2 interrupt"]
    #[inline(always)]
    pub fn awd2ie(&mut self) -> Awd2ieW<IerSpec> {
        Awd2ieW::new(self, 8)
    }
    #[doc = "Bit 9 - ADC analog watchdog 3 interrupt"]
    #[inline(always)]
    pub fn awd3ie(&mut self) -> Awd3ieW<IerSpec> {
        Awd3ieW::new(self, 9)
    }
    #[doc = "Bit 10 - ADC group injected contexts queue overflow interrupt"]
    #[inline(always)]
    pub fn jqovfie(&mut self) -> JqovfieW<IerSpec> {
        JqovfieW::new(self, 10)
    }
}
#[doc = "ADC interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
