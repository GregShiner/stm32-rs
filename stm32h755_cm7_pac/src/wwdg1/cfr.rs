#[doc = "Register `CFR` reader"]
pub type R = crate::R<CfrSpec>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CfrSpec>;
#[doc = "Field `W` reader - 7-bit window value These bits contain the window value to be compared to the downcounter."]
pub type WR = crate::FieldReader;
#[doc = "Field `W` writer - 7-bit window value These bits contain the window value to be compared to the downcounter."]
pub type WW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `EWI` reader - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EwiR = crate::BitReader;
#[doc = "Field `EWI` writer - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
pub type EwiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WDGTB` reader - Timer base The time base of the prescaler can be modified as follows:"]
pub type WdgtbR = crate::FieldReader;
#[doc = "Field `WDGTB` writer - Timer base The time base of the prescaler can be modified as follows:"]
pub type WdgtbW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared to the downcounter."]
    #[inline(always)]
    pub fn w(&self) -> WR {
        WR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn ewi(&self) -> EwiR {
        EwiR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Timer base The time base of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&self) -> WdgtbR {
        WdgtbR::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - 7-bit window value These bits contain the window value to be compared to the downcounter."]
    #[inline(always)]
    pub fn w(&mut self) -> WW<CfrSpec> {
        WW::new(self, 0)
    }
    #[doc = "Bit 9 - Early wakeup interrupt When set, an interrupt occurs whenever the counter reaches the value 0x40. This interrupt is only cleared by hardware after a reset."]
    #[inline(always)]
    pub fn ewi(&mut self) -> EwiW<CfrSpec> {
        EwiW::new(self, 9)
    }
    #[doc = "Bits 11:12 - Timer base The time base of the prescaler can be modified as follows:"]
    #[inline(always)]
    pub fn wdgtb(&mut self) -> WdgtbW<CfrSpec> {
        WdgtbW::new(self, 11)
    }
}
#[doc = "Configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfrSpec;
impl crate::RegisterSpec for CfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CfrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFR to value 0x7f"]
impl crate::Resettable for CfrSpec {
    const RESET_VALUE: u32 = 0x7f;
}
