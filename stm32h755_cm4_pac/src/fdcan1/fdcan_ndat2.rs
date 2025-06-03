#[doc = "Register `FDCAN_NDAT2` reader"]
pub type R = crate::R<FdcanNdat2Spec>;
#[doc = "Field `ND32` reader - New data"]
pub type Nd32R = crate::BitReader;
#[doc = "Field `ND33` reader - New data"]
pub type Nd33R = crate::BitReader;
#[doc = "Field `ND34` reader - New data"]
pub type Nd34R = crate::BitReader;
#[doc = "Field `ND35` reader - New data"]
pub type Nd35R = crate::BitReader;
#[doc = "Field `ND36` reader - New data"]
pub type Nd36R = crate::BitReader;
#[doc = "Field `ND37` reader - New data"]
pub type Nd37R = crate::BitReader;
#[doc = "Field `ND38` reader - New data"]
pub type Nd38R = crate::BitReader;
#[doc = "Field `ND39` reader - New data"]
pub type Nd39R = crate::BitReader;
#[doc = "Field `ND40` reader - New data"]
pub type Nd40R = crate::BitReader;
#[doc = "Field `ND41` reader - New data"]
pub type Nd41R = crate::BitReader;
#[doc = "Field `ND42` reader - New data"]
pub type Nd42R = crate::BitReader;
#[doc = "Field `ND43` reader - New data"]
pub type Nd43R = crate::BitReader;
#[doc = "Field `ND44` reader - New data"]
pub type Nd44R = crate::BitReader;
#[doc = "Field `ND45` reader - New data"]
pub type Nd45R = crate::BitReader;
#[doc = "Field `ND46` reader - New data"]
pub type Nd46R = crate::BitReader;
#[doc = "Field `ND47` reader - New data"]
pub type Nd47R = crate::BitReader;
#[doc = "Field `ND48` reader - New data"]
pub type Nd48R = crate::BitReader;
#[doc = "Field `ND49` reader - New data"]
pub type Nd49R = crate::BitReader;
#[doc = "Field `ND50` reader - New data"]
pub type Nd50R = crate::BitReader;
#[doc = "Field `ND51` reader - New data"]
pub type Nd51R = crate::BitReader;
#[doc = "Field `ND52` reader - New data"]
pub type Nd52R = crate::BitReader;
#[doc = "Field `ND53` reader - New data"]
pub type Nd53R = crate::BitReader;
#[doc = "Field `ND54` reader - New data"]
pub type Nd54R = crate::BitReader;
#[doc = "Field `ND55` reader - New data"]
pub type Nd55R = crate::BitReader;
#[doc = "Field `ND56` reader - New data"]
pub type Nd56R = crate::BitReader;
#[doc = "Field `ND57` reader - New data"]
pub type Nd57R = crate::BitReader;
#[doc = "Field `ND58` reader - New data"]
pub type Nd58R = crate::BitReader;
#[doc = "Field `ND59` reader - New data"]
pub type Nd59R = crate::BitReader;
#[doc = "Field `ND60` reader - New data"]
pub type Nd60R = crate::BitReader;
#[doc = "Field `ND61` reader - New data"]
pub type Nd61R = crate::BitReader;
#[doc = "Field `ND62` reader - New data"]
pub type Nd62R = crate::BitReader;
#[doc = "Field `ND63` reader - New data"]
pub type Nd63R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - New data"]
    #[inline(always)]
    pub fn nd32(&self) -> Nd32R {
        Nd32R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New data"]
    #[inline(always)]
    pub fn nd33(&self) -> Nd33R {
        Nd33R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New data"]
    #[inline(always)]
    pub fn nd34(&self) -> Nd34R {
        Nd34R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New data"]
    #[inline(always)]
    pub fn nd35(&self) -> Nd35R {
        Nd35R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New data"]
    #[inline(always)]
    pub fn nd36(&self) -> Nd36R {
        Nd36R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - New data"]
    #[inline(always)]
    pub fn nd37(&self) -> Nd37R {
        Nd37R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New data"]
    #[inline(always)]
    pub fn nd38(&self) -> Nd38R {
        Nd38R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - New data"]
    #[inline(always)]
    pub fn nd39(&self) -> Nd39R {
        Nd39R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - New data"]
    #[inline(always)]
    pub fn nd40(&self) -> Nd40R {
        Nd40R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - New data"]
    #[inline(always)]
    pub fn nd41(&self) -> Nd41R {
        Nd41R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - New data"]
    #[inline(always)]
    pub fn nd42(&self) -> Nd42R {
        Nd42R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New data"]
    #[inline(always)]
    pub fn nd43(&self) -> Nd43R {
        Nd43R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - New data"]
    #[inline(always)]
    pub fn nd44(&self) -> Nd44R {
        Nd44R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - New data"]
    #[inline(always)]
    pub fn nd45(&self) -> Nd45R {
        Nd45R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - New data"]
    #[inline(always)]
    pub fn nd46(&self) -> Nd46R {
        Nd46R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New data"]
    #[inline(always)]
    pub fn nd47(&self) -> Nd47R {
        Nd47R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - New data"]
    #[inline(always)]
    pub fn nd48(&self) -> Nd48R {
        Nd48R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - New data"]
    #[inline(always)]
    pub fn nd49(&self) -> Nd49R {
        Nd49R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - New data"]
    #[inline(always)]
    pub fn nd50(&self) -> Nd50R {
        Nd50R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - New data"]
    #[inline(always)]
    pub fn nd51(&self) -> Nd51R {
        Nd51R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - New data"]
    #[inline(always)]
    pub fn nd52(&self) -> Nd52R {
        Nd52R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - New data"]
    #[inline(always)]
    pub fn nd53(&self) -> Nd53R {
        Nd53R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - New data"]
    #[inline(always)]
    pub fn nd54(&self) -> Nd54R {
        Nd54R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - New data"]
    #[inline(always)]
    pub fn nd55(&self) -> Nd55R {
        Nd55R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - New data"]
    #[inline(always)]
    pub fn nd56(&self) -> Nd56R {
        Nd56R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - New data"]
    #[inline(always)]
    pub fn nd57(&self) -> Nd57R {
        Nd57R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - New data"]
    #[inline(always)]
    pub fn nd58(&self) -> Nd58R {
        Nd58R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - New data"]
    #[inline(always)]
    pub fn nd59(&self) -> Nd59R {
        Nd59R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - New data"]
    #[inline(always)]
    pub fn nd60(&self) -> Nd60R {
        Nd60R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - New data"]
    #[inline(always)]
    pub fn nd61(&self) -> Nd61R {
        Nd61R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - New data"]
    #[inline(always)]
    pub fn nd62(&self) -> Nd62R {
        Nd62R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - New data"]
    #[inline(always)]
    pub fn nd63(&self) -> Nd63R {
        Nd63R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FDCAN New Data 2 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ndat2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanNdat2Spec;
impl crate::RegisterSpec for FdcanNdat2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ndat2::R`](R) reader structure"]
impl crate::Readable for FdcanNdat2Spec {}
#[doc = "`reset()` method sets FDCAN_NDAT2 to value 0"]
impl crate::Resettable for FdcanNdat2Spec {}
