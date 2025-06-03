#[doc = "Register `CPUPR1` reader"]
pub type R = crate::R<Cpupr1Spec>;
#[doc = "Register `CPUPR1` writer"]
pub type W = crate::W<Cpupr1Spec>;
#[doc = "Field `PR0` reader - CPU Event mask on Event input x"]
pub type Pr0R = crate::BitReader;
#[doc = "Field `PR0` writer - CPU Event mask on Event input x"]
pub type Pr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR1` reader - CPU Event mask on Event input x"]
pub type Pr1R = crate::BitReader;
#[doc = "Field `PR1` writer - CPU Event mask on Event input x"]
pub type Pr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR2` reader - CPU Event mask on Event input x"]
pub type Pr2R = crate::BitReader;
#[doc = "Field `PR2` writer - CPU Event mask on Event input x"]
pub type Pr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR3` reader - CPU Event mask on Event input x"]
pub type Pr3R = crate::BitReader;
#[doc = "Field `PR3` writer - CPU Event mask on Event input x"]
pub type Pr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR4` reader - CPU Event mask on Event input x"]
pub type Pr4R = crate::BitReader;
#[doc = "Field `PR4` writer - CPU Event mask on Event input x"]
pub type Pr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR5` reader - CPU Event mask on Event input x"]
pub type Pr5R = crate::BitReader;
#[doc = "Field `PR5` writer - CPU Event mask on Event input x"]
pub type Pr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR6` reader - CPU Event mask on Event input x"]
pub type Pr6R = crate::BitReader;
#[doc = "Field `PR6` writer - CPU Event mask on Event input x"]
pub type Pr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR7` reader - CPU Event mask on Event input x"]
pub type Pr7R = crate::BitReader;
#[doc = "Field `PR7` writer - CPU Event mask on Event input x"]
pub type Pr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR8` reader - CPU Event mask on Event input x"]
pub type Pr8R = crate::BitReader;
#[doc = "Field `PR8` writer - CPU Event mask on Event input x"]
pub type Pr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR9` reader - CPU Event mask on Event input x"]
pub type Pr9R = crate::BitReader;
#[doc = "Field `PR9` writer - CPU Event mask on Event input x"]
pub type Pr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR10` reader - CPU Event mask on Event input x"]
pub type Pr10R = crate::BitReader;
#[doc = "Field `PR10` writer - CPU Event mask on Event input x"]
pub type Pr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR11` reader - CPU Event mask on Event input x"]
pub type Pr11R = crate::BitReader;
#[doc = "Field `PR11` writer - CPU Event mask on Event input x"]
pub type Pr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR12` reader - CPU Event mask on Event input x"]
pub type Pr12R = crate::BitReader;
#[doc = "Field `PR12` writer - CPU Event mask on Event input x"]
pub type Pr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR13` reader - CPU Event mask on Event input x"]
pub type Pr13R = crate::BitReader;
#[doc = "Field `PR13` writer - CPU Event mask on Event input x"]
pub type Pr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR14` reader - CPU Event mask on Event input x"]
pub type Pr14R = crate::BitReader;
#[doc = "Field `PR14` writer - CPU Event mask on Event input x"]
pub type Pr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR15` reader - CPU Event mask on Event input x"]
pub type Pr15R = crate::BitReader;
#[doc = "Field `PR15` writer - CPU Event mask on Event input x"]
pub type Pr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR16` reader - CPU Event mask on Event input x"]
pub type Pr16R = crate::BitReader;
#[doc = "Field `PR16` writer - CPU Event mask on Event input x"]
pub type Pr16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR17` reader - CPU Event mask on Event input x"]
pub type Pr17R = crate::BitReader;
#[doc = "Field `PR17` writer - CPU Event mask on Event input x"]
pub type Pr17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR18` reader - CPU Event mask on Event input x"]
pub type Pr18R = crate::BitReader;
#[doc = "Field `PR18` writer - CPU Event mask on Event input x"]
pub type Pr18W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR19` reader - CPU Event mask on Event input x"]
pub type Pr19R = crate::BitReader;
#[doc = "Field `PR19` writer - CPU Event mask on Event input x"]
pub type Pr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR20` reader - CPU Event mask on Event input x"]
pub type Pr20R = crate::BitReader;
#[doc = "Field `PR20` writer - CPU Event mask on Event input x"]
pub type Pr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PR21` reader - CPU Event mask on Event input x"]
pub type Pr21R = crate::BitReader;
#[doc = "Field `PR21` writer - CPU Event mask on Event input x"]
pub type Pr21W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr0(&self) -> Pr0R {
        Pr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr1(&self) -> Pr1R {
        Pr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr2(&self) -> Pr2R {
        Pr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr3(&self) -> Pr3R {
        Pr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr4(&self) -> Pr4R {
        Pr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr5(&self) -> Pr5R {
        Pr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr6(&self) -> Pr6R {
        Pr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr7(&self) -> Pr7R {
        Pr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr8(&self) -> Pr8R {
        Pr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr9(&self) -> Pr9R {
        Pr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr10(&self) -> Pr10R {
        Pr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr11(&self) -> Pr11R {
        Pr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr12(&self) -> Pr12R {
        Pr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr13(&self) -> Pr13R {
        Pr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr14(&self) -> Pr14R {
        Pr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr15(&self) -> Pr15R {
        Pr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr16(&self) -> Pr16R {
        Pr16R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr17(&self) -> Pr17R {
        Pr17R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr18(&self) -> Pr18R {
        Pr18R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr19(&self) -> Pr19R {
        Pr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr20(&self) -> Pr20R {
        Pr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr21(&self) -> Pr21R {
        Pr21R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr0(&mut self) -> Pr0W<Cpupr1Spec> {
        Pr0W::new(self, 0)
    }
    #[doc = "Bit 1 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr1(&mut self) -> Pr1W<Cpupr1Spec> {
        Pr1W::new(self, 1)
    }
    #[doc = "Bit 2 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr2(&mut self) -> Pr2W<Cpupr1Spec> {
        Pr2W::new(self, 2)
    }
    #[doc = "Bit 3 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr3(&mut self) -> Pr3W<Cpupr1Spec> {
        Pr3W::new(self, 3)
    }
    #[doc = "Bit 4 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr4(&mut self) -> Pr4W<Cpupr1Spec> {
        Pr4W::new(self, 4)
    }
    #[doc = "Bit 5 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr5(&mut self) -> Pr5W<Cpupr1Spec> {
        Pr5W::new(self, 5)
    }
    #[doc = "Bit 6 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr6(&mut self) -> Pr6W<Cpupr1Spec> {
        Pr6W::new(self, 6)
    }
    #[doc = "Bit 7 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr7(&mut self) -> Pr7W<Cpupr1Spec> {
        Pr7W::new(self, 7)
    }
    #[doc = "Bit 8 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr8(&mut self) -> Pr8W<Cpupr1Spec> {
        Pr8W::new(self, 8)
    }
    #[doc = "Bit 9 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr9(&mut self) -> Pr9W<Cpupr1Spec> {
        Pr9W::new(self, 9)
    }
    #[doc = "Bit 10 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr10(&mut self) -> Pr10W<Cpupr1Spec> {
        Pr10W::new(self, 10)
    }
    #[doc = "Bit 11 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr11(&mut self) -> Pr11W<Cpupr1Spec> {
        Pr11W::new(self, 11)
    }
    #[doc = "Bit 12 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr12(&mut self) -> Pr12W<Cpupr1Spec> {
        Pr12W::new(self, 12)
    }
    #[doc = "Bit 13 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr13(&mut self) -> Pr13W<Cpupr1Spec> {
        Pr13W::new(self, 13)
    }
    #[doc = "Bit 14 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr14(&mut self) -> Pr14W<Cpupr1Spec> {
        Pr14W::new(self, 14)
    }
    #[doc = "Bit 15 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr15(&mut self) -> Pr15W<Cpupr1Spec> {
        Pr15W::new(self, 15)
    }
    #[doc = "Bit 16 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr16(&mut self) -> Pr16W<Cpupr1Spec> {
        Pr16W::new(self, 16)
    }
    #[doc = "Bit 17 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr17(&mut self) -> Pr17W<Cpupr1Spec> {
        Pr17W::new(self, 17)
    }
    #[doc = "Bit 18 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr18(&mut self) -> Pr18W<Cpupr1Spec> {
        Pr18W::new(self, 18)
    }
    #[doc = "Bit 19 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr19(&mut self) -> Pr19W<Cpupr1Spec> {
        Pr19W::new(self, 19)
    }
    #[doc = "Bit 20 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr20(&mut self) -> Pr20W<Cpupr1Spec> {
        Pr20W::new(self, 20)
    }
    #[doc = "Bit 21 - CPU Event mask on Event input x"]
    #[inline(always)]
    pub fn pr21(&mut self) -> Pr21W<Cpupr1Spec> {
        Pr21W::new(self, 21)
    }
}
#[doc = "EXTI pending register\n\nYou can [`read`](crate::Reg::read) this register and get [`cpupr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpupr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cpupr1Spec;
impl crate::RegisterSpec for Cpupr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cpupr1::R`](R) reader structure"]
impl crate::Readable for Cpupr1Spec {}
#[doc = "`write(|w| ..)` method takes [`cpupr1::W`](W) writer structure"]
impl crate::Writable for Cpupr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CPUPR1 to value 0"]
impl crate::Resettable for Cpupr1Spec {}
