#[doc = "Register `FDCAN_TTIE` reader"]
pub type R = crate::R<FdcanTtieSpec>;
#[doc = "Register `FDCAN_TTIE` writer"]
pub type W = crate::W<FdcanTtieSpec>;
#[doc = "Field `SBCE` reader - Start of Basic Cycle Interrupt Enable"]
pub type SbceR = crate::BitReader;
#[doc = "Field `SBCE` writer - Start of Basic Cycle Interrupt Enable"]
pub type SbceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMCE` reader - Start of Matrix Cycle Interrupt Enable"]
pub type SmceR = crate::BitReader;
#[doc = "Field `SMCE` writer - Start of Matrix Cycle Interrupt Enable"]
pub type SmceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSME` reader - Change of Synchronization Mode Interrupt Enable"]
pub type CsmeR = crate::BitReader;
#[doc = "Field `CSME` writer - Change of Synchronization Mode Interrupt Enable"]
pub type CsmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOGE` reader - Start of Gap Interrupt Enable"]
pub type SogeR = crate::BitReader;
#[doc = "Field `SOGE` writer - Start of Gap Interrupt Enable"]
pub type SogeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMIE` reader - Register Time Mark Interrupt Enable"]
pub type RtmieR = crate::BitReader;
#[doc = "Field `RTMIE` writer - Register Time Mark Interrupt Enable"]
pub type RtmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMIE` reader - Trigger Time Mark Event Internal Interrupt Enable"]
pub type TtmieR = crate::BitReader;
#[doc = "Field `TTMIE` writer - Trigger Time Mark Event Internal Interrupt Enable"]
pub type TtmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWEE` reader - Stop Watch Event Interrupt Enable"]
pub type SweeR = crate::BitReader;
#[doc = "Field `SWEE` writer - Stop Watch Event Interrupt Enable"]
pub type SweeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTWE` reader - Global Time Wrap Interrupt Enable"]
pub type GtweR = crate::BitReader;
#[doc = "Field `GTWE` writer - Global Time Wrap Interrupt Enable"]
pub type GtweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTDE` reader - Global Time Discontinuity Interrupt Enable"]
pub type GtdeR = crate::BitReader;
#[doc = "Field `GTDE` writer - Global Time Discontinuity Interrupt Enable"]
pub type GtdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTEE` reader - Global Time Error Interrupt Enable"]
pub type GteeR = crate::BitReader;
#[doc = "Field `GTEE` writer - Global Time Error Interrupt Enable"]
pub type GteeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUE` reader - Tx Count Underflow Interrupt Enable"]
pub type TxueR = crate::BitReader;
#[doc = "Field `TXUE` writer - Tx Count Underflow Interrupt Enable"]
pub type TxueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXOE` reader - Tx Count Overflow Interrupt Enable"]
pub type TxoeR = crate::BitReader;
#[doc = "Field `TXOE` writer - Tx Count Overflow Interrupt Enable"]
pub type TxoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1E` reader - Scheduling Error 1 Interrupt Enable"]
pub type Se1eR = crate::BitReader;
#[doc = "Field `SE1E` writer - Scheduling Error 1 Interrupt Enable"]
pub type Se1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2E` reader - Scheduling Error 2 Interrupt Enable"]
pub type Se2eR = crate::BitReader;
#[doc = "Field `SE2E` writer - Scheduling Error 2 Interrupt Enable"]
pub type Se2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELCE` reader - Change Error Level Interrupt Enable"]
pub type ElceR = crate::BitReader;
#[doc = "Field `ELCE` writer - Change Error Level Interrupt Enable"]
pub type ElceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTGE` reader - Initialization Watch Trigger Interrupt Enable"]
pub type IwtgeR = crate::BitReader;
#[doc = "Field `IWTGE` writer - Initialization Watch Trigger Interrupt Enable"]
pub type IwtgeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WTE` reader - Watch Trigger Interrupt Enable"]
pub type WteR = crate::BitReader;
#[doc = "Field `WTE` writer - Watch Trigger Interrupt Enable"]
pub type WteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AWE` reader - Application Watchdog Interrupt Enable"]
pub type AweR = crate::BitReader;
#[doc = "Field `AWE` writer - Application Watchdog Interrupt Enable"]
pub type AweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CERE` reader - Configuration Error Interrupt Enable"]
pub type CereR = crate::BitReader;
#[doc = "Field `CERE` writer - Configuration Error Interrupt Enable"]
pub type CereW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&self) -> SbceR {
        SbceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&self) -> SmceR {
        SmceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&self) -> CsmeR {
        CsmeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&self) -> SogeR {
        SogeR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&self) -> RtmieR {
        RtmieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&self) -> TtmieR {
        TtmieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&self) -> SweeR {
        SweeR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&self) -> GtweR {
        GtweR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&self) -> GtdeR {
        GtdeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&self) -> GteeR {
        GteeR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&self) -> TxueR {
        TxueR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&self) -> TxoeR {
        TxoeR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&self) -> Se1eR {
        Se1eR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&self) -> Se2eR {
        Se2eR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&self) -> ElceR {
        ElceR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&self) -> IwtgeR {
        IwtgeR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&self) -> WteR {
        WteR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&self) -> AweR {
        AweR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&self) -> CereR {
        CereR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn sbce(&mut self) -> SbceW<FdcanTtieSpec> {
        SbceW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle Interrupt Enable"]
    #[inline(always)]
    pub fn smce(&mut self) -> SmceW<FdcanTtieSpec> {
        SmceW::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode Interrupt Enable"]
    #[inline(always)]
    pub fn csme(&mut self) -> CsmeW<FdcanTtieSpec> {
        CsmeW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap Interrupt Enable"]
    #[inline(always)]
    pub fn soge(&mut self) -> SogeW<FdcanTtieSpec> {
        SogeW::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt Enable"]
    #[inline(always)]
    pub fn rtmie(&mut self) -> RtmieW<FdcanTtieSpec> {
        RtmieW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal Interrupt Enable"]
    #[inline(always)]
    pub fn ttmie(&mut self) -> TtmieW<FdcanTtieSpec> {
        TtmieW::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event Interrupt Enable"]
    #[inline(always)]
    pub fn swee(&mut self) -> SweeW<FdcanTtieSpec> {
        SweeW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap Interrupt Enable"]
    #[inline(always)]
    pub fn gtwe(&mut self) -> GtweW<FdcanTtieSpec> {
        GtweW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity Interrupt Enable"]
    #[inline(always)]
    pub fn gtde(&mut self) -> GtdeW<FdcanTtieSpec> {
        GtdeW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error Interrupt Enable"]
    #[inline(always)]
    pub fn gtee(&mut self) -> GteeW<FdcanTtieSpec> {
        GteeW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow Interrupt Enable"]
    #[inline(always)]
    pub fn txue(&mut self) -> TxueW<FdcanTtieSpec> {
        TxueW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow Interrupt Enable"]
    #[inline(always)]
    pub fn txoe(&mut self) -> TxoeW<FdcanTtieSpec> {
        TxoeW::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1 Interrupt Enable"]
    #[inline(always)]
    pub fn se1e(&mut self) -> Se1eW<FdcanTtieSpec> {
        Se1eW::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2 Interrupt Enable"]
    #[inline(always)]
    pub fn se2e(&mut self) -> Se2eW<FdcanTtieSpec> {
        Se2eW::new(self, 13)
    }
    #[doc = "Bit 14 - Change Error Level Interrupt Enable"]
    #[inline(always)]
    pub fn elce(&mut self) -> ElceW<FdcanTtieSpec> {
        ElceW::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn iwtge(&mut self) -> IwtgeW<FdcanTtieSpec> {
        IwtgeW::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger Interrupt Enable"]
    #[inline(always)]
    pub fn wte(&mut self) -> WteW<FdcanTtieSpec> {
        WteW::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog Interrupt Enable"]
    #[inline(always)]
    pub fn awe(&mut self) -> AweW<FdcanTtieSpec> {
        AweW::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error Interrupt Enable"]
    #[inline(always)]
    pub fn cere(&mut self) -> CereW<FdcanTtieSpec> {
        CereW::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtieSpec;
impl crate::RegisterSpec for FdcanTtieSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttie::R`](R) reader structure"]
impl crate::Readable for FdcanTtieSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttie::W`](W) writer structure"]
impl crate::Writable for FdcanTtieSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTIE to value 0"]
impl crate::Resettable for FdcanTtieSpec {}
