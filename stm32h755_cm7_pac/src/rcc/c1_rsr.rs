#[doc = "Register `C1_RSR` reader"]
pub type R = crate::R<C1RsrSpec>;
#[doc = "Register `C1_RSR` writer"]
pub type W = crate::W<C1RsrSpec>;
#[doc = "Field `RMVF` reader - Remove reset flag"]
pub type RmvfR = crate::BitReader;
#[doc = "Field `RMVF` writer - Remove reset flag"]
pub type RmvfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CPURSTF` reader - CPU reset flag"]
pub type CpurstfR = crate::BitReader;
#[doc = "Field `CPURSTF` writer - CPU reset flag"]
pub type CpurstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1RSTF` reader - D1 domain power switch reset flag"]
pub type D1rstfR = crate::BitReader;
#[doc = "Field `D1RSTF` writer - D1 domain power switch reset flag"]
pub type D1rstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2RSTF` reader - D2 domain power switch reset flag"]
pub type D2rstfR = crate::BitReader;
#[doc = "Field `D2RSTF` writer - D2 domain power switch reset flag"]
pub type D2rstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BORRSTF` reader - BOR reset flag"]
pub type BorrstfR = crate::BitReader;
#[doc = "Field `BORRSTF` writer - BOR reset flag"]
pub type BorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINRSTF` reader - Pin reset flag (NRST)"]
pub type PinrstfR = crate::BitReader;
#[doc = "Field `PINRSTF` writer - Pin reset flag (NRST)"]
pub type PinrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PORRSTF` reader - POR/PDR reset flag"]
pub type PorrstfR = crate::BitReader;
#[doc = "Field `PORRSTF` writer - POR/PDR reset flag"]
pub type PorrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SFTRSTF` reader - System reset from CPU reset flag"]
pub type SftrstfR = crate::BitReader;
#[doc = "Field `SFTRSTF` writer - System reset from CPU reset flag"]
pub type SftrstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IWDG1RSTF` reader - Independent Watchdog reset flag"]
pub type Iwdg1rstfR = crate::BitReader;
#[doc = "Field `IWDG1RSTF` writer - Independent Watchdog reset flag"]
pub type Iwdg1rstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WWDG1RSTF` reader - Window Watchdog reset flag"]
pub type Wwdg1rstfR = crate::BitReader;
#[doc = "Field `WWDG1RSTF` writer - Window Watchdog reset flag"]
pub type Wwdg1rstfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPWRRSTF` reader - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LpwrrstfR = crate::BitReader;
#[doc = "Field `LPWRRSTF` writer - Reset due to illegal D1 DStandby or CPU CStop flag"]
pub type LpwrrstfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&self) -> RmvfR {
        RmvfR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&self) -> CpurstfR {
        CpurstfR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&self) -> D1rstfR {
        D1rstfR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&self) -> D2rstfR {
        D2rstfR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&self) -> BorrstfR {
        BorrstfR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&self) -> PinrstfR {
        PinrstfR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&self) -> PorrstfR {
        PorrstfR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&self) -> SftrstfR {
        SftrstfR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&self) -> Iwdg1rstfR {
        Iwdg1rstfR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&self) -> Wwdg1rstfR {
        Wwdg1rstfR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&self) -> LpwrrstfR {
        LpwrrstfR::new(((self.bits >> 30) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 16 - Remove reset flag"]
    #[inline(always)]
    pub fn rmvf(&mut self) -> RmvfW<C1RsrSpec> {
        RmvfW::new(self, 16)
    }
    #[doc = "Bit 17 - CPU reset flag"]
    #[inline(always)]
    pub fn cpurstf(&mut self) -> CpurstfW<C1RsrSpec> {
        CpurstfW::new(self, 17)
    }
    #[doc = "Bit 19 - D1 domain power switch reset flag"]
    #[inline(always)]
    pub fn d1rstf(&mut self) -> D1rstfW<C1RsrSpec> {
        D1rstfW::new(self, 19)
    }
    #[doc = "Bit 20 - D2 domain power switch reset flag"]
    #[inline(always)]
    pub fn d2rstf(&mut self) -> D2rstfW<C1RsrSpec> {
        D2rstfW::new(self, 20)
    }
    #[doc = "Bit 21 - BOR reset flag"]
    #[inline(always)]
    pub fn borrstf(&mut self) -> BorrstfW<C1RsrSpec> {
        BorrstfW::new(self, 21)
    }
    #[doc = "Bit 22 - Pin reset flag (NRST)"]
    #[inline(always)]
    pub fn pinrstf(&mut self) -> PinrstfW<C1RsrSpec> {
        PinrstfW::new(self, 22)
    }
    #[doc = "Bit 23 - POR/PDR reset flag"]
    #[inline(always)]
    pub fn porrstf(&mut self) -> PorrstfW<C1RsrSpec> {
        PorrstfW::new(self, 23)
    }
    #[doc = "Bit 24 - System reset from CPU reset flag"]
    #[inline(always)]
    pub fn sftrstf(&mut self) -> SftrstfW<C1RsrSpec> {
        SftrstfW::new(self, 24)
    }
    #[doc = "Bit 26 - Independent Watchdog reset flag"]
    #[inline(always)]
    pub fn iwdg1rstf(&mut self) -> Iwdg1rstfW<C1RsrSpec> {
        Iwdg1rstfW::new(self, 26)
    }
    #[doc = "Bit 28 - Window Watchdog reset flag"]
    #[inline(always)]
    pub fn wwdg1rstf(&mut self) -> Wwdg1rstfW<C1RsrSpec> {
        Wwdg1rstfW::new(self, 28)
    }
    #[doc = "Bit 30 - Reset due to illegal D1 DStandby or CPU CStop flag"]
    #[inline(always)]
    pub fn lpwrrstf(&mut self) -> LpwrrstfW<C1RsrSpec> {
        LpwrrstfW::new(self, 30)
    }
}
#[doc = "RCC Reset Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`c1_rsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`c1_rsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct C1RsrSpec;
impl crate::RegisterSpec for C1RsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`c1_rsr::R`](R) reader structure"]
impl crate::Readable for C1RsrSpec {}
#[doc = "`write(|w| ..)` method takes [`c1_rsr::W`](W) writer structure"]
impl crate::Writable for C1RsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets C1_RSR to value 0"]
impl crate::Resettable for C1RsrSpec {}
