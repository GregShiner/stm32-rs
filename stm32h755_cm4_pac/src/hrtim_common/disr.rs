#[doc = "Register `DISR` reader"]
pub type R = crate::R<DisrSpec>;
#[doc = "Register `DISR` writer"]
pub type W = crate::W<DisrSpec>;
#[doc = "Field `TA1ODIS` reader - TA1ODIS"]
pub type Ta1odisR = crate::BitReader;
#[doc = "Field `TA1ODIS` writer - TA1ODIS"]
pub type Ta1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TA2ODIS` reader - TA2ODIS"]
pub type Ta2odisR = crate::BitReader;
#[doc = "Field `TA2ODIS` writer - TA2ODIS"]
pub type Ta2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB1ODIS` reader - TB1ODIS"]
pub type Tb1odisR = crate::BitReader;
#[doc = "Field `TB1ODIS` writer - TB1ODIS"]
pub type Tb1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TB2ODIS` reader - TB2ODIS"]
pub type Tb2odisR = crate::BitReader;
#[doc = "Field `TB2ODIS` writer - TB2ODIS"]
pub type Tb2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC1ODIS` reader - TC1ODIS"]
pub type Tc1odisR = crate::BitReader;
#[doc = "Field `TC1ODIS` writer - TC1ODIS"]
pub type Tc1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TC2ODIS` reader - TC2ODIS"]
pub type Tc2odisR = crate::BitReader;
#[doc = "Field `TC2ODIS` writer - TC2ODIS"]
pub type Tc2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD1ODIS` reader - TD1ODIS"]
pub type Td1odisR = crate::BitReader;
#[doc = "Field `TD1ODIS` writer - TD1ODIS"]
pub type Td1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TD2ODIS` reader - TD2ODIS"]
pub type Td2odisR = crate::BitReader;
#[doc = "Field `TD2ODIS` writer - TD2ODIS"]
pub type Td2odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE1ODIS` reader - TE1ODIS"]
pub type Te1odisR = crate::BitReader;
#[doc = "Field `TE1ODIS` writer - TE1ODIS"]
pub type Te1odisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE2ODIS` reader - TE2ODIS"]
pub type Te2odisR = crate::BitReader;
#[doc = "Field `TE2ODIS` writer - TE2ODIS"]
pub type Te2odisW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&self) -> Ta1odisR {
        Ta1odisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&self) -> Ta2odisR {
        Ta2odisR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&self) -> Tb1odisR {
        Tb1odisR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&self) -> Tb2odisR {
        Tb2odisR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&self) -> Tc1odisR {
        Tc1odisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&self) -> Tc2odisR {
        Tc2odisR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&self) -> Td1odisR {
        Td1odisR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&self) -> Td2odisR {
        Td2odisR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&self) -> Te1odisR {
        Te1odisR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&self) -> Te2odisR {
        Te2odisR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TA1ODIS"]
    #[inline(always)]
    pub fn ta1odis(&mut self) -> Ta1odisW<DisrSpec> {
        Ta1odisW::new(self, 0)
    }
    #[doc = "Bit 1 - TA2ODIS"]
    #[inline(always)]
    pub fn ta2odis(&mut self) -> Ta2odisW<DisrSpec> {
        Ta2odisW::new(self, 1)
    }
    #[doc = "Bit 2 - TB1ODIS"]
    #[inline(always)]
    pub fn tb1odis(&mut self) -> Tb1odisW<DisrSpec> {
        Tb1odisW::new(self, 2)
    }
    #[doc = "Bit 3 - TB2ODIS"]
    #[inline(always)]
    pub fn tb2odis(&mut self) -> Tb2odisW<DisrSpec> {
        Tb2odisW::new(self, 3)
    }
    #[doc = "Bit 4 - TC1ODIS"]
    #[inline(always)]
    pub fn tc1odis(&mut self) -> Tc1odisW<DisrSpec> {
        Tc1odisW::new(self, 4)
    }
    #[doc = "Bit 5 - TC2ODIS"]
    #[inline(always)]
    pub fn tc2odis(&mut self) -> Tc2odisW<DisrSpec> {
        Tc2odisW::new(self, 5)
    }
    #[doc = "Bit 6 - TD1ODIS"]
    #[inline(always)]
    pub fn td1odis(&mut self) -> Td1odisW<DisrSpec> {
        Td1odisW::new(self, 6)
    }
    #[doc = "Bit 7 - TD2ODIS"]
    #[inline(always)]
    pub fn td2odis(&mut self) -> Td2odisW<DisrSpec> {
        Td2odisW::new(self, 7)
    }
    #[doc = "Bit 8 - TE1ODIS"]
    #[inline(always)]
    pub fn te1odis(&mut self) -> Te1odisW<DisrSpec> {
        Te1odisW::new(self, 8)
    }
    #[doc = "Bit 9 - TE2ODIS"]
    #[inline(always)]
    pub fn te2odis(&mut self) -> Te2odisW<DisrSpec> {
        Te2odisW::new(self, 9)
    }
}
#[doc = "DISR\n\nYou can [`read`](crate::Reg::read) this register and get [`disr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisrSpec;
impl crate::RegisterSpec for DisrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disr::R`](R) reader structure"]
impl crate::Readable for DisrSpec {}
#[doc = "`write(|w| ..)` method takes [`disr::W`](W) writer structure"]
impl crate::Writable for DisrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISR to value 0"]
impl crate::Resettable for DisrSpec {}
