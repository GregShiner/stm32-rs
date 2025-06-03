#[doc = "Register `DSI_FIR0` writer"]
pub type W = crate::W<DsiFir0Spec>;
#[doc = "Field `FAE0` writer - FAE0"]
pub type Fae0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE1` writer - FAE1"]
pub type Fae1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE2` writer - FAE2"]
pub type Fae2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE3` writer - FAE3"]
pub type Fae3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE4` writer - FAE4"]
pub type Fae4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE5` writer - FAE5"]
pub type Fae5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE6` writer - FAE6"]
pub type Fae6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE7` writer - FAE7"]
pub type Fae7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE8` writer - FAE8"]
pub type Fae8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE9` writer - FAE9"]
pub type Fae9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE10` writer - FAE10"]
pub type Fae10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE11` writer - FAE11"]
pub type Fae11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE12` writer - FAE12"]
pub type Fae12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE13` writer - FAE13"]
pub type Fae13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE14` writer - FAE14"]
pub type Fae14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FAE15` writer - FAE15"]
pub type Fae15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE0` writer - FPE0"]
pub type Fpe0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE1` writer - FPE1"]
pub type Fpe1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE2` writer - FPE2"]
pub type Fpe2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE3` writer - FPE3"]
pub type Fpe3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPE4` writer - FPE4"]
pub type Fpe4W<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - FAE0"]
    #[inline(always)]
    pub fn fae0(&mut self) -> Fae0W<DsiFir0Spec> {
        Fae0W::new(self, 0)
    }
    #[doc = "Bit 1 - FAE1"]
    #[inline(always)]
    pub fn fae1(&mut self) -> Fae1W<DsiFir0Spec> {
        Fae1W::new(self, 1)
    }
    #[doc = "Bit 2 - FAE2"]
    #[inline(always)]
    pub fn fae2(&mut self) -> Fae2W<DsiFir0Spec> {
        Fae2W::new(self, 2)
    }
    #[doc = "Bit 3 - FAE3"]
    #[inline(always)]
    pub fn fae3(&mut self) -> Fae3W<DsiFir0Spec> {
        Fae3W::new(self, 3)
    }
    #[doc = "Bit 4 - FAE4"]
    #[inline(always)]
    pub fn fae4(&mut self) -> Fae4W<DsiFir0Spec> {
        Fae4W::new(self, 4)
    }
    #[doc = "Bit 5 - FAE5"]
    #[inline(always)]
    pub fn fae5(&mut self) -> Fae5W<DsiFir0Spec> {
        Fae5W::new(self, 5)
    }
    #[doc = "Bit 6 - FAE6"]
    #[inline(always)]
    pub fn fae6(&mut self) -> Fae6W<DsiFir0Spec> {
        Fae6W::new(self, 6)
    }
    #[doc = "Bit 7 - FAE7"]
    #[inline(always)]
    pub fn fae7(&mut self) -> Fae7W<DsiFir0Spec> {
        Fae7W::new(self, 7)
    }
    #[doc = "Bit 8 - FAE8"]
    #[inline(always)]
    pub fn fae8(&mut self) -> Fae8W<DsiFir0Spec> {
        Fae8W::new(self, 8)
    }
    #[doc = "Bit 9 - FAE9"]
    #[inline(always)]
    pub fn fae9(&mut self) -> Fae9W<DsiFir0Spec> {
        Fae9W::new(self, 9)
    }
    #[doc = "Bit 10 - FAE10"]
    #[inline(always)]
    pub fn fae10(&mut self) -> Fae10W<DsiFir0Spec> {
        Fae10W::new(self, 10)
    }
    #[doc = "Bit 11 - FAE11"]
    #[inline(always)]
    pub fn fae11(&mut self) -> Fae11W<DsiFir0Spec> {
        Fae11W::new(self, 11)
    }
    #[doc = "Bit 12 - FAE12"]
    #[inline(always)]
    pub fn fae12(&mut self) -> Fae12W<DsiFir0Spec> {
        Fae12W::new(self, 12)
    }
    #[doc = "Bit 13 - FAE13"]
    #[inline(always)]
    pub fn fae13(&mut self) -> Fae13W<DsiFir0Spec> {
        Fae13W::new(self, 13)
    }
    #[doc = "Bit 14 - FAE14"]
    #[inline(always)]
    pub fn fae14(&mut self) -> Fae14W<DsiFir0Spec> {
        Fae14W::new(self, 14)
    }
    #[doc = "Bit 15 - FAE15"]
    #[inline(always)]
    pub fn fae15(&mut self) -> Fae15W<DsiFir0Spec> {
        Fae15W::new(self, 15)
    }
    #[doc = "Bit 16 - FPE0"]
    #[inline(always)]
    pub fn fpe0(&mut self) -> Fpe0W<DsiFir0Spec> {
        Fpe0W::new(self, 16)
    }
    #[doc = "Bit 17 - FPE1"]
    #[inline(always)]
    pub fn fpe1(&mut self) -> Fpe1W<DsiFir0Spec> {
        Fpe1W::new(self, 17)
    }
    #[doc = "Bit 18 - FPE2"]
    #[inline(always)]
    pub fn fpe2(&mut self) -> Fpe2W<DsiFir0Spec> {
        Fpe2W::new(self, 18)
    }
    #[doc = "Bit 19 - FPE3"]
    #[inline(always)]
    pub fn fpe3(&mut self) -> Fpe3W<DsiFir0Spec> {
        Fpe3W::new(self, 19)
    }
    #[doc = "Bit 20 - FPE4"]
    #[inline(always)]
    pub fn fpe4(&mut self) -> Fpe4W<DsiFir0Spec> {
        Fpe4W::new(self, 20)
    }
}
#[doc = "DSI Host force interrupt register 0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fir0::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFir0Spec;
impl crate::RegisterSpec for DsiFir0Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_fir0::W`](W) writer structure"]
impl crate::Writable for DsiFir0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_FIR0 to value 0"]
impl crate::Resettable for DsiFir0Spec {}
