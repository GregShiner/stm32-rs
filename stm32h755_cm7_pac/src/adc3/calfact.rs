#[doc = "Register `CALFACT` reader"]
pub type R = crate::R<CalfactSpec>;
#[doc = "Register `CALFACT` writer"]
pub type W = crate::W<CalfactSpec>;
#[doc = "Field `CALFACT_S` reader - ADC calibration factor in single-ended mode"]
pub type CalfactSR = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_S` writer - ADC calibration factor in single-ended mode"]
pub type CalfactSW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CALFACT_D` reader - ADC calibration factor in differential mode"]
pub type CalfactDR = crate::FieldReader<u16>;
#[doc = "Field `CALFACT_D` writer - ADC calibration factor in differential mode"]
pub type CalfactDW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&self) -> CalfactSR {
        CalfactSR::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&self) -> CalfactDR {
        CalfactDR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - ADC calibration factor in single-ended mode"]
    #[inline(always)]
    pub fn calfact_s(&mut self) -> CalfactSW<CalfactSpec> {
        CalfactSW::new(self, 0)
    }
    #[doc = "Bits 16:26 - ADC calibration factor in differential mode"]
    #[inline(always)]
    pub fn calfact_d(&mut self) -> CalfactDW<CalfactSpec> {
        CalfactDW::new(self, 16)
    }
}
#[doc = "ADC calibration factors register\n\nYou can [`read`](crate::Reg::read) this register and get [`calfact::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`calfact::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CalfactSpec;
impl crate::RegisterSpec for CalfactSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`calfact::R`](R) reader structure"]
impl crate::Readable for CalfactSpec {}
#[doc = "`write(|w| ..)` method takes [`calfact::W`](W) writer structure"]
impl crate::Writable for CalfactSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CALFACT to value 0"]
impl crate::Resettable for CalfactSpec {}
