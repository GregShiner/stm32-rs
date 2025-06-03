#[doc = "Register `D3PMR1` reader"]
pub type R = crate::R<D3pmr1Spec>;
#[doc = "Register `D3PMR1` writer"]
pub type W = crate::W<D3pmr1Spec>;
#[doc = "Field `MR0` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr0R = crate::BitReader;
#[doc = "Field `MR0` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR1` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr1R = crate::BitReader;
#[doc = "Field `MR1` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR2` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr2R = crate::BitReader;
#[doc = "Field `MR2` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR3` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr3R = crate::BitReader;
#[doc = "Field `MR3` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR4` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr4R = crate::BitReader;
#[doc = "Field `MR4` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR5` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr5R = crate::BitReader;
#[doc = "Field `MR5` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR6` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr6R = crate::BitReader;
#[doc = "Field `MR6` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR7` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr7R = crate::BitReader;
#[doc = "Field `MR7` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR8` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr8R = crate::BitReader;
#[doc = "Field `MR8` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR9` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr9R = crate::BitReader;
#[doc = "Field `MR9` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr9W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR10` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr10R = crate::BitReader;
#[doc = "Field `MR10` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR11` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr11R = crate::BitReader;
#[doc = "Field `MR11` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr11W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR12` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr12R = crate::BitReader;
#[doc = "Field `MR12` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR13` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr13R = crate::BitReader;
#[doc = "Field `MR13` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR14` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr14R = crate::BitReader;
#[doc = "Field `MR14` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR15` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr15R = crate::BitReader;
#[doc = "Field `MR15` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR19` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr19R = crate::BitReader;
#[doc = "Field `MR19` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr19W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR20` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr20R = crate::BitReader;
#[doc = "Field `MR20` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr20W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR21` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr21R = crate::BitReader;
#[doc = "Field `MR21` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr21W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MR25` reader - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr25R = crate::BitReader;
#[doc = "Field `MR25` writer - Rising trigger event configuration bit of Configurable Event input"]
pub type Mr25W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr0(&self) -> Mr0R {
        Mr0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr1(&self) -> Mr1R {
        Mr1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr2(&self) -> Mr2R {
        Mr2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr3(&self) -> Mr3R {
        Mr3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr4(&self) -> Mr4R {
        Mr4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr5(&self) -> Mr5R {
        Mr5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr6(&self) -> Mr6R {
        Mr6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr7(&self) -> Mr7R {
        Mr7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr8(&self) -> Mr8R {
        Mr8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr9(&self) -> Mr9R {
        Mr9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr10(&self) -> Mr10R {
        Mr10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr11(&self) -> Mr11R {
        Mr11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr12(&self) -> Mr12R {
        Mr12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr13(&self) -> Mr13R {
        Mr13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr14(&self) -> Mr14R {
        Mr14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr15(&self) -> Mr15R {
        Mr15R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr19(&self) -> Mr19R {
        Mr19R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr20(&self) -> Mr20R {
        Mr20R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr21(&self) -> Mr21R {
        Mr21R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 25 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr25(&self) -> Mr25R {
        Mr25R::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr0(&mut self) -> Mr0W<D3pmr1Spec> {
        Mr0W::new(self, 0)
    }
    #[doc = "Bit 1 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr1(&mut self) -> Mr1W<D3pmr1Spec> {
        Mr1W::new(self, 1)
    }
    #[doc = "Bit 2 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr2(&mut self) -> Mr2W<D3pmr1Spec> {
        Mr2W::new(self, 2)
    }
    #[doc = "Bit 3 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr3(&mut self) -> Mr3W<D3pmr1Spec> {
        Mr3W::new(self, 3)
    }
    #[doc = "Bit 4 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr4(&mut self) -> Mr4W<D3pmr1Spec> {
        Mr4W::new(self, 4)
    }
    #[doc = "Bit 5 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr5(&mut self) -> Mr5W<D3pmr1Spec> {
        Mr5W::new(self, 5)
    }
    #[doc = "Bit 6 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr6(&mut self) -> Mr6W<D3pmr1Spec> {
        Mr6W::new(self, 6)
    }
    #[doc = "Bit 7 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr7(&mut self) -> Mr7W<D3pmr1Spec> {
        Mr7W::new(self, 7)
    }
    #[doc = "Bit 8 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr8(&mut self) -> Mr8W<D3pmr1Spec> {
        Mr8W::new(self, 8)
    }
    #[doc = "Bit 9 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr9(&mut self) -> Mr9W<D3pmr1Spec> {
        Mr9W::new(self, 9)
    }
    #[doc = "Bit 10 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr10(&mut self) -> Mr10W<D3pmr1Spec> {
        Mr10W::new(self, 10)
    }
    #[doc = "Bit 11 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr11(&mut self) -> Mr11W<D3pmr1Spec> {
        Mr11W::new(self, 11)
    }
    #[doc = "Bit 12 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr12(&mut self) -> Mr12W<D3pmr1Spec> {
        Mr12W::new(self, 12)
    }
    #[doc = "Bit 13 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr13(&mut self) -> Mr13W<D3pmr1Spec> {
        Mr13W::new(self, 13)
    }
    #[doc = "Bit 14 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr14(&mut self) -> Mr14W<D3pmr1Spec> {
        Mr14W::new(self, 14)
    }
    #[doc = "Bit 15 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr15(&mut self) -> Mr15W<D3pmr1Spec> {
        Mr15W::new(self, 15)
    }
    #[doc = "Bit 19 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr19(&mut self) -> Mr19W<D3pmr1Spec> {
        Mr19W::new(self, 19)
    }
    #[doc = "Bit 20 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr20(&mut self) -> Mr20W<D3pmr1Spec> {
        Mr20W::new(self, 20)
    }
    #[doc = "Bit 21 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr21(&mut self) -> Mr21W<D3pmr1Spec> {
        Mr21W::new(self, 21)
    }
    #[doc = "Bit 25 - Rising trigger event configuration bit of Configurable Event input"]
    #[inline(always)]
    pub fn mr25(&mut self) -> Mr25W<D3pmr1Spec> {
        Mr25W::new(self, 25)
    }
}
#[doc = "EXTI D3 pending mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`d3pmr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d3pmr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct D3pmr1Spec;
impl crate::RegisterSpec for D3pmr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`d3pmr1::R`](R) reader structure"]
impl crate::Readable for D3pmr1Spec {}
#[doc = "`write(|w| ..)` method takes [`d3pmr1::W`](W) writer structure"]
impl crate::Writable for D3pmr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets D3PMR1 to value 0"]
impl crate::Resettable for D3pmr1Spec {}
