#[doc = "Register `TIM16_AF1` reader"]
pub type R = crate::R<Tim16Af1Spec>;
#[doc = "Register `TIM16_AF1` writer"]
pub type W = crate::W<Tim16Af1Spec>;
#[doc = "Field `BKINE` reader - BRK BKIN input enable"]
pub type BkineR = crate::BitReader;
#[doc = "Field `BKINE` writer - BRK BKIN input enable"]
pub type BkineW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1E` reader - BRK COMP1 enable"]
pub type Bkcmp1eR = crate::BitReader;
#[doc = "Field `BKCMP1E` writer - BRK COMP1 enable"]
pub type Bkcmp1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2E` reader - BRK COMP2 enable"]
pub type Bkcmp2eR = crate::BitReader;
#[doc = "Field `BKCMP2E` writer - BRK COMP2 enable"]
pub type Bkcmp2eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKDFBK1E` reader - BRK dfsdm1_break\\[1\\] enable"]
pub type Bkdfbk1eR = crate::BitReader;
#[doc = "Field `BKDFBK1E` writer - BRK dfsdm1_break\\[1\\] enable"]
pub type Bkdfbk1eW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKINP` reader - BRK BKIN input polarity"]
pub type BkinpR = crate::BitReader;
#[doc = "Field `BKINP` writer - BRK BKIN input polarity"]
pub type BkinpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP1P` reader - BRK COMP1 input polarity"]
pub type Bkcmp1pR = crate::BitReader;
#[doc = "Field `BKCMP1P` writer - BRK COMP1 input polarity"]
pub type Bkcmp1pW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKCMP2P` reader - BRK COMP2 input polarity"]
pub type Bkcmp2pR = crate::BitReader;
#[doc = "Field `BKCMP2P` writer - BRK COMP2 input polarity"]
pub type Bkcmp2pW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&self) -> BkineR {
        BkineR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&self) -> Bkcmp1eR {
        Bkcmp1eR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&self) -> Bkcmp2eR {
        Bkcmp2eR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 8 - BRK dfsdm1_break\\[1\\] enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&self) -> Bkdfbk1eR {
        Bkdfbk1eR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&self) -> BkinpR {
        BkinpR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&self) -> Bkcmp1pR {
        Bkcmp1pR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&self) -> Bkcmp2pR {
        Bkcmp2pR::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - BRK BKIN input enable"]
    #[inline(always)]
    pub fn bkine(&mut self) -> BkineW<Tim16Af1Spec> {
        BkineW::new(self, 0)
    }
    #[doc = "Bit 1 - BRK COMP1 enable"]
    #[inline(always)]
    pub fn bkcmp1e(&mut self) -> Bkcmp1eW<Tim16Af1Spec> {
        Bkcmp1eW::new(self, 1)
    }
    #[doc = "Bit 2 - BRK COMP2 enable"]
    #[inline(always)]
    pub fn bkcmp2e(&mut self) -> Bkcmp2eW<Tim16Af1Spec> {
        Bkcmp2eW::new(self, 2)
    }
    #[doc = "Bit 8 - BRK dfsdm1_break\\[1\\] enable"]
    #[inline(always)]
    pub fn bkdfbk1e(&mut self) -> Bkdfbk1eW<Tim16Af1Spec> {
        Bkdfbk1eW::new(self, 8)
    }
    #[doc = "Bit 9 - BRK BKIN input polarity"]
    #[inline(always)]
    pub fn bkinp(&mut self) -> BkinpW<Tim16Af1Spec> {
        BkinpW::new(self, 9)
    }
    #[doc = "Bit 10 - BRK COMP1 input polarity"]
    #[inline(always)]
    pub fn bkcmp1p(&mut self) -> Bkcmp1pW<Tim16Af1Spec> {
        Bkcmp1pW::new(self, 10)
    }
    #[doc = "Bit 11 - BRK COMP2 input polarity"]
    #[inline(always)]
    pub fn bkcmp2p(&mut self) -> Bkcmp2pW<Tim16Af1Spec> {
        Bkcmp2pW::new(self, 11)
    }
}
#[doc = "TIM16 alternate function register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`tim16_af1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tim16_af1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Tim16Af1Spec;
impl crate::RegisterSpec for Tim16Af1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tim16_af1::R`](R) reader structure"]
impl crate::Readable for Tim16Af1Spec {}
#[doc = "`write(|w| ..)` method takes [`tim16_af1::W`](W) writer structure"]
impl crate::Writable for Tim16Af1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIM16_AF1 to value 0"]
impl crate::Resettable for Tim16Af1Spec {}
