#[doc = "Register `FDCAN_TTILS` reader"]
pub type R = crate::R<FdcanTtilsSpec>;
#[doc = "Register `FDCAN_TTILS` writer"]
pub type W = crate::W<FdcanTtilsSpec>;
#[doc = "Field `SBCL` reader - Start of Basic Cycle Interrupt Line"]
pub type SbclR = crate::BitReader;
#[doc = "Field `SBCL` writer - Start of Basic Cycle Interrupt Line"]
pub type SbclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCL` reader - Start of Matrix Cycle Interrupt Line"]
pub type SmclR = crate::BitReader;
#[doc = "Field `SMCL` writer - Start of Matrix Cycle Interrupt Line"]
pub type SmclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSML` reader - Change of Synchronization Mode Interrupt Line"]
pub type CsmlR = crate::BitReader;
#[doc = "Field `CSML` writer - Change of Synchronization Mode Interrupt Line"]
pub type CsmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGL` reader - Start of Gap Interrupt Line"]
pub type SoglR = crate::BitReader;
#[doc = "Field `SOGL` writer - Start of Gap Interrupt Line"]
pub type SoglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIL` reader - Register Time Mark Interrupt Line"]
pub type RtmilR = crate::BitReader;
#[doc = "Field `RTMIL` writer - Register Time Mark Interrupt Line"]
pub type RtmilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIL` reader - Trigger Time Mark Event Internal Interrupt Line"]
pub type TtmilR = crate::BitReader;
#[doc = "Field `TTMIL` writer - Trigger Time Mark Event Internal Interrupt Line"]
pub type TtmilW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEL` reader - Stop Watch Event Interrupt Line"]
pub type SwelR = crate::BitReader;
#[doc = "Field `SWEL` writer - Stop Watch Event Interrupt Line"]
pub type SwelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWL` reader - Global Time Wrap Interrupt Line"]
pub type GtwlR = crate::BitReader;
#[doc = "Field `GTWL` writer - Global Time Wrap Interrupt Line"]
pub type GtwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDL` reader - Global Time Discontinuity Interrupt Line"]
pub type GtdlR = crate::BitReader;
#[doc = "Field `GTDL` writer - Global Time Discontinuity Interrupt Line"]
pub type GtdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEL` reader - Global Time Error Interrupt Line"]
pub type GtelR = crate::BitReader;
#[doc = "Field `GTEL` writer - Global Time Error Interrupt Line"]
pub type GtelW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUL` reader - Tx Count Underflow Interrupt Line"]
pub type TxulR = crate::BitReader;
#[doc = "Field `TXUL` writer - Tx Count Underflow Interrupt Line"]
pub type TxulW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOL` reader - Tx Count Overflow Interrupt Line"]
pub type TxolR = crate::BitReader;
#[doc = "Field `TXOL` writer - Tx Count Overflow Interrupt Line"]
pub type TxolW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1L` reader - Scheduling Error 1 Interrupt Line"]
pub type Se1lR = crate::BitReader;
#[doc = "Field `SE1L` writer - Scheduling Error 1 Interrupt Line"]
pub type Se1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2L` reader - Scheduling Error 2 Interrupt Line"]
pub type Se2lR = crate::BitReader;
#[doc = "Field `SE2L` writer - Scheduling Error 2 Interrupt Line"]
pub type Se2lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCL` reader - Change Error Level Interrupt Line"]
pub type ElclR = crate::BitReader;
#[doc = "Field `ELCL` writer - Change Error Level Interrupt Line"]
pub type ElclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTGL` reader - Initialization Watch Trigger Interrupt Line"]
pub type IwtglR = crate::BitReader;
#[doc = "Field `IWTGL` writer - Initialization Watch Trigger Interrupt Line"]
pub type IwtglW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTL` reader - Watch Trigger Interrupt Line"]
pub type WtlR = crate::BitReader;
#[doc = "Field `WTL` writer - Watch Trigger Interrupt Line"]
pub type WtlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWL` reader - Application Watchdog Interrupt Line"]
pub type AwlR = crate::BitReader;
#[doc = "Field `AWL` writer - Application Watchdog Interrupt Line"]
pub type AwlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERL` reader - Configuration Error Interrupt Line"]
pub type CerlR = crate::BitReader;
#[doc = "Field `CERL` writer - Configuration Error Interrupt Line"]
pub type CerlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn sbcl(&self) -> SbclR {
        SbclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn smcl(&self) -> SmclR {
        SmclR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn csml(&self) -> CsmlR {
        CsmlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn sogl(&self) -> SoglR {
        SoglR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn rtmil(&self) -> RtmilR {
        RtmilR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    pub fn ttmil(&self) -> TtmilR {
        TtmilR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&self) -> SwelR {
        SwelR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn gtwl(&self) -> GtwlR {
        GtwlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn gtdl(&self) -> GtdlR {
        GtdlR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn gtel(&self) -> GtelR {
        GtelR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn txul(&self) -> TxulR {
        TxulR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn txol(&self) -> TxolR {
        TxolR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn se1l(&self) -> Se1lR {
        Se1lR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn se2l(&self) -> Se2lR {
        Se2lR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn elcl(&self) -> ElclR {
        ElclR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn iwtgl(&self) -> IwtglR {
        IwtglR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn wtl(&self) -> WtlR {
        WtlR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn awl(&self) -> AwlR {
        AwlR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn cerl(&self) -> CerlR {
        CerlR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Line"]
    #[inline(always)]
    pub fn sbcl(&mut self) -> SbclW<FdcanTtilsSpec> {
        SbclW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Line"]
    #[inline(always)]
    pub fn smcl(&mut self) -> SmclW<FdcanTtilsSpec> {
        SmclW::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Line"]
    #[inline(always)]
    pub fn csml(&mut self) -> CsmlW<FdcanTtilsSpec> {
        CsmlW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Line"]
    #[inline(always)]
    pub fn sogl(&mut self) -> SoglW<FdcanTtilsSpec> {
        SoglW::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Line"]
    #[inline(always)]
    pub fn rtmil(&mut self) -> RtmilW<FdcanTtilsSpec> {
        RtmilW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Line"]
    #[inline(always)]
    pub fn ttmil(&mut self) -> TtmilW<FdcanTtilsSpec> {
        TtmilW::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Line"]
    #[inline(always)]
    pub fn swel(&mut self) -> SwelW<FdcanTtilsSpec> {
        SwelW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Line"]
    #[inline(always)]
    pub fn gtwl(&mut self) -> GtwlW<FdcanTtilsSpec> {
        GtwlW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Line"]
    #[inline(always)]
    pub fn gtdl(&mut self) -> GtdlW<FdcanTtilsSpec> {
        GtdlW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Line"]
    #[inline(always)]
    pub fn gtel(&mut self) -> GtelW<FdcanTtilsSpec> {
        GtelW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Line"]
    #[inline(always)]
    pub fn txul(&mut self) -> TxulW<FdcanTtilsSpec> {
        TxulW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Line"]
    #[inline(always)]
    pub fn txol(&mut self) -> TxolW<FdcanTtilsSpec> {
        TxolW::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Line"]
    #[inline(always)]
    pub fn se1l(&mut self) -> Se1lW<FdcanTtilsSpec> {
        Se1lW::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Line"]
    #[inline(always)]
    pub fn se2l(&mut self) -> Se2lW<FdcanTtilsSpec> {
        Se2lW::new(self, 13)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Line"]
    #[inline(always)]
    pub fn elcl(&mut self) -> ElclW<FdcanTtilsSpec> {
        ElclW::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn iwtgl(&mut self) -> IwtglW<FdcanTtilsSpec> {
        IwtglW::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Line"]
    #[inline(always)]
    pub fn wtl(&mut self) -> WtlW<FdcanTtilsSpec> {
        WtlW::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Line"]
    #[inline(always)]
    pub fn awl(&mut self) -> AwlW<FdcanTtilsSpec> {
        AwlW::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Line"]
    #[inline(always)]
    pub fn cerl(&mut self) -> CerlW<FdcanTtilsSpec> {
        CerlW::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Line Select Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttils::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttils::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtilsSpec;
impl crate::RegisterSpec for FdcanTtilsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttils::R`](R) reader structure"]
impl crate::Readable for FdcanTtilsSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttils::W`](W) writer structure"]
impl crate::Writable for FdcanTtilsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTILS to value 0"]
impl crate::Resettable for FdcanTtilsSpec {}
