#[doc = "Register `PLL3DIVR` reader"]
pub type R = crate::R<Pll3divrSpec>;
#[doc = "Register `PLL3DIVR` writer"]
pub type W = crate::W<Pll3divrSpec>;
#[doc = "Field `DIVN3` reader - Multiplication factor for PLL1 VCO"]
pub type Divn3R = crate::FieldReader<u16>;
#[doc = "Field `DIVN3` writer - Multiplication factor for PLL1 VCO"]
pub type Divn3W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `DIVP3` reader - PLL DIVP division factor"]
pub type Divp3R = crate::FieldReader;
#[doc = "Field `DIVP3` writer - PLL DIVP division factor"]
pub type Divp3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVQ3` reader - PLL DIVQ division factor"]
pub type Divq3R = crate::FieldReader;
#[doc = "Field `DIVQ3` writer - PLL DIVQ division factor"]
pub type Divq3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `DIVR3` reader - PLL DIVR division factor"]
pub type Divr3R = crate::FieldReader;
#[doc = "Field `DIVR3` writer - PLL DIVR division factor"]
pub type Divr3W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&self) -> Divn3R {
        Divn3R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&self) -> Divp3R {
        Divp3R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&self) -> Divq3R {
        Divq3R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&self) -> Divr3R {
        Divr3R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Multiplication factor for PLL1 VCO"]
    #[inline(always)]
    pub fn divn3(&mut self) -> Divn3W<Pll3divrSpec> {
        Divn3W::new(self, 0)
    }
    #[doc = "Bits 9:15 - PLL DIVP division factor"]
    #[inline(always)]
    pub fn divp3(&mut self) -> Divp3W<Pll3divrSpec> {
        Divp3W::new(self, 9)
    }
    #[doc = "Bits 16:22 - PLL DIVQ division factor"]
    #[inline(always)]
    pub fn divq3(&mut self) -> Divq3W<Pll3divrSpec> {
        Divq3W::new(self, 16)
    }
    #[doc = "Bits 24:30 - PLL DIVR division factor"]
    #[inline(always)]
    pub fn divr3(&mut self) -> Divr3W<Pll3divrSpec> {
        Divr3W::new(self, 24)
    }
}
#[doc = "RCC PLL3 Dividers Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pll3divr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll3divr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Pll3divrSpec;
impl crate::RegisterSpec for Pll3divrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pll3divr::R`](R) reader structure"]
impl crate::Readable for Pll3divrSpec {}
#[doc = "`write(|w| ..)` method takes [`pll3divr::W`](W) writer structure"]
impl crate::Writable for Pll3divrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLL3DIVR to value 0x0101_0280"]
impl crate::Resettable for Pll3divrSpec {
    const RESET_VALUE: u32 = 0x0101_0280;
}
