#[doc = "Register `FDCAN_TTIR` reader"]
pub type R = crate::R<FdcanTtirSpec>;
#[doc = "Register `FDCAN_TTIR` writer"]
pub type W = crate::W<FdcanTtirSpec>;
#[doc = "Field `SBC` reader - Start of Basic Cycle"]
pub type SbcR = crate::BitReader;
#[doc = "Field `SBC` writer - Start of Basic Cycle"]
pub type SbcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMC` reader - Start of Matrix Cycle"]
pub type SmcR = crate::BitReader;
#[doc = "Field `SMC` writer - Start of Matrix Cycle"]
pub type SmcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSM` reader - Change of Synchronization Mode"]
pub type CsmR = crate::BitReader;
#[doc = "Field `CSM` writer - Change of Synchronization Mode"]
pub type CsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOG` reader - Start of Gap"]
pub type SogR = crate::BitReader;
#[doc = "Field `SOG` writer - Start of Gap"]
pub type SogW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTMI` reader - Register Time Mark Interrupt."]
pub type RtmiR = crate::BitReader;
#[doc = "Field `RTMI` writer - Register Time Mark Interrupt."]
pub type RtmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TTMI` reader - Trigger Time Mark Event Internal"]
pub type TtmiR = crate::BitReader;
#[doc = "Field `TTMI` writer - Trigger Time Mark Event Internal"]
pub type TtmiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWE` reader - Stop Watch Event"]
pub type SweR = crate::BitReader;
#[doc = "Field `SWE` writer - Stop Watch Event"]
pub type SweW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTW` reader - Global Time Wrap"]
pub type GtwR = crate::BitReader;
#[doc = "Field `GTW` writer - Global Time Wrap"]
pub type GtwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTD` reader - Global Time Discontinuity"]
pub type GtdR = crate::BitReader;
#[doc = "Field `GTD` writer - Global Time Discontinuity"]
pub type GtdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GTE` reader - Global Time Error"]
pub type GteR = crate::BitReader;
#[doc = "Field `GTE` writer - Global Time Error"]
pub type GteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXU` reader - Tx Count Underflow"]
pub type TxuR = crate::BitReader;
#[doc = "Field `TXU` writer - Tx Count Underflow"]
pub type TxuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXO` reader - Tx Count Overflow"]
pub type TxoR = crate::BitReader;
#[doc = "Field `TXO` writer - Tx Count Overflow"]
pub type TxoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE1` reader - Scheduling Error 1"]
pub type Se1R = crate::BitReader;
#[doc = "Field `SE1` writer - Scheduling Error 1"]
pub type Se1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SE2` reader - Scheduling Error 2"]
pub type Se2R = crate::BitReader;
#[doc = "Field `SE2` writer - Scheduling Error 2"]
pub type Se2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ELC` reader - Error Level Changed."]
pub type ElcR = crate::BitReader;
#[doc = "Field `ELC` writer - Error Level Changed."]
pub type ElcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWTG` reader - Initialization Watch Trigger"]
pub type IwtgR = crate::BitReader;
#[doc = "Field `IWTG` writer - Initialization Watch Trigger"]
pub type IwtgW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WT` reader - Watch Trigger"]
pub type WtR = crate::BitReader;
#[doc = "Field `WT` writer - Watch Trigger"]
pub type WtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AW` reader - Application Watchdog"]
pub type AwR = crate::BitReader;
#[doc = "Field `AW` writer - Application Watchdog"]
pub type AwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CER` reader - Configuration Error"]
pub type CerR = crate::BitReader;
#[doc = "Field `CER` writer - Configuration Error"]
pub type CerW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    pub fn sbc(&self) -> SbcR {
        SbcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    pub fn smc(&self) -> SmcR {
        SmcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    pub fn csm(&self) -> CsmR {
        CsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    pub fn sog(&self) -> SogR {
        SogR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    pub fn rtmi(&self) -> RtmiR {
        RtmiR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    pub fn ttmi(&self) -> TtmiR {
        TtmiR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&self) -> SweR {
        SweR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    pub fn gtw(&self) -> GtwR {
        GtwR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    pub fn gtd(&self) -> GtdR {
        GtdR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    pub fn gte(&self) -> GteR {
        GteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    pub fn txu(&self) -> TxuR {
        TxuR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    pub fn txo(&self) -> TxoR {
        TxoR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    pub fn se1(&self) -> Se1R {
        Se1R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    pub fn se2(&self) -> Se2R {
        Se2R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    pub fn elc(&self) -> ElcR {
        ElcR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    pub fn iwtg(&self) -> IwtgR {
        IwtgR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    pub fn wt(&self) -> WtR {
        WtR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    pub fn aw(&self) -> AwR {
        AwR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    pub fn cer(&self) -> CerR {
        CerR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Start of Basic Cycle"]
    #[inline(always)]
    pub fn sbc(&mut self) -> SbcW<FdcanTtirSpec> {
        SbcW::new(self, 0)
    }
    #[doc = "Bit 1 - Start of Matrix Cycle"]
    #[inline(always)]
    pub fn smc(&mut self) -> SmcW<FdcanTtirSpec> {
        SmcW::new(self, 1)
    }
    #[doc = "Bit 2 - Change of Synchronization Mode"]
    #[inline(always)]
    pub fn csm(&mut self) -> CsmW<FdcanTtirSpec> {
        CsmW::new(self, 2)
    }
    #[doc = "Bit 3 - Start of Gap"]
    #[inline(always)]
    pub fn sog(&mut self) -> SogW<FdcanTtirSpec> {
        SogW::new(self, 3)
    }
    #[doc = "Bit 4 - Register Time Mark Interrupt."]
    #[inline(always)]
    pub fn rtmi(&mut self) -> RtmiW<FdcanTtirSpec> {
        RtmiW::new(self, 4)
    }
    #[doc = "Bit 5 - Trigger Time Mark Event Internal"]
    #[inline(always)]
    pub fn ttmi(&mut self) -> TtmiW<FdcanTtirSpec> {
        TtmiW::new(self, 5)
    }
    #[doc = "Bit 6 - Stop Watch Event"]
    #[inline(always)]
    pub fn swe(&mut self) -> SweW<FdcanTtirSpec> {
        SweW::new(self, 6)
    }
    #[doc = "Bit 7 - Global Time Wrap"]
    #[inline(always)]
    pub fn gtw(&mut self) -> GtwW<FdcanTtirSpec> {
        GtwW::new(self, 7)
    }
    #[doc = "Bit 8 - Global Time Discontinuity"]
    #[inline(always)]
    pub fn gtd(&mut self) -> GtdW<FdcanTtirSpec> {
        GtdW::new(self, 8)
    }
    #[doc = "Bit 9 - Global Time Error"]
    #[inline(always)]
    pub fn gte(&mut self) -> GteW<FdcanTtirSpec> {
        GteW::new(self, 9)
    }
    #[doc = "Bit 10 - Tx Count Underflow"]
    #[inline(always)]
    pub fn txu(&mut self) -> TxuW<FdcanTtirSpec> {
        TxuW::new(self, 10)
    }
    #[doc = "Bit 11 - Tx Count Overflow"]
    #[inline(always)]
    pub fn txo(&mut self) -> TxoW<FdcanTtirSpec> {
        TxoW::new(self, 11)
    }
    #[doc = "Bit 12 - Scheduling Error 1"]
    #[inline(always)]
    pub fn se1(&mut self) -> Se1W<FdcanTtirSpec> {
        Se1W::new(self, 12)
    }
    #[doc = "Bit 13 - Scheduling Error 2"]
    #[inline(always)]
    pub fn se2(&mut self) -> Se2W<FdcanTtirSpec> {
        Se2W::new(self, 13)
    }
    #[doc = "Bit 14 - Error Level Changed."]
    #[inline(always)]
    pub fn elc(&mut self) -> ElcW<FdcanTtirSpec> {
        ElcW::new(self, 14)
    }
    #[doc = "Bit 15 - Initialization Watch Trigger"]
    #[inline(always)]
    pub fn iwtg(&mut self) -> IwtgW<FdcanTtirSpec> {
        IwtgW::new(self, 15)
    }
    #[doc = "Bit 16 - Watch Trigger"]
    #[inline(always)]
    pub fn wt(&mut self) -> WtW<FdcanTtirSpec> {
        WtW::new(self, 16)
    }
    #[doc = "Bit 17 - Application Watchdog"]
    #[inline(always)]
    pub fn aw(&mut self) -> AwW<FdcanTtirSpec> {
        AwW::new(self, 17)
    }
    #[doc = "Bit 18 - Configuration Error"]
    #[inline(always)]
    pub fn cer(&mut self) -> CerW<FdcanTtirSpec> {
        CerW::new(self, 18)
    }
}
#[doc = "FDCAN TT Interrupt Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ttir::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_ttir::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTtirSpec;
impl crate::RegisterSpec for FdcanTtirSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ttir::R`](R) reader structure"]
impl crate::Readable for FdcanTtirSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_ttir::W`](W) writer structure"]
impl crate::Writable for FdcanTtirSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TTIR to value 0"]
impl crate::Resettable for FdcanTtirSpec {}
