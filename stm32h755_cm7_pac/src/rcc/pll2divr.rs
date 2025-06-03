#[doc = "Register `PLL2DIVR` reader"]
pub type R = crate::R<Pll2divrSpec>;
#[doc = "Register `PLL2DIVR` writer"]
pub type W = crate::W<Pll2divrSpec>;
#[doc = "Field `DIVN2` reader - Multiplication factor for PLL1 VCO"]
pub type Divn2R = crate::FieldReader<u16>;
#[doc = "Field `DIVN2` writer - Multiplication factor for PLL1 VCO"]
pub type Divn2W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP2` reader - PLL1 DIVP division factor"]
pub type Divp2R = crate::FieldReader;
#[doc = "Field `DIVP2` writer - PLL1 DIVP division factor"]
pub type Divp2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVQ2` reader - PLL1 DIVQ division factor"]
pub type Divq2R = crate::FieldReader;
#[doc = "Field `DIVQ2` writer - PLL1 DIVQ division factor"]
pub type Divq2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVR2` reader - PLL1 DIVR division factor"]
pub type Divr2R = crate::FieldReader;
#[doc = "Field `DIVR2` writer - PLL1 DIVR division factor"]
pub type Divr2W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&self) -> Divn2R {
        Divn2R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&self) -> Divp2R {
        Divp2R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&self) -> Divq2R {
        Divq2R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&self) -> Divr2R {
        Divr2R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn2(&mut self) -> Divn2W<Pll2divrSpec> {
        Divn2W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL1 DIVP division factor"]
    #[inline(always)]
    pub fn divp2(&mut self) -> Divp2W<Pll2divrSpec> {
        Divp2W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL1 DIVQ division factor"]
    #[inline(always)]
    pub fn divq2(&mut self) -> Divq2W<Pll2divrSpec> {
        Divq2W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL1 DIVR division factor"]
    #[inline(always)]
    pub fn divr2(&mut self) -> Divr2W<Pll2divrSpec> {
        Divr2W::new(self, 24)
    }
}
#[doc = "RCC PLL2 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll2divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll2divrSpec;
impl crate::RegisterSpec for Pll2divrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll2divr::R`](R) reader structure"]
impl crate::Readable for Pll2divrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll2divr::W`](W) writer structure"]
impl crate::Writable for Pll2divrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL2DIVR to value 0x0101_0280"]
impl crate::Resettable for Pll2divrSpec {
    const RESET_VALUE: u32 = 0x0101_0280;
}
