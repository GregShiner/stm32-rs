#[doc = "Register `FDCAN_CCCR` reader"]
pub type R = crate::R<FdcanCccrSpec>;
#[doc = "Register `FDCAN_CCCR` writer"]
pub type W = crate::W<FdcanCccrSpec>;
#[doc = "Field `INIT` reader - Initialization"]
pub type InitR = crate::BitReader;
#[doc = "Field `INIT` writer - Initialization"]
pub type InitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CCE` reader - Configuration Change Enable"]
pub type CceR = crate::BitReader;
#[doc = "Field `CCE` writer - Configuration Change Enable"]
pub type CceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ASM` reader - ASM Restricted Operation Mode"]
pub type AsmR = crate::BitReader;
#[doc = "Field `ASM` writer - ASM Restricted Operation Mode"]
pub type AsmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSA` reader - Clock Stop Acknowledge"]
pub type CsaR = crate::BitReader;
#[doc = "Field `CSA` writer - Clock Stop Acknowledge"]
pub type CsaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSR` reader - Clock Stop Request"]
pub type CsrR = crate::BitReader;
#[doc = "Field `CSR` writer - Clock Stop Request"]
pub type CsrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MON` reader - Bus Monitoring Mode"]
pub type MonR = crate::BitReader;
#[doc = "Field `MON` writer - Bus Monitoring Mode"]
pub type MonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DAR` reader - Disable Automatic Retransmission"]
pub type DarR = crate::BitReader;
#[doc = "Field `DAR` writer - Disable Automatic Retransmission"]
pub type DarW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEST` reader - Test Mode Enable"]
pub type TestR = crate::BitReader;
#[doc = "Field `TEST` writer - Test Mode Enable"]
pub type TestW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDOE` reader - FD Operation Enable"]
pub type FdoeR = crate::BitReader;
#[doc = "Field `FDOE` writer - FD Operation Enable"]
pub type FdoeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BSE` reader - FDCAN Bit Rate Switching"]
pub type BseR = crate::BitReader;
#[doc = "Field `BSE` writer - FDCAN Bit Rate Switching"]
pub type BseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PXHD` reader - Protocol Exception Handling Disable"]
pub type PxhdR = crate::BitReader;
#[doc = "Field `PXHD` writer - Protocol Exception Handling Disable"]
pub type PxhdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFBI` reader - Edge Filtering during Bus Integration"]
pub type EfbiR = crate::BitReader;
#[doc = "Field `EFBI` writer - Edge Filtering during Bus Integration"]
pub type EfbiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXP` reader - TXP"]
pub type TxpR = crate::BitReader;
#[doc = "Field `TXP` writer - TXP"]
pub type TxpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NISO` reader - Non ISO Operation"]
pub type NisoR = crate::BitReader;
#[doc = "Field `NISO` writer - Non ISO Operation"]
pub type NisoW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&self) -> InitR {
        InitR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&self) -> CceR {
        CceR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&self) -> AsmR {
        AsmR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&self) -> CsaR {
        CsaR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&self) -> CsrR {
        CsrR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&self) -> MonR {
        MonR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&self) -> DarR {
        DarR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&self) -> TestR {
        TestR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&self) -> FdoeR {
        FdoeR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FDCAN Bit Rate Switching"]
    #[inline(always)]
    pub fn bse(&self) -> BseR {
        BseR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&self) -> PxhdR {
        PxhdR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&self) -> EfbiR {
        EfbiR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&self) -> TxpR {
        TxpR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&self) -> NisoR {
        NisoR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Initialization"]
    #[inline(always)]
    pub fn init(&mut self) -> InitW<FdcanCccrSpec> {
        InitW::new(self, 0)
    }
    #[doc = "Bit 1 - Configuration Change Enable"]
    #[inline(always)]
    pub fn cce(&mut self) -> CceW<FdcanCccrSpec> {
        CceW::new(self, 1)
    }
    #[doc = "Bit 2 - ASM Restricted Operation Mode"]
    #[inline(always)]
    pub fn asm(&mut self) -> AsmW<FdcanCccrSpec> {
        AsmW::new(self, 2)
    }
    #[doc = "Bit 3 - Clock Stop Acknowledge"]
    #[inline(always)]
    pub fn csa(&mut self) -> CsaW<FdcanCccrSpec> {
        CsaW::new(self, 3)
    }
    #[doc = "Bit 4 - Clock Stop Request"]
    #[inline(always)]
    pub fn csr(&mut self) -> CsrW<FdcanCccrSpec> {
        CsrW::new(self, 4)
    }
    #[doc = "Bit 5 - Bus Monitoring Mode"]
    #[inline(always)]
    pub fn mon(&mut self) -> MonW<FdcanCccrSpec> {
        MonW::new(self, 5)
    }
    #[doc = "Bit 6 - Disable Automatic Retransmission"]
    #[inline(always)]
    pub fn dar(&mut self) -> DarW<FdcanCccrSpec> {
        DarW::new(self, 6)
    }
    #[doc = "Bit 7 - Test Mode Enable"]
    #[inline(always)]
    pub fn test(&mut self) -> TestW<FdcanCccrSpec> {
        TestW::new(self, 7)
    }
    #[doc = "Bit 8 - FD Operation Enable"]
    #[inline(always)]
    pub fn fdoe(&mut self) -> FdoeW<FdcanCccrSpec> {
        FdoeW::new(self, 8)
    }
    #[doc = "Bit 9 - FDCAN Bit Rate Switching"]
    #[inline(always)]
    pub fn bse(&mut self) -> BseW<FdcanCccrSpec> {
        BseW::new(self, 9)
    }
    #[doc = "Bit 12 - Protocol Exception Handling Disable"]
    #[inline(always)]
    pub fn pxhd(&mut self) -> PxhdW<FdcanCccrSpec> {
        PxhdW::new(self, 12)
    }
    #[doc = "Bit 13 - Edge Filtering during Bus Integration"]
    #[inline(always)]
    pub fn efbi(&mut self) -> EfbiW<FdcanCccrSpec> {
        EfbiW::new(self, 13)
    }
    #[doc = "Bit 14 - TXP"]
    #[inline(always)]
    pub fn txp(&mut self) -> TxpW<FdcanCccrSpec> {
        TxpW::new(self, 14)
    }
    #[doc = "Bit 15 - Non ISO Operation"]
    #[inline(always)]
    pub fn niso(&mut self) -> NisoW<FdcanCccrSpec> {
        NisoW::new(self, 15)
    }
}
#[doc = "FDCAN CC Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_cccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_cccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanCccrSpec;
impl crate::RegisterSpec for FdcanCccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_cccr::R`](R) reader structure"]
impl crate::Readable for FdcanCccrSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_cccr::W`](W) writer structure"]
impl crate::Writable for FdcanCccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_CCCR to value 0"]
impl crate::Resettable for FdcanCccrSpec {}
