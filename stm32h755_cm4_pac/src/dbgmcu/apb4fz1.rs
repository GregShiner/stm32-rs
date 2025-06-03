#[doc = "Register `APB4FZ1` reader"]
pub type R = crate::R<Apb4fz1Spec>;
#[doc = "Register `APB4FZ1` writer"]
pub type W = crate::W<Apb4fz1Spec>;
#[doc = "Field `DBG_I2C4` reader - I2C4 SMBUS timeout stop in debug"]
pub type DbgI2c4R = crate::BitReader;
#[doc = "Field `DBG_I2C4` writer - I2C4 SMBUS timeout stop in debug"]
pub type DbgI2c4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM2` reader - LPTIM2 stop in debug"]
pub type DbgLptim2R = crate::BitReader;
#[doc = "Field `DBG_LPTIM2` writer - LPTIM2 stop in debug"]
pub type DbgLptim2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM3` reader - LPTIM2 stop in debug"]
pub type DbgLptim3R = crate::BitReader;
#[doc = "Field `DBG_LPTIM3` writer - LPTIM2 stop in debug"]
pub type DbgLptim3W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM4` reader - LPTIM4 stop in debug"]
pub type DbgLptim4R = crate::BitReader;
#[doc = "Field `DBG_LPTIM4` writer - LPTIM4 stop in debug"]
pub type DbgLptim4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_LPTIM5` reader - LPTIM5 stop in debug"]
pub type DbgLptim5R = crate::BitReader;
#[doc = "Field `DBG_LPTIM5` writer - LPTIM5 stop in debug"]
pub type DbgLptim5W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_RTC` reader - RTC stop in debug"]
pub type DbgRtcR = crate::BitReader;
#[doc = "Field `DBG_RTC` writer - RTC stop in debug"]
pub type DbgRtcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WDGLSD1` reader - Independent watchdog for D1 stop in debug"]
pub type DbgWdglsd1R = crate::BitReader;
#[doc = "Field `DBG_WDGLSD1` writer - Independent watchdog for D1 stop in debug"]
pub type DbgWdglsd1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBG_WDGLSD2` reader - Independent watchdog for D2 stop in debug"]
pub type DbgWdglsd2R = crate::BitReader;
#[doc = "Field `DBG_WDGLSD2` writer - Independent watchdog for D2 stop in debug"]
pub type DbgWdglsd2W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4(&self) -> DbgI2c4R {
        DbgI2c4R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2(&self) -> DbgLptim2R {
        DbgLptim2R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3(&self) -> DbgLptim3R {
        DbgLptim3R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4(&self) -> DbgLptim4R {
        DbgLptim4R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5(&self) -> DbgLptim5R {
        DbgLptim5R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc(&self) -> DbgRtcR {
        DbgRtcR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd1(&self) -> DbgWdglsd1R {
        DbgWdglsd1R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Independent watchdog for D2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd2(&self) -> DbgWdglsd2R {
        DbgWdglsd2R::new(((self.bits >> 19) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - I2C4 SMBUS timeout stop in debug"]
    #[inline(always)]
    pub fn dbg_i2c4(&mut self) -> DbgI2c4W<Apb4fz1Spec> {
        DbgI2c4W::new(self, 7)
    }
    #[doc = "Bit 9 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim2(&mut self) -> DbgLptim2W<Apb4fz1Spec> {
        DbgLptim2W::new(self, 9)
    }
    #[doc = "Bit 10 - LPTIM2 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim3(&mut self) -> DbgLptim3W<Apb4fz1Spec> {
        DbgLptim3W::new(self, 10)
    }
    #[doc = "Bit 11 - LPTIM4 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim4(&mut self) -> DbgLptim4W<Apb4fz1Spec> {
        DbgLptim4W::new(self, 11)
    }
    #[doc = "Bit 12 - LPTIM5 stop in debug"]
    #[inline(always)]
    pub fn dbg_lptim5(&mut self) -> DbgLptim5W<Apb4fz1Spec> {
        DbgLptim5W::new(self, 12)
    }
    #[doc = "Bit 16 - RTC stop in debug"]
    #[inline(always)]
    pub fn dbg_rtc(&mut self) -> DbgRtcW<Apb4fz1Spec> {
        DbgRtcW::new(self, 16)
    }
    #[doc = "Bit 18 - Independent watchdog for D1 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd1(&mut self) -> DbgWdglsd1W<Apb4fz1Spec> {
        DbgWdglsd1W::new(self, 18)
    }
    #[doc = "Bit 19 - Independent watchdog for D2 stop in debug"]
    #[inline(always)]
    pub fn dbg_wdglsd2(&mut self) -> DbgWdglsd2W<Apb4fz1Spec> {
        DbgWdglsd2W::new(self, 19)
    }
}
#[doc = "DBGMCU APB4 peripheral freeze register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb4fz1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb4fz1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb4fz1Spec;
impl crate::RegisterSpec for Apb4fz1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb4fz1::R`](R) reader structure"]
impl crate::Readable for Apb4fz1Spec {}
#[doc = "`write(|w| ..)` method takes [`apb4fz1::W`](W) writer structure"]
impl crate::Writable for Apb4fz1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB4FZ1 to value 0"]
impl crate::Resettable for Apb4fz1Spec {}
