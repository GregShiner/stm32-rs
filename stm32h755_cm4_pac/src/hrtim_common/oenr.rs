#[doc = "Register `OENR` writer"]
pub type W = crate::W<OenrSpec>;
#[doc = "Field `TA1OEN` writer - Timer A Output 1 Enable"]
pub type Ta1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2OEN` writer - Timer A Output 2 Enable"]
pub type Ta2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1OEN` writer - Timer B Output 1 Enable"]
pub type Tb1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2OEN` writer - Timer B Output 2 Enable"]
pub type Tb2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1OEN` writer - Timer C Output 1 Enable"]
pub type Tc1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2OEN` writer - Timer C Output 2 Enable"]
pub type Tc2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1OEN` writer - Timer D Output 1 Enable"]
pub type Td1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2OEN` writer - Timer D Output 2 Enable"]
pub type Td2oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1OEN` writer - Timer E Output 1 Enable"]
pub type Te1oenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2OEN` writer - Timer E Output 2 Enable"]
pub type Te2oenW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - Timer A Output 1 Enable"]
    #[inline(always)]
    pub fn ta1oen(&mut self) -> Ta1oenW<OenrSpec> {
        Ta1oenW::new(self, 0)
    }
    #[doc = "Bit 1 - Timer A Output 2 Enable"]
    #[inline(always)]
    pub fn ta2oen(&mut self) -> Ta2oenW<OenrSpec> {
        Ta2oenW::new(self, 1)
    }
    #[doc = "Bit 2 - Timer B Output 1 Enable"]
    #[inline(always)]
    pub fn tb1oen(&mut self) -> Tb1oenW<OenrSpec> {
        Tb1oenW::new(self, 2)
    }
    #[doc = "Bit 3 - Timer B Output 2 Enable"]
    #[inline(always)]
    pub fn tb2oen(&mut self) -> Tb2oenW<OenrSpec> {
        Tb2oenW::new(self, 3)
    }
    #[doc = "Bit 4 - Timer C Output 1 Enable"]
    #[inline(always)]
    pub fn tc1oen(&mut self) -> Tc1oenW<OenrSpec> {
        Tc1oenW::new(self, 4)
    }
    #[doc = "Bit 5 - Timer C Output 2 Enable"]
    #[inline(always)]
    pub fn tc2oen(&mut self) -> Tc2oenW<OenrSpec> {
        Tc2oenW::new(self, 5)
    }
    #[doc = "Bit 6 - Timer D Output 1 Enable"]
    #[inline(always)]
    pub fn td1oen(&mut self) -> Td1oenW<OenrSpec> {
        Td1oenW::new(self, 6)
    }
    #[doc = "Bit 7 - Timer D Output 2 Enable"]
    #[inline(always)]
    pub fn td2oen(&mut self) -> Td2oenW<OenrSpec> {
        Td2oenW::new(self, 7)
    }
    #[doc = "Bit 8 - Timer E Output 1 Enable"]
    #[inline(always)]
    pub fn te1oen(&mut self) -> Te1oenW<OenrSpec> {
        Te1oenW::new(self, 8)
    }
    #[doc = "Bit 9 - Timer E Output 2 Enable"]
    #[inline(always)]
    pub fn te2oen(&mut self) -> Te2oenW<OenrSpec> {
        Te2oenW::new(self, 9)
    }
}
#[doc = "Output Enable Register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`oenr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OenrSpec;
impl crate::RegisterSpec for OenrSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`oenr::W`](W) writer structure"]
impl crate::Writable for OenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OENR to value 0"]
impl crate::Resettable for OenrSpec {}
