#[doc = "Register `CIER` reader"]
pub type R = crate::R<CierSpec>;
#[doc = "Register `CIER` writer"]
pub type W = crate::W<CierSpec>;
#[doc = "Field `LSIRDYIE` reader - LSI ready Interrupt Enable"]
pub type LsirdyieR = crate::BitReader;
#[doc = "Field `LSIRDYIE` writer - LSI ready Interrupt Enable"]
pub type LsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYIE` reader - LSE ready Interrupt Enable"]
pub type LserdyieR = crate::BitReader;
#[doc = "Field `LSERDYIE` writer - LSE ready Interrupt Enable"]
pub type LserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYIE` reader - HSI ready Interrupt Enable"]
pub type HsirdyieR = crate::BitReader;
#[doc = "Field `HSIRDYIE` writer - HSI ready Interrupt Enable"]
pub type HsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYIE` reader - HSE ready Interrupt Enable"]
pub type HserdyieR = crate::BitReader;
#[doc = "Field `HSERDYIE` writer - HSE ready Interrupt Enable"]
pub type HserdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIRDYIE` reader - CSI ready Interrupt Enable"]
pub type CsirdyieR = crate::BitReader;
#[doc = "Field `CSIRDYIE` writer - CSI ready Interrupt Enable"]
pub type CsirdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC48RDYIE` reader - RC48 ready Interrupt Enable"]
pub type Rc48rdyieR = crate::BitReader;
#[doc = "Field `RC48RDYIE` writer - RC48 ready Interrupt Enable"]
pub type Rc48rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDYIE` reader - PLL1 ready Interrupt Enable"]
pub type Pll1rdyieR = crate::BitReader;
#[doc = "Field `PLL1RDYIE` writer - PLL1 ready Interrupt Enable"]
pub type Pll1rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RDYIE` reader - PLL2 ready Interrupt Enable"]
pub type Pll2rdyieR = crate::BitReader;
#[doc = "Field `PLL2RDYIE` writer - PLL2 ready Interrupt Enable"]
pub type Pll2rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RDYIE` reader - PLL3 ready Interrupt Enable"]
pub type Pll3rdyieR = crate::BitReader;
#[doc = "Field `PLL3RDYIE` writer - PLL3 ready Interrupt Enable"]
pub type Pll3rdyieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSIE` reader - LSE clock security system Interrupt Enable"]
pub type LsecssieR = crate::BitReader;
#[doc = "Field `LSECSSIE` writer - LSE clock security system Interrupt Enable"]
pub type LsecssieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&self) -> LsirdyieR {
        LsirdyieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&self) -> LserdyieR {
        LserdyieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&self) -> HsirdyieR {
        HsirdyieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&self) -> HserdyieR {
        HserdyieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn csirdyie(&self) -> CsirdyieR {
        CsirdyieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    pub fn rc48rdyie(&self) -> Rc48rdyieR {
        Rc48rdyieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll1rdyie(&self) -> Pll1rdyieR {
        Pll1rdyieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll2rdyie(&self) -> Pll2rdyieR {
        Pll2rdyieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll3rdyie(&self) -> Pll3rdyieR {
        Pll3rdyieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    pub fn lsecssie(&self) -> LsecssieR {
        LsecssieR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn lsirdyie(&mut self) -> LsirdyieW<CierSpec> {
        LsirdyieW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn lserdyie(&mut self) -> LserdyieW<CierSpec> {
        LserdyieW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn hsirdyie(&mut self) -> HsirdyieW<CierSpec> {
        HsirdyieW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Enable"]
    #[inline(always)]
    pub fn hserdyie(&mut self) -> HserdyieW<CierSpec> {
        HserdyieW::new(self, 3)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Enable"]
    #[inline(always)]
    pub fn csirdyie(&mut self) -> CsirdyieW<CierSpec> {
        CsirdyieW::new(self, 4)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Enable"]
    #[inline(always)]
    pub fn rc48rdyie(&mut self) -> Rc48rdyieW<CierSpec> {
        Rc48rdyieW::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll1rdyie(&mut self) -> Pll1rdyieW<CierSpec> {
        Pll1rdyieW::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll2rdyie(&mut self) -> Pll2rdyieW<CierSpec> {
        Pll2rdyieW::new(self, 7)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Enable"]
    #[inline(always)]
    pub fn pll3rdyie(&mut self) -> Pll3rdyieW<CierSpec> {
        Pll3rdyieW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Enable"]
    #[inline(always)]
    pub fn lsecssie(&mut self) -> LsecssieW<CierSpec> {
        LsecssieW::new(self, 9)
    }
}
#[doc = "RCC Clock Source Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CierSpec;
impl crate::RegisterSpec for CierSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cier::R`](R) reader structure"]
impl crate::Readable for CierSpec {}
#[doc = "`write(|w| ..)` method takes [`cier::W`](W) writer structure"]
impl crate::Writable for CierSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIER to value 0"]
impl crate::Resettable for CierSpec {}
