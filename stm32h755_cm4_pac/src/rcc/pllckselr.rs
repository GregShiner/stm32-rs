#[doc = "Register `PLLCKSELR` reader"]
pub type R = crate::R<PllckselrSpec>;
#[doc = "Register `PLLCKSELR` writer"]
pub type W = crate::W<PllckselrSpec>;
#[doc = "Field `PLLSRC` reader - DIVMx and PLLs clock source selection"]
pub type PllsrcR = crate::FieldReader;
#[doc = "Field `PLLSRC` writer - DIVMx and PLLs clock source selection"]
pub type PllsrcW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DIVM1` reader - Prescaler for PLL1"]
pub type Divm1R = crate::FieldReader;
#[doc = "Field `DIVM1` writer - Prescaler for PLL1"]
pub type Divm1W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DIVM2` reader - Prescaler for PLL2"]
pub type Divm2R = crate::FieldReader;
#[doc = "Field `DIVM2` writer - Prescaler for PLL2"]
pub type Divm2W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `DIVM3` reader - Prescaler for PLL3"]
pub type Divm3R = crate::FieldReader;
#[doc = "Field `DIVM3` writer - Prescaler for PLL3"]
pub type Divm3W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PllsrcR {
        PllsrcR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&self) -> Divm1R {
        Divm1R::new(((self.bits >> 4) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&self) -> Divm2R {
        Divm2R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&self) -> Divm3R {
        Divm3R::new(((self.bits >> 20) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - DIVMx and PLLs clock source selection"]
    #[inline(always)]
    pub fn pllsrc(&mut self) -> PllsrcW<PllckselrSpec> {
        PllsrcW::new(self, 0)
    }
    #[doc = "Bits 4:9 - Prescaler for PLL1"]
    #[inline(always)]
    pub fn divm1(&mut self) -> Divm1W<PllckselrSpec> {
        Divm1W::new(self, 4)
    }
    #[doc = "Bits 12:17 - Prescaler for PLL2"]
    #[inline(always)]
    pub fn divm2(&mut self) -> Divm2W<PllckselrSpec> {
        Divm2W::new(self, 12)
    }
    #[doc = "Bits 20:25 - Prescaler for PLL3"]
    #[inline(always)]
    pub fn divm3(&mut self) -> Divm3W<PllckselrSpec> {
        Divm3W::new(self, 20)
    }
}
#[doc = "RCC PLLs Clock Source Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`pllckselr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllckselr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PllckselrSpec;
impl crate::RegisterSpec for PllckselrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pllckselr::R`](R) reader structure"]
impl crate::Readable for PllckselrSpec {}
#[doc = "`write(|w| ..)` method takes [`pllckselr::W`](W) writer structure"]
impl crate::Writable for PllckselrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PLLCKSELR to value 0x0202_0200"]
impl crate::Resettable for PllckselrSpec {
    const RESET_VALUE: u32 = 0x0202_0200;
}
