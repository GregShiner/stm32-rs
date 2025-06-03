#[doc = "Register `PLL1DIVR` reader"]
pub type R = crate::R<Pll1divrSpec>;
#[doc = "Register `PLL1DIVR` writer"]
pub type W = crate::W<Pll1divrSpec>;
#[doc = "Field `DIVN1` reader - Multiplication factor for PLL1 VCO"]
pub type Divn1R = crate::FieldReader<u16>;
#[doc = "Field `DIVN1` writer - Multiplication factor for PLL1 VCO"]
pub type Divn1W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP1` reader - PLL1 DIVP division factor"]
pub type Divp1R = crate::FieldReader;
#[doc = "Field `DIVP1` writer - PLL1 DIVP division factor"]
pub type Divp1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVQ1` reader - PLL1 DIVQ division factor"]
pub type Divq1R = crate::FieldReader;
#[doc = "Field `DIVQ1` writer - PLL1 DIVQ division factor"]
pub type Divq1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVR1` reader - PLL1 DIVR division factor"]
pub type Divr1R = crate::FieldReader;
#[doc = "Field `DIVR1` writer - PLL1 DIVR division factor"]
pub type Divr1W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn1(&self) -> Divn1R {
        Divn1R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp1(&self) -> Divp1R {
        Divp1R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq1(&self) -> Divq1R {
        Divq1R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr1(&self) -> Divr1R {
        Divr1R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn1(&mut self) -> Divn1W<Pll1divrSpec> {
        Divn1W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp1(&mut self) -> Divp1W<Pll1divrSpec> {
        Divp1W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq1(&mut self) -> Divq1W<Pll1divrSpec> {
        Divq1W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr1(&mut self) -> Divr1W<Pll1divrSpec> {
        Divr1W::new(self, 24)
    }
}
#[doc = "RCC PLL1 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll1divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll1divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll1divrSpec;
impl crate::RegisterSpec for Pll1divrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll1divr::R`](R) reader structure"]
impl crate::Readable for Pll1divrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll1divr::W`](W) writer structure"]
impl crate::Writable for Pll1divrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL1DIVR to value 0x0101_0280"]
impl crate::Resettable for Pll1divrSpec {
    const RESET_VALUE: u32 = 0x0101_0280;
}
