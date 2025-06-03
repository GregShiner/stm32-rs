#[doc = "Register `AF2` reader"]
pub type R = crate::R<Af2Spec>;
#[doc = "Register `AF2` writer"]
pub type W = crate::W<Af2Spec>;
#[doc = "Field `BK2INE` reader - BRK2 BKIN input enable"]
pub type Bk2ineR = crate::BitReader;
#[doc = "Field `BK2INE` writer - BRK2 BKIN input enable"]
pub type Bk2ineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1E` reader - BRK2 COMP1 enable"]
pub type Bk2cmp1eR = crate::BitReader;
#[doc = "Field `BK2CMP1E` writer - BRK2 COMP1 enable"]
pub type Bk2cmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2E` reader - BRK2 COMP2 enable"]
pub type Bk2cmp2eR = crate::BitReader;
#[doc = "Field `BK2CMP2E` writer - BRK2 COMP2 enable"]
pub type Bk2cmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2DF1BK1E` reader - BRK2 dfsdm1_break\\[1\\] enable"]
pub type Bk2df1bk1eR = crate::BitReader;
#[doc = "Field `BK2DF1BK1E` writer - BRK2 dfsdm1_break\\[1\\] enable"]
pub type Bk2df1bk1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2INP` reader - BRK2 BKIN2 input polarity"]
pub type Bk2inpR = crate::BitReader;
#[doc = "Field `BK2INP` writer - BRK2 BKIN2 input polarity"]
pub type Bk2inpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP1P` reader - BRK2 COMP1 input polarit"]
pub type Bk2cmp1pR = crate::BitReader;
#[doc = "Field `BK2CMP1P` writer - BRK2 COMP1 input polarit"]
pub type Bk2cmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BK2CMP2P` reader - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pR = crate::BitReader;
#[doc = "Field `BK2CMP2P` writer - BRK2 COMP2 input polarity"]
pub type Bk2cmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&self) -> Bk2ineR {
        Bk2ineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&self) -> Bk2cmp1eR {
        Bk2cmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&self) -> Bk2cmp2eR {
        Bk2cmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK2 dfsdm1_break\\[1\\] enable"]
    #[inline(always)]
    pub fn bk2df1bk1e(&self) -> Bk2df1bk1eR {
        Bk2df1bk1eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&self) -> Bk2inpR {
        Bk2inpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarit"]
    #[inline(always)]
    pub fn bk2cmp1p(&self) -> Bk2cmp1pR {
        Bk2cmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&self) -> Bk2cmp2pR {
        Bk2cmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK2 BKIN input enable"]
    #[inline(always)]
    pub fn bk2ine(&mut self) -> Bk2ineW<Af2Spec> {
        Bk2ineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK2 COMP1 enable"]
    #[inline(always)]
    pub fn bk2cmp1e(&mut self) -> Bk2cmp1eW<Af2Spec> {
        Bk2cmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK2 COMP2 enable"]
    #[inline(always)]
    pub fn bk2cmp2e(&mut self) -> Bk2cmp2eW<Af2Spec> {
        Bk2cmp2eW::new(self, 2)
    }
    #[doc = "Bit 8 - BRK2 dfsdm1_break\\[1\\] enable"]
    #[inline(always)]
    pub fn bk2df1bk1e(&mut self) -> Bk2df1bk1eW<Af2Spec> {
        Bk2df1bk1eW::new(self, 8)
    }
    #[doc = "Bit 9 - BRK2 BKIN2 input polarity"]
    #[inline(always)]
    pub fn bk2inp(&mut self) -> Bk2inpW<Af2Spec> {
        Bk2inpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK2 COMP1 input polarit"]
    #[inline(always)]
    pub fn bk2cmp1p(&mut self) -> Bk2cmp1pW<Af2Spec> {
        Bk2cmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK2 COMP2 input polarity"]
    #[inline(always)]
    pub fn bk2cmp2p(&mut self) -> Bk2cmp2pW<Af2Spec> {
        Bk2cmp2pW::new(self, 11)
    }
}
#[doc = "TIM1 Alternate function odfsdm1_breakster 2\n\nYou can [`read`](crate::Reg::read) this register and get [`af2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`af2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Af2Spec;
impl crate::RegisterSpec for Af2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`af2::R`](R) reader structure"]
impl crate::Readable for Af2Spec {}
#[doc = "`write(|w| ..)` method takes [`af2::W`](W) writer structure"]
impl crate::Writable for Af2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AF2 to value 0"]
impl crate::Resettable for Af2Spec {}
