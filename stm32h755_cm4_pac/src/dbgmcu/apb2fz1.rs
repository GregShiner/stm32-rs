#[doc = "Register `APB2FZ1` reader"]
pub type R = crate::R<Apb2fz1Spec>;
#[doc = "Register `APB2FZ1` writer"]
pub type W = crate::W<Apb2fz1Spec>;
#[doc = "Field `DBG_TIM1` reader - TIM1 stop in debug"]
pub type DbgTim1R = crate::BitReader;
#[doc = "Field `DBG_TIM1` writer - TIM1 stop in debug"]
pub type DbgTim1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM8` reader - TIM8 stop in debug"]
pub type DbgTim8R = crate::BitReader;
#[doc = "Field `DBG_TIM8` writer - TIM8 stop in debug"]
pub type DbgTim8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM15` reader - TIM15 stop in debug"]
pub type DbgTim15R = crate::BitReader;
#[doc = "Field `DBG_TIM15` writer - TIM15 stop in debug"]
pub type DbgTim15W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM16` reader - TIM16 stop in debug"]
pub type DbgTim16R = crate::BitReader;
#[doc = "Field `DBG_TIM16` writer - TIM16 stop in debug"]
pub type DbgTim16W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_TIM17` reader - TIM17 stop in debug"]
pub type DbgTim17R = crate::BitReader;
#[doc = "Field `DBG_TIM17` writer - TIM17 stop in debug"]
pub type DbgTim17W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_HRTIM` reader - HRTIM stop in debug"]
pub type DbgHrtimR = crate::BitReader;
#[doc = "Field `DBG_HRTIM` writer - HRTIM stop in debug"]
pub type DbgHrtimW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&self) -> DbgTim1R {
        DbgTim1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&self) -> DbgTim8R {
        DbgTim8R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&self) -> DbgTim15R {
        DbgTim15R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&self) -> DbgTim16R {
        DbgTim16R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&self) -> DbgTim17R {
        DbgTim17R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&self) -> DbgHrtimR {
        DbgHrtimR::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TIM1 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim1(&mut self) -> DbgTim1W<Apb2fz1Spec> {
        DbgTim1W::new(self, 0)
    }
    #[doc = "Bit 1 - TIM8 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim8(&mut self) -> DbgTim8W<Apb2fz1Spec> {
        DbgTim8W::new(self, 1)
    }
    #[doc = "Bit 16 - TIM15 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim15(&mut self) -> DbgTim15W<Apb2fz1Spec> {
        DbgTim15W::new(self, 16)
    }
    #[doc = "Bit 17 - TIM16 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim16(&mut self) -> DbgTim16W<Apb2fz1Spec> {
        DbgTim16W::new(self, 17)
    }
    #[doc = "Bit 18 - TIM17 stop in debug"]
    #[inline(always)]
    pub fn dbg_tim17(&mut self) -> DbgTim17W<Apb2fz1Spec> {
        DbgTim17W::new(self, 18)
    }
    #[doc = "Bit 29 - HRTIM stop in debug"]
    #[inline(always)]
    pub fn dbg_hrtim(&mut self) -> DbgHrtimW<Apb2fz1Spec> {
        DbgHrtimW::new(self, 29)
    }
}
#[doc = "DBGMCU APB2 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb2fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb2fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb2fz1Spec;
impl crate::RegisterSpec for Apb2fz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb2fz1::R`](R) reader structure"]
impl crate::Readable for Apb2fz1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb2fz1::W`](W) writer structure"]
impl crate::Writable for Apb2fz1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB2FZ1 to value 0"]
impl crate::Resettable for Apb2fz1Spec {}
