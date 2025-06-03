#[doc = "Register `CICR` reader"]
pub type R = crate::R<CicrSpec>;
#[doc = "Register `CICR` writer"]
pub type W = crate::W<CicrSpec>;
#[doc = "Field `LSIRDYC` reader - LSI ready Interrupt Clear"]
pub type LsirdycR = crate::BitReader;
#[doc = "Field `LSIRDYC` writer - LSI ready Interrupt Clear"]
pub type LsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYC` reader - LSE ready Interrupt Clear"]
pub type LserdycR = crate::BitReader;
#[doc = "Field `LSERDYC` writer - LSE ready Interrupt Clear"]
pub type LserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYC` reader - HSI ready Interrupt Clear"]
pub type HsirdycR = crate::BitReader;
#[doc = "Field `HSIRDYC` writer - HSI ready Interrupt Clear"]
pub type HsirdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYC` reader - HSE ready Interrupt Clear"]
pub type HserdycR = crate::BitReader;
#[doc = "Field `HSERDYC` writer - HSE ready Interrupt Clear"]
pub type HserdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSE_ready_Interrupt_Clear` reader - CSI ready Interrupt Clear"]
pub type HseReadyInterruptClearR = crate::BitReader;
#[doc = "Field `HSE_ready_Interrupt_Clear` writer - CSI ready Interrupt Clear"]
pub type HseReadyInterruptClearW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC48RDYC` reader - RC48 ready Interrupt Clear"]
pub type Rc48rdycR = crate::BitReader;
#[doc = "Field `RC48RDYC` writer - RC48 ready Interrupt Clear"]
pub type Rc48rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDYC` reader - PLL1 ready Interrupt Clear"]
pub type Pll1rdycR = crate::BitReader;
#[doc = "Field `PLL1RDYC` writer - PLL1 ready Interrupt Clear"]
pub type Pll1rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RDYC` reader - PLL2 ready Interrupt Clear"]
pub type Pll2rdycR = crate::BitReader;
#[doc = "Field `PLL2RDYC` writer - PLL2 ready Interrupt Clear"]
pub type Pll2rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RDYC` reader - PLL3 ready Interrupt Clear"]
pub type Pll3rdycR = crate::BitReader;
#[doc = "Field `PLL3RDYC` writer - PLL3 ready Interrupt Clear"]
pub type Pll3rdycW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSC` reader - LSE clock security system Interrupt Clear"]
pub type LsecsscR = crate::BitReader;
#[doc = "Field `LSECSSC` writer - LSE clock security system Interrupt Clear"]
pub type LsecsscW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSC` reader - HSE clock security system Interrupt Clear"]
pub type HsecsscR = crate::BitReader;
#[doc = "Field `HSECSSC` writer - HSE clock security system Interrupt Clear"]
pub type HsecsscW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&self) -> LsirdycR {
        LsirdycR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&self) -> LserdycR {
        LserdycR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&self) -> HsirdycR {
        HsirdycR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&self) -> HserdycR {
        HserdycR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hse_ready_interrupt_clear(&self) -> HseReadyInterruptClearR {
        HseReadyInterruptClearR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Clear"]
    #[inline(always)]
    pub fn rc48rdyc(&self) -> Rc48rdycR {
        Rc48rdycR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll1rdyc(&self) -> Pll1rdycR {
        Pll1rdycR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll2rdyc(&self) -> Pll2rdycR {
        Pll2rdycR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll3rdyc(&self) -> Pll3rdycR {
        Pll3rdycR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn lsecssc(&self) -> LsecsscR {
        LsecsscR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn hsecssc(&self) -> HsecsscR {
        HsecsscR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn lsirdyc(&mut self) -> LsirdycW<CicrSpec> {
        LsirdycW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn lserdyc(&mut self) -> LserdycW<CicrSpec> {
        LserdycW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hsirdyc(&mut self) -> HsirdycW<CicrSpec> {
        HsirdycW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Clear"]
    #[inline(always)]
    pub fn hserdyc(&mut self) -> HserdycW<CicrSpec> {
        HserdycW::new(self, 3)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Clear"]
    #[inline(always)]
    pub fn hse_ready_interrupt_clear(&mut self) -> HseReadyInterruptClearW<CicrSpec> {
        HseReadyInterruptClearW::new(self, 4)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Clear"]
    #[inline(always)]
    pub fn rc48rdyc(&mut self) -> Rc48rdycW<CicrSpec> {
        Rc48rdycW::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll1rdyc(&mut self) -> Pll1rdycW<CicrSpec> {
        Pll1rdycW::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll2rdyc(&mut self) -> Pll2rdycW<CicrSpec> {
        Pll2rdycW::new(self, 7)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Clear"]
    #[inline(always)]
    pub fn pll3rdyc(&mut self) -> Pll3rdycW<CicrSpec> {
        Pll3rdycW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn lsecssc(&mut self) -> LsecsscW<CicrSpec> {
        LsecsscW::new(self, 9)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Clear"]
    #[inline(always)]
    pub fn hsecssc(&mut self) -> HsecsscW<CicrSpec> {
        HsecsscW::new(self, 10)
    }
}
#[doc = "RCC Clock Source Interrupt Clear Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cicr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cicr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CicrSpec;
impl crate::RegisterSpec for CicrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cicr::R`](R) reader structure"]
impl crate::Readable for CicrSpec {}
#[doc = "`write(|w| ..)` method takes [`cicr::W`](W) writer structure"]
impl crate::Writable for CicrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CICR to value 0"]
impl crate::Resettable for CicrSpec {}
