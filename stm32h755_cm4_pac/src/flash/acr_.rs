#[doc = "Register `ACR_` reader"]
pub type R = crate::R<Acr_Spec>;
#[doc = "Register `ACR_` writer"]
pub type W = crate::W<Acr_Spec>;
#[doc = "Field `LATENCY` reader - Read latency"]
pub type LatencyR = crate::FieldReader;
#[doc = "Field `LATENCY` writer - Read latency"]
pub type LatencyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `WRHIGHFREQ` reader - Flash signal delay"]
pub type WrhighfreqR = crate::FieldReader;
#[doc = "Field `WRHIGHFREQ` writer - Flash signal delay"]
pub type WrhighfreqW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    pub fn latency(&self) -> LatencyR {
        LatencyR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&self) -> WrhighfreqR {
        WrhighfreqR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Read latency"]
    #[inline(always)]
    pub fn latency(&mut self) -> LatencyW<Acr_Spec> {
        LatencyW::new(self, 0)
    }
    #[doc = "Bits 4:5 - Flash signal delay"]
    #[inline(always)]
    pub fn wrhighfreq(&mut self) -> WrhighfreqW<Acr_Spec> {
        WrhighfreqW::new(self, 4)
    }
}
#[doc = "Access control register\n\nYou can [`read`](crate::Reg::read) this register and get [`acr_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acr_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Acr_Spec;
impl crate::RegisterSpec for Acr_Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`acr_::R`](R) reader structure"]
impl crate::Readable for Acr_Spec {}
#[doc = "`write(|w| ..)` method takes [`acr_::W`](W) writer structure"]
impl crate::Writable for Acr_Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ACR_ to value 0"]
impl crate::Resettable for Acr_Spec {}
