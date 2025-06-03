#[doc = "Register `FDCAN_NDAT1` reader"]
pub type R = crate::R<FdcanNdat1Spec>;
#[doc = "Field `ND0` reader - New data"]
pub type Nd0R = crate::BitReader;
#[doc = "Field `ND1` reader - New data"]
pub type Nd1R = crate::BitReader;
#[doc = "Field `ND2` reader - New data"]
pub type Nd2R = crate::BitReader;
#[doc = "Field `ND3` reader - New data"]
pub type Nd3R = crate::BitReader;
#[doc = "Field `ND4` reader - New data"]
pub type Nd4R = crate::BitReader;
#[doc = "Field `ND5` reader - New data"]
pub type Nd5R = crate::BitReader;
#[doc = "Field `ND6` reader - New data"]
pub type Nd6R = crate::BitReader;
#[doc = "Field `ND7` reader - New data"]
pub type Nd7R = crate::BitReader;
#[doc = "Field `ND8` reader - New data"]
pub type Nd8R = crate::BitReader;
#[doc = "Field `ND9` reader - New data"]
pub type Nd9R = crate::BitReader;
#[doc = "Field `ND10` reader - New data"]
pub type Nd10R = crate::BitReader;
#[doc = "Field `ND11` reader - New data"]
pub type Nd11R = crate::BitReader;
#[doc = "Field `ND12` reader - New data"]
pub type Nd12R = crate::BitReader;
#[doc = "Field `ND13` reader - New data"]
pub type Nd13R = crate::BitReader;
#[doc = "Field `ND14` reader - New data"]
pub type Nd14R = crate::BitReader;
#[doc = "Field `ND15` reader - New data"]
pub type Nd15R = crate::BitReader;
#[doc = "Field `ND16` reader - New data"]
pub type Nd16R = crate::BitReader;
#[doc = "Field `ND17` reader - New data"]
pub type Nd17R = crate::BitReader;
#[doc = "Field `ND18` reader - New data"]
pub type Nd18R = crate::BitReader;
#[doc = "Field `ND19` reader - New data"]
pub type Nd19R = crate::BitReader;
#[doc = "Field `ND20` reader - New data"]
pub type Nd20R = crate::BitReader;
#[doc = "Field `ND21` reader - New data"]
pub type Nd21R = crate::BitReader;
#[doc = "Field `ND22` reader - New data"]
pub type Nd22R = crate::BitReader;
#[doc = "Field `ND23` reader - New data"]
pub type Nd23R = crate::BitReader;
#[doc = "Field `ND24` reader - New data"]
pub type Nd24R = crate::BitReader;
#[doc = "Field `ND25` reader - New data"]
pub type Nd25R = crate::BitReader;
#[doc = "Field `ND26` reader - New data"]
pub type Nd26R = crate::BitReader;
#[doc = "Field `ND27` reader - New data"]
pub type Nd27R = crate::BitReader;
#[doc = "Field `ND28` reader - New data"]
pub type Nd28R = crate::BitReader;
#[doc = "Field `ND29` reader - New data"]
pub type Nd29R = crate::BitReader;
#[doc = "Field `ND30` reader - New data"]
pub type Nd30R = crate::BitReader;
#[doc = "Field `ND31` reader - New data"]
pub type Nd31R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - New data"]
    #[inline(always)]
    pub fn nd0(&self) -> Nd0R {
        Nd0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - New data"]
    #[inline(always)]
    pub fn nd1(&self) -> Nd1R {
        Nd1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - New data"]
    #[inline(always)]
    pub fn nd2(&self) -> Nd2R {
        Nd2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - New data"]
    #[inline(always)]
    pub fn nd3(&self) -> Nd3R {
        Nd3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - New data"]
    #[inline(always)]
    pub fn nd4(&self) -> Nd4R {
        Nd4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - New data"]
    #[inline(always)]
    pub fn nd5(&self) -> Nd5R {
        Nd5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - New data"]
    #[inline(always)]
    pub fn nd6(&self) -> Nd6R {
        Nd6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - New data"]
    #[inline(always)]
    pub fn nd7(&self) -> Nd7R {
        Nd7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - New data"]
    #[inline(always)]
    pub fn nd8(&self) -> Nd8R {
        Nd8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - New data"]
    #[inline(always)]
    pub fn nd9(&self) -> Nd9R {
        Nd9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - New data"]
    #[inline(always)]
    pub fn nd10(&self) -> Nd10R {
        Nd10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - New data"]
    #[inline(always)]
    pub fn nd11(&self) -> Nd11R {
        Nd11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - New data"]
    #[inline(always)]
    pub fn nd12(&self) -> Nd12R {
        Nd12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - New data"]
    #[inline(always)]
    pub fn nd13(&self) -> Nd13R {
        Nd13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - New data"]
    #[inline(always)]
    pub fn nd14(&self) -> Nd14R {
        Nd14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - New data"]
    #[inline(always)]
    pub fn nd15(&self) -> Nd15R {
        Nd15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - New data"]
    #[inline(always)]
    pub fn nd16(&self) -> Nd16R {
        Nd16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - New data"]
    #[inline(always)]
    pub fn nd17(&self) -> Nd17R {
        Nd17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - New data"]
    #[inline(always)]
    pub fn nd18(&self) -> Nd18R {
        Nd18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - New data"]
    #[inline(always)]
    pub fn nd19(&self) -> Nd19R {
        Nd19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - New data"]
    #[inline(always)]
    pub fn nd20(&self) -> Nd20R {
        Nd20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - New data"]
    #[inline(always)]
    pub fn nd21(&self) -> Nd21R {
        Nd21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - New data"]
    #[inline(always)]
    pub fn nd22(&self) -> Nd22R {
        Nd22R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - New data"]
    #[inline(always)]
    pub fn nd23(&self) -> Nd23R {
        Nd23R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - New data"]
    #[inline(always)]
    pub fn nd24(&self) -> Nd24R {
        Nd24R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - New data"]
    #[inline(always)]
    pub fn nd25(&self) -> Nd25R {
        Nd25R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - New data"]
    #[inline(always)]
    pub fn nd26(&self) -> Nd26R {
        Nd26R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - New data"]
    #[inline(always)]
    pub fn nd27(&self) -> Nd27R {
        Nd27R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - New data"]
    #[inline(always)]
    pub fn nd28(&self) -> Nd28R {
        Nd28R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - New data"]
    #[inline(always)]
    pub fn nd29(&self) -> Nd29R {
        Nd29R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - New data"]
    #[inline(always)]
    pub fn nd30(&self) -> Nd30R {
        Nd30R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - New data"]
    #[inline(always)]
    pub fn nd31(&self) -> Nd31R {
        Nd31R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[doc = "FDCAN New Data 1 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_ndat1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanNdat1Spec;
impl crate::RegisterSpec for FdcanNdat1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_ndat1::R`](R) reader structure"]
impl crate::Readable for FdcanNdat1Spec {}
#[doc = "`reset()` method sets FDCAN_NDAT1 to value 0"]
impl crate::Resettable for FdcanNdat1Spec {}
