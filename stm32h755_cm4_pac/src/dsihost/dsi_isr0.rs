#[doc = "Register `DSI_ISR0` reader"]
pub type R = crate::R<DsiIsr0Spec>;
#[doc = "Field `AE0` reader - AE0"]
pub type Ae0R = crate::BitReader;
#[doc = "Field `AE1` reader - AE1"]
pub type Ae1R = crate::BitReader;
#[doc = "Field `AE2` reader - AE2"]
pub type Ae2R = crate::BitReader;
#[doc = "Field `AE3` reader - AE3"]
pub type Ae3R = crate::BitReader;
#[doc = "Field `AE4` reader - AE4"]
pub type Ae4R = crate::BitReader;
#[doc = "Field `AE5` reader - AE5"]
pub type Ae5R = crate::BitReader;
#[doc = "Field `AE6` reader - AE6"]
pub type Ae6R = crate::BitReader;
#[doc = "Field `AE7` reader - AE7"]
pub type Ae7R = crate::BitReader;
#[doc = "Field `AE8` reader - AE8"]
pub type Ae8R = crate::BitReader;
#[doc = "Field `AE9` reader - AE9"]
pub type Ae9R = crate::BitReader;
#[doc = "Field `AE10` reader - AE10"]
pub type Ae10R = crate::BitReader;
#[doc = "Field `AE11` reader - AE11"]
pub type Ae11R = crate::BitReader;
#[doc = "Field `AE12` reader - AE12"]
pub type Ae12R = crate::BitReader;
#[doc = "Field `AE13` reader - AE13"]
pub type Ae13R = crate::BitReader;
#[doc = "Field `AE14` reader - AE14"]
pub type Ae14R = crate::BitReader;
#[doc = "Field `AE15` reader - AE15"]
pub type Ae15R = crate::BitReader;
#[doc = "Field `PE0` reader - PE0"]
pub type Pe0R = crate::BitReader;
#[doc = "Field `PE1` reader - PE1"]
pub type Pe1R = crate::BitReader;
#[doc = "Field `PE2` reader - PE2"]
pub type Pe2R = crate::BitReader;
#[doc = "Field `PE3` reader - PE3"]
pub type Pe3R = crate::BitReader;
#[doc = "Field `PE4` reader - PE4"]
pub type Pe4R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - AE0"]
    #[inline(always)]
    pub fn ae0(&self) -> Ae0R {
        Ae0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - AE1"]
    #[inline(always)]
    pub fn ae1(&self) -> Ae1R {
        Ae1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - AE2"]
    #[inline(always)]
    pub fn ae2(&self) -> Ae2R {
        Ae2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - AE3"]
    #[inline(always)]
    pub fn ae3(&self) -> Ae3R {
        Ae3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - AE4"]
    #[inline(always)]
    pub fn ae4(&self) -> Ae4R {
        Ae4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - AE5"]
    #[inline(always)]
    pub fn ae5(&self) -> Ae5R {
        Ae5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - AE6"]
    #[inline(always)]
    pub fn ae6(&self) -> Ae6R {
        Ae6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - AE7"]
    #[inline(always)]
    pub fn ae7(&self) -> Ae7R {
        Ae7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - AE8"]
    #[inline(always)]
    pub fn ae8(&self) -> Ae8R {
        Ae8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - AE9"]
    #[inline(always)]
    pub fn ae9(&self) -> Ae9R {
        Ae9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - AE10"]
    #[inline(always)]
    pub fn ae10(&self) -> Ae10R {
        Ae10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - AE11"]
    #[inline(always)]
    pub fn ae11(&self) -> Ae11R {
        Ae11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - AE12"]
    #[inline(always)]
    pub fn ae12(&self) -> Ae12R {
        Ae12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - AE13"]
    #[inline(always)]
    pub fn ae13(&self) -> Ae13R {
        Ae13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - AE14"]
    #[inline(always)]
    pub fn ae14(&self) -> Ae14R {
        Ae14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AE15"]
    #[inline(always)]
    pub fn ae15(&self) -> Ae15R {
        Ae15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - PE0"]
    #[inline(always)]
    pub fn pe0(&self) -> Pe0R {
        Pe0R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - PE1"]
    #[inline(always)]
    pub fn pe1(&self) -> Pe1R {
        Pe1R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - PE2"]
    #[inline(always)]
    pub fn pe2(&self) -> Pe2R {
        Pe2R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - PE3"]
    #[inline(always)]
    pub fn pe3(&self) -> Pe3R {
        Pe3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - PE4"]
    #[inline(always)]
    pub fn pe4(&self) -> Pe4R {
        Pe4R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[doc = "DSI Host interrupt and status register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_isr0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiIsr0Spec;
impl crate::RegisterSpec for DsiIsr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_isr0::R`](R) reader structure"]
impl crate::Readable for DsiIsr0Spec {}
#[doc = "`reset()` method sets DSI_ISR0 to value 0"]
impl crate::Resettable for DsiIsr0Spec {}
