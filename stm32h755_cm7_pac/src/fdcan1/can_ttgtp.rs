#[doc = "Register `CAN_TTGTP` reader"]
pub type R = crate::R<CanTtgtpSpec>;
#[doc = "Register `CAN_TTGTP` writer"]
pub type W = crate::W<CanTtgtpSpec>;
#[doc = "Field `NCL` reader - Time Preset"]
pub type NclR = crate::FieldReader<u16>;
#[doc = "Field `NCL` writer - Time Preset"]
pub type NclW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CTP` reader - Cycle Time Target Phase"]
pub type CtpR = crate::FieldReader<u16>;
#[doc = "Field `CTP` writer - Cycle Time Target Phase"]
pub type CtpW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&self) -> NclR {
        NclR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&self) -> CtpR {
        CtpR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Time Preset"]
    #[inline(always)]
    pub fn ncl(&mut self) -> NclW<CanTtgtpSpec> {
        NclW::new(self, 0)
    }
    #[doc = "Bits 16:31 - Cycle Time Target Phase"]
    #[inline(always)]
    pub fn ctp(&mut self) -> CtpW<CanTtgtpSpec> {
        CtpW::new(self, 16)
    }
}
#[doc = "FDCAN TT Global Time Preset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`can_ttgtp::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`can_ttgtp::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CanTtgtpSpec;
impl crate::RegisterSpec for CanTtgtpSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`can_ttgtp::R`](R) reader structure"]
impl crate::Readable for CanTtgtpSpec {}
#[doc = "`write(|w| ..)` method takes [`can_ttgtp::W`](W) writer structure"]
impl crate::Writable for CanTtgtpSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAN_TTGTP to value 0"]
impl crate::Resettable for CanTtgtpSpec {}
