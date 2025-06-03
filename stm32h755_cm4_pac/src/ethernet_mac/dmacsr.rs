#[doc = "Register `DMACSR` reader"]
pub type R = crate::R<DmacsrSpec>;
#[doc = "Register `DMACSR` writer"]
pub type W = crate::W<DmacsrSpec>;
#[doc = "Field `TI` reader - Transmit Interrupt"]
pub type TiR = crate::BitReader;
#[doc = "Field `TI` writer - Transmit Interrupt"]
pub type TiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TPS` reader - Transmit Process Stopped"]
pub type TpsR = crate::BitReader;
#[doc = "Field `TPS` writer - Transmit Process Stopped"]
pub type TpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBU` reader - Transmit Buffer Unavailable"]
pub type TbuR = crate::BitReader;
#[doc = "Field `TBU` writer - Transmit Buffer Unavailable"]
pub type TbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RI` reader - Receive Interrupt"]
pub type RiR = crate::BitReader;
#[doc = "Field `RI` writer - Receive Interrupt"]
pub type RiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBU` reader - Receive Buffer Unavailable"]
pub type RbuR = crate::BitReader;
#[doc = "Field `RBU` writer - Receive Buffer Unavailable"]
pub type RbuW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RPS` reader - Receive Process Stopped"]
pub type RpsR = crate::BitReader;
#[doc = "Field `RPS` writer - Receive Process Stopped"]
pub type RpsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWT` reader - Receive Watchdog Timeout"]
pub type RwtR = crate::BitReader;
#[doc = "Field `RWT` writer - Receive Watchdog Timeout"]
pub type RwtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ET` reader - Early Transmit Interrupt"]
pub type EtR = crate::BitReader;
#[doc = "Field `ET` writer - Early Transmit Interrupt"]
pub type EtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ER` reader - Early Receive Interrupt"]
pub type ErR = crate::BitReader;
#[doc = "Field `ER` writer - Early Receive Interrupt"]
pub type ErW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBE` reader - Fatal Bus Error"]
pub type FbeR = crate::BitReader;
#[doc = "Field `FBE` writer - Fatal Bus Error"]
pub type FbeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDE` reader - Context Descriptor Error"]
pub type CdeR = crate::BitReader;
#[doc = "Field `CDE` writer - Context Descriptor Error"]
pub type CdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIS` reader - Abnormal Interrupt Summary"]
pub type AisR = crate::BitReader;
#[doc = "Field `AIS` writer - Abnormal Interrupt Summary"]
pub type AisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIS` reader - Normal Interrupt Summary"]
pub type NisR = crate::BitReader;
#[doc = "Field `NIS` writer - Normal Interrupt Summary"]
pub type NisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEB` reader - Tx DMA Error Bits"]
pub type TebR = crate::FieldReader;
#[doc = "Field `REB` reader - Rx DMA Error Bits"]
pub type RebR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&self) -> TiR {
        TiR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&self) -> TpsR {
        TpsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&self) -> TbuR {
        TbuR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&self) -> RiR {
        RiR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&self) -> RbuR {
        RbuR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&self) -> RpsR {
        RpsR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn et(&self) -> EtR {
        EtR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn er(&self) -> ErR {
        ErR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&self) -> FbeR {
        FbeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&self) -> CdeR {
        CdeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&self) -> AisR {
        AisR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&self) -> NisR {
        NisR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - Tx DMA Error Bits"]
    #[inline(always)]
    pub fn teb(&self) -> TebR {
        TebR::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 19:21 - Rx DMA Error Bits"]
    #[inline(always)]
    pub fn reb(&self) -> RebR {
        RebR::new(((self.bits >> 19) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt"]
    #[inline(always)]
    pub fn ti(&mut self) -> TiW<DmacsrSpec> {
        TiW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Process Stopped"]
    #[inline(always)]
    pub fn tps(&mut self) -> TpsW<DmacsrSpec> {
        TpsW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable"]
    #[inline(always)]
    pub fn tbu(&mut self) -> TbuW<DmacsrSpec> {
        TbuW::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt"]
    #[inline(always)]
    pub fn ri(&mut self) -> RiW<DmacsrSpec> {
        RiW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable"]
    #[inline(always)]
    pub fn rbu(&mut self) -> RbuW<DmacsrSpec> {
        RbuW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Process Stopped"]
    #[inline(always)]
    pub fn rps(&mut self) -> RpsW<DmacsrSpec> {
        RpsW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RwtW<DmacsrSpec> {
        RwtW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt"]
    #[inline(always)]
    pub fn et(&mut self) -> EtW<DmacsrSpec> {
        EtW::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt"]
    #[inline(always)]
    pub fn er(&mut self) -> ErW<DmacsrSpec> {
        ErW::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error"]
    #[inline(always)]
    pub fn fbe(&mut self) -> FbeW<DmacsrSpec> {
        FbeW::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error"]
    #[inline(always)]
    pub fn cde(&mut self) -> CdeW<DmacsrSpec> {
        CdeW::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary"]
    #[inline(always)]
    pub fn ais(&mut self) -> AisW<DmacsrSpec> {
        AisW::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary"]
    #[inline(always)]
    pub fn nis(&mut self) -> NisW<DmacsrSpec> {
        NisW::new(self, 15)
    }
}
#[doc = "Channel status register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacsrSpec;
impl crate::RegisterSpec for DmacsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacsr::R`](R) reader structure"]
impl crate::Readable for DmacsrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacsr::W`](W) writer structure"]
impl crate::Writable for DmacsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACSR to value 0"]
impl crate::Resettable for DmacsrSpec {}
