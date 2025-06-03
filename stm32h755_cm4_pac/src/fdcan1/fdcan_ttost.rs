#[doc = "Register `FDCAN_TTOST` reader"]
pub type R = crate::R<FdcanTtostSpec>;
#[doc = "Register `FDCAN_TTOST` writer"]
pub type W = crate::W<FdcanTtostSpec>;
#[doc = "Field `EL` reader - Error Level"]
pub type ElR = crate::FieldReader;
#[doc = "Field `EL` writer - Error Level"]
pub type ElW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MS` reader - Master State."]
pub type MsR = crate::FieldReader;
#[doc = "Field `MS` writer - Master State."]
pub type MsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SYS` reader - Synchronization State"]
pub type SysR = crate::FieldReader;
#[doc = "Field `SYS` writer - Synchronization State"]
pub type SysW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `GTP` reader - Quality of Global Time Phase"]
pub type GtpR = crate::BitReader;
#[doc = "Field `GTP` writer - Quality of Global Time Phase"]
pub type GtpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `QCS` reader - Quality of Clock Speed"]
pub type QcsR = crate::BitReader;
#[doc = "Field `QCS` writer - Quality of Clock Speed"]
pub type QcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTO` reader - Reference Trigger Offset"]
pub type RtoR = crate::FieldReader;
#[doc = "Field `RTO` writer - Reference Trigger Offset"]
pub type RtoW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WGTD` reader - Wait for Global Time Discontinuity"]
pub type WgtdR = crate::BitReader;
#[doc = "Field `WGTD` writer - Wait for Global Time Discontinuity"]
pub type WgtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GFI` reader - Gap Finished Indicator."]
pub type GfiR = crate::BitReader;
#[doc = "Field `GFI` writer - Gap Finished Indicator."]
pub type GfiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMP` reader - Time Master Priority"]
pub type TmpR = crate::FieldReader;
#[doc = "Field `TMP` writer - Time Master Priority"]
pub type TmpW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GSI` reader - Gap Started Indicator."]
pub type GsiR = crate::BitReader;
#[doc = "Field `GSI` writer - Gap Started Indicator."]
pub type GsiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WFE` reader - Wait for Event"]
pub type WfeR = crate::BitReader;
#[doc = "Field `WFE` writer - Wait for Event"]
pub type WfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWE` reader - Application Watchdog Event"]
pub type AweR = crate::BitReader;
#[doc = "Field `AWE` writer - Application Watchdog Event"]
pub type AweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WECS` reader - Wait for External Clock Synchronization"]
pub type WecsR = crate::BitReader;
#[doc = "Field `WECS` writer - Wait for External Clock Synchronization"]
pub type WecsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPL` reader - Schedule Phase Lock"]
pub type SplR = crate::BitReader;
#[doc = "Field `SPL` writer - Schedule Phase Lock"]
pub type SplW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&self) -> ElR {
        ElR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&self) -> MsR {
        MsR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&self) -> SysR {
        SysR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn gtp(&self) -> GtpR {
        GtpR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&self) -> QcsR {
        QcsR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&self) -> RtoR {
        RtoR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&self) -> WgtdR {
        WgtdR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&self) -> GfiR {
        GfiR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&self) -> TmpR {
        TmpR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&self) -> GsiR {
        GsiR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&self) -> WfeR {
        WfeR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&self) -> AweR {
        AweR::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&self) -> WecsR {
        WecsR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&self) -> SplR {
        SplR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Error Level"]
    #[inline(always)]
    pub fn el(&mut self) -> ElW<FdcanTtostSpec> {
        ElW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Master State."]
    #[inline(always)]
    pub fn ms(&mut self) -> MsW<FdcanTtostSpec> {
        MsW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Synchronization State"]
    #[inline(always)]
    pub fn sys(&mut self) -> SysW<FdcanTtostSpec> {
        SysW::new(self, 4)
    }
    #[doc = "Bit 6 - Quality of Global Time Phase"]
    #[inline(always)]
    pub fn gtp(&mut self) -> GtpW<FdcanTtostSpec> {
        GtpW::new(self, 6)
    }
    #[doc = "Bit 7 - Quality of Clock Speed"]
    #[inline(always)]
    pub fn qcs(&mut self) -> QcsW<FdcanTtostSpec> {
        QcsW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Reference Trigger Offset"]
    #[inline(always)]
    pub fn rto(&mut self) -> RtoW<FdcanTtostSpec> {
        RtoW::new(self, 8)
    }
    #[doc = "Bit 22 - Wait for Global Time Discontinuity"]
    #[inline(always)]
    pub fn wgtd(&mut self) -> WgtdW<FdcanTtostSpec> {
        WgtdW::new(self, 22)
    }
    #[doc = "Bit 23 - Gap Finished Indicator."]
    #[inline(always)]
    pub fn gfi(&mut self) -> GfiW<FdcanTtostSpec> {
        GfiW::new(self, 23)
    }
    #[doc = "Bits 24:26 - Time Master Priority"]
    #[inline(always)]
    pub fn tmp(&mut self) -> TmpW<FdcanTtostSpec> {
        TmpW::new(self, 24)
    }
    #[doc = "Bit 27 - Gap Started Indicator."]
    #[inline(always)]
    pub fn gsi(&mut self) -> GsiW<FdcanTtostSpec> {
        GsiW::new(self, 27)
    }
    #[doc = "Bit 28 - Wait for Event"]
    #[inline(always)]
    pub fn wfe(&mut self) -> WfeW<FdcanTtostSpec> {
        WfeW::new(self, 28)
    }
    #[doc = "Bit 29 - Application Watchdog Event"]
    #[inline(always)]
    pub fn awe(&mut self) -> AweW<FdcanTtostSpec> {
        AweW::new(self, 29)
    }
    #[doc = "Bit 30 - Wait for External Clock Synchronization"]
    #[inline(always)]
    pub fn wecs(&mut self) -> WecsW<FdcanTtostSpec> {
        WecsW::new(self, 30)
    }
    #[doc = "Bit 31 - Schedule Phase Lock"]
    #[inline(always)]
    pub fn spl(&mut self) -> SplW<FdcanTtostSpec> {
        SplW::new(self, 31)
    }
}
#[doc = "FDCAN TT Operation Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttost::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttost::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtostSpec;
impl crate::RegisterSpec for FdcanTtostSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttost::R`](R) reader structure"]
impl crate::Readable for FdcanTtostSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttost::W`](W) writer structure"]
impl crate::Writable for FdcanTtostSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTOST to value 0"]
impl crate::Resettable for FdcanTtostSpec {}
