#[doc = "Register `CREL` reader"]
pub type R = crate::R<CrelSpec>;
#[doc = "Register `CREL` writer"]
pub type W = crate::W<CrelSpec>;
#[doc = "Field `DAY` reader - Time Stamp Day"]
pub type DayR = crate::FieldReader;
#[doc = "Field `DAY` writer - Time Stamp Day"]
pub type DayW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `MON` reader - Time Stamp Month"]
pub type MonR = crate::FieldReader;
#[doc = "Field `MON` writer - Time Stamp Month"]
pub type MonW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `YEAR` reader - Time Stamp Year"]
pub type YearR = crate::FieldReader;
#[doc = "Field `YEAR` writer - Time Stamp Year"]
pub type YearW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SUBSTEP` reader - Sub-step of Core Release"]
pub type SubstepR = crate::FieldReader;
#[doc = "Field `SUBSTEP` writer - Sub-step of Core Release"]
pub type SubstepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `STEP` reader - Step of Core Release"]
pub type StepR = crate::FieldReader;
#[doc = "Field `STEP` writer - Step of Core Release"]
pub type StepW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `REL` reader - Core Release"]
pub type RelR = crate::FieldReader;
#[doc = "Field `REL` writer - Core Release"]
pub type RelW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    pub fn day(&self) -> DayR {
        DayR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    pub fn year(&self) -> YearR {
        YearR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&self) -> SubstepR {
        SubstepR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&self) -> StepR {
        StepR::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&self) -> RelR {
        RelR::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Time Stamp Day"]
    #[inline(always)]
    pub fn day(&mut self) -> DayW<CrelSpec> {
        DayW::new(self, 0)
    }
    #[doc = "Bits 8:15 - Time Stamp Month"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<CrelSpec> {
        MonW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Time Stamp Year"]
    #[inline(always)]
    pub fn year(&mut self) -> YearW<CrelSpec> {
        YearW::new(self, 16)
    }
    #[doc = "Bits 20:23 - Sub-step of Core Release"]
    #[inline(always)]
    pub fn substep(&mut self) -> SubstepW<CrelSpec> {
        SubstepW::new(self, 20)
    }
    #[doc = "Bits 24:27 - Step of Core Release"]
    #[inline(always)]
    pub fn step(&mut self) -> StepW<CrelSpec> {
        StepW::new(self, 24)
    }
    #[doc = "Bits 28:31 - Core Release"]
    #[inline(always)]
    pub fn rel(&mut self) -> RelW<CrelSpec> {
        RelW::new(self, 28)
    }
}
#[doc = "Clock Calibration Unit Core Release Register\n\nYou can [`read`](crate::Reg::read) this register and get [`crel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrelSpec;
impl crate::RegisterSpec for CrelSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crel::R`](R) reader structure"]
impl crate::Readable for CrelSpec {}
#[doc = "`write(|w| ..)` method takes [`crel::W`](W) writer structure"]
impl crate::Writable for CrelSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CREL to value 0"]
impl crate::Resettable for CrelSpec {}
