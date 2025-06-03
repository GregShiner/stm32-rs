#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `HSION` reader - Internal high-speed clock enable"]
pub type HsionR = crate::BitReader;
#[doc = "Field `HSION` writer - Internal high-speed clock enable"]
pub type HsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIKERON` reader - High Speed Internal clock enable in Stop mode"]
pub type HsikeronR = crate::BitReader;
#[doc = "Field `HSIKERON` writer - High Speed Internal clock enable in Stop mode"]
pub type HsikeronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIRDY` reader - HSI clock ready flag"]
pub type HsirdyR = crate::BitReader;
#[doc = "Field `HSIRDY` writer - HSI clock ready flag"]
pub type HsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSIDIV` reader - HSI clock divider"]
pub type HsidivR = crate::FieldReader;
#[doc = "Field `HSIDIV` writer - HSI clock divider"]
pub type HsidivW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSIDIVF` reader - HSI divider flag"]
pub type HsidivfR = crate::BitReader;
#[doc = "Field `HSIDIVF` writer - HSI divider flag"]
pub type HsidivfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSION` reader - CSI clock enable"]
pub type CsionR = crate::BitReader;
#[doc = "Field `CSION` writer - CSI clock enable"]
pub type CsionW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIRDY` reader - CSI clock ready flag"]
pub type CsirdyR = crate::BitReader;
#[doc = "Field `CSIRDY` writer - CSI clock ready flag"]
pub type CsirdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSIKERON` reader - CSI clock enable in Stop mode"]
pub type CsikeronR = crate::BitReader;
#[doc = "Field `CSIKERON` writer - CSI clock enable in Stop mode"]
pub type CsikeronW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC48ON` reader - RC48 clock enable"]
pub type Rc48onR = crate::BitReader;
#[doc = "Field `RC48ON` writer - RC48 clock enable"]
pub type Rc48onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RC48RDY` reader - RC48 clock ready flag"]
pub type Rc48rdyR = crate::BitReader;
#[doc = "Field `RC48RDY` writer - RC48 clock ready flag"]
pub type Rc48rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D1CKRDY` reader - D1 domain clocks ready flag"]
pub type D1ckrdyR = crate::BitReader;
#[doc = "Field `D1CKRDY` writer - D1 domain clocks ready flag"]
pub type D1ckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `D2CKRDY` reader - D2 domain clocks ready flag"]
pub type D2ckrdyR = crate::BitReader;
#[doc = "Field `D2CKRDY` writer - D2 domain clocks ready flag"]
pub type D2ckrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEON` reader - HSE clock enable"]
pub type HseonR = crate::BitReader;
#[doc = "Field `HSEON` writer - HSE clock enable"]
pub type HseonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSERDY` reader - HSE clock ready flag"]
pub type HserdyR = crate::BitReader;
#[doc = "Field `HSERDY` writer - HSE clock ready flag"]
pub type HserdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSEBYP` reader - HSE clock bypass"]
pub type HsebypR = crate::BitReader;
#[doc = "Field `HSEBYP` writer - HSE clock bypass"]
pub type HsebypW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSECSSON` reader - HSE Clock Security System enable"]
pub type HsecssonR = crate::BitReader;
#[doc = "Field `HSECSSON` writer - HSE Clock Security System enable"]
pub type HsecssonW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1ON` reader - PLL1 enable"]
pub type Pll1onR = crate::BitReader;
#[doc = "Field `PLL1ON` writer - PLL1 enable"]
pub type Pll1onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RDY` reader - PLL1 clock ready flag"]
pub type Pll1rdyR = crate::BitReader;
#[doc = "Field `PLL1RDY` writer - PLL1 clock ready flag"]
pub type Pll1rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2ON` reader - PLL2 enable"]
pub type Pll2onR = crate::BitReader;
#[doc = "Field `PLL2ON` writer - PLL2 enable"]
pub type Pll2onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RDY` reader - PLL2 clock ready flag"]
pub type Pll2rdyR = crate::BitReader;
#[doc = "Field `PLL2RDY` writer - PLL2 clock ready flag"]
pub type Pll2rdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3ON` reader - PLL3 enable"]
pub type Pll3onR = crate::BitReader;
#[doc = "Field `PLL3ON` writer - PLL3 enable"]
pub type Pll3onW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RDY` reader - PLL3 clock ready flag"]
pub type Pll3rdyR = crate::BitReader;
#[doc = "Field `PLL3RDY` writer - PLL3 clock ready flag"]
pub type Pll3rdyW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&self) -> HsionR {
        HsionR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&self) -> HsikeronR {
        HsikeronR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&self) -> HsirdyR {
        HsirdyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&self) -> HsidivR {
        HsidivR::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&self) -> HsidivfR {
        HsidivfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&self) -> CsionR {
        CsionR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&self) -> CsirdyR {
        CsirdyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&self) -> CsikeronR {
        CsikeronR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn rc48on(&self) -> Rc48onR {
        Rc48onR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn rc48rdy(&self) -> Rc48rdyR {
        Rc48rdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&self) -> D1ckrdyR {
        D1ckrdyR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&self) -> D2ckrdyR {
        D2ckrdyR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&self) -> HseonR {
        HseonR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&self) -> HserdyR {
        HserdyR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&self) -> HsebypR {
        HsebypR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&self) -> HsecssonR {
        HsecssonR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&self) -> Pll1onR {
        Pll1onR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&self) -> Pll1rdyR {
        Pll1rdyR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&self) -> Pll2onR {
        Pll2onR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&self) -> Pll2rdyR {
        Pll2rdyR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&self) -> Pll3onR {
        Pll3onR::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&self) -> Pll3rdyR {
        Pll3rdyR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Internal high-speed clock enable"]
    #[inline(always)]
    pub fn hsion(&mut self) -> HsionW<CrSpec> {
        HsionW::new(self, 0)
    }
    #[doc = "Bit 1 - High Speed Internal clock enable in Stop mode"]
    #[inline(always)]
    pub fn hsikeron(&mut self) -> HsikeronW<CrSpec> {
        HsikeronW::new(self, 1)
    }
    #[doc = "Bit 2 - HSI clock ready flag"]
    #[inline(always)]
    pub fn hsirdy(&mut self) -> HsirdyW<CrSpec> {
        HsirdyW::new(self, 2)
    }
    #[doc = "Bits 3:4 - HSI clock divider"]
    #[inline(always)]
    pub fn hsidiv(&mut self) -> HsidivW<CrSpec> {
        HsidivW::new(self, 3)
    }
    #[doc = "Bit 5 - HSI divider flag"]
    #[inline(always)]
    pub fn hsidivf(&mut self) -> HsidivfW<CrSpec> {
        HsidivfW::new(self, 5)
    }
    #[doc = "Bit 7 - CSI clock enable"]
    #[inline(always)]
    pub fn csion(&mut self) -> CsionW<CrSpec> {
        CsionW::new(self, 7)
    }
    #[doc = "Bit 8 - CSI clock ready flag"]
    #[inline(always)]
    pub fn csirdy(&mut self) -> CsirdyW<CrSpec> {
        CsirdyW::new(self, 8)
    }
    #[doc = "Bit 9 - CSI clock enable in Stop mode"]
    #[inline(always)]
    pub fn csikeron(&mut self) -> CsikeronW<CrSpec> {
        CsikeronW::new(self, 9)
    }
    #[doc = "Bit 12 - RC48 clock enable"]
    #[inline(always)]
    pub fn rc48on(&mut self) -> Rc48onW<CrSpec> {
        Rc48onW::new(self, 12)
    }
    #[doc = "Bit 13 - RC48 clock ready flag"]
    #[inline(always)]
    pub fn rc48rdy(&mut self) -> Rc48rdyW<CrSpec> {
        Rc48rdyW::new(self, 13)
    }
    #[doc = "Bit 14 - D1 domain clocks ready flag"]
    #[inline(always)]
    pub fn d1ckrdy(&mut self) -> D1ckrdyW<CrSpec> {
        D1ckrdyW::new(self, 14)
    }
    #[doc = "Bit 15 - D2 domain clocks ready flag"]
    #[inline(always)]
    pub fn d2ckrdy(&mut self) -> D2ckrdyW<CrSpec> {
        D2ckrdyW::new(self, 15)
    }
    #[doc = "Bit 16 - HSE clock enable"]
    #[inline(always)]
    pub fn hseon(&mut self) -> HseonW<CrSpec> {
        HseonW::new(self, 16)
    }
    #[doc = "Bit 17 - HSE clock ready flag"]
    #[inline(always)]
    pub fn hserdy(&mut self) -> HserdyW<CrSpec> {
        HserdyW::new(self, 17)
    }
    #[doc = "Bit 18 - HSE clock bypass"]
    #[inline(always)]
    pub fn hsebyp(&mut self) -> HsebypW<CrSpec> {
        HsebypW::new(self, 18)
    }
    #[doc = "Bit 19 - HSE Clock Security System enable"]
    #[inline(always)]
    pub fn hsecsson(&mut self) -> HsecssonW<CrSpec> {
        HsecssonW::new(self, 19)
    }
    #[doc = "Bit 24 - PLL1 enable"]
    #[inline(always)]
    pub fn pll1on(&mut self) -> Pll1onW<CrSpec> {
        Pll1onW::new(self, 24)
    }
    #[doc = "Bit 25 - PLL1 clock ready flag"]
    #[inline(always)]
    pub fn pll1rdy(&mut self) -> Pll1rdyW<CrSpec> {
        Pll1rdyW::new(self, 25)
    }
    #[doc = "Bit 26 - PLL2 enable"]
    #[inline(always)]
    pub fn pll2on(&mut self) -> Pll2onW<CrSpec> {
        Pll2onW::new(self, 26)
    }
    #[doc = "Bit 27 - PLL2 clock ready flag"]
    #[inline(always)]
    pub fn pll2rdy(&mut self) -> Pll2rdyW<CrSpec> {
        Pll2rdyW::new(self, 27)
    }
    #[doc = "Bit 28 - PLL3 enable"]
    #[inline(always)]
    pub fn pll3on(&mut self) -> Pll3onW<CrSpec> {
        Pll3onW::new(self, 28)
    }
    #[doc = "Bit 29 - PLL3 clock ready flag"]
    #[inline(always)]
    pub fn pll3rdy(&mut self) -> Pll3rdyW<CrSpec> {
        Pll3rdyW::new(self, 29)
    }
}
#[doc = "clock control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0x83"]
impl crate::Resettable for CrSpec {
    const RESET_VALUE: u32 = 0x83;
}
