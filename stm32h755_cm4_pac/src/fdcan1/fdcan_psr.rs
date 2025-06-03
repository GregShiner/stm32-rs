#[doc = "Register `FDCAN_PSR` reader"]
pub type R = crate::R<FdcanPsrSpec>;
#[doc = "Register `FDCAN_PSR` writer"]
pub type W = crate::W<FdcanPsrSpec>;
#[doc = "Field `LEC` reader - Last Error Code"]
pub type LecR = crate::FieldReader;
#[doc = "Field `LEC` writer - Last Error Code"]
pub type LecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ACT` reader - Activity"]
pub type ActR = crate::FieldReader;
#[doc = "Field `ACT` writer - Activity"]
pub type ActW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EP` reader - Error Passive"]
pub type EpR = crate::BitReader;
#[doc = "Field `EP` writer - Error Passive"]
pub type EpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EW` reader - Warning Status"]
pub type EwR = crate::BitReader;
#[doc = "Field `EW` writer - Warning Status"]
pub type EwW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BO` reader - Bus_Off Status"]
pub type BoR = crate::BitReader;
#[doc = "Field `BO` writer - Bus_Off Status"]
pub type BoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLEC` reader - Data Last Error Code"]
pub type DlecR = crate::FieldReader;
#[doc = "Field `DLEC` writer - Data Last Error Code"]
pub type DlecW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RESI` reader - ESI flag of last received FDCAN Message"]
pub type ResiR = crate::BitReader;
#[doc = "Field `RESI` writer - ESI flag of last received FDCAN Message"]
pub type ResiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RBRS` reader - BRS flag of last received FDCAN Message"]
pub type RbrsR = crate::BitReader;
#[doc = "Field `RBRS` writer - BRS flag of last received FDCAN Message"]
pub type RbrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REDL` reader - Received FDCAN Message"]
pub type RedlR = crate::BitReader;
#[doc = "Field `REDL` writer - Received FDCAN Message"]
pub type RedlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXE` reader - Protocol Exception Event"]
pub type PxeR = crate::BitReader;
#[doc = "Field `PXE` writer - Protocol Exception Event"]
pub type PxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TDCV` reader - Transmitter Delay Compensation Value"]
pub type TdcvR = crate::FieldReader;
#[doc = "Field `TDCV` writer - Transmitter Delay Compensation Value"]
pub type TdcvW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&self) -> LecR {
        LecR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&self) -> ActR {
        ActR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&self) -> EpR {
        EpR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&self) -> EwR {
        EwR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&self) -> BoR {
        BoR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&self) -> DlecR {
        DlecR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&self) -> ResiR {
        ResiR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&self) -> RbrsR {
        RbrsR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&self) -> RedlR {
        RedlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&self) -> PxeR {
        PxeR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&self) -> TdcvR {
        TdcvR::new(((self.bits >> 16) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn lec(&mut self) -> LecW<FdcanPsrSpec> {
        LecW::new(self, 0)
    }
    #[doc = "Bits 3:4 - Activity"]
    #[inline(always)]
    pub fn act(&mut self) -> ActW<FdcanPsrSpec> {
        ActW::new(self, 3)
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn ep(&mut self) -> EpW<FdcanPsrSpec> {
        EpW::new(self, 5)
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn ew(&mut self) -> EwW<FdcanPsrSpec> {
        EwW::new(self, 6)
    }
    #[doc = "Bit 7 - Bus_Off Status"]
    #[inline(always)]
    pub fn bo(&mut self) -> BoW<FdcanPsrSpec> {
        BoW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Data Last Error Code"]
    #[inline(always)]
    pub fn dlec(&mut self) -> DlecW<FdcanPsrSpec> {
        DlecW::new(self, 8)
    }
    #[doc = "Bit 11 - ESI flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn resi(&mut self) -> ResiW<FdcanPsrSpec> {
        ResiW::new(self, 11)
    }
    #[doc = "Bit 12 - BRS flag of last received FDCAN Message"]
    #[inline(always)]
    pub fn rbrs(&mut self) -> RbrsW<FdcanPsrSpec> {
        RbrsW::new(self, 12)
    }
    #[doc = "Bit 13 - Received FDCAN Message"]
    #[inline(always)]
    pub fn redl(&mut self) -> RedlW<FdcanPsrSpec> {
        RedlW::new(self, 13)
    }
    #[doc = "Bit 14 - Protocol Exception Event"]
    #[inline(always)]
    pub fn pxe(&mut self) -> PxeW<FdcanPsrSpec> {
        PxeW::new(self, 14)
    }
    #[doc = "Bits 16:22 - Transmitter Delay Compensation Value"]
    #[inline(always)]
    pub fn tdcv(&mut self) -> TdcvW<FdcanPsrSpec> {
        TdcvW::new(self, 16)
    }
}
#[doc = "FDCAN Protocol Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_psr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_psr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanPsrSpec;
impl crate::RegisterSpec for FdcanPsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_psr::R`](R) reader structure"]
impl crate::Readable for FdcanPsrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_psr::W`](W) writer structure"]
impl crate::Writable for FdcanPsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_PSR to value 0"]
impl crate::Resettable for FdcanPsrSpec {}
