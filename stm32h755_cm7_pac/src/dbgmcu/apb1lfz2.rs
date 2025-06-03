#[doc = "Register `APB1LFZ2` reader"]
pub type R = crate::R<Apb1lfz2Spec>;
#[doc = "Register `APB1LFZ2` writer"]
pub type W = crate::W<Apb1lfz2Spec>;
#[doc = "Field `DBG_TIM2` reader - TIM2 stop in debug"]
pub type DbgTim2R = crate::BitReader;
#[doc = "Field `DBG_TIM2` writer - TIM2 stop in debug"]
pub type DbgTim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM3` reader - TIM3 stop in debug"]
pub type DbgTim3R = crate::BitReader;
#[doc = "Field `DBG_TIM3` writer - TIM3 stop in debug"]
pub type DbgTim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM4` reader - TIM4 stop in debug"]
pub type DbgTim4R = crate::BitReader;
#[doc = "Field `DBG_TIM4` writer - TIM4 stop in debug"]
pub type DbgTim4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM5` reader - TIM5 stop in debug"]
pub type DbgTim5R = crate::BitReader;
#[doc = "Field `DBG_TIM5` writer - TIM5 stop in debug"]
pub type DbgTim5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM6` reader - TIM6 stop in debug"]
pub type DbgTim6R = crate::BitReader;
#[doc = "Field `DBG_TIM6` writer - TIM6 stop in debug"]
pub type DbgTim6W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM7` reader - TIM4 stop in debug"]
pub type DbgTim7R = crate::BitReader;
#[doc = "Field `DBG_TIM7` writer - TIM4 stop in debug"]
pub type DbgTim7W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM12` reader - TIM12 stop in debug"]
pub type DbgTim12R = crate::BitReader;
#[doc = "Field `DBG_TIM12` writer - TIM12 stop in debug"]
pub type DbgTim12W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM13` reader - TIM13 stop in debug"]
pub type DbgTim13R = crate::BitReader;
#[doc = "Field `DBG_TIM13` writer - TIM13 stop in debug"]
pub type DbgTim13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM14` reader - TIM14 stop in debug"]
pub type DbgTim14R = crate::BitReader;
#[doc = "Field `DBG_TIM14` writer - TIM14 stop in debug"]
pub type DbgTim14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM1` reader - LPTIM1 stop in debug"]
pub type DbgLptim1R = crate::BitReader;
#[doc = "Field `DBG_LPTIM1` writer - LPTIM1 stop in debug"]
pub type DbgLptim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WWDG2` reader - WWDG2 stop in debug"]
pub type DbgWwdg2R = crate::BitReader;
#[doc = "Field `DBG_WWDG2` writer - WWDG2 stop in debug"]
pub type DbgWwdg2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C1` reader - I2C1 SMBUS timeout stop in debug"]
pub type DbgI2c1R = crate::BitReader;
#[doc = "Field `DBG_I2C1` writer - I2C1 SMBUS timeout stop in debug"]
pub type DbgI2c1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C2` reader - I2C2 SMBUS timeout stop in debug"]
pub type DbgI2c2R = crate::BitReader;
#[doc = "Field `DBG_I2C2` writer - I2C2 SMBUS timeout stop in debug"]
pub type DbgI2c2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_I2C3` reader - I2C3 SMBUS timeout stop in debug"]
pub type DbgI2c3R = crate::BitReader;
#[doc = "Field `DBG_I2C3` writer - I2C3 SMBUS timeout stop in debug"]
pub type DbgI2c3W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&self) -> DbgTim2R {
        DbgTim2R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&self) -> DbgTim3R {
        DbgTim3R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&self) -> DbgTim4R {
        DbgTim4R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&self) -> DbgTim5R {
        DbgTim5R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&self) -> DbgTim6R {
        DbgTim6R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&self) -> DbgTim7R {
        DbgTim7R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&self) -> DbgTim12R {
        DbgTim12R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&self) -> DbgTim13R {
        DbgTim13R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&self) -> DbgTim14R {
        DbgTim14R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&self) -> DbgLptim1R {
        DbgLptim1R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&self) -> DbgWwdg2R {
        DbgWwdg2R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&self) -> DbgI2c1R {
        DbgI2c1R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&self) -> DbgI2c2R {
        DbgI2c2R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&self) -> DbgI2c3R {
        DbgI2c3R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim2(&mut self) -> DbgTim2W<Apb1lfz2Spec> {
        DbgTim2W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM3 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim3(&mut self) -> DbgTim3W<Apb1lfz2Spec> {
        DbgTim3W::new(self, 1)
    }
    #[doc = "Bit 2 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim4(&mut self) -> DbgTim4W<Apb1lfz2Spec> {
        DbgTim4W::new(self, 2)
    }
    #[doc = "Bit 3 - TIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim5(&mut self) -> DbgTim5W<Apb1lfz2Spec> {
        DbgTim5W::new(self, 3)
    }
    #[doc = "Bit 4 - TIM6 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim6(&mut self) -> DbgTim6W<Apb1lfz2Spec> {
        DbgTim6W::new(self, 4)
    }
    #[doc = "Bit 5 - TIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim7(&mut self) -> DbgTim7W<Apb1lfz2Spec> {
        DbgTim7W::new(self, 5)
    }
    #[doc = "Bit 6 - TIM12 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim12(&mut self) -> DbgTim12W<Apb1lfz2Spec> {
        DbgTim12W::new(self, 6)
    }
    #[doc = "Bit 7 - TIM13 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim13(&mut self) -> DbgTim13W<Apb1lfz2Spec> {
        DbgTim13W::new(self, 7)
    }
    #[doc = "Bit 8 - TIM14 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim14(&mut self) -> DbgTim14W<Apb1lfz2Spec> {
        DbgTim14W::new(self, 8)
    }
    #[doc = "Bit 9 - LPTIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim1(&mut self) -> DbgLptim1W<Apb1lfz2Spec> {
        DbgLptim1W::new(self, 9)
    }
    #[doc = "Bit 11 - WWDG2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wwdg2(&mut self) -> DbgWwdg2W<Apb1lfz2Spec> {
        DbgWwdg2W::new(self, 11)
    }
    #[doc = "Bit 21 - I2C1 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c1(&mut self) -> DbgI2c1W<Apb1lfz2Spec> {
        DbgI2c1W::new(self, 21)
    }
    #[doc = "Bit 22 - I2C2 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c2(&mut self) -> DbgI2c2W<Apb1lfz2Spec> {
        DbgI2c2W::new(self, 22)
    }
    #[doc = "Bit 23 - I2C3 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c3(&mut self) -> DbgI2c3W<Apb1lfz2Spec> {
        DbgI2c3W::new(self, 23)
    }
}
#[doc = "DBGMCU APB1L peripheral freeze register CPU2\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1lfz2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1lfz2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1lfz2Spec;
impl crate::RegisterSpec for Apb1lfz2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1lfz2::R`](R) reader structure"]
impl crate::Readable for Apb1lfz2Spec {}
#[doc = "`write(|w| ..)` method takes [`apb1lfz2::W`](W) writer structure"]
impl crate::Writable for Apb1lfz2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1LFZ2 to value 0"]
impl crate::Resettable for Apb1lfz2Spec {}
