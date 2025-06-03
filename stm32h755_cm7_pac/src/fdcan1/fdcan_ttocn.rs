#[doc = "Register `FDCAN_TTOCN` reader"]
pub type R = crate::R<FdcanTtocnSpec>;
#[doc = "Register `FDCAN_TTOCN` writer"]
pub type W = crate::W<FdcanTtocnSpec>;
#[doc = "Field `SGT` reader - Set Global time"]
pub type SgtR = crate::BitReader;
#[doc = "Field `SGT` writer - Set Global time"]
pub type SgtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECS` reader - External Clock Synchronization"]
pub type EcsR = crate::BitReader;
#[doc = "Field `ECS` writer - External Clock Synchronization"]
pub type EcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWP` reader - Stop Watch Polarity"]
pub type SwpR = crate::BitReader;
#[doc = "Field `SWP` writer - Stop Watch Polarity"]
pub type SwpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWS` reader - Stop Watch Source."]
pub type SwsR = crate::FieldReader;
#[doc = "Field `SWS` writer - Stop Watch Source."]
pub type SwsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RTIE` reader - Register Time Mark Interrupt Pulse Enable"]
pub type RtieR = crate::BitReader;
#[doc = "Field `RTIE` writer - Register Time Mark Interrupt Pulse Enable"]
pub type RtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMC` reader - Register Time Mark Compare"]
pub type TmcR = crate::FieldReader;
#[doc = "Field `TMC` writer - Register Time Mark Compare"]
pub type TmcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TTIE` reader - Trigger Time Mark Interrupt Pulse Enable"]
pub type TtieR = crate::BitReader;
#[doc = "Field `TTIE` writer - Trigger Time Mark Interrupt Pulse Enable"]
pub type TtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GCS` reader - Gap Control Select"]
pub type GcsR = crate::BitReader;
#[doc = "Field `GCS` writer - Gap Control Select"]
pub type GcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGP` reader - Finish Gap."]
pub type FgpR = crate::BitReader;
#[doc = "Field `FGP` writer - Finish Gap."]
pub type FgpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMG` reader - Time Mark Gap"]
pub type TmgR = crate::BitReader;
#[doc = "Field `TMG` writer - Time Mark Gap"]
pub type TmgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIG` reader - Next is Gap"]
pub type NigR = crate::BitReader;
#[doc = "Field `NIG` writer - Next is Gap"]
pub type NigW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ESCN` reader - External Synchronization Control"]
pub type EscnR = crate::BitReader;
#[doc = "Field `ESCN` writer - External Synchronization Control"]
pub type EscnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LCKC` reader - TT Operation Control Register Locked"]
pub type LckcR = crate::BitReader;
#[doc = "Field `LCKC` writer - TT Operation Control Register Locked"]
pub type LckcW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    pub fn sgt(&self) -> SgtR {
        SgtR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    pub fn ecs(&self) -> EcsR {
        EcsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    pub fn swp(&self) -> SwpR {
        SwpR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    pub fn sws(&self) -> SwsR {
        SwsR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn rtie(&self) -> RtieR {
        RtieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    pub fn tmc(&self) -> TmcR {
        TmcR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn ttie(&self) -> TtieR {
        TtieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    pub fn gcs(&self) -> GcsR {
        GcsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    pub fn fgp(&self) -> FgpR {
        FgpR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    pub fn tmg(&self) -> TmgR {
        TmgR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    pub fn nig(&self) -> NigR {
        NigR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    pub fn escn(&self) -> EscnR {
        EscnR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    pub fn lckc(&self) -> LckcR {
        LckcR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Set Global time"]
    #[inline(always)]
    pub fn sgt(&mut self) -> SgtW<FdcanTtocnSpec> {
        SgtW::new(self, 0)
    }
    #[doc = "Bit 1 - External Clock Synchronization"]
    #[inline(always)]
    pub fn ecs(&mut self) -> EcsW<FdcanTtocnSpec> {
        EcsW::new(self, 1)
    }
    #[doc = "Bit 2 - Stop Watch Polarity"]
    #[inline(always)]
    pub fn swp(&mut self) -> SwpW<FdcanTtocnSpec> {
        SwpW::new(self, 2)
    }
    #[doc = "Bits 3:4 - Stop Watch Source."]
    #[inline(always)]
    pub fn sws(&mut self) -> SwsW<FdcanTtocnSpec> {
        SwsW::new(self, 3)
    }
    #[doc = "Bit 5 - Register Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn rtie(&mut self) -> RtieW<FdcanTtocnSpec> {
        RtieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Register Time Mark Compare"]
    #[inline(always)]
    pub fn tmc(&mut self) -> TmcW<FdcanTtocnSpec> {
        TmcW::new(self, 6)
    }
    #[doc = "Bit 8 - Trigger Time Mark Interrupt Pulse Enable"]
    #[inline(always)]
    pub fn ttie(&mut self) -> TtieW<FdcanTtocnSpec> {
        TtieW::new(self, 8)
    }
    #[doc = "Bit 9 - Gap Control Select"]
    #[inline(always)]
    pub fn gcs(&mut self) -> GcsW<FdcanTtocnSpec> {
        GcsW::new(self, 9)
    }
    #[doc = "Bit 10 - Finish Gap."]
    #[inline(always)]
    pub fn fgp(&mut self) -> FgpW<FdcanTtocnSpec> {
        FgpW::new(self, 10)
    }
    #[doc = "Bit 11 - Time Mark Gap"]
    #[inline(always)]
    pub fn tmg(&mut self) -> TmgW<FdcanTtocnSpec> {
        TmgW::new(self, 11)
    }
    #[doc = "Bit 12 - Next is Gap"]
    #[inline(always)]
    pub fn nig(&mut self) -> NigW<FdcanTtocnSpec> {
        NigW::new(self, 12)
    }
    #[doc = "Bit 13 - External Synchronization Control"]
    #[inline(always)]
    pub fn escn(&mut self) -> EscnW<FdcanTtocnSpec> {
        EscnW::new(self, 13)
    }
    #[doc = "Bit 15 - TT Operation Control Register Locked"]
    #[inline(always)]
    pub fn lckc(&mut self) -> LckcW<FdcanTtocnSpec> {
        LckcW::new(self, 15)
    }
}
#[doc = "FDCAN TT Operation Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttocn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttocn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtocnSpec;
impl crate::RegisterSpec for FdcanTtocnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttocn::R`](R) reader structure"]
impl crate::Readable for FdcanTtocnSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttocn::W`](W) writer structure"]
impl crate::Writable for FdcanTtocnSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTOCN to value 0"]
impl crate::Resettable for FdcanTtocnSpec {}
