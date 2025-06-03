#[doc = "Register `CIFR` reader"]
pub type R = crate::R<CifrSpec>;
#[doc = "Register `CIFR` writer"]
pub type W = crate::W<CifrSpec>;
#[doc = "Field `LSIRDYF` reader - LSI ready Interrupt Flag"]
pub type LsirdyfR = crate::BitReader;
#[doc = "Field `LSIRDYF` writer - LSI ready Interrupt Flag"]
pub type LsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSERDYF` reader - LSE ready Interrupt Flag"]
pub type LserdyfR = crate::BitReader;
#[doc = "Field `LSERDYF` writer - LSE ready Interrupt Flag"]
pub type LserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDYF` reader - HSI ready Interrupt Flag"]
pub type HsirdyfR = crate::BitReader;
#[doc = "Field `HSIRDYF` writer - HSI ready Interrupt Flag"]
pub type HsirdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDYF` reader - HSE ready Interrupt Flag"]
pub type HserdyfR = crate::BitReader;
#[doc = "Field `HSERDYF` writer - HSE ready Interrupt Flag"]
pub type HserdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIRDY` reader - CSI ready Interrupt Flag"]
pub type CsirdyR = crate::BitReader;
#[doc = "Field `CSIRDY` writer - CSI ready Interrupt Flag"]
pub type CsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC48RDYF` reader - RC48 ready Interrupt Flag"]
pub type Rc48rdyfR = crate::BitReader;
#[doc = "Field `RC48RDYF` writer - RC48 ready Interrupt Flag"]
pub type Rc48rdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDYF` reader - PLL1 ready Interrupt Flag"]
pub type Pll1rdyfR = crate::BitReader;
#[doc = "Field `PLL1RDYF` writer - PLL1 ready Interrupt Flag"]
pub type Pll1rdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RDYF` reader - PLL2 ready Interrupt Flag"]
pub type Pll2rdyfR = crate::BitReader;
#[doc = "Field `PLL2RDYF` writer - PLL2 ready Interrupt Flag"]
pub type Pll2rdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RDYF` reader - PLL3 ready Interrupt Flag"]
pub type Pll3rdyfR = crate::BitReader;
#[doc = "Field `PLL3RDYF` writer - PLL3 ready Interrupt Flag"]
pub type Pll3rdyfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LSECSSF` reader - LSE clock security system Interrupt Flag"]
pub type LsecssfR = crate::BitReader;
#[doc = "Field `LSECSSF` writer - LSE clock security system Interrupt Flag"]
pub type LsecssfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSF` reader - HSE clock security system Interrupt Flag"]
pub type HsecssfR = crate::BitReader;
#[doc = "Field `HSECSSF` writer - HSE clock security system Interrupt Flag"]
pub type HsecssfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&self) -> LsirdyfR {
        LsirdyfR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&self) -> LserdyfR {
        LserdyfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&self) -> HsirdyfR {
        HsirdyfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&self) -> HserdyfR {
        HserdyfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CsirdyR {
        CsirdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn rc48rdyf(&self) -> Rc48rdyfR {
        Rc48rdyfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&self) -> Pll1rdyfR {
        Pll1rdyfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&self) -> Pll2rdyfR {
        Pll2rdyfR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&self) -> Pll3rdyfR {
        Pll3rdyfR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&self) -> LsecssfR {
        LsecssfR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&self) -> HsecssfR {
        HsecssfR::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - LSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn lsirdyf(&mut self) -> LsirdyfW<CifrSpec> {
        LsirdyfW::new(self, 0)
    }
    #[doc = "Bit 1 - LSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn lserdyf(&mut self) -> LserdyfW<CifrSpec> {
        LserdyfW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn hsirdyf(&mut self) -> HsirdyfW<CifrSpec> {
        HsirdyfW::new(self, 2)
    }
    #[doc = "Bit 3 - HSE ready Interrupt Flag"]
    #[inline(always)]
    pub fn hserdyf(&mut self) -> HserdyfW<CifrSpec> {
        HserdyfW::new(self, 3)
    }
    #[doc = "Bit 4 - CSI ready Interrupt Flag"]
    #[inline(always)]
    pub fn csirdy(&mut self) -> CsirdyW<CifrSpec> {
        CsirdyW::new(self, 4)
    }
    #[doc = "Bit 5 - RC48 ready Interrupt Flag"]
    #[inline(always)]
    pub fn rc48rdyf(&mut self) -> Rc48rdyfW<CifrSpec> {
        Rc48rdyfW::new(self, 5)
    }
    #[doc = "Bit 6 - PLL1 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll1rdyf(&mut self) -> Pll1rdyfW<CifrSpec> {
        Pll1rdyfW::new(self, 6)
    }
    #[doc = "Bit 7 - PLL2 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll2rdyf(&mut self) -> Pll2rdyfW<CifrSpec> {
        Pll2rdyfW::new(self, 7)
    }
    #[doc = "Bit 8 - PLL3 ready Interrupt Flag"]
    #[inline(always)]
    pub fn pll3rdyf(&mut self) -> Pll3rdyfW<CifrSpec> {
        Pll3rdyfW::new(self, 8)
    }
    #[doc = "Bit 9 - LSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn lsecssf(&mut self) -> LsecssfW<CifrSpec> {
        LsecssfW::new(self, 9)
    }
    #[doc = "Bit 10 - HSE clock security system Interrupt Flag"]
    #[inline(always)]
    pub fn hsecssf(&mut self) -> HsecssfW<CifrSpec> {
        HsecssfW::new(self, 10)
    }
}
#[doc = "RCC Clock Source Interrupt Flag Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cifr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cifr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CifrSpec;
impl crate::RegisterSpec for CifrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cifr::R`](R) reader structure"]
impl crate::Readable for CifrSpec {}
#[doc = "`write(|w| ..)` method takes [`cifr::W`](W) writer structure"]
impl crate::Writable for CifrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CIFR to value 0"]
impl crate::Resettable for CifrSpec {}
