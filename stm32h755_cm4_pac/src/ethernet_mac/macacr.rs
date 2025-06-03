#[doc = "Register `MACACR` reader"]
pub type R = crate::R<MacacrSpec>;
#[doc = "Register `MACACR` writer"]
pub type W = crate::W<MacacrSpec>;
#[doc = "Field `ATSFC` reader - ATSFC"]
pub type AtsfcR = crate::BitReader;
#[doc = "Field `ATSFC` writer - ATSFC"]
pub type AtsfcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN0` reader - ATSEN0"]
pub type Atsen0R = crate::BitReader;
#[doc = "Field `ATSEN0` writer - ATSEN0"]
pub type Atsen0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN1` reader - ATSEN1"]
pub type Atsen1R = crate::BitReader;
#[doc = "Field `ATSEN1` writer - ATSEN1"]
pub type Atsen1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN2` reader - ATSEN2"]
pub type Atsen2R = crate::BitReader;
#[doc = "Field `ATSEN2` writer - ATSEN2"]
pub type Atsen2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ATSEN3` reader - ATSEN3"]
pub type Atsen3R = crate::BitReader;
#[doc = "Field `ATSEN3` writer - ATSEN3"]
pub type Atsen3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&self) -> AtsfcR {
        AtsfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&self) -> Atsen0R {
        Atsen0R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&self) -> Atsen1R {
        Atsen1R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&self) -> Atsen2R {
        Atsen2R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&self) -> Atsen3R {
        Atsen3R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ATSFC"]
    #[inline(always)]
    pub fn atsfc(&mut self) -> AtsfcW<MacacrSpec> {
        AtsfcW::new(self, 0)
    }
    #[doc = "Bit 4 - ATSEN0"]
    #[inline(always)]
    pub fn atsen0(&mut self) -> Atsen0W<MacacrSpec> {
        Atsen0W::new(self, 4)
    }
    #[doc = "Bit 5 - ATSEN1"]
    #[inline(always)]
    pub fn atsen1(&mut self) -> Atsen1W<MacacrSpec> {
        Atsen1W::new(self, 5)
    }
    #[doc = "Bit 6 - ATSEN2"]
    #[inline(always)]
    pub fn atsen2(&mut self) -> Atsen2W<MacacrSpec> {
        Atsen2W::new(self, 6)
    }
    #[doc = "Bit 7 - ATSEN3"]
    #[inline(always)]
    pub fn atsen3(&mut self) -> Atsen3W<MacacrSpec> {
        Atsen3W::new(self, 7)
    }
}
#[doc = "Auxiliary control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacacrSpec;
impl crate::RegisterSpec for MacacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macacr::R`](R) reader structure"]
impl crate::Readable for MacacrSpec {}
#[doc = "`write(|w| ..)` method takes [`macacr::W`](W) writer structure"]
impl crate::Writable for MacacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACACR to value 0"]
impl crate::Resettable for MacacrSpec {}
