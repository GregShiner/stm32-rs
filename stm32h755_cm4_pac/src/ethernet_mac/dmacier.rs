#[doc = "Register `DMACIER` reader"]
pub type R = crate::R<DmacierSpec>;
#[doc = "Register `DMACIER` writer"]
pub type W = crate::W<DmacierSpec>;
#[doc = "Field `TIE` reader - Transmit Interrupt Enable"]
pub type TieR = crate::BitReader;
#[doc = "Field `TIE` writer - Transmit Interrupt Enable"]
pub type TieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXSE` reader - Transmit Stopped Enable"]
pub type TxseR = crate::BitReader;
#[doc = "Field `TXSE` writer - Transmit Stopped Enable"]
pub type TxseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBUE` reader - Transmit Buffer Unavailable Enable"]
pub type TbueR = crate::BitReader;
#[doc = "Field `TBUE` writer - Transmit Buffer Unavailable Enable"]
pub type TbueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RIE` reader - Receive Interrupt Enable"]
pub type RieR = crate::BitReader;
#[doc = "Field `RIE` writer - Receive Interrupt Enable"]
pub type RieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBUE` reader - Receive Buffer Unavailable Enable"]
pub type RbueR = crate::BitReader;
#[doc = "Field `RBUE` writer - Receive Buffer Unavailable Enable"]
pub type RbueW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RSE` reader - Receive Stopped Enable"]
pub type RseR = crate::BitReader;
#[doc = "Field `RSE` writer - Receive Stopped Enable"]
pub type RseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWTE` reader - Receive Watchdog Timeout Enable"]
pub type RwteR = crate::BitReader;
#[doc = "Field `RWTE` writer - Receive Watchdog Timeout Enable"]
pub type RwteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ETIE` reader - Early Transmit Interrupt Enable"]
pub type EtieR = crate::BitReader;
#[doc = "Field `ETIE` writer - Early Transmit Interrupt Enable"]
pub type EtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ERIE` reader - Early Receive Interrupt Enable"]
pub type ErieR = crate::BitReader;
#[doc = "Field `ERIE` writer - Early Receive Interrupt Enable"]
pub type ErieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FBEE` reader - Fatal Bus Error Enable"]
pub type FbeeR = crate::BitReader;
#[doc = "Field `FBEE` writer - Fatal Bus Error Enable"]
pub type FbeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CDEE` reader - Context Descriptor Error Enable"]
pub type CdeeR = crate::BitReader;
#[doc = "Field `CDEE` writer - Context Descriptor Error Enable"]
pub type CdeeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AIE` reader - Abnormal Interrupt Summary Enable"]
pub type AieR = crate::BitReader;
#[doc = "Field `AIE` writer - Abnormal Interrupt Summary Enable"]
pub type AieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NIE` reader - Normal Interrupt Summary Enable"]
pub type NieR = crate::BitReader;
#[doc = "Field `NIE` writer - Normal Interrupt Summary Enable"]
pub type NieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&self) -> TieR {
        TieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&self) -> TxseR {
        TxseR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&self) -> TbueR {
        TbueR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&self) -> RieR {
        RieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&self) -> RbueR {
        RbueR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&self) -> RseR {
        RseR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&self) -> RwteR {
        RwteR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&self) -> EtieR {
        EtieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&self) -> ErieR {
        ErieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&self) -> FbeeR {
        FbeeR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&self) -> CdeeR {
        CdeeR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&self) -> AieR {
        AieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&self) -> NieR {
        NieR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn tie(&mut self) -> TieW<DmacierSpec> {
        TieW::new(self, 0)
    }
    #[doc = "Bit 1 - Transmit Stopped Enable"]
    #[inline(always)]
    pub fn txse(&mut self) -> TxseW<DmacierSpec> {
        TxseW::new(self, 1)
    }
    #[doc = "Bit 2 - Transmit Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn tbue(&mut self) -> TbueW<DmacierSpec> {
        TbueW::new(self, 2)
    }
    #[doc = "Bit 6 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn rie(&mut self) -> RieW<DmacierSpec> {
        RieW::new(self, 6)
    }
    #[doc = "Bit 7 - Receive Buffer Unavailable Enable"]
    #[inline(always)]
    pub fn rbue(&mut self) -> RbueW<DmacierSpec> {
        RbueW::new(self, 7)
    }
    #[doc = "Bit 8 - Receive Stopped Enable"]
    #[inline(always)]
    pub fn rse(&mut self) -> RseW<DmacierSpec> {
        RseW::new(self, 8)
    }
    #[doc = "Bit 9 - Receive Watchdog Timeout Enable"]
    #[inline(always)]
    pub fn rwte(&mut self) -> RwteW<DmacierSpec> {
        RwteW::new(self, 9)
    }
    #[doc = "Bit 10 - Early Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn etie(&mut self) -> EtieW<DmacierSpec> {
        EtieW::new(self, 10)
    }
    #[doc = "Bit 11 - Early Receive Interrupt Enable"]
    #[inline(always)]
    pub fn erie(&mut self) -> ErieW<DmacierSpec> {
        ErieW::new(self, 11)
    }
    #[doc = "Bit 12 - Fatal Bus Error Enable"]
    #[inline(always)]
    pub fn fbee(&mut self) -> FbeeW<DmacierSpec> {
        FbeeW::new(self, 12)
    }
    #[doc = "Bit 13 - Context Descriptor Error Enable"]
    #[inline(always)]
    pub fn cdee(&mut self) -> CdeeW<DmacierSpec> {
        CdeeW::new(self, 13)
    }
    #[doc = "Bit 14 - Abnormal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn aie(&mut self) -> AieW<DmacierSpec> {
        AieW::new(self, 14)
    }
    #[doc = "Bit 15 - Normal Interrupt Summary Enable"]
    #[inline(always)]
    pub fn nie(&mut self) -> NieW<DmacierSpec> {
        NieW::new(self, 15)
    }
}
#[doc = "Channel interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacierSpec;
impl crate::RegisterSpec for DmacierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacier::R`](R) reader structure"]
impl crate::Readable for DmacierSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacier::W`](W) writer structure"]
impl crate::Writable for DmacierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACIER to value 0"]
impl crate::Resettable for DmacierSpec {}
