#[doc = "Register `CFGR1` reader"]
pub type R = crate::R<Cfgr1Spec>;
#[doc = "Register `CFGR1` writer"]
pub type W = crate::W<Cfgr1Spec>;
#[doc = "Field `EN` reader - COMP channel 1 enable bit"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - COMP channel 1 enable bit"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRGEN` reader - Scaler bridge enable"]
pub type BrgenR = crate::BitReader;
#[doc = "Field `BRGEN` writer - Scaler bridge enable"]
pub type BrgenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SCALEN` reader - Voltage scaler enable bit"]
pub type ScalenR = crate::BitReader;
#[doc = "Field `SCALEN` writer - Voltage scaler enable bit"]
pub type ScalenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POLARITY` reader - COMP channel 1 polarity selection bit"]
pub type PolarityR = crate::BitReader;
#[doc = "Field `POLARITY` writer - COMP channel 1 polarity selection bit"]
pub type PolarityW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITEN` reader - COMP channel 1 interrupt enable"]
pub type ItenR = crate::BitReader;
#[doc = "Field `ITEN` writer - COMP channel 1 interrupt enable"]
pub type ItenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HYST` reader - COMP channel 1 hysteresis selection bits"]
pub type HystR = crate::FieldReader;
#[doc = "Field `HYST` writer - COMP channel 1 hysteresis selection bits"]
pub type HystW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PWRMODE` reader - Power Mode of the COMP channel 1"]
pub type PwrmodeR = crate::FieldReader;
#[doc = "Field `PWRMODE` writer - Power Mode of the COMP channel 1"]
pub type PwrmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `INMSEL` reader - COMP channel 1 inverting input selection field"]
pub type InmselR = crate::FieldReader;
#[doc = "Field `INMSEL` writer - COMP channel 1 inverting input selection field"]
pub type InmselW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `INPSEL` reader - COMP channel 1 non-inverting input selection bit"]
pub type InpselR = crate::BitReader;
#[doc = "Field `INPSEL` writer - COMP channel 1 non-inverting input selection bit"]
pub type InpselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLANKING` reader - COMP channel 1 blanking source selection bits"]
pub type BlankingR = crate::FieldReader;
#[doc = "Field `BLANKING` writer - COMP channel 1 blanking source selection bits"]
pub type BlankingW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `LOCK` reader - Lock bit"]
pub type LockR = crate::BitReader;
#[doc = "Field `LOCK` writer - Lock bit"]
pub type LockW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&self) -> BrgenR {
        BrgenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&self) -> ScalenR {
        ScalenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - COMP channel 1 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&self) -> PolarityR {
        PolarityR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - COMP channel 1 interrupt enable"]
    #[inline(always)]
    pub fn iten(&self) -> ItenR {
        ItenR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:9 - COMP channel 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&self) -> HystR {
        HystR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Power Mode of the COMP channel 1"]
    #[inline(always)]
    pub fn pwrmode(&self) -> PwrmodeR {
        PwrmodeR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 16:18 - COMP channel 1 inverting input selection field"]
    #[inline(always)]
    pub fn inmsel(&self) -> InmselR {
        InmselR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 20 - COMP channel 1 non-inverting input selection bit"]
    #[inline(always)]
    pub fn inpsel(&self) -> InpselR {
        InpselR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 24:27 - COMP channel 1 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&self) -> BlankingR {
        BlankingR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Lock bit"]
    #[inline(always)]
    pub fn lock(&self) -> LockR {
        LockR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - COMP channel 1 enable bit"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Cfgr1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Scaler bridge enable"]
    #[inline(always)]
    pub fn brgen(&mut self) -> BrgenW<Cfgr1Spec> {
        BrgenW::new(self, 1)
    }
    #[doc = "Bit 2 - Voltage scaler enable bit"]
    #[inline(always)]
    pub fn scalen(&mut self) -> ScalenW<Cfgr1Spec> {
        ScalenW::new(self, 2)
    }
    #[doc = "Bit 3 - COMP channel 1 polarity selection bit"]
    #[inline(always)]
    pub fn polarity(&mut self) -> PolarityW<Cfgr1Spec> {
        PolarityW::new(self, 3)
    }
    #[doc = "Bit 6 - COMP channel 1 interrupt enable"]
    #[inline(always)]
    pub fn iten(&mut self) -> ItenW<Cfgr1Spec> {
        ItenW::new(self, 6)
    }
    #[doc = "Bits 8:9 - COMP channel 1 hysteresis selection bits"]
    #[inline(always)]
    pub fn hyst(&mut self) -> HystW<Cfgr1Spec> {
        HystW::new(self, 8)
    }
    #[doc = "Bits 12:13 - Power Mode of the COMP channel 1"]
    #[inline(always)]
    pub fn pwrmode(&mut self) -> PwrmodeW<Cfgr1Spec> {
        PwrmodeW::new(self, 12)
    }
    #[doc = "Bits 16:18 - COMP channel 1 inverting input selection field"]
    #[inline(always)]
    pub fn inmsel(&mut self) -> InmselW<Cfgr1Spec> {
        InmselW::new(self, 16)
    }
    #[doc = "Bit 20 - COMP channel 1 non-inverting input selection bit"]
    #[inline(always)]
    pub fn inpsel(&mut self) -> InpselW<Cfgr1Spec> {
        InpselW::new(self, 20)
    }
    #[doc = "Bits 24:27 - COMP channel 1 blanking source selection bits"]
    #[inline(always)]
    pub fn blanking(&mut self) -> BlankingW<Cfgr1Spec> {
        BlankingW::new(self, 24)
    }
    #[doc = "Bit 31 - Lock bit"]
    #[inline(always)]
    pub fn lock(&mut self) -> LockW<Cfgr1Spec> {
        LockW::new(self, 31)
    }
}
#[doc = "Comparator configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cfgr1Spec;
impl crate::RegisterSpec for Cfgr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr1::R`](R) reader structure"]
impl crate::Readable for Cfgr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cfgr1::W`](W) writer structure"]
impl crate::Writable for Cfgr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR1 to value 0"]
impl crate::Resettable for Cfgr1Spec {}
