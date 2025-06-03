#[doc = "Register `PLLCFGR` reader"]
pub type R = crate::R<PllcfgrSpec>;
#[doc = "Register `PLLCFGR` writer"]
pub type W = crate::W<PllcfgrSpec>;
#[doc = "Field `PLL1FRACEN` reader - PLL1 fractional latch enable"]
pub type Pll1fracenR = crate::BitReader;
#[doc = "Field `PLL1FRACEN` writer - PLL1 fractional latch enable"]
pub type Pll1fracenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1VCOSEL` reader - PLL1 VCO selection"]
pub type Pll1vcoselR = crate::BitReader;
#[doc = "Field `PLL1VCOSEL` writer - PLL1 VCO selection"]
pub type Pll1vcoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL1RGE` reader - PLL1 input frequency range"]
pub type Pll1rgeR = crate::FieldReader;
#[doc = "Field `PLL1RGE` writer - PLL1 input frequency range"]
pub type Pll1rgeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL2FRACEN` reader - PLL2 fractional latch enable"]
pub type Pll2fracenR = crate::BitReader;
#[doc = "Field `PLL2FRACEN` writer - PLL2 fractional latch enable"]
pub type Pll2fracenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2VCOSEL` reader - PLL2 VCO selection"]
pub type Pll2vcoselR = crate::BitReader;
#[doc = "Field `PLL2VCOSEL` writer - PLL2 VCO selection"]
pub type Pll2vcoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL2RGE` reader - PLL2 input frequency range"]
pub type Pll2rgeR = crate::FieldReader;
#[doc = "Field `PLL2RGE` writer - PLL2 input frequency range"]
pub type Pll2rgeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PLL3FRACEN` reader - PLL3 fractional latch enable"]
pub type Pll3fracenR = crate::BitReader;
#[doc = "Field `PLL3FRACEN` writer - PLL3 fractional latch enable"]
pub type Pll3fracenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3VCOSEL` reader - PLL3 VCO selection"]
pub type Pll3vcoselR = crate::BitReader;
#[doc = "Field `PLL3VCOSEL` writer - PLL3 VCO selection"]
pub type Pll3vcoselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLL3RGE` reader - PLL3 input frequency range"]
pub type Pll3rgeR = crate::FieldReader;
#[doc = "Field `PLL3RGE` writer - PLL3 input frequency range"]
pub type Pll3rgeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIVP1EN` reader - PLL1 DIVP divider output enable"]
pub type Divp1enR = crate::BitReader;
#[doc = "Field `DIVP1EN` writer - PLL1 DIVP divider output enable"]
pub type Divp1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVQ1EN` reader - PLL1 DIVQ divider output enable"]
pub type Divq1enR = crate::BitReader;
#[doc = "Field `DIVQ1EN` writer - PLL1 DIVQ divider output enable"]
pub type Divq1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVR1EN` reader - PLL1 DIVR divider output enable"]
pub type Divr1enR = crate::BitReader;
#[doc = "Field `DIVR1EN` writer - PLL1 DIVR divider output enable"]
pub type Divr1enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVP2EN` reader - PLL2 DIVP divider output enable"]
pub type Divp2enR = crate::BitReader;
#[doc = "Field `DIVP2EN` writer - PLL2 DIVP divider output enable"]
pub type Divp2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVQ2EN` reader - PLL2 DIVQ divider output enable"]
pub type Divq2enR = crate::BitReader;
#[doc = "Field `DIVQ2EN` writer - PLL2 DIVQ divider output enable"]
pub type Divq2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVR2EN` reader - PLL2 DIVR divider output enable"]
pub type Divr2enR = crate::BitReader;
#[doc = "Field `DIVR2EN` writer - PLL2 DIVR divider output enable"]
pub type Divr2enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVP3EN` reader - PLL3 DIVP divider output enable"]
pub type Divp3enR = crate::BitReader;
#[doc = "Field `DIVP3EN` writer - PLL3 DIVP divider output enable"]
pub type Divp3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVQ3EN` reader - PLL3 DIVQ divider output enable"]
pub type Divq3enR = crate::BitReader;
#[doc = "Field `DIVQ3EN` writer - PLL3 DIVQ divider output enable"]
pub type Divq3enW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIVR3EN` reader - PLL3 DIVR divider output enable"]
pub type Divr3enR = crate::BitReader;
#[doc = "Field `DIVR3EN` writer - PLL3 DIVR divider output enable"]
pub type Divr3enW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&self) -> Pll1fracenR {
        Pll1fracenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&self) -> Pll1vcoselR {
        Pll1vcoselR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&self) -> Pll1rgeR {
        Pll1rgeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&self) -> Pll2fracenR {
        Pll2fracenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&self) -> Pll2vcoselR {
        Pll2vcoselR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&self) -> Pll2rgeR {
        Pll2rgeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&self) -> Pll3fracenR {
        Pll3fracenR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&self) -> Pll3vcoselR {
        Pll3vcoselR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&self) -> Pll3rgeR {
        Pll3rgeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&self) -> Divp1enR {
        Divp1enR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&self) -> Divq1enR {
        Divq1enR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&self) -> Divr1enR {
        Divr1enR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&self) -> Divp2enR {
        Divp2enR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&self) -> Divq2enR {
        Divq2enR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&self) -> Divr2enR {
        Divr2enR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&self) -> Divp3enR {
        Divp3enR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&self) -> Divq3enR {
        Divq3enR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&self) -> Divr3enR {
        Divr3enR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - PLL1 fractional latch enable"]
    #[inline(always)]
    pub fn pll1fracen(&mut self) -> Pll1fracenW<PllcfgrSpec> {
        Pll1fracenW::new(self, 0)
    }
    #[doc = "Bit 1 - PLL1 VCO selection"]
    #[inline(always)]
    pub fn pll1vcosel(&mut self) -> Pll1vcoselW<PllcfgrSpec> {
        Pll1vcoselW::new(self, 1)
    }
    #[doc = "Bits 2:3 - PLL1 input frequency range"]
    #[inline(always)]
    pub fn pll1rge(&mut self) -> Pll1rgeW<PllcfgrSpec> {
        Pll1rgeW::new(self, 2)
    }
    #[doc = "Bit 4 - PLL2 fractional latch enable"]
    #[inline(always)]
    pub fn pll2fracen(&mut self) -> Pll2fracenW<PllcfgrSpec> {
        Pll2fracenW::new(self, 4)
    }
    #[doc = "Bit 5 - PLL2 VCO selection"]
    #[inline(always)]
    pub fn pll2vcosel(&mut self) -> Pll2vcoselW<PllcfgrSpec> {
        Pll2vcoselW::new(self, 5)
    }
    #[doc = "Bits 6:7 - PLL2 input frequency range"]
    #[inline(always)]
    pub fn pll2rge(&mut self) -> Pll2rgeW<PllcfgrSpec> {
        Pll2rgeW::new(self, 6)
    }
    #[doc = "Bit 8 - PLL3 fractional latch enable"]
    #[inline(always)]
    pub fn pll3fracen(&mut self) -> Pll3fracenW<PllcfgrSpec> {
        Pll3fracenW::new(self, 8)
    }
    #[doc = "Bit 9 - PLL3 VCO selection"]
    #[inline(always)]
    pub fn pll3vcosel(&mut self) -> Pll3vcoselW<PllcfgrSpec> {
        Pll3vcoselW::new(self, 9)
    }
    #[doc = "Bits 10:11 - PLL3 input frequency range"]
    #[inline(always)]
    pub fn pll3rge(&mut self) -> Pll3rgeW<PllcfgrSpec> {
        Pll3rgeW::new(self, 10)
    }
    #[doc = "Bit 16 - PLL1 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp1en(&mut self) -> Divp1enW<PllcfgrSpec> {
        Divp1enW::new(self, 16)
    }
    #[doc = "Bit 17 - PLL1 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq1en(&mut self) -> Divq1enW<PllcfgrSpec> {
        Divq1enW::new(self, 17)
    }
    #[doc = "Bit 18 - PLL1 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr1en(&mut self) -> Divr1enW<PllcfgrSpec> {
        Divr1enW::new(self, 18)
    }
    #[doc = "Bit 19 - PLL2 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp2en(&mut self) -> Divp2enW<PllcfgrSpec> {
        Divp2enW::new(self, 19)
    }
    #[doc = "Bit 20 - PLL2 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq2en(&mut self) -> Divq2enW<PllcfgrSpec> {
        Divq2enW::new(self, 20)
    }
    #[doc = "Bit 21 - PLL2 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr2en(&mut self) -> Divr2enW<PllcfgrSpec> {
        Divr2enW::new(self, 21)
    }
    #[doc = "Bit 22 - PLL3 DIVP divider output enable"]
    #[inline(always)]
    pub fn divp3en(&mut self) -> Divp3enW<PllcfgrSpec> {
        Divp3enW::new(self, 22)
    }
    #[doc = "Bit 23 - PLL3 DIVQ divider output enable"]
    #[inline(always)]
    pub fn divq3en(&mut self) -> Divq3enW<PllcfgrSpec> {
        Divq3enW::new(self, 23)
    }
    #[doc = "Bit 24 - PLL3 DIVR divider output enable"]
    #[inline(always)]
    pub fn divr3en(&mut self) -> Divr3enW<PllcfgrSpec> {
        Divr3enW::new(self, 24)
    }
}
#[doc = "RCC PLLs Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllcfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllcfgrSpec;
impl crate::RegisterSpec for PllcfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllcfgr::R`](R) reader structure"]
impl crate::Readable for PllcfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllcfgr::W`](W) writer structure"]
impl crate::Writable for PllcfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCFGR to value 0x01ff_0000"]
impl crate::Resettable for PllcfgrSpec {
    const RESET_VALUE: u32 = 0x01ff_0000;
}
